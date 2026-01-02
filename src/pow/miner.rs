use crate::chain::block::Block;
use crate::chain::header::BlockHeader;
use crate::chain::tx::Transaction;
use crate::chain::block::merkle_root;
use crate::chain::reward::BLOCK_REWARD;
use crate::chain::hash::hash_header;
use crate::pow::target::bits_to_target;
use crate::mempool::Mempool;

pub fn mine_block_with_fees(
    prev_hash: [u8; 32],
    height: u64,
    miner_address: Vec<u8>,
    mempool: &Mempool,
    max_txs: usize,
) -> Block {
    // --- coinbase ---
    let coinbase = Transaction {
        inputs: vec![],
        outputs: vec![crate::chain::tx::TxOutput {
            value: BLOCK_REWARD,
            to_address: miner_address,
        }],
        data: format!("coinbase height {}", height).into_bytes(),
    };


    // --- select txs by fee (already sorted inside mempool) ---
    let mut txs = Vec::new();
    txs.push(coinbase);

    let picked: Vec<Transaction> = mempool.select_for_block(max_txs);
    txs.extend(picked);

    let merkle = merkle_root(&txs);

    let mut header = BlockHeader {
        version: 1,
        prev_hash,
        merkle_root: merkle,
        timestamp: now(),
        bits: 0x1f00ffff,
        nonce: 0,
    };

    let target = bits_to_target(header.bits);

    loop {
        if hash_header(&header) <= target {
            break;
        }
        header.nonce += 1;
    }

    Block {
        header,
        transactions: txs,
    }
}

fn now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
