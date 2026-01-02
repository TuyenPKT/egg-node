use std::collections::HashMap;
use std::time::{Instant, Duration};

const WINDOW: Duration = Duration::from_secs(1);
const MAX_MSG_PER_WINDOW: u32 = 50;

pub struct RateLimiter {
    peers: HashMap<String, (u32, Instant)>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn allow(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        let entry = self.peers.entry(ip.to_string()).or_insert((0, now));

        if now.duration_since(entry.1) > WINDOW {
            entry.0 = 0;
            entry.1 = now;
        }

        entry.0 += 1;
        entry.0 <= MAX_MSG_PER_WINDOW
    }
}
