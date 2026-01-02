use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;
use crate::chain::block::Block;
use crate::chain::hash::hash_header;
use crate::mempool::Mempool;


pub fn handle_peer(
    mut stream: TcpStream,
    chain: Arc<Mutex<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
) {
    let mut buf = [0u8; 8192];

    loop {
        let size = match stream.read(&mut buf) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let msg: Message = bincode::deserialize(&buf[..size]).unwrap();

        match msg {
            Message::GetHeaders { from, limit } => {
                let chain = chain.lock().unwrap();
                let mut headers = Vec::new();
                let mut cur = from;

                for _ in 0..limit {
                    let meta = match chain.blocks.get(&cur) {
                        Some(m) => m,
                        None => break,
                    };
                    headers.push(meta.block.header.clone());
                    cur = hash_header(&meta.block.header);
                }

                send(&mut stream, &Message::Headers { headers });
            }

            Message::Headers { headers } => {
                // headers-first: chỉ verify, không apply block
                let chain = chain.lock().unwrap();
                for h in headers {
                    let _ = chain.accept_header(&h);
                }
            }

            Message::CompactBlock { header, txids } => {
                // ===== COMPACT RECONSTRUCTION =====
                let mut txs = Vec::new();
                let mut missing = false;

                let mem = mempool.lock().unwrap();

                for id in txids.iter() {
                    if let Some(mtx) = mem.txs.get(id) {
                        txs.push(mtx.tx.clone());
                    } else {
                        missing = true;
                        break;
                    }
                }

                drop(mem);

                if missing {
                    let hash = hash_header(&header);
                    send(&mut stream, &Message::GetBlock { hash });
                    continue;
                }

let block = Block {
    header,
    transactions: txs,
};

// tính hash TRƯỚC khi move
let hash = hash_header(&block.header);

let mut chain = chain.lock().unwrap();
if !chain.add_block(block) {
    send(&mut stream, &Message::GetBlock { hash });
}

            }

            Message::Block { block } => {
                let mut chain = chain.lock().unwrap();
                chain.add_block(block);
            }

            _ => {}
        }
    }
}

fn send(stream: &mut TcpStream, msg: &Message) {
    let data = bincode::serialize(msg).unwrap();
    let _ = stream.write_all(&data);
}
