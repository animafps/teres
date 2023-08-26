use clap::Parser;
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;

/// Add motion blur to videos
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file name(s) (space separated)
    #[arg(value_hint=clap::ValueHint::FilePath)]
    pub input: Vec<PathBuf>,

    /// Disable user interface (CLI only)
    #[arg(short, long)]
    pub noui: bool,

    #[command(flatten)]
    pub verbose: Verbosity,

    /// Writes the output to standard output (STDOUT) (disables output to file)
    #[arg(long)]
    pub stdout: bool,
}
