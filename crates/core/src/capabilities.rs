use serde::{Deserialize, Serialize};

/// Flags that tell the UI which features are available for a given driver.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DriverCapabilities {
    // Schema
    pub has_structured_schema: bool,  // Tables, columns, constraints (PG: yes, Redis: no)
    pub has_collections: bool,         // Document collections (Mongo: yes)
    pub has_keyspace: bool,            // Flat key-value space (Redis: yes)

    // Query
    pub supports_sql: bool,            // SQL query interface
    pub supports_explain: bool,        // EXPLAIN / query plan visualization
    pub supports_transactions: bool,   // BEGIN/COMMIT/ROLLBACK

    // Data editing
    pub editable_results: bool,        // In-grid row editing and saving

    // Redis-specific
    pub has_queues: bool,              // List-based queues + Redis Streams
    pub has_pubsub: bool,              // PUBLISH/SUBSCRIBE channels
    pub has_server_metrics: bool,      // INFO command metrics panel

    // MongoDB-specific
    pub has_aggregation_pipeline: bool, // Pipeline builder UI
    pub has_document_editor: bool,      // Raw BSON/JSON document editing
}
