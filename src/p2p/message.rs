use serde::{Serialize, Deserialize};

use crate::chain::header::BlockHeader;
use crate::chain::block::Block;
use crate::chain::tx::Transaction;

#[derive(Serialize, Deserialize)]
pub enum Message {
    // ---- handshake ----
    Handshake {
        protocol_version: u32,
        genesis_hash: [u8; 32],
        node_id: [u8; 32],
    },

    // ---- headers-first sync ----
    GetHeaders {
        from: [u8; 32],
        limit: u32,
    },
    Headers {
        headers: Vec<BlockHeader>,
    },

    // ---- compact block ----
    CompactBlock {
        header: BlockHeader,
        txids: Vec<[u8; 32]>,
    },

    // ---- full block fallback ----
    GetBlock {
        hash: [u8; 32],
    },
    Block {
        block: Block,
    },

    // ---- transaction broadcast ----
    Tx {
        tx: Transaction,
    },
}
