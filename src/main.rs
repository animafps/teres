use clap::Parser;
mod config;
mod helpers;
mod rendering;
mod script_handler;
mod teres;
use human_panic::setup_panic;

/// Add motion blur to videos
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file name(s) (comma separated)
    input: Option<String>,
    /// Disable user interface (CLI only)
    #[clap(short, long)]
    noui: bool,
}

#[cfg(windows)]
fn main() {
    setup_panic!();
    let args = Cli::parse();
    winconsole::console::set_title("Teres").unwrap();

    teres::run(args);

    helpers::exit(exitcode::OK);
}

#[cfg(unix)]
fn main() {
    setup_panic!();
    let args = Cli::parse();

    teres::run(args);

    helpers::exit(exitcode::OK);
}
