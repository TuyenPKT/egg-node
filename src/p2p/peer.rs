use std::net::TcpStream;
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;
use crate::mempool::Mempool;
use crate::orphan::tx::OrphanTxPool;
use crate::orphan::block::OrphanBlockPool;
use crate::net::ban::BanManager;
use crate::net::rate::RateLimiter;

pub fn handle_peer(
    mut stream: TcpStream,
    ip: String,
    chain: Arc<Mutex<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
    orphan_tx: Arc<Mutex<OrphanTxPool>>,
    orphan_block: Arc<Mutex<OrphanBlockPool>>,
    ban: Arc<Mutex<BanManager>>,
    rate: Arc<Mutex<RateLimiter>>,
) {
    let mut buf = [0u8; 8192];

    loop {
        if !rate.lock().unwrap().allow(&ip) {
            if ban.lock().unwrap().add_score(&ip, 20) {
                return;
            }
            continue;
        }

        let size = match stream.read(&mut buf) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let msg: Message = match bincode::deserialize(&buf[..size]) {
            Ok(m) => m,
            Err(_) => {
                ban.lock().unwrap().add_score(&ip, 10);
                continue;
            }
        };

        match msg {
            Message::Tx { tx } => {
                let chain = chain.lock().unwrap();
                let mut mem = mempool.lock().unwrap();

                if !mem.add(tx.clone(), &chain.utxos) {
                    orphan_tx.lock().unwrap().add(tx);
                }
            }

            Message::Block { block } => {
                let mut chain = chain.lock().unwrap();
                if !chain.add_block(block.clone()) {
                    orphan_block.lock().unwrap().add(block);
                    ban.lock().unwrap().add_score(&ip, 5);
                }
            }

            _ => {}
        }
    }
}
