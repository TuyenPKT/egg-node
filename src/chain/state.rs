use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::storage::sleddb::ChainDB;
use crate::pow::verify::verify_pow;
use crate::pow::work::work_from_bits;
use crate::chain::utxo::UTXO;
use crate::chain::txid::txid;


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
        let genesis_hash = hash_header(&genesis.header);

        let mut blocks = HashMap::new();
        let mut utxos = HashMap::new();

        // luôn đảm bảo genesis tồn tại trong DB
        db.put_block(&genesis_hash, &genesis);

        blocks.insert(
            genesis_hash,
            BlockMeta {
                block: genesis.clone(),
                parent: [0u8; 32],
                height: 0,
                total_work: 0,
            },
        );

        let mut tip = genesis_hash;

        if let Some((stored_tip, _)) = db.get_tip() {
            if let Some(block) = db.get_block(&stored_tip) {
                let hash = hash_header(&block.header);

                blocks.insert(
                    hash,
                    BlockMeta {
                        block: block.clone(),
                        parent: block.header.prev_hash,
                        height: 1, // tạm thời
                        total_work: work_from_bits(block.header.bits),
                    },
                );

                tip = hash;
            } else {
                eprintln!("DB tip missing block, reset to genesis");
                db.set_tip(&genesis_hash, 0);
            }
        } else {
            db.set_tip(&genesis_hash, 0);
        }

        // load utxo
        for u in db.iter_utxos() {
            utxos.insert((u.txid, u.vout), u);
        }

        ChainState {
            blocks,
            tip,
            utxos,
            db,
        }
    }




    pub fn add_block(&mut self, block: Block) -> bool {
    // 1. verify PoW
    if !verify_pow(&block.header) {
        return false;
    }

    // 2. validate coinbase
    if block.transactions.is_empty() {
        return false;
    }

    let coinbase = &block.transactions[0];
    if !coinbase.is_coinbase() {
        return false;
    }

    if block.transactions.iter().filter(|t| t.is_coinbase()).count() != 1 {
        return false;
    }

    let parent = block.header.prev_hash;

    // 3. parent must exist
    let parent_meta = match self.blocks.get(&parent) {
        Some(m) => m.clone(),
        None => return false,
    };

    let hash = hash_header(&block.header);

    // 4. compute meta
    let work = work_from_bits(block.header.bits);

    let meta = BlockMeta {
        block: block.clone(),
        parent,
        height: parent_meta.height + 1,
        total_work: parent_meta.total_work.saturating_add(work),
    };

    // 5. store block
    self.blocks.insert(hash, meta.clone());
    self.db.put_block(&hash, &block);

    // 6. fork choice
    self.maybe_update_tip(hash);

    // 7. CREATE UTXO FROM COINBASE
    let coinbase_txid = txid(coinbase);

    for (vout, out) in coinbase.outputs.iter().enumerate() {
        let utxo = UTXO {
            txid: coinbase_txid,
            vout: vout as u32,
            value: out.value,
            address: out.to_address.clone(),
            height: meta.height,
        };

        // in-memory
        self.utxos.insert((coinbase_txid, vout as u32), utxo.clone());

        // persist
        self.db.put_utxo(&utxo);
    }

    true
}


    fn maybe_update_tip(&mut self, candidate: [u8; 32]) {
        let cand = match self.blocks.get(&candidate) {
            Some(c) => c,
            None => return,
        };

        let best = match self.blocks.get(&self.tip) {
            Some(b) => b,
            None => {
                self.tip = candidate;
                self.db.set_tip(&candidate, cand.height);
                return;
            }
        };

        let better = cand.total_work > best.total_work
            || (cand.total_work == best.total_work && cand.height > best.height);

        if better {
            self.tip = candidate;
            self.db.set_tip(&candidate, cand.height);
        }
    }

}
