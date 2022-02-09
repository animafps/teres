use clap::Parser;
mod blur;
mod config;
mod helpers;
mod rendering;
mod script_handler;
use human_panic::setup_panic;

/// Add motion blur to videos
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file name(s)
    input: Option<String>,
    /// Output file name(s)
    #[clap(short, long)]
    output: Option<String>,
    /// Disable user interface
    #[clap(short, long)]
    noui: bool,
}

fn main() {
    setup_panic!();

    let args = Cli::parse();
    blur::run(args);

    helpers::exit(0);
}
