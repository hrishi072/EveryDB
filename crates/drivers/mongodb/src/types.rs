use everydb_core::value::CoreValue;
use mongodb::bson::{Bson, Document};
use std::collections::HashMap;

pub fn bson_to_core_value(bson: Bson) -> CoreValue {
    match bson {
        Bson::Double(f) => CoreValue::Float(f),
        Bson::String(s) => CoreValue::String(s),
        Bson::Array(arr) => CoreValue::Array(arr.into_iter().map(bson_to_core_value).collect()),
        Bson::Document(doc) => {
            let mut map = HashMap::new();
            for (key, val) in doc {
                map.insert(key, bson_to_core_value(val));
            }
            CoreValue::Object(map)
        }
        Bson::Boolean(b) => CoreValue::Boolean(b),
        Bson::Null => CoreValue::Null,
        Bson::Int32(i) => CoreValue::Integer(i as i64),
        Bson::Int64(i) => CoreValue::Integer(i),
        Bson::DateTime(dt) => CoreValue::DateTime(dt.to_chrono()),
        Bson::Binary(bin) => CoreValue::Binary(bin.bytes),
        Bson::ObjectId(oid) => CoreValue::String(oid.to_hex()),
        Bson::Decimal128(d) => {
            // Decimal128 to rust_decimal is tricky, for now string
            CoreValue::String(d.to_string())
        }
        _ => CoreValue::String(bson.to_string()),
    }
}

pub fn mongo_doc_to_core_row(doc: Document) -> everydb_core::query::Row {
    let mut cells = Vec::new();
    for (_key, val) in doc {
        cells.push(bson_to_core_value(val));
    }
    everydb_core::query::Row { cells }
}
