mod product;
mod category;

pub use self::product::*;
pub use self::category::*;

use std::str::FromStr;
use std::time::Duration;

use error_stack::{Report, ResultExt};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::errors::FailedInitializeDataBase;

pub async fn init(url: &str) -> Result<SqlitePool, Report<FailedInitializeDataBase>> {
    let opts = SqliteConnectOptions::from_str(url)
        .change_context_lazy(|| FailedInitializeDataBase)
        .attach_printable_lazy(|| format!("`{url}` may not be valid"))?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .acquire_timeout(Duration::from_millis(5000))
        .max_connections(8)
        .connect_with(opts)
        .await
        .change_context_lazy(|| FailedInitializeDataBase)
        .attach_printable_lazy(|| "On failure connection phase.")?;

    sqlx::migrate!("../migrations")
        .run(&pool).await
        .change_context_lazy(|| FailedInitializeDataBase)
        .attach_printable_lazy(|| "On failure migration phase")?;
    
    Ok(pool)
}
