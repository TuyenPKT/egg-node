use std::collections::{HashMap, VecDeque};
use std::time::{Instant, Duration};

use crate::chain::block::Block;
use crate::chain::hash::hash_header;

const MAX_ORPHAN_BLOCKS: usize = 2048;
const ORPHAN_BLOCK_TTL: Duration = Duration::from_secs(600);

#[derive(Clone)]
pub struct OrphanBlock {
    pub block: Block,
    pub added: Instant,
}

pub struct OrphanBlockPool {
    pub blocks: HashMap<[u8; 32], OrphanBlock>,
    pub order: VecDeque<[u8; 32]>,
}

impl OrphanBlockPool {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn add(&mut self, block: Block) {
        let hash = hash_header(&block.header);
        if self.blocks.contains_key(&hash) {
            return;
        }

        if self.blocks.len() >= MAX_ORPHAN_BLOCKS {
            if let Some(old) = self.order.pop_front() {
                self.blocks.remove(&old);
            }
        }

        self.order.push_back(hash);
        self.blocks.insert(hash, OrphanBlock {
            block,
            added: Instant::now(),
        });
    }

    pub fn try_promote<F>(
        &mut self,
        mut has_parent: F,
        mut accept: impl FnMut(Block),
    )
    where
        F: FnMut(&[u8; 32]) -> bool,
    {
        let now = Instant::now();
        let mut promoted = Vec::new();

        for (hash, orphan) in &self.blocks {
            if now.duration_since(orphan.added) > ORPHAN_BLOCK_TTL {
                promoted.push(*hash);
                continue;
            }

            if has_parent(&orphan.block.header.prev_hash) {
                accept(orphan.block.clone());
                promoted.push(*hash);
            }
        }

        for h in promoted {
            self.blocks.remove(&h);
            self.order.retain(|x| x != &h);
        }
    }
}
