use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::config::NodeConfig;
use crate::chain::state::ChainState;
use crate::mempool::Mempool;
use crate::orphan::tx::OrphanTxPool;
use crate::orphan::block::OrphanBlockPool;
use crate::p2p::peer::handle_peer;
use crate::net::ban::BanManager;
use crate::net::rate::RateLimiter;

pub fn run_node(config: NodeConfig, chain: ChainState) {
    let chain = Arc::new(Mutex::new(chain));
    let mempool = Arc::new(Mutex::new(Mempool::new()));
    let orphan_tx = Arc::new(Mutex::new(OrphanTxPool::new()));
    let orphan_block = Arc::new(Mutex::new(OrphanBlockPool::new()));
    let ban = Arc::new(Mutex::new(BanManager::new()));
    let rate = Arc::new(Mutex::new(RateLimiter::new()));

    let listener = TcpListener::bind(&config.bind_addr).unwrap();
    println!("Listening on {}", config.bind_addr);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let ip = stream.peer_addr().unwrap().ip().to_string();

            if ban.lock().unwrap().is_banned(&ip) {
                continue;
            }

            let c = chain.clone();
            let m = mempool.clone();
            let ot = orphan_tx.clone();
            let ob = orphan_block.clone();
            let b = ban.clone();
            let r = rate.clone();

            thread::spawn(move || {
                handle_peer(stream, ip, c, m, ot, ob, b, r);
            });
        }
    }
}
