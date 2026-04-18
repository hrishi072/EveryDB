use async_trait::async_trait;
use deadpool_redis::{Config, Pool, Runtime};
use everydb_core::{
    capabilities::DriverCapabilities,
    connection::ConnectionConfig,
    driver::DatabaseDriver,
    error::CoreError,
    query::{QueryRequest, QueryResult},
    schema::SchemaNode,
};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RedisDriver {
    pool: Arc<RwLock<Option<Pool>>>,
}

impl RedisDriver {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(RwLock::new(None)),
        }
    }

    async fn get_pool(&self) -> Result<Pool, CoreError> {
        let pool = self.pool.read().await;
        pool.clone().ok_or(CoreError::NotConnected)
    }
}

#[async_trait]
impl DatabaseDriver for RedisDriver {
    fn name(&self) -> &'static str {
        "Redis"
    }

    fn type_id(&self) -> &'static str {
        "redis"
    }

    fn icon(&self) -> &'static str {
        "redis-icon"
    }

    async fn connect(&self, config: &ConnectionConfig) -> Result<(), CoreError> {
        let url = format!(
            "redis://{}:{}@{}:{}/{}",
            config.username.as_deref().unwrap_or(""),
            config.password.as_deref().unwrap_or(""),
            config.host,
            config.port,
            config.database.as_deref().unwrap_or("0")
        );

        let mut cfg = Config::from_url(url);
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1))
            .map_err(|e| CoreError::ConnectionError(e.to_string()))?;

        // Test connection
        let mut conn = pool
            .get()
            .await
            .map_err(|e| CoreError::ConnectionError(e.to_string()))?;
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| CoreError::ConnectionError(e.to_string()))?;

        let mut pool_lock = self.pool.write().await;
        *pool_lock = Some(pool);

        Ok(())
    }

    async fn disconnect(&self) -> Result<(), CoreError> {
        let mut pool_lock = self.pool.write().await;
        *pool_lock = None;
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.pool.read().await.is_some()
    }

    async fn execute(&self, req: QueryRequest) -> Result<QueryResult, CoreError> {
        let pool = self.get_pool().await?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| CoreError::DatabaseError(e.to_string()))?;

        // Execute raw redis command
        let parts: Vec<&str> = req.raw_query.split_whitespace().collect();
        if parts.is_empty() {
            return Err(CoreError::QueryError("Empty query".to_string()));
        }

        let mut cmd = redis::cmd(parts[0]);
        for part in parts.iter().skip(1) {
            cmd.arg(part);
        }

        let start = std::time::Instant::now();
        let value: redis::Value = cmd
            .query_async(&mut conn)
            .await
            .map_err(|e| CoreError::QueryError(e.to_string()))?;
        let execution_time_ms = start.elapsed().as_millis() as u64;

        // Map redis::Value to QueryResult
        Ok(crate::types::redis_value_to_query_result(
            value,
            execution_time_ms,
        ))
    }

    async fn introspect_schema(&self) -> Result<Vec<SchemaNode>, CoreError> {
        let pool = self.get_pool().await?;
        let mut conn = pool
            .get()
            .await
            .map_err(|e| CoreError::DatabaseError(e.to_string()))?;
        crate::keyspace::introspect(&mut conn).await
    }

    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            has_keyspace: true,
            has_queues: true,
            has_pubsub: true,
            has_server_metrics: true,
            ..Default::default()
        }
    }
}
