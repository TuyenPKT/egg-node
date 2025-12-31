use crate::chain::block::Block;
use crate::chain::header::BlockHeader;
use crate::chain::tx::Transaction;
use crate::chain::block::merkle_root;
use crate::chain::reward::BLOCK_REWARD;
use crate::chain::hash::hash_header;


pub fn mine_block(
    prev_hash: [u8; 32],
    _height: u64,
    miner_address: Vec<u8>,
) -> Block {
    let coinbase = Transaction::coinbase(
        miner_address,
        BLOCK_REWARD,
        "Egg Core block reward",
    );

    let txs = vec![coinbase];
    let merkle = merkle_root(&txs);

    let mut header = BlockHeader {
        version: 1,
        prev_hash,
        merkle_root: merkle,
        timestamp: current_time(),
        bits: 0x1f00ffff,
        nonce: 0,
    };

    loop {
        let hash = hash_header(&header);
        if hash <= crate::pow::target::bits_to_target(header.bits) {
            break;
        }
        header.nonce += 1;
    }

    Block {
        header,
        transactions: txs,
    }
}

fn current_time() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
