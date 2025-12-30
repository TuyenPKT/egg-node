use crate::chain::genesis_block;
use crate::chain::validation::validate_genesis;
use crate::chain::block::Block;

use rand::RngCore;

/// Giả lập việc load blockchain khi node start
pub fn start_node() {
    println!("Starting Egg Node...");

    // BƯỚC 1: Load genesis (hiện tại ta CHƯA có storage)
    let genesis: Block = genesis_block();

    // BƯỚC 2: Validate genesis
    if !validate_genesis(&genesis) {
        // Nếu genesis sai → dừng node ngay
        panic!("Genesis block INVALID. Refusing to start node.");
    }

    // BƯỚC 3: Nếu OK
    println!("Genesis block valid.");
    println!("Node started successfully.");
}

pub fn generate_node_id() -> [u8; 32] {
    let mut id = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut id);
    id
}