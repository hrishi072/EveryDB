#[cxx_qt::bridge]
mod result_model {
    extern "Rust" {
        #[cxx_qt::qobject(qml_element = "ResultModel")]
        type ResultModel = super::ResultModelRust;
    }

    unsafe extern "C++" {
        #[cxx_qt::notify]
        fn data_changed(self: Pin<&mut ResultModel>);
    }

    extern "Rust" {
        #[cxx_qt::invokable]
        fn get_column_count(self: &ResultModel) -> usize;

        #[cxx_qt::invokable]
        fn get_row_count(self: &ResultModel) -> usize;
        
        #[cxx_qt::invokable]
        fn get_cell_json(self: &ResultModel, row: usize, col: usize) -> String;
    }
}

use everydb_core::query::QueryResult;
use std::pin::Pin;

#[derive(Default)]
pub struct ResultModelRust {
    pub result: Option<QueryResult>,
}

impl ResultModelRust {
    pub fn get_column_count(&self) -> usize {
        self.result.as_ref().map(|r| r.columns.len()).unwrap_or(0)
    }

    pub fn get_row_count(&self) -> usize {
        self.result.as_ref().map(|r| r.rows.len()).unwrap_or(0)
    }

    pub fn get_cell_json(&self, row: usize, col: usize) -> String {
        if let Some(r) = &self.result {
            if row < r.rows.len() && col < r.columns.len() {
                return serde_json::to_string(&r.rows[row].cells[col]).unwrap_or_default();
            }
        }
        String::new()
    }
}
