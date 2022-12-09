use clap_verbosity_flag::Verbosity;
use clap::Parser;

/// Add motion blur to videos
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file name(s) (space separated)
    pub input: Vec<String>,
    /// Disable user interface (CLI only)
    #[clap(short, long)]
    pub noui: bool,

    #[clap(flatten)]
    pub verbose: Verbosity,
}