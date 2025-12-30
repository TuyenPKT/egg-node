use std::collections::HashMap;
use super::block::Block;
use super::hash::hash_header;
use crate::chain::validation::validate_genesis;


pub struct ChainState {
    pub blocks: HashMap<[u8; 32], Block>,
    pub tip: [u8; 32],
    pub height: u64,
}

impl ChainState {
    pub fn new(genesis: Block) -> Self {
    assert!(validate_genesis(&genesis), "Invalid genesis block");

    let hash = hash_header(&genesis.header);
    let mut blocks = HashMap::new();
    blocks.insert(hash, genesis);

    ChainState {
        blocks,
        tip: hash,
        height: 0,
    }
}


    pub fn has_block(&self, hash: &[u8; 32]) -> bool {
        self.blocks.contains_key(hash)
    }

    pub fn add_block(&mut self, block: Block) {
        let hash = hash_header(&block.header);
        self.blocks.insert(hash, block);
        self.tip = hash;
        self.height += 1;
    }
}
