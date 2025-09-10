use std::sync::Arc;

use lalrpop_util::{ErrorRecovery, ParseError};
use miette::{miette, LabeledSpan, NamedSource};

use crate::{
    ast::Program,
    lexer::Lexer,
    parser::ProgramParser,
    tokens::{LexicalError, Token},
};

pub mod ast;
mod lexer;
mod parser;
mod tokens;

#[cfg(test)]
mod tests;

pub fn parse(source: impl AsRef<str>, input: Arc<String>) -> miette::Result<Program> {
    let source_input = Arc::clone(&input);
    let source = NamedSource::new(source, source_input);
    let lexer = Lexer::new(&input);
    let mut errors: Vec<ErrorRecovery<usize, Token, LexicalError>> = Vec::new();
    let parser = ProgramParser::new();
    let parsed = parser.parse(&mut errors, lexer);
    for error in &errors {
        println!(
            "{:?}",
            resolve_parse_error(source.clone(), error.error.clone())
        );
    }
    match parsed {
        Ok(ast) => {
            if errors.len() > 0 {
                miette::bail!("there are errors above");
            }
            Ok(ast)
        }
        Err(err) => Err(resolve_parse_error(source, err)),
    }
}

fn resolve_parse_error(
    source: NamedSource<Arc<String>>,
    error: ParseError<usize, Token, LexicalError>,
) -> miette::Error {
    match error {
        ParseError::InvalidToken { location } => {
            return miette!(
                labels = vec![LabeledSpan::at((location - 1, 1usize), "here")],
                "invalid token"
            )
            .with_source_code(source);
        }
        ParseError::UnrecognizedEof { location, expected } => {
            return miette!(
                labels = vec![LabeledSpan::at((location - 1, 1usize), "here")],
                "unrecognized end of file, expected one of: {}",
                expected.join(", ")
            )
            .with_source_code(source);
        }
        ParseError::UnrecognizedToken {
            token: (l, token, r),
            expected,
        } => {
            return miette!(
                labels = vec![LabeledSpan::at((l, r - l), "here")],
                "unrecognized token: {}, expected one of: {}",
                token,
                expected.join(", ")
            )
            .with_source_code(source);
        }
        ParseError::ExtraToken {
            token: (l, token, r),
        } => {
            return miette!(
                labels = vec![LabeledSpan::at((l, r - l), "here")],
                "extra token: {}",
                token
            )
            .with_source_code(source);
        }
        ParseError::User { error } => {
            return miette!("{:?}", error).with_source_code(source);
        }
    }
}
