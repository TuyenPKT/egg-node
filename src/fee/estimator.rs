use crate::mempool::Mempool;

#[derive(Debug)]
pub struct FeeEstimate {
    pub low: u64,
    pub medium: u64,
    pub high: u64,
}

pub struct FeeEstimator {
    pub history: Vec<Vec<u64>>, // fee list per block
    pub max_blocks: usize,
}

impl FeeEstimator {
    pub fn new(max_blocks: usize) -> Self {
        FeeEstimator {
            history: Vec::new(),
            max_blocks,
        }
    }

    pub fn record_block(&mut self, fees: Vec<u64>) {
        self.history.push(fees);
        if self.history.len() > self.max_blocks {
            self.history.remove(0);
        }
    }

    pub fn estimate(&self, mempool: &Mempool) -> FeeEstimate {
        let mut samples: Vec<u64> = Vec::new();

        // from mempool
        samples.extend(mempool.fee_list());

        // from recent blocks
        for block_fees in &self.history {
            samples.extend(block_fees.clone());
        }

        if samples.is_empty() {
            return FeeEstimate {
                low: 1,
                medium: 1,
                high: 1,
            };
        }

        samples.sort();

        let len = samples.len();
        let p25 = samples[len * 25 / 100];
        let p50 = samples[len * 50 / 100];
        let p75 = samples[len * 75 / 100];

        FeeEstimate {
            low: p25.max(1),
            medium: p50.max(1),
            high: p75.max(1),
        }
    }
}
