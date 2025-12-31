use egg_node::chain::block::Block;
use egg_node::chain::header::BlockHeader;
use egg_node::chain::genesis_block;
use egg_node::chain::state::ChainState;
use egg_node::storage::sleddb::ChainDB;
use egg_node::pow::verify::verify_pow;
use egg_node::chain::hash::hash_header;

/// Mine một block HỢP LỆ PoW (brute-force nonce)
fn mine_block(prev: [u8; 32], bits: u32) -> Block {
    let mut nonce: u64 = 0;

    loop {
        let header = BlockHeader {
            version: 1,
            prev_hash: prev,
            merkle_root: [1u8; 32],
            timestamp: 1,
            bits,
            nonce,
        };

        if verify_pow(&header) {
            return Block {
                header,
                transactions: vec![],
            };
        }

        nonce += 1;
    }
}

fn main() {
    // DB test riêng, không ảnh hưởng chain thật
    let db = ChainDB::open("./fork-test-db");

    let genesis = genesis_block();
    let mut chain = ChainState::load_or_init(genesis, db);

    let genesis_hash = chain.tip;

    // ===============================
    // Fork A và Fork B từ genesis
    // ===============================

    // Fork A (work thấp)
    let block_a = mine_block(genesis_hash, 0x1f00ffff);
    let hash_a = hash_header(&block_a.header);
    chain.add_block(block_a);

    // Fork B (work cao hơn)
    let block_b = mine_block(genesis_hash, 0x1f00aaaa);
    let hash_b = hash_header(&block_b.header);
    chain.add_block(block_b);

    println!("Tip after A/B: {:?}", chain.tip);

    // ===============================
    // Kéo dài fork A (yếu hơn ban đầu)
    // ===============================

    let block_c = mine_block(hash_a, 0x1f00ffff);
    let _hash_c = hash_header(&block_c.header);
    chain.add_block(block_c);

    println!("Tip after extend A: {:?}", chain.tip);

    // ===============================
    // Kéo dài fork B bằng block khó hơn
    // ===============================

    let block_d = mine_block(hash_b, 0x1e00ffff);

    let hash_d = hash_header(&block_d.header);
    chain.add_block(block_d);

    println!("Final tip (should be D): {:?}", chain.tip);
    println!("Expected tip hash (D): {:?}", hash_d);
}
