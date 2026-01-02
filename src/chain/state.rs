use std::collections::HashMap;

use crate::chain::block::Block;
use crate::chain::header::BlockHeader;
use crate::chain::hash::hash_header;
use crate::chain::txid::txid;
use crate::chain::utxo::UTXO;
use crate::chain::undo::BlockUndo;
use crate::storage::sleddb::ChainDB;
use crate::pow::verify::verify_pow;
use crate::pow::work::work_from_bits;

#[derive(Clone)]
pub struct BlockMeta {
    pub block: Block,
    pub parent: [u8; 32],
    pub height: u64,
    pub total_work: u128,
    pub undo: BlockUndo,
}

pub struct ChainState {
    pub blocks: HashMap<[u8; 32], BlockMeta>,
    pub tip: [u8; 32],
    pub utxos: HashMap<([u8; 32], u32), UTXO>,
    pub db: ChainDB,
}

/* =========================
   INIT / HEADER STAGE
   ========================= */

impl ChainState {
    pub fn load_or_init(genesis: Block, db: ChainDB) -> Self {
        let genesis_hash = hash_header(&genesis.header);

        if db.get_tip().is_none() {
            let mut utxos = HashMap::new();

            let coinbase = &genesis.transactions[0];
            let cbid = txid(coinbase);

            for (vout, out) in coinbase.outputs.iter().enumerate() {
                let utxo = UTXO {
                    txid: cbid,
                    vout: vout as u32,
                    value: out.value,
                    address: out.to_address.clone(),
                    height: 0,
                };
                utxos.insert((utxo.txid, utxo.vout), utxo);
            }

            let meta = BlockMeta {
                block: genesis.clone(),
                parent: [0u8; 32],
                height: 0,
                total_work: 0,
                undo: BlockUndo {
                    spent: vec![],
                    created: vec![],
                },
            };

            let mut blocks = HashMap::new();
            blocks.insert(genesis_hash, meta);

            db.put_block(&genesis_hash, &genesis);
            db.set_tip(&genesis_hash, 0);

            return ChainState {
                blocks,
                tip: genesis_hash,
                utxos,
                db,
            };
        }

        let (tip, _) = db.get_tip().unwrap();
        let block = db.get_block(&tip).unwrap();

        let mut blocks = HashMap::new();
        blocks.insert(
            tip,
            BlockMeta {
                block: block.clone(),
                parent: block.header.prev_hash,
                height: 0,
                total_work: 0,
                undo: BlockUndo {
                    spent: vec![],
                    created: vec![],
                },
            },
        );

        ChainState {
            blocks,
            tip,
            utxos: HashMap::new(),
            db,
        }
    }

    pub fn accept_header(&self, header: &BlockHeader) -> bool {
        verify_pow(header) && self.blocks.contains_key(&header.prev_hash)
    }
}

/* =========================
   BLOCK APPLY + REORG
   ========================= */

impl ChainState {
    pub fn add_block(&mut self, block: Block) -> bool {
        if !verify_pow(&block.header) {
            return false;
        }

        let parent = block.header.prev_hash;
        let parent_meta = match self.blocks.get(&parent) {
            Some(m) => m.clone(),
            None => return false,
        };

        let mut undo = BlockUndo {
            spent: vec![],
            created: vec![],
        };

        for tx in block.transactions.iter().skip(1) {
            let mut in_sum = 0;
            let mut out_sum = 0;

for inp in &tx.inputs {
    let key = (inp.prev_txid, inp.vout);
    let utxo = match self.utxos.get(&key) {
        Some(u) => u.clone(),
        None => return false,
    };
    in_sum += utxo.value;
    undo.spent.push(utxo);
}


            for out in &tx.outputs {
                out_sum += out.value;
            }

            if in_sum < out_sum {
                return false;
            }
        }

        for u in &undo.spent {
            self.utxos.remove(&(u.txid, u.vout));
        }

        let cb = &block.transactions[0];
        let cbid = txid(cb);

        for (vout, out) in cb.outputs.iter().enumerate() {
            let utxo = UTXO {
                txid: cbid,
                vout: vout as u32,
                value: out.value,
                address: out.to_address.clone(),
                height: parent_meta.height + 1,
            };
            self.utxos.insert((utxo.txid, utxo.vout), utxo.clone());
            undo.created.push(utxo);
        }

        let hash = hash_header(&block.header);
        let meta = BlockMeta {
            block: block.clone(),
            parent,
            height: parent_meta.height + 1,
            total_work: parent_meta.total_work + work_from_bits(block.header.bits),
            undo,
        };

        self.blocks.insert(hash, meta);
        self.maybe_reorg(hash);
        true
    }

    fn maybe_reorg(&mut self, candidate: [u8; 32]) {
        let cand = self.blocks.get(&candidate).unwrap().clone();
        let best = self.blocks.get(&self.tip).unwrap().clone();

        if cand.total_work > best.total_work
            || (cand.total_work == best.total_work && cand.height > best.height)
        {
            self.reorg(self.tip, candidate);
        }
    }

    fn reorg(&mut self, old_tip: [u8; 32], new_tip: [u8; 32]) {
        if old_tip == new_tip {
            return;
        }

        let mut a = old_tip;
        let mut b = new_tip;
        let mut path_a = Vec::new();
        let mut path_b = Vec::new();

        while a != b {
            let ma = self.blocks.get(&a).unwrap();
            let mb = self.blocks.get(&b).unwrap();

            if ma.height >= mb.height {
                path_a.push(a);
                a = ma.parent;
            } else {
                path_b.push(b);
                b = mb.parent;
            }
        }

        for h in &path_a {
            let undo = self.blocks.get(h).unwrap().undo.clone();
            self.rollback_block(&undo);
        }

        for h in path_b.iter().rev() {
            let undo = self.blocks.get(h).unwrap().undo.clone();
            self.apply_block(&undo);
        }

        self.tip = new_tip;
        self.db.set_tip(&new_tip, self.blocks.get(&new_tip).unwrap().height);
    }

    fn rollback_block(&mut self, undo: &BlockUndo) {
        for u in &undo.created {
            self.utxos.remove(&(u.txid, u.vout));
        }
        for u in &undo.spent {
            self.utxos.insert((u.txid, u.vout), u.clone());
        }
    }

    fn apply_block(&mut self, undo: &BlockUndo) {
        for u in &undo.spent {
            self.utxos.remove(&(u.txid, u.vout));
        }
        for u in &undo.created {
            self.utxos.insert((u.txid, u.vout), u.clone());
        }
    }
}
