use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl, QString};
use everydb_core::driver::DriverRegistry;
use everydb_mongodb::MongoDriver;
use everydb_postgres::PostgresDriver;
use everydb_redis::RedisDriver;
use std::sync::Arc;

fn main() {
    // 1. Initialize logging
    tracing_subscriber::fmt::init();
    tracing::info!("EveryDB starting...");

    // 2. Setup Driver Registry
    let mut registry = DriverRegistry::new();
    registry.register(Arc::new(PostgresDriver::new()));
    registry.register(Arc::new(MongoDriver::new()));
    registry.register(Arc::new(RedisDriver::new()));
    tracing::info!("Registered {} drivers", registry.all().count());

    // 3. Initialize Qt Application
    let mut app = QGuiApplication::new();

    // 4. Initialize QML Engine
    let mut engine = QQmlApplicationEngine::new();

    // 5. Load Main QML File
    if let Some(mut engine) = engine.as_mut() {
        let qml_path = format!("{}/crates/app/qml/main.qml", env!("CARGO_MANIFEST_DIR"));
        let qml_url = QUrl::from_local_file(&QString::from(&qml_path));
        engine.load(&qml_url);
    }

    // 6. Start Event Loop
    if let Some(mut app) = app.as_mut() {
        app.exec();
    }
}
