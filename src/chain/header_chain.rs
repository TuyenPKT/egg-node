use std::collections::HashMap;
use crate::chain::header::BlockHeader;
use crate::chain::hash::hash_header;
use crate::pow::verify::verify_pow;

pub struct HeaderChain {
    pub headers: HashMap<[u8; 32], BlockHeader>,
    pub tip: [u8; 32],
}

impl HeaderChain {
    pub fn new(genesis: BlockHeader) -> Self {
        let hash = hash_header(&genesis);
        let mut headers = HashMap::new();
        headers.insert(hash, genesis);

        HeaderChain {
            headers,
            tip: hash,
        }
    }

    pub fn add(&mut self, header: BlockHeader) -> bool {
        if !verify_pow(&header) {
            return false;
        }

        if !self.headers.contains_key(&header.prev_hash) {
            return false;
        }

        let hash = hash_header(&header);
        self.headers.insert(hash, header);
        self.tip = hash;
        true
    }
}
