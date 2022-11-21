use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Arguments {
    /// Assembly file to be executed.
    #[arg(value_hint = ValueHint::FilePath)]
    pub file_path: PathBuf,
}
