mod config;
mod node;
mod cli;
mod chain;
mod p2p;
mod storage;




use crate::cli::Cli;
use clap::Parser;

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    cli.execute();
}
