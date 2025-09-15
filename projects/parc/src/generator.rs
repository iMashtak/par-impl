use std::path::Path;

use par_syntax::ast::Program;
use tokio::fs::{create_dir, write};

pub async fn transpile_to_rust(program: Program, out_dir: impl AsRef<Path>) -> anyhow::Result<()> {
    println!("{:?}", program);
    let out_dir = out_dir.as_ref().to_path_buf();
    let mut cargo_toml_path = out_dir.clone();
    cargo_toml_path.push("Cargo.toml");
    write(cargo_toml_path, CARGO_TOML).await?;
    let mut src_path = out_dir.clone();
    src_path.push("src");
    create_dir(&src_path).await?;
    Ok(())
}

const CARGO_TOML: &'static str = r#"[package]
name = "tmp"
version = "0.0.1"
edition = "2024"

[dependencies]
par-run = { version = "*", path = "../../../libs/par-run" }

anyhow = { version = "*" }
tokio = { version = "*", features = ["full"] }
"#;

