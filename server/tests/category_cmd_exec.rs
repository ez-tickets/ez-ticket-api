use error_stack::{Report, ResultExt};
use kernel::commands::{CategoriesCommand, CategoryCommand};
use kernel::entities::{CategoryId, CategoryName};
use server::errors::UnrecoverableError;
use server::{logging, AppModule};
use tracing_subscriber::EnvFilter;
use application_query::models::CategoryQueryService;

#[tokio::test]
async fn categories_category_add() -> Result<(), Report<UnrecoverableError>> {
    logging::init();
    
    use application_command::services::commands::{CategoryCommandExecutor, DependOnCategoryCommandExecutor};
    
    let app = AppModule::setup().await?;

    for i in 0..2 {
        app.clone()
            .category_command_executor()
            .execute(None, CategoryCommand::Create {
                name: CategoryName::new(format!("category-{i}")),
            })
            .await
            .change_context_lazy(|| UnrecoverableError)?;
    }
    
    let categories = app.clone().find_all_category().await
        .change_context_lazy(|| UnrecoverableError)?;

    for category in categories.0 {
        app.clone()
            .category_command_executor()
            .execute(Some(CategoryId::new(category.id)), CategoryCommand::Delete)
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
    
    let app = AppModule::setup().await?;

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