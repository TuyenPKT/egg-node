#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Wallet,
    Mining,
    Governance,
    Node,
    AI,
    Module(String),
}

impl Role {
    pub fn as_domain(&self) -> String {
        match self {
            Role::Wallet => "role:wallet".to_string(),
            Role::Mining => "role:mining".to_string(),
            Role::Governance => "role:governance".to_string(),
            Role::Node => "role:node".to_string(),
            Role::AI => "role:ai".to_string(),
            Role::Module(name) => format!("role:module:{}", name),
        }
    }
}
