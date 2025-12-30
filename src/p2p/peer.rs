use std::net::TcpStream;
use std::io::{Read, Write};

use crate::p2p::message::Handshake;
use crate::chain::genesis_hash;

pub fn perform_handshake(mut stream: TcpStream) -> std::io::Result<()> {
    // gửi handshake của mình
    let my_handshake = Handshake {
        protocol_version: 1,
        genesis_hash: genesis_hash(),
        node_id: crate::node::generate_node_id(),
    };

    let payload = bincode::serialize(&my_handshake).unwrap();
    stream.write_all(&payload)?;

    // nhận handshake từ peer
    let mut buf = [0u8; 512];
    let size = stream.read(&mut buf)?;
    let peer_handshake: Handshake =
        bincode::deserialize(&buf[..size]).unwrap();

    // === BƯỚC 4: KIỂM TRA HANDSHAKE ===
    validate_handshake(&peer_handshake)?;

    Ok(())
}

// ===============================
// BƯỚC 4 — KIỂM TRA HANDSHAKE
// ===============================
fn validate_handshake(peer: &Handshake) -> std::io::Result<()> {
    // kiểm tra version
    if peer.protocol_version != 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Protocol version mismatch",
        ));
    }

    // kiểm tra genesis
    if peer.genesis_hash != genesis_hash() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Genesis hash mismatch",
        ));
    }

    Ok(())
}
