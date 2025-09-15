use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    #[arg(help = "Path to package root directory")]
    pub input_dir: PathBuf,
}