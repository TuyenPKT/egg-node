use sled::Db;
use bincode;

use crate::chain::block::Block;

pub struct ChainDB {
    db: Db,
}

impl ChainDB {
    pub fn open(path: &str) -> Self {
        let db = sled::open(path).expect("cannot open db");
        ChainDB { db }
    }

    pub fn put_block(&self, hash: &[u8; 32], block: &Block) {
        let key = format!("block:{:x?}", hash);
        let val = bincode::serialize(block).unwrap();
        self.db.insert(key.as_bytes(), val).unwrap();
    }

    pub fn get_block(&self, hash: &[u8; 32]) -> Option<Block> {
        let key = format!("block:{:x?}", hash);
        self.db
            .get(key.as_bytes())
            .unwrap()
            .map(|v| bincode::deserialize(&v).unwrap())
    }

    pub fn set_tip(&self, hash: &[u8; 32], height: u64) {
        self.db
            .insert(b"meta:tip", hash)
            .unwrap();
        self.db
            .insert(b"meta:height", &height.to_le_bytes())
            .unwrap();
    }

    pub fn get_tip(&self) -> Option<([u8; 32], u64)> {
        let tip = self.db.get(b"meta:tip").unwrap()?;
        let height = self.db.get(b"meta:height").unwrap()?;
        Some((
            tip.as_ref().try_into().unwrap(),
            u64::from_le_bytes(height.as_ref().try_into().unwrap()),
        ))
    }
}
