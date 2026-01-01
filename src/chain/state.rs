use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::chain::utxo::UTXO;
use crate::chain::txid::txid;
use crate::storage::sleddb::ChainDB;
use crate::pow::verify::verify_pow;
use crate::pow::work::work_from_bits;
use crate::chain::sign::verify_tx;

#[derive(Clone)]
pub struct BlockMeta {
    pub block: Block,
    pub parent: [u8; 32],
    pub height: u64,
    pub total_work: u128,
}

pub struct ChainState {
    pub blocks: HashMap<[u8; 32], BlockMeta>,
    pub tip: [u8; 32],
    pub utxos: HashMap<([u8; 32], u32), UTXO>,
    pub db: ChainDB,
}

impl ChainState {
    pub fn load_or_init(genesis: Block, db: ChainDB) -> Self {
        let hash = hash_header(&genesis.header);

        let meta = BlockMeta {
            block: genesis,
            parent: [0u8; 32],
            height: 0,
            total_work: 0,
        };

        db.put_block(&hash, &meta.block);
        db.set_tip(&hash, 0);

        let mut blocks = HashMap::new();
        blocks.insert(hash, meta);

        let utxos = db
            .iter_utxos()
            .into_iter()
            .map(|u| ((u.txid, u.vout), u))
            .collect();

        ChainState {
            blocks,
            tip: hash,
            utxos,
            db,
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        // 1. verify PoW
        if !verify_pow(&block.header) {
            return false;
        }

        // 2. parent check
        let parent = block.header.prev_hash;
        let parent_meta = match self.blocks.get(&parent) {
            Some(m) => m.clone(),
            None => return false,
        };

        // 3. verify & spend normal tx
        for tx in block.transactions.iter().skip(1) {
            if !verify_tx(tx) {
                return false;
            }
            for inp in &tx.inputs {
                let key = (inp.prev_txid, inp.vout);
                if !self.utxos.contains_key(&key) {
                    return false;
                }
            }
        }

        // 4. remove spent UTXO
        for tx in block.transactions.iter().skip(1) {
            for inp in &tx.inputs {
                let key = (inp.prev_txid, inp.vout);
                self.utxos.remove(&key);
            }
        }

        // 5. compute meta
        let hash = hash_header(&block.header);
        let work = work_from_bits(block.header.bits);

        let meta = BlockMeta {
            block: block.clone(),
            parent,
            height: parent_meta.height + 1,
            total_work: parent_meta.total_work + work,
        };

        // 6. store block
        self.blocks.insert(hash, meta.clone());
        self.db.put_block(&hash, &block);

        // 7. create UTXO from coinbase
        let coinbase = &block.transactions[0];
        let id = txid(coinbase);

        for (vout, out) in coinbase.outputs.iter().enumerate() {
            let utxo = UTXO {
                txid: id,
                vout: vout as u32,
                value: out.value,
                address: out.to_address.clone(),
                height: meta.height,
            };
            self.utxos.insert((id, vout as u32), utxo.clone());
            self.db.put_utxo(&utxo);
        }

        // 8. fork choice
        self.maybe_update_tip(hash);

        true
    }

    fn maybe_update_tip(&mut self, candidate: [u8; 32]) {
        let cand = self.blocks.get(&candidate).unwrap();
        let best = self.blocks.get(&self.tip).unwrap();

        let better = cand.total_work > best.total_work
            || (cand.total_work == best.total_work && cand.height > best.height);

        if better {
            self.tip = candidate;
            self.db.set_tip(&candidate, cand.height);
        }
    }
}
