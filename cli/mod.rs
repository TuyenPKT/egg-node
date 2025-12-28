use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "egg-node")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Start {
        #[arg(long, default_value = "mainnet")]
        network: String,

        #[arg(long, default_value = "./data")]
        datadir: String,

        #[arg(long, default_value_t = 9333)]
        p2p_port: u16,

        #[arg(long, default_value_t = 8332)]
        rpc_port: u16,

        #[arg(long, default_value = "full")]
        role: String,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
