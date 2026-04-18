use everydb_core::{
    error::CoreError,
    schema::{SchemaNode, SchemaNodeType},
};
use futures::stream::StreamExt;
use mongodb::Client;
use std::collections::HashMap;

pub async fn introspect(client: &Client) -> Result<Vec<SchemaNode>, CoreError> {
    let mut db_nodes = Vec::new();

    let db_names = client
        .list_database_names(None, None)
        .await
        .map_err(|e| CoreError::DatabaseError(e.to_string()))?;

    for db_name in db_names {
        let mut db_node = SchemaNode {
            name: db_name.clone(),
            node_type: SchemaNodeType::Database,
            children: vec![],
            metadata: HashMap::new(),
        };

        let db = client.database(&db_name);
        let mut collections = db
            .list_collections(None, None)
            .await
            .map_err(|e| CoreError::DatabaseError(e.to_string()))?;

        while let Some(result) = collections.next().await {
            let coll_spec = result.map_err(|e| CoreError::DatabaseError(e.to_string()))?;
            db_node.children.push(SchemaNode {
                name: coll_spec.name,
                node_type: SchemaNodeType::Collection,
                children: vec![],
                metadata: HashMap::new(),
            });
        }

        db_nodes.push(db_node);
    }

    Ok(db_nodes)
}
