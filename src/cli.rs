use clap::{Parser, Subcommand};
use crate::node::run_node;

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
                run_node();
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