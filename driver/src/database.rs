use std::str::FromStr;
use crate::errors::DriverError;
use error_stack::{Report, ResultExt};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::time::Duration;
use nitinol::protocol::adapter::sqlite::SqliteEventStore;

pub fn mkdir_if_none() -> Result<(), Report<DriverError>> {
    if !std::fs::exists(".database").change_context_lazy(|| DriverError::Setup)? {
        std::fs::create_dir(".database")
            .map_err(|_| Report::new(DriverError::Setup))?;
    }
    Ok(())
}

pub async fn init_sqlite(url: &str) -> Result<Pool<Sqlite>, Report<DriverError>> {
    let opts = SqliteConnectOptions::from_str(url)
        .change_context_lazy(|| DriverError::Connection(url.to_string()))?
        .create_if_missing(true);
    
    let pool = SqlitePoolOptions::new()
        .acquire_timeout(Duration::from_millis(5000))
        .max_connections(8)
        .connect_with(opts)
        .await
        .change_context_lazy(|| DriverError::Connection(url.to_string()))?;
    
    sqlx::migrate!("../migrations")
        .run(&pool).await
        .change_context_lazy(|| DriverError::Migration)?;
    
    Ok(pool)
}

pub async fn setup_eventstore(url: impl AsRef<str>) -> Result<SqliteEventStore, Report<DriverError>> {
    SqliteEventStore::setup(url).await
        .change_context_lazy(|| DriverError::SetupEventStore)
}