use crate::errors::DriverError;
use error_stack::{Report, ResultExt};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub async fn init_postgres(url: &str) -> Result<Pool<Postgres>, Report<DriverError>> {
    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(5000))
        .max_connections(8)
        .connect(url)
        .await
        .change_context_lazy(|| DriverError::Connection)?;
    
    sqlx::migrate!("../migrations")
        .run(&pool).await
        .change_context_lazy(|| DriverError::Migration)?;
    
    Ok(pool)
}

