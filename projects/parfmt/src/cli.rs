use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[arg(help = "Path to input file")]
    pub input: PathBuf,
}