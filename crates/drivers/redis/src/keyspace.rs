use everydb_core::{
    error::CoreError,
    schema::{SchemaNode, SchemaNodeType},
};
use redis::AsyncCommands;
use std::collections::HashMap;

pub async fn introspect(
    conn: &mut deadpool_redis::Connection,
) -> Result<Vec<SchemaNode>, CoreError> {
    let mut keys_node = SchemaNode {
        name: "Keys".to_string(),
        node_type: SchemaNodeType::Key, // Simplified mapping
        children: vec![],
        metadata: HashMap::new(),
    };

    // Use SCAN to avoid blocking
    let mut iter: redis::AsyncIter<String> = redis::cmd("SCAN")
        .arg(0)
        .arg("COUNT")
        .arg(100)
        .iter_async(conn)
        .await
        .map_err(|e| CoreError::DatabaseError(e.to_string()))?;

    while let Some(key) = iter.next_item().await {
        keys_node.children.push(SchemaNode {
            name: key,
            node_type: SchemaNodeType::Key,
            children: vec![],
            metadata: HashMap::new(),
        });

        // Limit to 100 keys for introspection in this scaffold
        if keys_node.children.len() >= 100 {
            break;
        }
    }

    Ok(vec![keys_node])
}
