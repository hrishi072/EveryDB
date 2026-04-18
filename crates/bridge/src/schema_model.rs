#[cxx_qt::bridge]
mod schema_model {
    extern "Rust" {
        #[cxx_qt::qobject(qml_element = "SchemaModel")]
        type SchemaModel = super::SchemaModelRust;
    }

    unsafe extern "C++" {
        #[cxx_qt::notify]
        fn schema_reloaded(self: Pin<&mut SchemaModel>);
    }

    extern "Rust" {
        #[cxx_qt::invokable]
        fn reload_schema(self: Pin<&mut SchemaModel>);
        
        #[cxx_qt::invokable]
        fn get_schema_json(self: &SchemaModel) -> String;
    }
}

use everydb_core::schema::SchemaNode;
use std::pin::Pin;

#[derive(Default)]
pub struct SchemaModelRust {
    pub nodes: Vec<SchemaNode>,
}

impl SchemaModelRust {
    pub fn reload_schema(self: Pin<&mut SchemaModelRust>) {
        // Real implementation would invoke introspect_schema on the active driver
        // via tokio::spawn and update the nodes
    }

    pub fn get_schema_json(&self) -> String {
        serde_json::to_string(&self.nodes).unwrap_or_else(|_| "[]".to_string())
    }
}
