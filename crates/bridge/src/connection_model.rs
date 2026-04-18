#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type ConnectionModel = super::ConnectionModelRust;
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn connections_changed(self: Pin<&mut ConnectionModel>);

        #[qinvokable]
        fn load_connections(self: Pin<&mut ConnectionModel>);

        #[qinvokable]
        fn add_connection(
            self: Pin<&mut ConnectionModel>,
            name: String,
            driver: String,
            host: String,
            port: u16,
            username: String,
            password: String,
        );
    }
}

use everydb_core::connection::ConnectionConfig;
use std::pin::Pin;
use cxx_qt::CxxQtType;

#[derive(Default)]
pub struct ConnectionModelRust {
    pub connections: Vec<ConnectionConfig>,
}

impl ffi::ConnectionModel {
    pub fn load_connections(self: Pin<&mut Self>) {
        // In a real app, this would load from ~/.config/everydb/connections.toml
    }

    pub fn add_connection(
        mut self: Pin<&mut Self>,
        name: String,
        driver: String,
        host: String,
        port: u16,
        username: String,
        password: String,
    ) {
        let config = ConnectionConfig {
            name,
            driver_type: driver,
            host,
            port,
            username: if username.is_empty() { None } else { Some(username) },
            password: if password.is_empty() { None } else { Some(password) },
            database: None,
            use_tls: false,
            extra_options: Default::default(),
        };

        self.as_mut().rust_mut().connections.push(config);
        self.connections_changed();
    }
}
