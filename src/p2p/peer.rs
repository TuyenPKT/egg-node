use std::net::TcpStream;
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::p2p::message::Message;
use crate::chain::state::ChainState;
use crate::chain::hash::hash_header;
use crate::mempool::Mempool;
use crate::orphan::tx::OrphanTxPool;
use crate::orphan::block::OrphanBlockPool;

pub fn handle_peer(
    mut stream: TcpStream,
    chain: Arc<Mutex<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
    orphan_tx: Arc<Mutex<OrphanTxPool>>,
    orphan_block: Arc<Mutex<OrphanBlockPool>>,
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
            // ======================================================
            // TRANSACTION
            // ======================================================
            Message::Tx { tx } => {
                let chain_guard = chain.lock().unwrap();
                let mut mem_guard = mempool.lock().unwrap();

                if !mem_guard.add(tx.clone(), &chain_guard.utxos) {
                    orphan_tx.lock().unwrap().add(tx);
                }
            }

            // ======================================================
            // BLOCK
            // ======================================================
            Message::Block { block } => {
                let mut chain_guard = chain.lock().unwrap();

                if chain_guard.add_block(block.clone()) {
                    // -------------------------------
                    // PROMOTE ORPHAN TX (SAFE)
                    // -------------------------------
                    {
                        let utxos_snapshot = chain_guard.utxos.clone();
                        orphan_tx.lock().unwrap().try_promote(
                            &utxos_snapshot,
                            |tx| {
                                mempool.lock().unwrap().add(tx, &utxos_snapshot);
                            },
                        );
                    }

                    // -------------------------------
                    // PROMOTE ORPHAN BLOCKS (2-PHASE)
                    // -------------------------------

                    // Phase 1: collect promotable blocks (immutable access only)
                    let promotable_blocks: Vec<_> = {
                        let ob = orphan_block.lock().unwrap();
                        ob.blocks
                            .values()
                            .filter(|o| {
                                chain_guard
                                    .blocks
                                    .contains_key(&o.block.header.prev_hash)
                            })
                            .map(|o| o.block.clone())
                            .collect()
                    };

                    // Phase 2: apply blocks (mutable access)
                    for b in promotable_blocks {
                        if chain_guard.add_block(b.clone()) {
                            orphan_block
                                .lock()
                                .unwrap()
                                .blocks
                                .remove(&hash_header(&b.header));
                        }
                    }
                } else {
                    // orphan block
                    orphan_block.lock().unwrap().add(block);
                }
            }

            _ => {}
        }
    }
}


