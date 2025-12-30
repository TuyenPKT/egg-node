use std::net::TcpStream;
use std::io::{Read, Write};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;
use crate::chain::hash::hash_header;
use crate::chain::genesis_hash;
use crate::node::generate_node_id;

//
// ===== HANDSHAKE =====
//

pub fn perform_handshake(stream: &TcpStream) -> std::io::Result<()> {
    // Clone stream chỉ để handshake (read/write)
    let mut stream = stream.try_clone()?;

    let msg = Message::Handshake {
        protocol_version: 1,
        genesis_hash: genesis_hash(),
        node_id: generate_node_id(),
    };

    send(&mut stream, &msg);

    let mut buf = [0u8; 512];
    let size = stream.read(&mut buf)?;

    let peer_msg: Message = bincode::deserialize(&buf[..size])
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid handshake message"
        ))?;

    match peer_msg {
        Message::Handshake {
            protocol_version,
            genesis_hash,
            ..
        } => {
            if protocol_version != 1 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Protocol version mismatch",
                ));
            }

            if genesis_hash != genesis_hash() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Genesis hash mismatch",
                ));
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Expected handshake message",
            ));
        }
    }

    Ok(())
}

//
// ===== PEER MESSAGE LOOP =====
//

pub fn handle_peer(mut stream: TcpStream, chain: &mut ChainStat_
