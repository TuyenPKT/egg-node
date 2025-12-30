use crate::chain::block::Block;
use crate::chain::header::BlockHeader;
use crate::chain::tx::Transaction;
use crate::chain::hash::hash_header;
use crate::chain::state::ChainState;
use crate::pow::miner::mine;
use crate::chain::block::merkle_root;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn build_block(chain: &ChainState) -> Block {
    let tx = Transaction {
        data: b"manual mining tx".to_vec(),
    };

    let txs = vec![tx];

    let mut header = BlockHeader {
        version: 1,
        prev_hash: chain.tip,
        merkle_root: merkle_root(&txs),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        bits: 0x1f00ffff,
        nonce: 0,
    };

    let target = [0x0f; 32]; // target rất dễ cho GĐ1
    mine(&mut header, &target);

    Block {
        header,
        transactions: txs,
    }
}
