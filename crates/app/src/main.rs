use everydb_core::driver::DriverRegistry;
use everydb_mongodb::MongoDriver;
use everydb_postgres::PostgresDriver;
use everydb_redis::RedisDriver;
use std::sync::Arc;

fn main() {
    println!("EveryDB starting...");

    let mut registry = DriverRegistry::new();

    registry.register(Arc::new(PostgresDriver::new()));
    registry.register(Arc::new(MongoDriver::new()));
    registry.register(Arc::new(RedisDriver::new()));

    println!("Registered {} drivers", registry.all().count());

    // In a real Qt app, we would launch the QGuiApplication here.
    println!("Starting Qt event loop (simulated)");
}
