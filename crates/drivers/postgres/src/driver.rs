use async_trait::async_trait;
use everydb_core::{
    capabilities::DriverCapabilities,
    connection::ConnectionConfig,
    driver::DatabaseDriver,
    error::CoreError,
    query::{QueryRequest, QueryResult},
    schema::SchemaNode,
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PostgresDriver {
    pool: Arc<RwLock<Option<PgPool>>>,
}

impl PostgresDriver {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(RwLock::new(None)),
        }
    }

    async fn get_pool(&self) -> Result<PgPool, CoreError> {
        let pool = self.pool.read().await;
        pool.clone().ok_or(CoreError::NotConnected)
    }
}

#[async_trait]
impl DatabaseDriver for PostgresDriver {
    fn name(&self) -> &'static str {
        "PostgreSQL"
    }

    fn type_id(&self) -> &'static str {
        "postgres"
    }

    fn icon(&self) -> &'static str {
        "postgres-icon"
    }

    async fn connect(&self, config: &ConnectionConfig) -> Result<(), CoreError> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username.as_deref().unwrap_or("postgres"),
            config.password.as_deref().unwrap_or(""),
            config.host,
            config.port,
            config.database.as_deref().unwrap_or("postgres")
        );

        let pool = PgPool::connect(&url)
            .await
            .map_err(|e| CoreError::ConnectionError(e.to_string()))?;

        let mut pool_lock = self.pool.write().await;
        *pool_lock = Some(pool);

        Ok(())
    }

    async fn disconnect(&self) -> Result<(), CoreError> {
        let mut pool_lock = self.pool.write().await;
        if let Some(pool) = pool_lock.take() {
            pool.close().await;
        }
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.pool.read().await.is_some()
    }

    async fn execute(&self, req: QueryRequest) -> Result<QueryResult, CoreError> {
        let pool = self.get_pool().await?;
        // Implementation will go in query.rs
        crate::query::execute_query(&pool, req).await
    }

    async fn introspect_schema(&self) -> Result<Vec<SchemaNode>, CoreError> {
        let pool = self.get_pool().await?;
        // Implementation will go in schema.rs
        crate::schema::introspect(&pool).await
    }

    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            has_structured_schema: true,
            supports_sql: true,
            supports_explain: true,
            supports_transactions: true,
            editable_results: true,
            ..Default::default()
        }
    }
}
