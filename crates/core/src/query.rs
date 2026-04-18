use serde::{Deserialize, Serialize};
use crate::value::CoreValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub raw_query: String,
    pub parameters: Vec<CoreValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Row>,
    pub execution_time_ms: u64,
    pub rows_affected: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub cells: Vec<CoreValue>,
}
