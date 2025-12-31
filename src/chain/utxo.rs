use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UTXO {
    pub txid: [u8; 32],
    pub vout: u32,
    pub value: u64,
    pub address: Vec<u8>,
    pub height: u64,
}
