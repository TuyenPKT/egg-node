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
    pub fn load_or_init(genesis: Block, db: ChainDB) -> Self {
        let ghash = hash_header(&genesis.header);
        let mut blocks = HashMap::new();
        let mut utxos = HashMap::new();

        if let Some((tip, height)) = db.get_tip() {
            let block = db.get_block(&tip).expect("missing tip block");
            blocks.insert(tip, BlockMeta {
                block, parent: [0u8;32], height, total_work: 0
            });
            for u in db.iter_utxos() {
                utxos.insert((u.txid,u.vout), u);
            }
            return ChainState { blocks, tip, utxos, db };
        }

        blocks.insert(ghash, BlockMeta {
            block: genesis.clone(), parent: [0u8;32], height: 0, total_work: 0
        });
        db.put_block(&ghash, &genesis);
        db.set_tip(&ghash, 0);

        ChainState { blocks, tip: ghash, utxos, db }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        // PoW
        if !verify_pow(&block.header) { return false; }

        let parent = block.header.prev_hash;
        let parent_meta = match self.blocks.get(&parent) {
            Some(m) => m.clone(),
            None => return false,
        };

        // verify spends & compute total fee
        let mut total_fee: u64 = 0;

        for tx in block.transactions.iter().skip(1) {
            let mut in_sum: u64 = 0;
            for i in &tx.inputs {
                let key = (i.prev_txid, i.vout);
                let u = match self.utxos.get(&key) {
                    Some(u) => u,
                    None => return false,
                };
                in_sum = in_sum.saturating_add(u.value);
            }
            let out_sum: u64 = tx.outputs.iter().map(|o| o.value).sum();
            if in_sum < out_sum { return false; }
            total_fee = total_fee.saturating_add(in_sum - out_sum);
        }

        // spend inputs
        for tx in block.transactions.iter().skip(1) {
            for i in &tx.inputs {
                self.utxos.remove(&(i.prev_txid, i.vout));
            }
        }

        // add coinbase UTXO(s) — coinbase outputs đã bao gồm reward + fee
        let coinbase = &block.transactions[0];
        let cb_txid = txid(coinbase);
        for (vout, out) in coinbase.outputs.iter().enumerate() {
            self.utxos.insert((cb_txid, vout as u32), UTXO {
                txid: cb_txid,
                vout: vout as u32,
                value: out.value,
                address: out.to_address.clone(),
                height: parent_meta.height + 1,
            });
        }

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
