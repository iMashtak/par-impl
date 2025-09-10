use std::sync::Arc;

use clap::Parser as _;

use crate::cli::Cli;

mod cli;
mod fmt;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let content = tokio::fs::read_to_string(&cli.input).await.unwrap();
    let content = Arc::new(content);
    let ast = par_syntax::parse(cli.input.to_str().unwrap(), content)?;
    let formatted = fmt::format(&ast);
    tokio::fs::write(&cli.input, formatted).await.unwrap();
    Ok(())
}
