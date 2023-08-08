use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, help = "Path to games drive (may improve performance).")]
    path: Option<PathBuf>
}

pub fn parse() -> Cli {
    Cli::parse()
}

