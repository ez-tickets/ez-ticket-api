use error_stack::{Report, ResultExt};
use kernel::commands::{CategoriesCommand, CategoryCommand};
use kernel::entities::CategoryName;
use server::errors::UnrecoverableError;
use server::Handler;
use tracing_subscriber::EnvFilter;

#[tokio::test]
async fn categories_category_add() -> Result<(), Report<UnrecoverableError>> {
    use application_command::services::commands::{CategoryCommandExecutor, DependOnCategoryCommandExecutor};
    
    let app = Handler::setup().await?;

    for i in 0..2 {
        app.clone()
            .category_command_executor()
            .execute(None, CategoryCommand::Create {
                name: CategoryName::new(format!("category-{i}")),
            })
            .await
            .change_context_lazy(|| UnrecoverableError)?;
    }
    
    Ok(())
}

#[tokio::test]
async fn categories_add() -> Result<(), Report<UnrecoverableError>> {
    use application_command::services::commands::{CategoriesCommandExecutor, DependOnCategoriesCommandExecutor};
    
    std::env::set_var("RUST_LOG", "application_command=trace,category_cmd_exec=trace,nitinol=trace");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    let app = Handler::setup().await?;

    for _ in 0..5 {
        app.clone()
            .categories_command_executor()
            .execute(CategoriesCommand::Add {
                id: Default::default(),
            })
            .await
            .change_context_lazy(|| UnrecoverableError)?;
    }
    
    Ok(())
}