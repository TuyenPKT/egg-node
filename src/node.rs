use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

use crate::config::NodeConfig;
use crate::chain::state::ChainState;
use crate::p2p::peer::{handle_peer, outbound_connect};

pub fn run_node(config: NodeConfig, chain: ChainState) {
    let chain = Arc::new(Mutex::new(chain));

    // --- LISTEN INBOUND ---
    let listener = TcpListener::bind(&config.bind_addr)
        .expect("Cannot bind address");

    println!("Listening on {}", config.bind_addr);

    // --- AUTO CONNECT OUTBOUND PEERS ---
    for peer in config.peers.iter() {
        let peer_addr = peer.clone();
        let chain_clone = chain.clone();

        thread::spawn(move || {
            match TcpStream::connect(&peer_addr) {
                Ok(stream) => {
                    println!("Connected to peer {}", peer_addr);
                    let mut chain = chain_clone.lock().unwrap();
                    if let Err(e) = outbound_connect(stream, &mut chain) {
                        eprintln!("Handshake failed {}: {}", peer_addr, e);
                    }
                }
                Err(e) => {
                    eprintln!("Cannot connect to {}: {}", peer_addr, e);
                }
            }
        });
    }

    // --- ACCEPT INBOUND PEERS ---
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let chain_clone = chain.clone();
                thread::spawn(move || {
                    let mut chain = chain_clone.lock().unwrap();
                    handle_peer(stream, &mut chain);
                });
            }
            Err(e) => {
                eprintln!("Inbound error: {}", e);
            }
        }
    }
}

use rand::RngCore;

pub fn generate_node_id() -> [u8; 32] {
    let mut id = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut id);
    id
}
