use crate::errors::DriverError;
use error_stack::{Report, ResultExt};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::time::Duration;
use nitinol::protocol::adapter::sqlite::SqliteEventStore;

pub async fn init_sqlite(url: &str) -> Result<Pool<Sqlite>, Report<DriverError>> {
    let pool = SqlitePoolOptions::new()
        .acquire_timeout(Duration::from_millis(5000))
        .max_connections(8)
        .connect(url)
        .await
        .change_context_lazy(|| DriverError::Connection(url.to_string()))?;
    
    sqlx::migrate!("../migrations")
        .run(&pool).await
        .change_context_lazy(|| DriverError::Migration)?;
    
    Ok(pool)
}

pub async fn setup_eventstore(pool: &Pool<Sqlite>) -> Result<SqliteEventStore, Report<DriverError>> {
    Ok(SqliteEventStore::setup(pool.clone()))
}