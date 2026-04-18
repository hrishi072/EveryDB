use everydb_core::{
    query::{ColumnInfo, QueryResult, Row},
    value::CoreValue,
};
use redis::Value;

pub fn redis_value_to_core_value(val: Value) -> CoreValue {
    match val {
        Value::Nil => CoreValue::Null,
        Value::Int(i) => CoreValue::Integer(i),
        Value::Data(bytes) => {
            if let Ok(s) = String::from_utf8(bytes.clone()) {
                CoreValue::String(s)
            } else {
                CoreValue::Binary(bytes)
            }
        }
        Value::Bulk(bytes) => {
            if let Ok(s) = String::from_utf8(bytes.clone()) {
                CoreValue::String(s)
            } else {
                CoreValue::Binary(bytes)
            }
        }
        Value::Status(s) => CoreValue::String(s),
        Value::Okay => CoreValue::String("OK".to_string()),
        Value::Array(arr) => {
            CoreValue::Array(arr.into_iter().map(redis_value_to_core_value).collect())
        }
        _ => CoreValue::String(format!("{:?}", val)),
    }
}

pub fn redis_value_to_query_result(val: Value, execution_time_ms: u64) -> QueryResult {
    let columns = vec![ColumnInfo {
        name: "Value".to_string(),
        data_type: "RedisValue".to_string(),
    }];

    let rows = match val {
        Value::Array(arr) => arr
            .into_iter()
            .map(|v| Row {
                cells: vec![redis_value_to_core_value(v)],
            })
            .collect(),
        _ => vec![Row {
            cells: vec![redis_value_to_core_value(val)],
        }],
    };

    QueryResult {
        columns,
        rows,
        execution_time_ms,
        rows_affected: 0,
    }
}
