use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::storage::ChainDB;
use crate::chain::validation::validate_genesis;



pub struct ChainState {
    pub blocks: HashMap<[u8; 32], Block>,
    pub tip: [u8; 32],
    pub height: u64,
    pub db: ChainDB,
}


impl ChainState {
    pub fn load_or_init(path: &str, genesis: Block) -> Self {
        let db = ChainDB::open(path);

        if let Some((tip, height)) = db.load_tip() {
            let mut blocks = HashMap::new();
            if let Some(block) = db.load_block(&tip) {
                blocks.insert(tip, block);
            }

            ChainState {
                blocks,
                tip,
                height,
                db,
            }
        } else {
            if !validate_genesis(&genesis) {
                panic!("Invalid genesis block");
            }

            let hash = hash_header(&genesis.header);
            db.save_block(&genesis, 0);

            let mut blocks = HashMap::new();
            blocks.insert(hash, genesis);

            ChainState {
                blocks,
                tip: hash,
                height: 0,
                db,
            }
        }
    }

    pub fn has_block(&self, hash: &[u8; 32]) -> bool {
        self.blocks.contains_key(hash) || self.db.load_block(hash).is_some()
    }

    pub fn add_block(&mut self, block: Block) {
        let hash = hash_header(&block.header);
        self.height += 1;

        self.db.save_block(&block, self.height);
        self.blocks.insert(hash, block);
        self.tip = hash;
    }
}
