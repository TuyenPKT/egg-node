use std::net::TcpStream;
use std::io::{Read, Write};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;

use rand::RngCore;


pub fn perform_handshake(stream: &TcpStream) -> std::io::Result<()> {
    let mut stream = stream.try_clone()?;

    let mut node_id = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut node_id);

    let handshake = Message::Handshake {
        protocol_version: 1,
        genesis_hash: crate::chain::genesis_hash(),
        node_id: [0u8; 32],
    };

    let data = bincode::serialize(&handshake).unwrap();
    stream.write_all(&data)?;

    let mut buf = [0u8; 1024];
    let size = stream.read(&mut buf)?;

    let peer_msg: Message = bincode::deserialize(&buf[..size]).unwrap();

    match peer_msg {
        Message::Handshake {
            protocol_version,
            genesis_hash,
            ..
        } => {
            if protocol_version != 1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Protocol mismatch",
                ));
            }

            if genesis_hash != crate::chain::genesis_hash() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Genesis mismatch",
                ));
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid handshake",
            ));
        }
    }

    Ok(())
}

pub fn handle_peer(mut stream: TcpStream, chain: &mut ChainState) {
    let mut buf = [0u8; 8192];

    loop {
        let size = match stream.read(&mut buf) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let msg: Message = match bincode::deserialize(&buf[..size]) {
            Ok(m) => m,
            Err(_) => return,
        };

        match msg {
            Message::GetTip => {
                let reply = Message::Tip {
                    hash: chain.tip,
                    height: chain.height,
                };
                send(&mut stream, &reply);
            }

            Message::Tip { hash, .. } => {
                if !chain.has_block(&hash) {
                    let req = Message::GetBlock { hash };
                    send(&mut stream, &req);
                }
            }

            Message::GetBlock { hash } => {
                if let Some(block) = chain.blocks.get(&hash) {
                    let reply = Message::Block {
                        block: block.clone(),
                    };
                    send(&mut stream, &reply);
                }
            }

            Message::Block { block } => {
                let prev = block.header.prev_hash;
                if chain.has_block(&prev) {
                    chain.add_block(block);
                }
            }

            _ => {}
        }
    }
}

fn send(stream: &mut TcpStream, msg: &Message) {
    let data = bincode::serialize(msg).unwrap();
    let _ = stream.write_all(&data);
}
