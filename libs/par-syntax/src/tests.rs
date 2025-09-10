use lalrpop_util::{ErrorRecovery, ParseError};
use miette::{bail, miette, LabeledSpan, NamedSource};

use crate::{
    lexer::Lexer,
    parser::ProgramParser,
    tokens::{LexicalError, Token},
};

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
            .empty => .end!,
            .insert => [x, set] '(.item(x)) set,
            .contains => [y, set] set.begin.case {
                .end: ! => .false!,
                .item: (x) xs => eq(x, y).case {
                    .true: ! => .true!,
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
            .next  => '(7) loop,
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

fn run(test_name: &'static str, input: &'static str) -> miette::Result<()> {
    let source = NamedSource::new(test_name, input);
    let lexer = Lexer::new(input);
    let mut errors: Vec<ErrorRecovery<usize, Token, LexicalError>> = Vec::new();
    let parser = ProgramParser::new();
    let parsed = parser.parse(&mut errors, lexer);
    for error in &errors {
        println!("recovery: {:?}", resolve_parse_error(source.clone(), error.error.clone()).unwrap_err());
    }
    match parsed {
        Ok(ast) => {
            println!("{:?}", ast);
            if errors.len() > 0 {
                bail!("there are errors above");
            }
            Ok(())
        }
        Err(err) => resolve_parse_error(source, err),
    }
}

fn resolve_parse_error(
    source: NamedSource<&'static str>,
    error: ParseError<usize, Token, LexicalError>,
) -> miette::Result<()> {
    match error {
        ParseError::InvalidToken { location } => {
            let report = miette!(
                labels = vec![LabeledSpan::at((location - 1, 1usize), "here")],
                "invalid token"
            )
            .with_source_code(source);
            return Err(report)?;
        }
        ParseError::UnrecognizedEof { location, expected } => {
            let report = miette!(
                labels = vec![LabeledSpan::at((location - 1, 1usize), "here")],
                "unrecognized end of file, expected one of: {}",
                expected.join(", ")
            )
            .with_source_code(source);
            return Err(report)?;
        }
        ParseError::UnrecognizedToken {
            token: (l, token, r),
            expected,
        } => {
            let report = miette!(
                labels = vec![LabeledSpan::at((l, r - l), "here")],
                "unrecognized token: {}, expected one of: {}",
                token,
                expected.join(", ")
            )
            .with_source_code(source);
            return Err(report)?;
        }
        ParseError::ExtraToken {
            token: (l, token, r),
        } => {
            let report = miette!(
                labels = vec![LabeledSpan::at((l, r - l), "here")],
                "extra token: {}",
                token
            )
            .with_source_code(source);
            return Err(report)?;
        }
        ParseError::User { error } => {
            let report = miette!("{:?}", error).with_source_code(source);
            return Err(report)?;
        }
    }
}
