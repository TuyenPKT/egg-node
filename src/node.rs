use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::config::NodeConfig;
use crate::chain::state::ChainState;
use crate::p2p::peer::handle_peer;

pub fn run_node(config: NodeConfig, mut chain: ChainState) {
    // 1. LISTEN
    let listener = TcpListener::bind(&config.bind_addr)
        .expect("Cannot bind address");

    println!("Node listening on {}", config.bind_addr);

    // 2. ACCEPT INBOUND PEER
    let chain_ptr = std::sync::Arc::new(std::sync::Mutex::new(chain));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let chain_clone = chain_ptr.clone();
                thread::spawn(move || {
                    let mut chain = chain_clone.lock().unwrap();
                    handle_peer(stream, &mut chain);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
