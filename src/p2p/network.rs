use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::chain::state::ChainState;
use crate::p2p::peer::{handle_peer, perform_handshake};

pub fn connect_to_peer(addr: &str, chain: Arc<Mutex<ChainState>>) {
    if let Ok(stream) = TcpStream::connect(addr) {
        if perform_handshake(&stream).is_ok() {
            let mut chain = chain.lock().unwrap();
            handle_peer(stream, &mut chain);
        }
    }
}
