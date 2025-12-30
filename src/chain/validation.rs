use crate::chain::block::Block;

pub fn validate_genesis(block: &Block) -> bool {
    // Điều kiện tối thiểu cho genesis
    if block.header.prev_hash != [0u8; 32] {
        return false;
    }

    if block.transactions.len() != 1 {
        return false;
    }

    if block.header.nonce != 0 {
        return false;
    }

    true
}
