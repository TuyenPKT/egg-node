use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::config::{P2P_PORT, SEED_PEERS};
use crate::chain::state::ChainState;
use crate::chain::genesis_block;
use crate::p2p::peer::{handle_peer, perform_handshake};

pub fn run_node() {
    let mut chain = ChainState::new(genesis_block());

    // 1. Lắng nghe incoming peer
    let listener = TcpListener::bind(("0.0.0.0", P2P_PORT))
        .expect("Bind P2P port failed");

    println!("Node listening on port {}", P2P_PORT);

    // Thread accept incoming
    let chain_ptr = std::sync::Arc::new(std::sync::Mutex::new(chain));
    let chain_accept = chain_ptr.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                let chain = chain_accept.clone();
                thread::spawn(move || {
                    let mut chain = chain.lock().unwrap();
                    handle_peer(stream, &mut chain);
                });
            }
        }
    });

    // 2. Chủ động connect seed peers
    for peer in SEED_PEERS {
        match TcpStream::connect(peer) {
            Ok(stream) => {
                println!("Connected to peer {}", peer);
                if perform_handshake(&stream).is_ok() {
                    let chain = chain_ptr.clone();
                    thread::spawn(move || {
                        let mut chain = chain.lock().unwrap();
                        handle_peer(stream, &mut chain);
                    });
                }
            }
            Err(e) => {
                println!("Cannot connect {}: {}", peer, e);
            }
        }
    }

    // Giữ process sống
    loop {
        std::thread::park();
    }
}
