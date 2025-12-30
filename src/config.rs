#[derive(Clone)]
pub struct NodeConfig {
    pub bind_addr: String,
    pub peers: Vec<String>,
}

impl NodeConfig {
    pub fn default() -> Self {
        NodeConfig {
            bind_addr: "0.0.0.0:8333".to_string(),
            peers: vec![],
        }
    }
}
