use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Handshake {
    pub protocol_version: u32,
    pub genesis_hash: [u8; 32],
    pub node_id: [u8; 32],
}
