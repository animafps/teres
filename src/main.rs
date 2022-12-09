use clap::Parser;
mod config;
mod helpers;
mod rendering;
mod script_handler;
mod teres;
mod cli;
use human_panic::setup_panic;

#[macro_use]
extern crate log;

#[cfg(windows)]
fn main() {
    setup_panic!();
    let args = cli::Cli::parse_from(wild::args());
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    winconsole::console::set_title("Teres").unwrap();

    teres::run(args);

    helpers::exit(exitcode::OK);
}

#[cfg(unix)]
fn main() {
    setup_panic!();
    let args = cli::Cli::parse_from(wild::args());
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    teres::run(args);

    helpers::exit(exitcode::OK);
}