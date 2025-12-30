use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

use crate::config::{P2P_PORT, DEFAULT_SEED};
use crate::chain::state::ChainState;
use crate::chain::genesis_block;
use crate::p2p::peer::{handle_peer, perform_handshake};
use crate::chain::validation::validate_genesis;

pub fn run_node() {
    // 0. Genesis check
    let genesis = genesis_block();
    if !validate_genesis(&genesis) {
        panic!("Invalid genesis block");
    }

    let chain = Arc::new(Mutex::new(ChainState::new(genesis)));

    // 1. Listen incoming peers
    let listener = TcpListener::bind(("0.0.0.0", P2P_PORT))
        .expect("Bind P2P port failed");

    println!("Node listening on port {}", P2P_PORT);

    // Accept incoming connections
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

    // 2. Client node connects to seed (NOT seed node)
    if !is_seed_node() {
        match TcpStream::connect(DEFAULT_SEED) {
            Ok(stream) => {
                println!("Connected to seed {}", DEFAULT_SEED);
                if perform_handshake(&stream).is_ok() {
                    let chain = Arc::clone(&chain);
                    thread::spawn(move || {
                        let mut chain = chain.lock().unwrap();
                        handle_peer(stream, &mut chain);
                    });
                }
            }
            Err(e) => {
                println!("Cannot connect seed {}: {}", DEFAULT_SEED, e);
            }
        }
    }

    // 3. Keep process alive
    loop {
        thread::park();
    }
}

fn is_seed_node() -> bool {
    std::env::var("EGG_SEED").is_ok()
}
