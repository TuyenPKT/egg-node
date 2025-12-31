use clap::Parser;
use egg_node::cli::Cli;

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    cli.execute();
}
