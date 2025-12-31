use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TxInput {
    pub prev_tx: Option<[u8; 32]>, // None = coinbase
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxOutput {
    pub value: u64,
    pub to_address: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn coinbase(to: Vec<u8>, reward: u64, message: &str) -> Self {
        Transaction {
            inputs: vec![TxInput { prev_tx: None }],
            outputs: vec![TxOutput {
                value: reward,
                to_address: to,
            }],
            data: message.as_bytes().to_vec(),
        }
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].prev_tx.is_none()
    }
}
