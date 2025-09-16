use std::path::Path;

use arcstr::literal;
use indexmap::IndexMap;
use par_syntax::ast::{Definition, Ident, Label, LocalIdent, ProcessExpr, ProcessStep, Program};
use proc_macro2::TokenStream;
use quote::quote;
use tokio::fs::{create_dir, write};

pub async fn transpile_to_rust(program: Program, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    let out_dir = out_dir.as_ref().to_path_buf();
    let mut cargo_toml_path = out_dir.clone();
    cargo_toml_path.push("Cargo.toml");
    write(cargo_toml_path, CARGO_TOML).await?;
    let mut src_path = out_dir.clone();
    src_path.push("src");
    create_dir(&src_path).await?;

    let code = gen_program(&program)?.to_string();
    let code = syn::parse_file(&code)?;
    let code = prettyplease::unparse(&code);

    let mut main_path = src_path.clone();
    main_path.push("main.rs");
    write(&main_path, code).await?;

    Ok(())
}

const CARGO_TOML: &'static str = r#"[package]
name = "tmp"
version = "0.0.1"
edition = "2024"

[workspace]

[dependencies]
par-run = { version = "*", path = "../../../libs/par-run" }

anyhow = { version = "*" }
arcstr = { version = "*" }
tokio = { version = "*", features = ["full"] }
"#;

fn gen_program(program: &Program) -> anyhow::Result<TokenStream> {
    let mut defs = Vec::new();

    for def in &program.defs {
        defs.push(gen_definition(def)?);
    }

    let result = quote! {
        use arcstr::literal;
        use par_run::operations::*;

        #(#defs)*

        #[tokio::main]
        async fn main() {
            let (Main_s, Main_r) = Main();
            Main_r.await.recv().await.unwrap();
        }
    };
    Ok(result)
}

fn gen_definition(definition: &Definition) -> anyhow::Result<TokenStream> {
    let result = match definition {
        Definition::Native { name, .. } => match name.content.as_ref() {
            "Println" => {
                quote! {
                    fn Println() -> (Sender, Receiver) {
                        chan((), async move |_, r, ()| {
                            let r = r.await;
                            let x = r.recv().await.unwrap();
                            println!("{:?}", x);
                        })
                    }
                }
            },
            _ => unreachable!(),
        },
        Definition::ProcessExpr { name, expr, .. } => {
            let name = syn::parse_str::<syn::Ident>(name.content.as_str())?;
            let expr = gen_expr(expr)?;
            quote! {
                fn #name() -> (Sender, Receiver) {
                    #expr
                }
            }
        }
    };
    Ok(result)
}

fn gen_expr(expr: &ProcessExpr) -> anyhow::Result<TokenStream> {
    let result = match expr {
        ProcessExpr::Chan { name, steps, .. } => {
            let (name_s, name_r) = make_sender_receiver(name)?;
            let (steps_tokens, defer_tokens) = make_steps(steps)?;
            quote! {
                chan((), async move |#name_s, #name_r, _| {
                    #(#steps_tokens)*
                    #(#defer_tokens)*
                })
            }
        }
        ProcessExpr::Str { value, .. } => {
            let value = value.as_str();
            quote! {
                value(Value::Str {x: literal!(#value)})
            }
        }
        ProcessExpr::I32 { value, .. } => quote! {
            value(Value::I32 {x: #value})
        },
        ProcessExpr::U64 { value, .. } => quote! {
            value(Value::U64 {x: #value})
        },
        ProcessExpr::Ident { value, .. } => match value {
            Ident::Local(x) => {
                let value = syn::parse_str::<syn::Ident>(x.content.as_str())?;
                quote! {#value}
            }
            Ident::Global(x) => {
                let value = syn::parse_str::<syn::Ident>(x.content.as_str())?;
                quote! {#value()}
            }
        },
        ProcessExpr::Unit { .. } => quote! {
            value(Value::Unit)
        },
    };
    Ok(result)
}

fn gen_step(
    step: &ProcessStep,
    defer: &mut IndexMap<String, TokenStream>,
) -> anyhow::Result<TokenStream> {
    let result = match step {
        ProcessStep::Let { name, expr, .. } => {
            let (name_s, name_r) = make_sender_receiver(name)?;
            defer.insert(
                name_r.to_string(),
                quote! {
                    drain(#name_r).await;
                },
            );
            
            match expr {
                ProcessExpr::Ident { value: Ident::Local(x), .. } => {
                    let (x_s, x_r) = make_sender_receiver(x)?;
                    quote! {
                        let (#name_s, #name_r) = (#x_s, #x_r);
                    }
                },
                _ => {
                    let expr = gen_expr(expr)?;
                    quote! {
                        let (#name_s, #name_r) = #expr;
                    }
                }
            }
        }
        ProcessStep::Link { target, expr, .. } => {
            let (target_s, _) = make_sender_receiver(target)?;
            match expr {
                ProcessExpr::Ident { value: Ident::Local(x), .. } => {
                    let (_, x_r) = make_sender_receiver(x)?;
                    defer.shift_remove(&x_r.to_string());
                    quote! {
                        link(#target_s, #x_r);
                    }
                },
                _ => {
                    let expr = gen_expr(expr)?;
                    quote! {
                        let (__s, __r) = #expr;
                        link(#target_s, __r);
                    }
                }
            }
        }
        ProcessStep::Begin {
            target,
            label,
            steps,
            ..
        } => {
            let (_, target_r) = make_sender_receiver(target)?;
            let (loop_s, _) = make_loop_label(label, target.r)?;
            let (steps_tokens, defer_tokens) = make_steps(steps)?;
            defer.insert(target_r.to_string(), quote! {
                drain(#target_r).await;
            });
            quote! {
                let #target_r = begin(#target_r, (), async move |#loop_s, #target_r, _| {
                    #(#steps_tokens)*
                    #(#defer_tokens)*
                    #target_r
                }).await;
            }
        }
        ProcessStep::Case { target, alts, .. } => {
            let (_, target_r) = make_sender_receiver(target)?;
            let mut alts_tokens = Vec::new();
            for alt in alts {
                let name = alt.name.content.to_string();
                let (steps_tokens, defer_tokens) = make_steps(&alt.inner)?;
                alts_tokens.push(quote! {
                    #name => {
                        #(#steps_tokens)*
                        #(#defer_tokens)*
                        #target_r
                    }
                });
            }
            quote! {
                let #target_r = case(#target_r, (), async move |label, #target_r, _| {
                    match label.as_str() {
                        #(#alts_tokens),*
                        _ => unreachable!()
                    }
                }).await;
            }
        }
        ProcessStep::Recv { target, name, .. } => {
            let (_, target_r) = make_sender_receiver(target)?;
            let (name_s, name_r) = make_sender_receiver(name)?;
            defer.insert(
                name_r.to_string(),
                quote! {
                    drop(#name_s.await);
                    drain(#name_r).await;
                },
            );
            quote! {
                let (#target_r, #name_s, #name_r) = recv(#target_r);
            }
        }
        ProcessStep::Send { target, expr, .. } => {
            let (target_s, _) = make_sender_receiver(target)?;
            match expr {
                ProcessExpr::Ident {
                    value: Ident::Local(x),
                    ..
                } => {
                    let (_, x_r) = make_sender_receiver(x)?;
                    quote! {
                        let (#target_s, #x_r) = send_one(#target_s, #x_r).await;
                    }
                }
                _ => {
                    let expr = gen_expr(expr)?;
                    quote! {
                        let (__s, __r) = #expr;
                        let (#target_s, __r) = send_one(#target_s, __r).await;
                    }
                }
            }
        }
        ProcessStep::Signal { target, signal, .. } => {
            let (target_s, _) = make_sender_receiver(target)?;
            let signal = signal.content.as_str();
            quote! {
                let #target_s = send_value(#target_s, Value::label(#signal)).await;
            }
        }
        ProcessStep::Break { label, l: _, r } => {
            let (loop_s, _) = make_loop_label(label, *r)?;
            quote! {
                #loop_s.send(None).unwrap();
            }
        }
        ProcessStep::Loop { label, l: _, r } => {
            let (loop_s, _) = make_loop_label(label, *r)?;
            quote! {
                #loop_s.send(Some(())).unwrap();
            }
        }
    };
    Ok(result)
}

fn make_steps(steps: &Vec<ProcessStep>) -> anyhow::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    let mut defer = IndexMap::new();
    let mut steps_tokens = Vec::new();
    for step in steps {
        let tokens = gen_step(step, &mut defer)?;
        steps_tokens.push(tokens);
    }
    let mut defer_tokens = Vec::new();
    for (_, tokens) in defer {
        defer_tokens.push(tokens);
    }
    defer_tokens.reverse();
    Ok((steps_tokens, defer_tokens))
}

fn make_sender_receiver(name: &LocalIdent) -> anyhow::Result<(syn::Ident, syn::Ident)> {
    let name_s = name.content.to_string() + "_s";
    let name_r = name.content.to_string() + "_r";
    let name_s = syn::parse_str::<syn::Ident>(&name_s)?;
    let name_r = syn::parse_str::<syn::Ident>(&name_r)?;
    Ok((name_s, name_r))
}

fn make_loop_label(label: &Option<Label>, i: usize) -> anyhow::Result<(syn::Ident, syn::Ident)> {
    match label {
        Some(label) => make_sender_receiver(&label.name),
        None => make_sender_receiver(&LocalIdent {
            content: literal!("loop"),
            l: i,
            r: i + "loop".len(),
        }),
    }
}
