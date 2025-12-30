use std::net::TcpStream;
use std::io::Write;

use crate::p2p::peer::handle_peer;
use crate::p2p::message::Message;
use crate::chain::state::ChainState;

pub fn connect_to_peer(addr: &str, chain: &mut ChainState) {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            println!("Connected to peer {}", addr);

            let msg = Message::GetTip;
            let data = bincode::serialize(&msg).unwrap();
            let _ = stream.write_all(&data);

            handle_peer(stream, chain);
        }
        Err(e) => {
            eprintln!("Cannot connect to {}: {}", addr, e);
        }
    }
}
