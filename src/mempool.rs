use std::collections::HashMap;
use crate::chain::tx::Transaction;
use crate::chain::txid::txid;
use crate::chain::utxo::UTXO;

#[derive(Clone)]
pub struct MempoolEntry {
    pub tx: Transaction,
    pub fee: u64,
    pub fee_rate: u64, // fee / vbytes (xấp xỉ)
}

pub struct Mempool {
    pub map: HashMap<[u8; 32], MempoolEntry>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool { map: HashMap::new() }
    }

    /// Verify inputs exist (caller đảm bảo chữ ký đã verify)
    pub fn add(&mut self, tx: Transaction, utxos: &HashMap<([u8;32],u32),UTXO>) -> bool {
        let id = txid(&tx);
        if self.map.contains_key(&id) { return false; }

        // coinbase không vào mempool
        if tx.inputs.len() == 1 && tx.inputs[0].prev_txid == [0u8;32] {
            return false;
        }

        // tính input sum
        let mut in_sum: u64 = 0;
        for i in &tx.inputs {
            if let Some(u) = utxos.get(&(i.prev_txid, i.vout)) {
                in_sum = in_sum.saturating_add(u.value);
            } else {
                return false;
            }
        }

        let out_sum: u64 = tx.outputs.iter().map(|o| o.value).sum();
        if in_sum < out_sum { return false; }

        let fee = in_sum - out_sum;

        // fee rate xấp xỉ theo kích thước serialized
        let vbytes = bincode::serialize(&tx).unwrap().len().max(1) as u64;
        let fee_rate = fee / vbytes;

        self.map.insert(id, MempoolEntry { tx, fee, fee_rate });
        true
    }

    pub fn remove(&mut self, tx: &Transaction) {
        let id = txid(tx);
        self.map.remove(&id);
    }

    /// Lấy tx theo fee-rate giảm dần, giới hạn số lượng
    pub fn pick_for_mining(&self, max_txs: usize) -> Vec<MempoolEntry> {
        let mut v: Vec<MempoolEntry> = self.map.values().cloned().collect();
        v.sort_by(|a,b| b.fee_rate.cmp(&a.fee_rate));
        v.truncate(max_txs);
        v
    }
}
