use crate::chain::block::{Block, merkle_root};
use crate::chain::header::BlockHeader;
use crate::chain::tx::Transaction;
use crate::chain::reward::BLOCK_REWARD;
use crate::chain::hash::hash_header;
use crate::mempool::{Mempool, MempoolEntry};
use crate::pow::target::bits_to_target;
use crate::chain::tx::{TxInput, TxOutput};

pub fn mine_block_with_fees(
    prev_hash: [u8; 32],
    _height: u64,
    miner_address: Vec<u8>,
    mempool: &Mempool,
    max_txs: usize,
) -> Block {
    // chọn tx theo fee-rate
    let picked: Vec<MempoolEntry> = mempool.pick_for_mining(max_txs);
    let total_fee: u64 = picked.iter().map(|e| e.fee).sum();

    // coinbase = reward + fee
    let coinbase = Transaction {
        inputs: vec![TxInput {
            prev_txid: [0u8;32],
            vout: 0,
            signature: vec![],
            pubkey: vec![],
        }],
        outputs: vec![TxOutput {
            value: BLOCK_REWARD + total_fee,
            to_address: miner_address,
        }],
        data: b"Egg Core coinbase".to_vec(),
    };

    let mut txs = vec![coinbase];
    txs.extend(picked.into_iter().map(|e| e.tx));

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
        let h = hash_header(&header);
        if h <= target { break; }
        header.nonce += 1;
    }

    Block { header, transactions: txs }
}

fn now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
