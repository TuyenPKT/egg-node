use std::net::TcpStream;
use std::io::{Read, Write};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;
use crate::chain::genesis_hash;
use crate::node::generate_node_id;

pub fn outbound_connect(
    mut stream: TcpStream,
    chain: &mut ChainState,
) -> std::io::Result<()> {
    // --- SEND HANDSHAKE ---
    let handshake = Message::Handshake {
        protocol_version: 1,
        genesis_hash: genesis_hash(),
        node_id: generate_node_id(),
    };
    send(&mut stream, &handshake);

    // --- RECEIVE HANDSHAKE ---
    let peer_msg = recv(&mut stream)?;
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

    // --- AFTER HANDSHAKE ---
    handle_peer(stream, chain);
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

        let msg: Message = bincode::deserialize(&buf[..size]).unwrap();

        match msg {
            Message::GetTip => {
                let meta = chain.blocks.get(&chain.tip).unwrap();

                let reply = Message::Tip {
                    hash: chain.tip,
                    height: meta.height,
                };
                send(&mut stream, &reply);
            }


            Message::Tip { hash, .. } => {
                if !chain.blocks.contains_key(&hash) {
                    let req = Message::GetBlock { hash };
                    send(&mut stream, &req);
                }
            }


            Message::GetBlock { hash } => {
                if let Some(meta) = chain.blocks.get(&hash) {
                    let reply = Message::Block {
                        block: meta.block.clone(),
                    };
                    send(&mut stream, &reply);
                }
            }


            Message::Block { block } => {
                if !chain.add_block(block) {
                    eprintln!("Rejected invalid PoW block");
                    return;
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

fn recv(stream: &mut TcpStream) -> std::io::Result<Message> {
    let mut buf = [0u8; 1024];
    let size = stream.read(&mut buf)?;
    Ok(bincode::deserialize(&buf[..size]).unwrap())
}
