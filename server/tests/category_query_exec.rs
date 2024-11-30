use error_stack::{Report, ResultExt};
use application_query::models::{CategoryQueryService, DependOnCategoryQueryService};
use server::errors::UnrecoverableError;
use server::Handler;

#[tokio::test]
async fn category_query() -> Result<(), Report<UnrecoverableError>> {
    let app = Handler::setup().await?;
    
    let categories = app.category_query_service()
        .find_all_category()
        .await
        .change_context_lazy(|| UnrecoverableError)?;
    
    let json = serde_json::to_string_pretty(&categories)
        .change_context_lazy(|| UnrecoverableError)?;
    
    println!("{}", json);
    Ok(())
}