#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type SchemaModel = super::SchemaModelRust;
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn schema_reloaded(self: Pin<&mut SchemaModel>);

        #[qinvokable]
        fn get_schema_json(self: &SchemaModel) -> String;

        #[qinvokable]
        fn reload_schema(self: Pin<&mut SchemaModel>);
    }
}

use everydb_core::schema::SchemaNode;
use std::pin::Pin;
use cxx_qt::CxxQtType;

#[derive(Default)]
pub struct SchemaModelRust {
    pub nodes: Vec<SchemaNode>,
}

impl ffi::SchemaModel {
    pub fn reload_schema(self: Pin<&mut Self>) {
        // Real implementation would invoke introspect_schema on the active driver
        // via tokio::spawn and update the nodes
    }

    pub fn get_schema_json(&self) -> String {
        serde_json::to_string(&self.rust().nodes).unwrap_or_else(|_| "[]".to_string())
    }
}
