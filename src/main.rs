use clap::Parser;
mod config;
mod helpers;
mod rendering;
mod script_handler;
mod teres;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use human_panic::setup_panic;
use log::{Metadata, Record, SetLoggerError};

/// Add motion blur to videos
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input file name(s) (comma separated)
    input: Option<String>,
    /// Disable user interface (CLI only)
    #[clap(short, long)]
    noui: bool,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[cfg(windows)]
fn main() {
    setup_panic!();
    let args = Cli::parse();
    setup_logger(args.verbose.log_level_filter()).unwrap();
    winconsole::console::set_title("Teres").unwrap();

    teres::run(args);

    helpers::exit(exitcode::OK);
}

#[cfg(unix)]
fn main() {
    setup_panic!();
    let args = Cli::parse();
    setup_logger();

    teres::run(args);

    helpers::exit(exitcode::OK);
}

static LOGGER: Logger = Logger;

pub fn setup_logger(log_level: log::LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level))
}
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("{}", record.args());
    }

    fn flush(&self) {}
}
