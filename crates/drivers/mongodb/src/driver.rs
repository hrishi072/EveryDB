use async_trait::async_trait;
use everydb_core::{
    capabilities::DriverCapabilities,
    connection::ConnectionConfig,
    driver::DatabaseDriver,
    error::CoreError,
    query::{QueryRequest, QueryResult},
    schema::SchemaNode,
};
use mongodb::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MongoDriver {
    client: Arc<RwLock<Option<Client>>>,
}

impl MongoDriver {
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
        }
    }

    async fn get_client(&self) -> Result<Client, CoreError> {
        let client = self.client.read().await;
        client.clone().ok_or(CoreError::NotConnected)
    }
}

#[async_trait]
impl DatabaseDriver for MongoDriver {
    fn name(&self) -> &'static str {
        "MongoDB"
    }

    fn type_id(&self) -> &'static str {
        "mongodb"
    }

    fn icon(&self) -> &'static str {
        "mongodb-icon"
    }

    async fn connect(&self, config: &ConnectionConfig) -> Result<(), CoreError> {
        let uri = if config.host.contains("://") {
            config.host.clone()
        } else {
            format!(
                "mongodb://{}:{}@{}:{}/{}",
                config.username.as_deref().unwrap_or(""),
                config.password.as_deref().unwrap_or(""),
                config.host,
                config.port,
                config.database.as_deref().unwrap_or("admin")
            )
        };

        let client = Client::with_uri_str(&uri)
            .await
            .map_err(|e| CoreError::ConnectionError(e.to_string()))?;

        // Ping the server to verify connection
        client
            .database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await
            .map_err(|e| CoreError::ConnectionError(format!("Ping failed: {}", e)))?;

        let mut client_lock = self.client.write().await;
        *client_lock = Some(client);

        Ok(())
    }

    async fn disconnect(&self) -> Result<(), CoreError> {
        let mut client_lock = self.client.write().await;
        *client_lock = None;
        Ok(())
    }

    async fn is_connected(&self) -> bool {
        self.client.read().await.is_some()
    }

    async fn execute(&self, req: QueryRequest) -> Result<QueryResult, CoreError> {
        let client = self.get_client().await?;
        crate::query::execute_query(&client, req).await
    }

    async fn introspect_schema(&self) -> Result<Vec<SchemaNode>, CoreError> {
        let client = self.get_client().await?;
        crate::schema::introspect(&client).await
    }

    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            has_collections: true,
            has_aggregation_pipeline: true,
            has_document_editor: true,
            ..Default::default()
        }
    }
}
