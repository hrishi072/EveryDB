use everydb_core::{connection::ConnectionConfig, error::CoreError};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::str::FromStr;
use std::time::Duration;

pub async fn create_pool(config: &ConnectionConfig) -> Result<sqlx::PgPool, CoreError> {
    let mut options = if let Some(uri) = config.extra_options.get("connection_string") {
        PgConnectOptions::from_str(uri)
            .map_err(|e| CoreError::ConnectionError(format!("Invalid connection string: {}", e)))?
    } else {
        let mut opts = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(config.username.as_deref().unwrap_or("postgres"))
            .database(config.database.as_deref().unwrap_or("postgres"));

        if let Some(password) = &config.password {
            opts = opts.password(password);
        }
        opts
    };

    // SSL Mode
    if let Some(ssl_mode_str) = config.extra_options.get("ssl_mode") {
        let ssl_mode = match ssl_mode_str.as_str() {
            "disable" => PgSslMode::Disable,
            "allow" => PgSslMode::Allow,
            "prefer" => PgSslMode::Prefer,
            "require" => PgSslMode::Require,
            "verify-ca" => PgSslMode::VerifyCa,
            "verify-full" => PgSslMode::VerifyFull,
            _ => {
                return Err(CoreError::ConnectionError(format!(
                    "Invalid SSL mode: {}",
                    ssl_mode_str
                )))
            }
        };
        options = options.ssl_mode(ssl_mode);
    }

    // Pool settings
    let max_connections = config
        .extra_options
        .get("max_connections")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let connect_timeout = config
        .extra_options
        .get("connect_timeout")
        .and_then(|s| s.parse().ok())
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(30));

    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(connect_timeout)
        .connect_with(options)
        .await
        .map_err(|e| CoreError::ConnectionError(e.to_string()))
}
