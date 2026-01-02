use crate::chain::utxo::UTXO;

/// Undo information cho 1 block
/// Dùng để rollback UTXO khi reorg
#[derive(Clone)]
pub struct BlockUndo {
    /// Các UTXO đã bị consume trong block này
    pub spent: Vec<UTXO>,

    /// Các UTXO được tạo ra trong block này (coinbase + outputs)
    pub created: Vec<UTXO>,
}

impl BlockUndo {
    pub fn new() -> Self {
        BlockUndo {
            spent: Vec::new(),
            created: Vec::new(),
        }
    }
}
