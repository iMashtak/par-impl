use std::sync::Arc;

use crate::parse;

#[test]
fn test_seq() -> miette::Result<()> {
    run(
        "test_seq",
        "
        type Seq = <a> recursive either {
            .end: !,
            .item: [a] self,
        }
    ",
    )
}

#[test]
fn test_queue() -> miette::Result<()> {
    run(
        "test_queue",
        "
        type Queue = <a>
            iterative@push recursive@pop
            choice {
                .push: [a] self@push,
                .pop: either {
                    .end: !,
                    .item: [a] self@pop,
                }
            }
    ",
    )
}

#[test]
fn test_list_set() -> miette::Result<()> {
    run(
        "test_list_set",
        r#"
        def ListSet: <a> [Eq<box a>] {SetModule<box a>} = <a> [eq] {List<box a>} box case {
            .empty => .end <- !,
            .insert => [x, set] .item(x) <- set,
            .contains => [y, set] set.begin.case {
                .end: ! => .false <- !,
                .item: (x) xs => eq(x, y).case {
                    .true: ! => .true <- !,
                    .false: ! => xs.loop,
                } 
            }
        }
    "#,
    )
}

#[test]
fn test_seven_forever() -> miette::Result<()> {
    run(
        "test_seven_forever",
        r#"
        def SevenForever = begin case {
            .close => !,
            .next => .(7) loop,
        }
    "#,
    )
}

#[test]
fn test_complex_process() -> miette::Result<()> {
    run(
        "test_complex_process",
        r#"
        def Main: ! = do {
            let seq = StaticSeq::<String>.builder.add("x").add("y").build + 8 < 10;
            let seq2 = StaticSeq::<String>.builder.add("a").add("b").build;
            let seq3 = SeqMethods::<String>(seq).concat(seq2);
            let console = ConsoleOpen;
            seq3.begin.case {
                .end: ! => {},
                .item: (x) xs => {
                    console.print(x);
                    xs.loop;
                }
            };
            console.close;
        } in !
    "#,
    )
}

#[test]
fn test_reverse() -> miette::Result<()> {
    run(
        "test_reverse",
        r#"
        def Reverse: <a> [List<a>] List<a> = <a> [list]
            let acc: List<a> = .end <- !
            in list.begin.case {
                .end: ! => acc,
                .item: (x) xs => let acc = .item(x) <- acc in xs.loop,
            }
    "#,
    )
}

#[test]
fn test_global_assoc() -> miette::Result<()> {
    run(
        "test_global_assoc",
        r#"
        def Module = case {
            .func  => Func,
        }
    "#,
    )
}

fn run(test_name: &'static str, input: &'static str) -> miette::Result<()> {
    parse(test_name, Arc::new(input.to_string()))?;
    Ok(())
}
