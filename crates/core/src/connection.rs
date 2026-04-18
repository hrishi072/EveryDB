use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub driver_type: String, // e.g., "postgres", "mongodb", "redis"
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub use_tls: bool,
    pub extra_options: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_state_equality() {
        assert_eq!(ConnectionState::Disconnected, ConnectionState::Disconnected);
        assert_ne!(ConnectionState::Disconnected, ConnectionState::Connecting);
        assert_eq!(ConnectionState::Error("foo".to_string()), ConnectionState::Error("foo".to_string()));
        assert_ne!(ConnectionState::Error("foo".to_string()), ConnectionState::Error("bar".to_string()));
    }
}
