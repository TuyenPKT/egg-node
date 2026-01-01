use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;
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
            Message::GetTip => {
                let chain = chain.lock().unwrap();
                let reply = Message::Tip {
                    hash: chain.tip,
                    height: chain.blocks.get(&chain.tip).unwrap().height,
                };
                send(&mut stream, &reply);
            }

            Message::GetBlock { hash } => {
                let chain = chain.lock().unwrap();
                if let Some(meta) = chain.blocks.get(&hash) {
                    send(&mut stream, &Message::Block {
                        block: meta.block.clone(),
                    });
                }
            }

            Message::Block { block } => {
                let mut chain = chain.lock().unwrap();
                let mut mempool = mempool.lock().unwrap();

                if chain.add_block(block.clone()) {
                    for tx in block.transactions.iter().skip(1) {
                        mempool.remove(tx);
                    }
                }
            }

            Message::Tx { tx } => {
                let mut mempool = mempool.lock().unwrap();
                let chain = chain.lock().unwrap();

                if mempool.add(tx.clone(), &chain.utxos) {
                    // rebroadcast
                    send(&mut stream, &Message::Tx { tx });
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
