use std::net::TcpListener;
use std::thread;
use std::sync::{Arc, Mutex};

use crate::config::NodeConfig;
use crate::chain::state::ChainState;
use crate::p2p::peer::{handle_peer, perform_handshake};
use crate::p2p::network::connect_to_peer;

pub fn run_node(config: NodeConfig, chain: ChainState) {
    let chain = Arc::new(Mutex::new(chain));

    // 1. LISTEN INBOUND
    let listener = TcpListener::bind(&config.bind_addr)
        .expect("Cannot bind address");

    println!("Node listening on {}", config.bind_addr);

    // 2. AUTO-CONNECT PEERS (OUTBOUND)
    for peer in config.peers.iter() {
        let chain_clone = chain.clone();
        let peer_addr = peer.clone();

        thread::spawn(move || {
            connect_to_peer(&peer_addr, chain_clone);
        });
    }

    // 3. ACCEPT INBOUND
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let chain_clone = chain.clone();

            thread::spawn(move || {
                if perform_handshake(&stream).is_ok() {
                    let mut chain = chain_clone.lock().unwrap();
                    handle_peer(stream, &mut chain);
                }
            });
        }
    }
}
