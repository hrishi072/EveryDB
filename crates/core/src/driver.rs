use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    capabilities::DriverCapabilities,
    connection::ConnectionConfig,
    query::{QueryRequest, QueryResult},
    schema::SchemaNode,
    error::CoreError,
};

/// The single extension point for all database integrations.
/// Implement this trait in a new crate under `crates/drivers/` to add any database.
#[async_trait]
pub trait DatabaseDriver: Send + Sync + 'static {
    /// Human-readable display name, e.g. "PostgreSQL 15"
    fn name(&self) -> &'static str;

    /// Unique stable identifier used in config files, e.g. "postgres"
    fn type_id(&self) -> &'static str;

    /// Icon name from the assets bundle, e.g. "postgres-icon"
    fn icon(&self) -> &'static str;

    /// Test connectivity and establish the internal connection pool
    async fn connect(&self, config: &ConnectionConfig) -> Result<(), CoreError>;

    /// Gracefully disconnect and release all pool resources
    async fn disconnect(&self) -> Result<(), CoreError>;

    /// Returns true if currently connected
    async fn is_connected(&self) -> bool;

    /// Execute a query or command, return structured results
    async fn execute(&self, req: QueryRequest) -> Result<QueryResult, CoreError>;

    /// Return the full schema tree for the sidebar explorer
    async fn introspect_schema(&self) -> Result<Vec<SchemaNode>, CoreError>;

    /// Capabilities bitmask — the UI uses this to show/hide features
    fn capabilities(&self) -> DriverCapabilities;
}

/// Central registry — the app crate registers all drivers here at startup.
/// Everything else uses `Arc<dyn DatabaseDriver>` — never concrete types.
pub struct DriverRegistry {
    drivers: std::collections::HashMap<String, Arc<dyn DatabaseDriver>>,
}

impl Default for DriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self { drivers: Default::default() }
    }

    pub fn register(&mut self, driver: Arc<dyn DatabaseDriver>) {
        self.drivers.insert(driver.type_id().to_string(), driver);
    }

    pub fn get(&self, type_id: &str) -> Option<Arc<dyn DatabaseDriver>> {
        self.drivers.get(type_id).cloned()
    }

    pub fn all(&self) -> impl Iterator<Item = &Arc<dyn DatabaseDriver>> {
        self.drivers.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDriver {
        id: String,
    }

    #[async_trait]
    impl DatabaseDriver for MockDriver {
        fn name(&self) -> &'static str { "Mock" }
        fn type_id(&self) -> &'static str { Box::leak(self.id.clone().into_boxed_str()) }
        fn icon(&self) -> &'static str { "mock-icon" }
        async fn connect(&self, _config: &ConnectionConfig) -> Result<(), CoreError> { Ok(()) }
        async fn disconnect(&self) -> Result<(), CoreError> { Ok(()) }
        async fn is_connected(&self) -> bool { true }
        async fn execute(&self, _req: QueryRequest) -> Result<QueryResult, CoreError> {
            Ok(QueryResult {
                columns: vec![],
                rows: vec![],
                execution_time_ms: 0,
                rows_affected: 0,
            })
        }
        async fn introspect_schema(&self) -> Result<Vec<SchemaNode>, CoreError> { Ok(vec![]) }
        fn capabilities(&self) -> DriverCapabilities { DriverCapabilities::default() }
    }

    #[test]
    fn test_registry() {
        let mut registry = DriverRegistry::new();
        let driver = Arc::new(MockDriver { id: "test-db".to_string() });
        registry.register(driver.clone());

        let retrieved = registry.get("test-db").unwrap();
        assert_eq!(retrieved.name(), "Mock");

        assert!(registry.get("invalid").is_none());
        assert_eq!(registry.all().count(), 1);
    }
}
