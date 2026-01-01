use std::collections::HashMap;
use crate::chain::tx::Transaction;
use crate::chain::txid::txid;

pub struct Mempool {
    pub txs: HashMap<[u8; 32], Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            txs: HashMap::new(),
        }
    }

    pub fn add(&mut self, tx: Transaction) -> bool {
        let id = txid(&tx);
        if self.txs.contains_key(&id) {
            return false;
        }
        self.txs.insert(id, tx);
        true
    }

    pub fn remove(&mut self, tx: &Transaction) {
        let id = txid(tx);
        self.txs.remove(&id);
    }

    pub fn all(&self) -> Vec<Transaction> {
        self.txs.values().cloned().collect()
    }
}
