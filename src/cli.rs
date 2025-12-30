use clap::{Parser, Subcommand};
use crate::node::start_node;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run,
    Mine,
    Status,
}

impl Cli {
    pub fn execute(&self) {
        match &self.command {
            Commands::Init => {
                println!("Initializing node...");
            }
            Commands::Run => {
                start_node();
            }
            Commands::Mine => {
                println!("Mining...");
            }
            Commands::Status => {
                println!("Node status");
            }
        }
    }
}