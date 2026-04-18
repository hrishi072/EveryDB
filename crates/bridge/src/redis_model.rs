#[cxx_qt::bridge]
mod redis_model {
    extern "Rust" {
        #[cxx_qt::qobject(qml_element = "RedisModel")]
        type RedisModel = super::RedisModelRust;
    }

    unsafe extern "C++" {
        #[cxx_qt::notify]
        fn keyspace_changed(self: Pin<&mut RedisModel>);
    }

    extern "Rust" {
        #[cxx_qt::invokable]
        fn fetch_keys(self: Pin<&mut RedisModel>, pattern: String);
    }
}

use std::pin::Pin;

#[derive(Default)]
pub struct RedisModelRust {
    pub keys: Vec<String>,
}

impl RedisModelRust {
    pub fn fetch_keys(self: Pin<&mut RedisModelRust>, _pattern: String) {
        // Implement async redis scanning here
    }
}
