use everydb_core::{
    error::CoreError,
    schema::{SchemaNode, SchemaNodeType},
};
use sqlx::{postgres::PgPool, Row};
use std::collections::HashMap;

pub async fn introspect(pool: &PgPool) -> Result<Vec<SchemaNode>, CoreError> {
    // Basic query to get schemas and tables
    let rows = sqlx::query(
        "SELECT table_schema, table_name, table_type
         FROM information_schema.tables
         WHERE table_schema NOT IN ('information_schema', 'pg_catalog')
         ORDER BY table_schema, table_name",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| CoreError::DatabaseError(e.to_string()))?;

    let mut schemas: HashMap<String, SchemaNode> = HashMap::new();

    for row in rows {
        let schema_name: String = row.get("table_schema");
        let table_name: String = row.get("table_name");
        let table_type: String = row.get("table_type");

        let node_type = if table_type == "VIEW" {
            SchemaNodeType::View
        } else {
            SchemaNodeType::Table
        };

        let table_node = SchemaNode {
            name: table_name,
            node_type,
            children: vec![],
            metadata: HashMap::new(),
        };

        schemas
            .entry(schema_name.clone())
            .or_insert_with(|| SchemaNode {
                name: schema_name,
                node_type: SchemaNodeType::Schema,
                children: vec![],
                metadata: HashMap::new(),
            })
            .children
            .push(table_node);
    }

    Ok(schemas.into_values().collect())
}
