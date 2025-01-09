use std::sync::Arc;
use std::ops::Deref;

use error_stack::{Report, ResultExt};
use crate::errors::UnrecoverableError;

pub struct AppModule {
    inner: Arc<Handler>
}

impl Clone for AppModule {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl Deref for AppModule {
    type Target = Handler;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

pub struct Handler {
}

impl AppModule {
    pub async fn setup() -> Result<AppModule, Report<UnrecoverableError>> {
        Ok(AppModule {
            inner: Arc::new(Handler {})
        })
    }
}
