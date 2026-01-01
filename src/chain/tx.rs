use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxInput {
    pub prev_txid: [u8; 32],
    pub vout: u32,
    pub signature: Vec<u8>,
    pub pubkey: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxOutput {
    pub value: u64,
    pub to_address: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub data: Vec<u8>,
}


impl Transaction {
    pub fn coinbase(to: Vec<u8>, reward: u64, message: &str) -> Self {
        Transaction {
            inputs: vec![TxInput {
                prev_txid: [0u8; 32], // coinbase marker
                vout: 0,
                signature: vec![],
                pubkey: vec![],
            }],
            outputs: vec![TxOutput {
                value: reward,
                to_address: to,
            }],
            data: message.as_bytes().to_vec(),
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].prev_txid == [0u8; 32]
    }
}

