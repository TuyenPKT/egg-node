use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn genesis(message: &str) -> Self {
        Transaction {
            data: message.as_bytes().to_vec(),
        }
    }
}
