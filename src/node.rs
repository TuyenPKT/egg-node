use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::chain::state::ChainState;
use crate::config::NodeConfig;
use crate::mempool::Mempool;
use crate::p2p::peer::handle_peer;

pub fn run_node(
    config: NodeConfig,
    chain: ChainState,
) {
    let chain = Arc::new(Mutex::new(chain));
    let mempool = Arc::new(Mutex::new(Mempool::new()));

    let listener = TcpListener::bind(&config.bind_addr).unwrap();
    println!("Listening on {}", config.bind_addr);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let c = chain.clone();
            let m = mempool.clone();
            thread::spawn(move || {
                handle_peer(stream, c, m);
            });
        }
    }
}
