use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::chain::utxo::UTXO;
use crate::chain::txid::txid;
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
    pub utxos: HashMap<([u8; 32], u32), UTXO>,
    pub db: ChainDB,
}

impl ChainState {
    /// Load chain from disk, or initialize with genesis
    pub fn load_or_init(genesis: Block, db: ChainDB) -> Self {
        let genesis_hash = hash_header(&genesis.header);

        let mut blocks = HashMap::new();
        let mut utxos = HashMap::new();

        // Nếu DB đã có tip → load chain tối thiểu
        if let Some((tip, height)) = db.get_tip() {
            let block = db
                .get_block(&tip)
                .expect("DB corrupted: missing tip block");

            let meta = BlockMeta {
                block,
                parent: [0u8; 32], // fork sâu load sau
                height,
                total_work: 0,
            };

            blocks.insert(tip, meta);

            // load UTXO
            for utxo in db.iter_utxos() {
                utxos.insert((utxo.txid, utxo.vout), utxo);
            }

            return ChainState {
                blocks,
                tip,
                utxos,
                db,
            };
        }

        // --- INIT GENESIS ---
        let meta = BlockMeta {
            block: genesis.clone(),
            parent: [0u8; 32],
            height: 0,
            total_work: 0,
        };

        blocks.insert(genesis_hash, meta);
        db.put_block(&genesis_hash, &genesis);
        db.set_tip(&genesis_hash, 0);

        ChainState {
            blocks,
            tip: genesis_hash,
            utxos,
            db,
        }
    }

    /// Add block with PoW + UTXO + fork-choice
    pub fn add_block(&mut self, block: Block) -> bool {
        // 1. Verify PoW
        if !verify_pow(&block.header) {
            return false;
        }

        let parent = block.header.prev_hash;
        let parent_meta = match self.blocks.get(&parent) {
            Some(m) => m.clone(),
            None => return false,
        };

        // 2. Verify & spend inputs (skip coinbase)
        for tx in block.transactions.iter().skip(1) {
            for inp in &tx.inputs {
                let key = (inp.prev_txid, inp.vout);
                if !self.utxos.contains_key(&key) {
                    return false;
                }
            }
        }

        // 3. Remove spent UTXO
        for tx in block.transactions.iter().skip(1) {
            for inp in &tx.inputs {
                self.utxos.remove(&(inp.prev_txid, inp.vout));
            }
        }

        // 4. Add coinbase UTXO
        let coinbase = &block.transactions[0];
        let cb_txid = txid(coinbase);

        for (vout, out) in coinbase.outputs.iter().enumerate() {
            let utxo = UTXO {
                txid: cb_txid,
                vout: vout as u32,
                value: out.value,
                address: out.to_address.clone(),
                height: parent_meta.height + 1,
            };
            self.utxos.insert((cb_txid, vout as u32), utxo);
        }

        // 5. Fork-choice (most-work)
        let hash = hash_header(&block.header);
        let work = work_from_bits(block.header.bits);

        let meta = BlockMeta {
            block: block.clone(),
            parent,
            height: parent_meta.height + 1,
            total_work: parent_meta.total_work + work,
        };

        self.blocks.insert(hash, meta.clone());
        self.db.put_block(&hash, &block);

        let best = self.blocks.get(&self.tip).unwrap();
        if meta.total_work > best.total_work
            || (meta.total_work == best.total_work && meta.height > best.height)
        {
            self.tip = hash;
            self.db.set_tip(&hash, meta.height);
        }

        true
    }
}
