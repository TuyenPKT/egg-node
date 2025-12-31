use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::storage::sleddb::ChainDB;
use crate::chain::validation::validate_genesis;
use crate::pow::verify::verify_pow;




pub struct ChainState {
    pub blocks: HashMap<[u8; 32], Block>,
    pub tip: [u8; 32],
    pub height: u64,
    pub db: ChainDB,
}

impl ChainState {
    pub fn new(genesis: Block, db: ChainDB) -> Self {
        let hash = hash_header(&genesis.header);

        db.put_block(&hash, &genesis);
        db.set_tip(&hash, 0);

        let mut blocks = HashMap::new();
        blocks.insert(hash, genesis);

        ChainState {
            blocks,
            tip: hash,
            height: 0,
            db,
        }
    }

    pub fn load_or_init(genesis: Block, db: ChainDB) -> Self {
        if !validate_genesis(&genesis) {
            panic!("Invalid genesis block");
        }

        if let Some((tip, height)) = db.get_tip() {
            let mut blocks = HashMap::new();
            let block = db.get_block(&tip).expect("missing tip block");
            blocks.insert(tip, block);

            ChainState {
                blocks,
                tip,
                height,
                db,
            }
        } else {
            Self::new(genesis, db)
        }
    }

    pub fn has_block(&self, hash: &[u8; 32]) -> bool {
        self.blocks.contains_key(hash)
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        let prev = block.header.prev_hash;

        // 1. prev block phải tồn tại
        if !self.blocks.contains_key(&prev) {
            return false;
        }

        // 2. verify PoW
        if !verify_pow(&block.header) {
            return false;
        }

        let hash = hash_header(&block.header);

        self.db.put_block(&hash, &block);
        self.height += 1;
        self.db.set_tip(&hash, self.height);

        self.tip = hash;
        self.blocks.insert(hash, block);
        true
    }
}
