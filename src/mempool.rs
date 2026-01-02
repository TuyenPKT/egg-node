use std::collections::HashMap;

use crate::chain::tx::Transaction;
use crate::chain::txid::txid;
use crate::chain::utxo::UTXO;

#[derive(Clone)]
pub struct MempoolTx {
    pub tx: Transaction,
    pub fee: u64,
}

pub struct Mempool {
    pub txs: HashMap<[u8; 32], MempoolTx>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            txs: HashMap::new(),
        }
    }

    /// Add transaction to mempool after computing fee
    pub fn add(
        &mut self,
        tx: Transaction,
        utxos: &HashMap<([u8; 32], u32), UTXO>,
    ) -> bool {
        let id = txid(&tx);
        if self.txs.contains_key(&id) {
            return false;
        }

        let fee = match calc_fee(&tx, utxos) {
            Some(f) => f,
            None => return false,
        };

        self.txs.insert(id, MempoolTx { tx, fee });
        true
    }

    /// Remove tx after it is mined
    pub fn remove(&mut self, tx: &Transaction) {
        let id = txid(tx);
        self.txs.remove(&id);
    }

    /// Used by fee estimator
    pub fn fee_list(&self) -> Vec<u64> {
        self.txs.values().map(|m| m.fee).collect()
    }

    /// Used by miner: select txs sorted by fee (desc)
    pub fn select_for_block(&self, max: usize) -> Vec<Transaction> {
        let mut list: Vec<MempoolTx> = self.txs.values().cloned().collect();

        // sort by fee descending
        list.sort_by(|a, b| b.fee.cmp(&a.fee));

        list.into_iter()
            .take(max)
            .map(|m| m.tx)
            .collect()
    }
}

fn calc_fee(
    tx: &Transaction,
    utxos: &HashMap<([u8; 32], u32), UTXO>,
) -> Option<u64> {
    let mut input_sum = 0u64;

    for inp in &tx.inputs {
        let key = (inp.prev_txid, inp.vout);
        let utxo = utxos.get(&key)?;
        input_sum += utxo.value;
    }

    let output_sum: u64 = tx.outputs.iter().map(|o| o.value).sum();

    if input_sum < output_sum {
        None
    } else {
        Some(input_sum - output_sum)
    }
}
