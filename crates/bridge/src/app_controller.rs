#[cxx_qt::bridge]
mod app_controller {
    extern "Rust" {
        #[cxx_qt::qobject]
        type AppController = super::AppControllerRust;
    }

    unsafe extern "C++" {
        #[cxx_qt::notify]
        fn current_driver_name_changed(self: Pin<&mut AppController>);
    }

    extern "Rust" {
        #[cxx_qt::qproperty]
        fn current_driver_name(self: &AppController) -> String;

        #[cxx_qt::invokable]
        fn connect_to_postgres(self: Pin<&mut AppController>, host: String, port: u16);
    }
}

use everydb_core::connection::ConnectionConfig;
use everydb_core::driver::{DatabaseDriver, DriverRegistry};
use std::pin::Pin;
use std::sync::Arc;

pub struct AppControllerRust {
    pub registry: Arc<DriverRegistry>,
    pub current_driver: Option<Arc<dyn DatabaseDriver>>,
}

impl Default for AppControllerRust {
    fn default() -> Self {
        Self {
            registry: Arc::new(DriverRegistry::new()),
            current_driver: None,
        }
    }
}

impl AppControllerRust {
    pub fn current_driver_name(&self) -> String {
        self.current_driver
            .as_ref()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|| "None".to_string())
    }

    pub fn connect_to_postgres(self: Pin<&mut AppControllerRust>, host: String, port: u16) {
        // This is a stub for now. In a real implementation, we'd use a background task.
    }
}
