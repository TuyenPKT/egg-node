use sled::Db;
use crate::chain::block::Block;
use crate::chain::hash::hash_header;

const BLOCKS: &str = "blocks";
const META: &str = "meta";
const TIP: &str = "tip";
const HEIGHT: &str = "height";

pub struct ChainDB {
    db: Db,
}

impl ChainDB {
    pub fn open(path: &str) -> Self {
        let db = sled::open(path).expect("Cannot open sled DB");
        ChainDB { db }
    }

    pub fn load_tip(&self) -> Option<([u8; 32], u64)> {
        let meta = self.db.open_tree(META).unwrap();

        let tip = meta.get(TIP).unwrap()?;
        let height = meta.get(HEIGHT).unwrap()?;

        let mut h = [0u8; 32];
        h.copy_from_slice(&tip);

        let mut height_bytes = [0u8; 8];
        height_bytes.copy_from_slice(&height);

        Some((h, u64::from_be_bytes(height_bytes)))
    }

    pub fn save_block(&self, block: &Block, height: u64) {
        let hash = hash_header(&block.header);

        let blocks = self.db.open_tree(BLOCKS).unwrap();
        let meta = self.db.open_tree(META).unwrap();

        blocks
            .insert(hash.as_slice(), bincode::serialize(block).unwrap())
            .unwrap();

        meta.insert(TIP, hash.as_slice()).unwrap();
        meta.insert(HEIGHT, height.to_be_bytes().as_slice()).unwrap();

        self.db.flush().unwrap();
    }


    pub fn load_block(&self, hash: &[u8; 32]) -> Option<Block> {
        let blocks = self.db.open_tree(BLOCKS).unwrap();
        blocks
            .get(hash)
            .unwrap()
            .map(|v| bincode::deserialize(&v).unwrap())
    }
}
