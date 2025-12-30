use sha2::{Sha256, Digest};
use bincode;

// QUAN TRỌNG: crate name là egg_node (underscore)
use egg_node::chain::genesis_block;

fn main() {
    let genesis = genesis_block();

    let encoded = bincode::serialize(&genesis.header).unwrap();
    let hash = Sha256::digest(encoded);

    println!("Genesis hash:");
    println!("{:x}", hash);
}
