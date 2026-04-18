use everydb_core::{
    error::CoreError,
    query::{ColumnInfo, QueryRequest, QueryResult},
};
use sqlx::{postgres::PgPool, Column, Executor, Row, TypeInfo};
use std::time::Instant;

pub async fn execute_query(pool: &PgPool, req: QueryRequest) -> Result<QueryResult, CoreError> {
    let start = Instant::now();

    // For now, we don't support parameters in this simple scaffold
    // In a real app, we'd bind parameters to the query
    let rows = sqlx::query(&req.raw_query)
        .fetch_all(pool)
        .await
        .map_err(|e| CoreError::QueryError(e.to_string()))?;

    let execution_time_ms = start.elapsed().as_millis() as u64;

    if rows.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
            execution_time_ms,
            rows_affected: 0,
        });
    }

    let columns = rows[0]
        .columns()
        .iter()
        .map(|col| ColumnInfo {
            name: col.name().to_string(),
            data_type: col.type_info().name().to_string(),
        })
        .collect();

    let core_rows = rows.iter().map(crate::types::pg_row_to_core_row).collect();

    Ok(QueryResult {
        columns,
        rows: core_rows,
        execution_time_ms,
        rows_affected: rows.len() as u64, // simplified
    })
}

pub fn parse_explain_json(json_str: &str) -> Result<serde_json::Value, CoreError> {
    serde_json::from_str(json_str)
        .map_err(|e| CoreError::InternalError(format!("Failed to parse EXPLAIN JSON: {}", e)))
}
