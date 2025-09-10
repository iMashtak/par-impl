use std::sync::Arc;

use clap::Parser;

use crate::{cli::Cli};

mod cli;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    let content = tokio::fs::read_to_string(&cli.input).await.unwrap();
    let content = Arc::new(content);
    let ast = par_syntax::parse(cli.input.to_str().unwrap(), content)?;
    println!("{:?}", ast);
    Ok(())
}