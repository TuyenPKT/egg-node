use std::collections::{HashMap, VecDeque};
use std::time::{Instant, Duration};

use crate::chain::tx::Transaction;
use crate::chain::txid::txid;
use crate::chain::utxo::UTXO;

const MAX_ORPHAN_TX: usize = 10_000;
const ORPHAN_TTL: Duration = Duration::from_secs(300);

#[derive(Clone)]
pub struct OrphanTx {
    pub tx: Transaction,
    pub added: Instant,
}

pub struct OrphanTxPool {
    pub txs: HashMap<[u8; 32], OrphanTx>,
    pub order: VecDeque<[u8; 32]>,
}

impl OrphanTxPool {
    pub fn new() -> Self {
        Self {
            txs: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn add(&mut self, tx: Transaction) {
        let id = txid(&tx);
        if self.txs.contains_key(&id) {
            return;
        }

        if self.txs.len() >= MAX_ORPHAN_TX {
            if let Some(old) = self.order.pop_front() {
                self.txs.remove(&old);
            }
        }

        self.order.push_back(id);
        self.txs.insert(id, OrphanTx {
            tx,
            added: Instant::now(),
        });
    }

    pub fn try_promote<F>(
        &mut self,
        utxos: &HashMap<([u8; 32], u32), UTXO>,
        mut accept: F,
    )
    where
        F: FnMut(Transaction),
    {
        let now = Instant::now();
        let mut promoted = Vec::new();

        for (id, orphan) in &self.txs {
            if now.duration_since(orphan.added) > ORPHAN_TTL {
                promoted.push(*id);
                continue;
            }

            let mut ok = true;
            for inp in &orphan.tx.inputs {
                if !utxos.contains_key(&(inp.prev_txid, inp.vout)) {
                    ok = false;
                    break;
                }
            }

            if ok {
                accept(orphan.tx.clone());
                promoted.push(*id);
            }
        }

        for id in promoted {
            self.txs.remove(&id);
            self.order.retain(|x| x != &id);
        }
    }
}
