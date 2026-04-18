#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(String, current_driver_name)]
        type AppController = super::AppControllerRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn connect_to_postgres(self: Pin<&mut AppController>, host: String, port: u16);

        #[qinvokable]
        fn register_driver(self: Pin<&mut AppController>, driver_type: String);
    }
}

use everydb_core::driver::DriverRegistry;
use everydb_core::driver::DatabaseDriver;
use everydb_postgres::PostgresDriver;
use everydb_mongodb::MongoDriver;
use everydb_redis::RedisDriver;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static REGISTRY: Lazy<Arc<Mutex<DriverRegistry>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DriverRegistry::new()))
});

pub struct AppControllerRust {
    pub current_driver_name: String,
    pub current_driver: Option<Arc<dyn DatabaseDriver>>,
}

impl Default for AppControllerRust {
    fn default() -> Self {
        Self {
            current_driver_name: "None".to_string(),
            current_driver: None,
        }
    }
}

impl ffi::AppController {
    pub fn connect_to_postgres(self: Pin<&mut Self>, _host: String, _port: u16) {
        // This is a stub for now.
    }

    pub fn register_driver(self: Pin<&mut Self>, driver_type: String) {
        let mut registry = REGISTRY.lock().unwrap();
        match driver_type.as_str() {
            "postgres" => registry.register(Arc::new(PostgresDriver::new())),
            "mongodb" => registry.register(Arc::new(MongoDriver::new())),
            "redis" => registry.register(Arc::new(RedisDriver::new())),
            _ => println!("Unknown driver type: {}", driver_type),
        }
    }
    
    pub fn global_registry() -> Arc<Mutex<DriverRegistry>> {
        REGISTRY.clone()
    }
}
