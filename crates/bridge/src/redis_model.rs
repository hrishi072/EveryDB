#[cxx_qt::bridge]
mod ffi {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type RedisModel = super::RedisModelRust;
    }

    unsafe extern "RustQt" {
        #[qsignal]
        fn keyspace_changed(self: Pin<&mut RedisModel>);

        #[qinvokable]
        fn fetch_keys(self: Pin<&mut RedisModel>, pattern: String);
    }
}

use std::pin::Pin;

#[derive(Default)]
pub struct RedisModelRust {
    pub keys: Vec<String>,
}

impl ffi::RedisModel {
    pub fn fetch_keys(self: Pin<&mut Self>, _pattern: String) {
        // Implement async redis scanning here
    }
}
