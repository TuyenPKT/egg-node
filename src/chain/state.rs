use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::storage::sleddb::ChainDB;
use crate::pow::verify::verify_pow;
use crate::pow::work::work_from_bits;

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

        ChainState {
            blocks,
            tip: hash,
            db,
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        // 1. verify PoW
        if !verify_pow(&block.header) {
            return false;
        }

        // 1.5 validate coinbase
        if block.transactions.is_empty() {
            return false;
        }

        let coinbase = &block.transactions[0];
        if !coinbase.is_coinbase() {
            return false;
        }

        // chỉ cho phép 1 coinbase
        if block.transactions.iter().filter(|t| t.is_coinbase()).count() != 1 {
            return false;
        }


        let parent = block.header.prev_hash;

        // 2. parent must exist
        let parent_meta = match self.blocks.get(&parent) {
            Some(m) => m.clone(),
            None => return false,
        };

        let hash = hash_header(&block.header);

        // 3. compute meta
        let work = work_from_bits(block.header.bits);

        let meta = BlockMeta {
            block: block.clone(),
            parent,
            height: parent_meta.height + 1,
            total_work: parent_meta.total_work.saturating_add(work),
        };


        // 4. store
        self.blocks.insert(hash, meta.clone());
        self.db.put_block(&hash, &block);

        // 5. fork choice
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
