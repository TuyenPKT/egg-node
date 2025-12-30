use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    pub version: u32,
    pub prev_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
}
