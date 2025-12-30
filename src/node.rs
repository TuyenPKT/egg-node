use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use crate::config::{P2P_PORT, SEED_PEERS};
use crate::chain::state::ChainState;
use crate::chain::genesis_block;
use crate::p2p::peer::{handle_peer, perform_handshake};
use crate::chain::validation::validate_genesis;


pub fn run_node() {
    let genesis = genesis_block();

    if !validate_genesis(&genesis) {
        panic!("Invalid genesis block");
    }

    let chain = ChainState::new(genesis);

    let chain = Arc::new(Mutex::new(chain));

    // 1. Lắng nghe incoming peer
    let listener = TcpListener::bind(("0.0.0.0", P2P_PORT))
        .expect("Bind P2P port failed");

    println!("Node listening on port {}", P2P_PORT);

    // Accept incoming peers
    {
        let chain_accept = Arc::clone(&chain);
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    let chain = Arc::clone(&chain_accept);
                    thread::spawn(move || {
                        let mut chain = chain.lock().unwrap();
                        handle_peer(stream, &mut chain);
                    });
                }
            }
        });
    }

    // 2. Chủ động connect seed peers
    let local_addr: SocketAddr = format!("0.0.0.0:{}", P2P_PORT)
    .parse()
    .unwrap();

// Nếu KHÔNG phải seed node thì mới connect
if !is_seed_node() {
    match TcpStream::connect(crate::config::DEFAULT_SEED) {
        Ok(stream) => {
            println!("Connected to seed {}", crate::config::DEFAULT_SEED);
            if perform_handshake(&stream).is_ok() {
                let chain = chain_ptr.clone();
                thread::spawn(move || {
                    let mut chain = chain.lock().unwrap();
                    handle_peer(stream, &mut chain);
                });
            }
        }
        Err(e) => {
            println!("Cannot connect seed: {}", e);
        }
    }
}

    // Giữ process sống
    loop {
        thread::park();
    }
}

fn is_seed_node() -> bool {
    // Seed node được xác định bằng biến môi trường
    std::env::var("EGG_SEED").is_ok()
}
