fn main() {
    let hash = egg_node::chain::genesis_hash();
    println!("Genesis hash: {:x?}", hash);
}
