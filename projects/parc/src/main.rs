use clap::Parser;
use lalrpop_util::{ErrorRecovery, ParseError};
use miette::{bail, miette, LabeledSpan, NamedSource};
use par_syntax::{lexer::Lexer, parser::ProgramParser, tokens::{LexicalError, Token}};

use crate::{cli::Cli};

mod cli;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let content = tokio::fs::read_to_string(&cli.input).await.unwrap();
    run(cli.input.to_str().unwrap(), &content)?;
    Ok(())
}

fn run(file_name: &str, input: &str) -> miette::Result<()> {
    let source = NamedSource::new(file_name, input.as_bytes().to_vec());
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
    source: NamedSource<Vec<u8>>,
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
