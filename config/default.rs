use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub network: String,
    pub datadir: String,
    pub p2p_port: u16,
    pub rpc_port: u16,
    pub role: String,
}
