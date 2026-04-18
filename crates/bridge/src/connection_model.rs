#[cxx_qt::bridge]
mod connection_model {
    extern "Rust" {
        #[cxx_qt::qobject(qml_element = "ConnectionModel")]
        type ConnectionModel = super::ConnectionModelRust;
    }

    unsafe extern "C++" {
        #[cxx_qt::notify]
        fn connections_changed(self: Pin<&mut ConnectionModel>);
    }

    extern "Rust" {
        #[cxx_qt::invokable]
        fn load_connections(self: Pin<&mut ConnectionModel>);

        #[cxx_qt::invokable]
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

#[derive(Default)]
pub struct ConnectionModelRust {
    connections: Vec<ConnectionConfig>,
}

impl ConnectionModelRust {
    pub fn load_connections(self: Pin<&mut ConnectionModelRust>) {
        // In a real app, this would load from ~/.config/everydb/connections.toml
    }

    pub fn add_connection(
        mut self: Pin<&mut ConnectionModelRust>,
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

        self.as_mut().connections_mut().push(config);
        // Emitting signals or calling methods on C++ side can be done here
    }

    fn connections_mut<'a>(self: &'a mut Pin<&mut ConnectionModelRust>) -> &'a mut Vec<ConnectionConfig> {
        // Workaround for accessing fields behind Pin
        unsafe {
            let unpinned = Pin::into_inner_unchecked(self.as_mut());
            &mut unpinned.connections
        }
    }
}
