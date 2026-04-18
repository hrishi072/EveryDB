use everydb_core::{
    error::CoreError,
    query::{ColumnInfo, QueryRequest, QueryResult},
};
use futures::stream::StreamExt;
use mongodb::Client;
use std::time::Instant;

pub async fn execute_query(client: &Client, req: QueryRequest) -> Result<QueryResult, CoreError> {
    let start = Instant::now();

    // Parse the raw query as a command or find operation
    // For now, let's assume it's "db_name.collection_name.find({filter})"
    // A more sophisticated parser is needed.

    let parts: Vec<&str> = req.raw_query.splitn(3, '.').collect();
    if parts.len() < 3 {
        return Err(CoreError::QueryError(
            "Query must be in format db.collection.find(filter)".to_string(),
        ));
    }

    let db_name = parts[0];
    let coll_name = parts[1];
    let op_part = parts[2];

    if !op_part.starts_with("find") {
        return Err(CoreError::UnsupportedFeature(
            "Only 'find' is supported in this scaffold".to_string(),
        ));
    }

    // Simple regex or string parsing to get filter
    let filter_str = if let Some(start) = op_part.find('(') {
        if let Some(end) = op_part.rfind(')') {
            &op_part[start + 1..end]
        } else {
            "{}"
        }
    } else {
        "{}"
    };

    let filter: mongodb::bson::Document = serde_json::from_str(if filter_str.trim().is_empty() {
        "{}"
    } else {
        filter_str
    })
    .map_err(|e| CoreError::QueryError(format!("Invalid filter JSON: {}", e)))?;

    let coll = client
        .database(db_name)
        .collection::<mongodb::bson::Document>(coll_name);
    let mut cursor = coll
        .find(filter, None)
        .await
        .map_err(|e| CoreError::QueryError(e.to_string()))?;

    let mut rows = Vec::new();
    let mut column_set = std::collections::HashSet::new();

    while let Some(result) = cursor.next().await {
        let doc = result.map_err(|e| CoreError::QueryError(e.to_string()))?;
        for key in doc.keys() {
            column_set.insert(key.clone());
        }
        rows.push(crate::types::mongo_doc_to_core_row(doc));
    }

    let columns = column_set
        .into_iter()
        .map(|name| ColumnInfo {
            name,
            data_type: "BSON".to_string(),
        })
        .collect();

    Ok(QueryResult {
        columns,
        rows,
        execution_time_ms: start.elapsed().as_millis() as u64,
        rows_affected: 0,
    })
}
