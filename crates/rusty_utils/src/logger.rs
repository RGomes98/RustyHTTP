use std::error::Error;

use rusty_config::Config;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub fn init_logger() -> Result<(), Box<dyn Error + Send + Sync>> {
    let log_level: String = Config::from_env("RUST_LOG").unwrap_or_else(|_| "info".into());

    let filter: EnvFilter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&log_level))
        .expect("Invalid log level");

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init()?;

    info!("Log level set to: {log_level}");
    Ok(())
}
