use serde::{Serialize, Deserialize};
use crate::chain::header::BlockHeader;
use crate::chain::block::Block;

#[derive(Serialize, Deserialize)]
pub enum Message {
    Handshake {
        protocol_version: u32,
        genesis_hash: [u8; 32],
        node_id: [u8; 32],
    },

    // headers-first
    GetHeaders {
        from: [u8; 32],
        limit: u32,
    },
    Headers {
        headers: Vec<BlockHeader>,
    },

    // compact block
    CompactBlock {
        header: BlockHeader,
        txids: Vec<[u8; 32]>,
    },

    GetBlock {
        hash: [u8; 32],
    },

    Block {
        block: Block,
    },
}
