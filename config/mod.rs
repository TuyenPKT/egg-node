use anyhow::Result;
use crate::cli::Cli;

mod default;
pub use default::Config;

pub fn load(cli: &Cli) -> Result<Config> {
    match &cli.command {
        crate::cli::Command::Start {
            network,
            datadir,
            p2p_port,
            rpc_port,
            role,
        } => Ok(Config {
            network: network.clone(),
            datadir: datadir.clone(),
            p2p_port: *p2p_port,
            rpc_port: *rpc_port,
            role: role.clone(),
        }),
    }
}
