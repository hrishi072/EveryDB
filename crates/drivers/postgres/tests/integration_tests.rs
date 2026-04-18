use everydb_core::connection::ConnectionConfig;
use everydb_core::driver::DatabaseDriver;
use everydb_core::query::QueryRequest;
use everydb_postgres::PostgresDriver;
use testcontainers::clients::Cli;
use testcontainers::images::generic::GenericImage;
use testcontainers::*;

#[tokio::test]
async fn test_postgres_connect_and_query() {
    let docker = Cli::default();
    let pg_image =
        GenericImage::new("postgres", "latest").with_env_var("POSTGRES_PASSWORD", "password");
    let node = docker.run(pg_image);
    let port = node.get_host_port_ipv4(5432);

    let config = ConnectionConfig {
        name: "test-pg".to_string(),
        driver_type: "postgres".to_string(),
        host: "localhost".to_string(),
        port,
        username: Some("postgres".to_string()),
        password: Some("password".to_string()),
        database: Some("postgres".to_string()),
        use_tls: false,
        extra_options: Default::default(),
    };

    let driver = PostgresDriver::new();
    driver.connect(&config).await.expect("Failed to connect");
    assert!(driver.is_connected().await);

    let req = QueryRequest {
        raw_query: "SELECT 1 + 1 as result".to_string(),
        parameters: vec![],
    };

    let res = driver.execute(req).await.expect("Failed to execute query");
    assert_eq!(res.rows.len(), 1);
    assert_eq!(res.columns[0].name, "result");

    driver.disconnect().await.expect("Failed to disconnect");
}
