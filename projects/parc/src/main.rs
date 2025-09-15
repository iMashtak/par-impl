use std::sync::Arc;

use anyhow::anyhow;
use clap::Parser;
use par_syntax::highlighter;
use tokio::fs::{create_dir, remove_dir_all, try_exists};

use crate::{cli::Cli, generator::transpile_to_rust};

mod cli;
mod generator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_miette()?;
    let cli = Cli::parse();
    let mut input_file = cli.input_dir.clone();
    input_file.push("src/Main.par");
    let content = tokio::fs::read_to_string(&input_file).await?;
    let content = Arc::new(content);
    let ast =
        par_syntax::parse(input_file.to_str().unwrap(), content).map_err(|x| anyhow!("{:?}", x))?;
    let mut out_dir = cli.input_dir.clone();
    out_dir.push("target");
    if try_exists(&out_dir).await? {
        remove_dir_all(&out_dir).await?;
    }
    create_dir(&out_dir).await?;
    transpile_to_rust(ast, &out_dir).await?;
    Ok(())
}

fn setup_miette() -> anyhow::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .with_syntax_highlighting(highlighter())
                .build(),
        )
    }))?;
    Ok(())
}
