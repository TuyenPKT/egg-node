use std::collections::HashMap;
use std::time::{Instant, Duration};

const BAN_THRESHOLD: i32 = 100;
const BAN_TIME: Duration = Duration::from_secs(60 * 60);

#[derive(Clone)]
pub struct PeerScore {
    pub score: i32,
    pub banned_until: Option<Instant>,
}

pub struct BanManager {
    peers: HashMap<String, PeerScore>,
}

impl BanManager {
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, ip: &str, delta: i32) -> bool {
        let entry = self.peers.entry(ip.to_string()).or_insert(
            PeerScore {
                score: 0,
                banned_until: None,
            }
        );

        // decay nhẹ
        entry.score -= 1;
        if entry.score < 0 {
            entry.score = 0;
        }

        entry.score += delta;

        if entry.score >= BAN_THRESHOLD {
            entry.banned_until = Some(Instant::now() + BAN_TIME);
            return true;
        }
        false
    }

    pub fn is_banned(&mut self, ip: &str) -> bool {
        if let Some(peer) = self.peers.get_mut(ip) {
            if let Some(until) = peer.banned_until {
                if Instant::now() < until {
                    return true;
                } else {
                    peer.banned_until = None;
                    peer.score = 0;
                }
            }
        }
        false
    }
}
