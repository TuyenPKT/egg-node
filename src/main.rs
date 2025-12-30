mod chain;
mod node;
mod cli;
mod config;
mod p2p;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    cli.execute();
}
