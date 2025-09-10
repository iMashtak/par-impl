use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[arg(help = "Path to root input file")]
    pub input: PathBuf,
}