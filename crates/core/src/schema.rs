use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaNode {
    pub name: String,
    pub node_type: SchemaNodeType,
    pub children: Vec<SchemaNode>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaNodeType {
    Database,
    Schema,
    Table,
    View,
    MaterializedView,
    Collection,
    Column,
    Index,
    Function,
    Sequence,
    Key, // Redis keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_type_equality() {
        assert_eq!(SchemaNodeType::Table, SchemaNodeType::Table);
        assert_ne!(SchemaNodeType::Table, SchemaNodeType::View);
    }
}
