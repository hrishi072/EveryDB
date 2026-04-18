#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type ResultModel = super::ResultModelRust;
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn data_changed(self: Pin<&mut ResultModel>);

        #[qinvokable]
        fn get_column_count(self: &ResultModel) -> usize;

        #[qinvokable]
        fn get_row_count(self: &ResultModel) -> usize;
        
        #[qinvokable]
        fn get_cell_json(self: &ResultModel, row: usize, col: usize) -> String;
    }
}

use everydb_core::query::QueryResult;
use std::pin::Pin;
use cxx_qt::CxxQtType;

#[derive(Default)]
pub struct ResultModelRust {
    pub result: Option<QueryResult>,
}

impl ffi::ResultModel {
    pub fn get_column_count(&self) -> usize {
        self.rust().result.as_ref().map(|r| r.columns.len()).unwrap_or(0)
    }

    pub fn get_row_count(&self) -> usize {
        self.rust().result.as_ref().map(|r| r.rows.len()).unwrap_or(0)
    }

    pub fn get_cell_json(&self, row: usize, col: usize) -> String {
        if let Some(r) = &self.rust().result {
            if row < r.rows.len() && col < r.columns.len() {
                return serde_json::to_string(&r.rows[row].cells[col]).unwrap_or_default();
            }
        }
        String::new()
    }
}
