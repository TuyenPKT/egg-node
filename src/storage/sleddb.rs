use sled::Db;
use bincode;

use crate::chain::block::Block;
use crate::chain::utxo::UTXO;

pub struct ChainDB {
    db: Db,
}

impl ChainDB {
    pub fn open(path: &str) -> Self {
        let db = sled::open(path).expect("cannot open db");
        ChainDB { db }
    }

    // ---------- BLOCK ----------

    pub fn put_block(&self, hash: &[u8; 32], block: &Block) {
        let mut key = b"block:".to_vec();
        key.extend_from_slice(hash);

        let val = bincode::serialize(block).unwrap();
        self.db.insert(key, val).unwrap();
    }

    pub fn get_block(&self, hash: &[u8; 32]) -> Option<Block> {
        let mut key = b"block:".to_vec();
        key.extend_from_slice(hash);

        self.db
            .get(key)
            .unwrap()
            .map(|v| bincode::deserialize(&v).unwrap())
    }

    // ---------- META ----------

    pub fn set_tip(&self, hash: &[u8; 32], height: u64) {
        self.db.insert(b"meta:tip", hash).unwrap();
        self.db
            .insert(b"meta:height", height.to_le_bytes())
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

    // ---------- UTXO ----------

    pub fn put_utxo(&self, utxo: &UTXO) {
        let mut key = b"utxo:".to_vec();
        key.extend_from_slice(&utxo.txid);
        key.extend_from_slice(&utxo.vout.to_le_bytes());

        let val = bincode::serialize(utxo).unwrap();
        self.db.insert(key, val).unwrap();
    }

    pub fn iter_utxos(&self) -> Vec<UTXO> {
        let mut list = Vec::new();

        for item in self.db.scan_prefix(b"utxo:") {
            let (_, val) = item.unwrap();
            let utxo: UTXO = bincode::deserialize(&val).unwrap();
            list.push(utxo);
        }

        list
    }
}
