use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum CoreValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Binary(Vec<u8>),
    DateTime(DateTime<Utc>),
    Decimal(Decimal),
    Json(serde_json::Value),
    Array(Vec<CoreValue>),
    Object(HashMap<String, CoreValue>),
}

impl Default for CoreValue {
    fn default() -> Self {
        CoreValue::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_serialization() {
        let values = vec![
            CoreValue::Null,
            CoreValue::Boolean(true),
            CoreValue::Integer(123),
            CoreValue::Float(45.67),
            CoreValue::String("hello".to_string()),
            CoreValue::Binary(vec![1, 2, 3]),
            CoreValue::DateTime(Utc::now()),
            CoreValue::Decimal(Decimal::new(123, 2)),
            CoreValue::Json(json!({"foo": "bar"})),
        ];

        for val in values {
            let serialized = serde_json::to_string(&val).unwrap();
            let deserialized: CoreValue = serde_json::from_str(&serialized).unwrap();
            assert_eq!(val, deserialized);
        }
    }

    #[test]
    fn test_complex_value_serialization() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), CoreValue::Integer(100));

        let value = CoreValue::Object(map);
        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: CoreValue = serde_json::from_str(&serialized).unwrap();
        assert_eq!(value, deserialized);

        let value = CoreValue::Array(vec![CoreValue::Integer(1), CoreValue::String("2".to_string())]);
        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: CoreValue = serde_json::from_str(&serialized).unwrap();
        assert_eq!(value, deserialized);
    }
}
