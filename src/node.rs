use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::config::NodeConfig;
use crate::chain::state::ChainState;
use crate::mempool::Mempool;
use crate::p2p::peer::handle_peer;
use crate::orphan::tx::OrphanTxPool;
use crate::orphan::block::OrphanBlockPool;

pub fn run_node(config: NodeConfig, chain: ChainState) {
    let chain = Arc::new(Mutex::new(chain));
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    let orphan_tx = Arc::new(Mutex::new(OrphanTxPool::new()));
    let orphan_block = Arc::new(Mutex::new(OrphanBlockPool::new()));

    let listener = TcpListener::bind(&config.bind_addr).unwrap();
    println!("Listening on {}", config.bind_addr);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let c = chain.clone();
            let m = mempool.clone();
            let ot = orphan_tx.clone();
            let ob = orphan_block.clone();

            thread::spawn(move || {
                handle_peer(stream, c, m, ot, ob);
            });
        }
    }
}
