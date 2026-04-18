use everydb_core::value::CoreValue;
use sqlx::{postgres::PgRow, Column, Row, TypeInfo};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

pub fn pg_row_to_core_row(row: &PgRow) -> everydb_core::query::Row {
    let mut cells = Vec::new();
    for i in 0..row.columns().len() {
        cells.push(pg_value_to_core_value(row, i));
    }
    everydb_core::query::Row { cells }
}

pub fn pg_value_to_core_value(row: &PgRow, index: usize) -> CoreValue {
    let column = &row.columns()[index];
    let type_name = column.type_info().name();

    match type_name {
        "BOOL" => row.try_get::<bool, _>(index).map(CoreValue::Boolean).unwrap_or(CoreValue::Null),
        "INT2" | "INT4" | "INT8" => row.try_get::<i64, _>(index).map(CoreValue::Integer).unwrap_or(CoreValue::Null),
        "FLOAT4" | "FLOAT8" => row.try_get::<f64, _>(index).map(CoreValue::Float).unwrap_or(CoreValue::Null),
        "TEXT" | "VARCHAR" | "CHAR" | "NAME" => row.try_get::<String, _>(index).map(CoreValue::String).unwrap_or(CoreValue::Null),
        "BYTEA" => row.try_get::<Vec<u8>, _>(index).map(CoreValue::Binary).unwrap_or(CoreValue::Null),
        "TIMESTAMPTZ" | "TIMESTAMP" => {
            // This might need more careful handling depending on sqlx features
            row.try_get::<DateTime<Utc>, _>(index).map(CoreValue::DateTime).unwrap_or(CoreValue::Null)
        },
        "NUMERIC" => row.try_get::<Decimal, _>(index).map(CoreValue::Decimal).unwrap_or(CoreValue::Null),
        "JSON" | "JSONB" => row.try_get::<serde_json::Value, _>(index).map(CoreValue::Json).unwrap_or(CoreValue::Null),
        _ => {
            // Fallback to string representation for unknown types
            row.try_get::<String, _>(index).map(CoreValue::String).unwrap_or(CoreValue::Null)
        }
    }
}
