mod cli;
mod commands;
mod storage;
mod todo;

use clap::Parser;
use cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Err(error) = commands::execute(cli.command, &cli.file) {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}
