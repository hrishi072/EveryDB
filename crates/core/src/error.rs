use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "message")]
pub enum CoreError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Not connected")]
    NotConnected,

    #[error("Feature not supported: {0}")]
    UnsupportedFeature(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_serialization() {
        let err = CoreError::DatabaseError("connection failed".to_string());
        let serialized = serde_json::to_string(&err).unwrap();
        let deserialized: CoreError = serde_json::from_str(&serialized).unwrap();

        match deserialized {
            CoreError::DatabaseError(msg) => assert_eq!(msg, "connection failed"),
            _ => panic!("Wrong error type"),
        }
    }
}
