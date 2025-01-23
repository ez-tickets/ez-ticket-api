use std::collections::BTreeMap;
use app_cmd::services::category::{CategoryCommandService, DependOnCategoryCommandService};
use kernel::entities::category::{CategoryId, CategoryName};
use kernel::io::commands::CategoryCommand;
use kernel::io::events::CategoryEvent;
use nitinol::Event;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use app_cmd::services::categories::DependOnCategoriesCommandService;
use kernel::entities::product::ProductId;

include!("./test_framework.rs");

impl DependOnCategoryCommandService for TestFramework {
    type CategoryCommandService = Self;
    fn category_command_service(&self) -> &Self::CategoryCommandService {
        self
    }
}

impl DependOnCategoriesCommandService for TestFramework {
    type CategoriesCommandService = Self;
    fn categories_command_service(&self) -> &Self::CategoriesCommandService {
        self
    }
}

fn setup_logging() {
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();
}

async fn extract_first_event(framework: &TestFramework) -> Result<CategoryEvent, Report<UnrecoverableError>> {
    let create_event = framework.journal()
        .read_all_by_event::<CategoryEvent>()
        .await
        .change_context_lazy(|| UnrecoverableError)?
        .first()
        .ok_or(Report::new(UnrecoverableError).attach_printable("No event found"))
        .map(|payload| CategoryEvent::from_bytes(&payload.bytes))?
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(create_event)
}

async fn create_category(framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.category_command_service();
    
    let cmd = CategoryCommand::Create {
        name: CategoryName::new("test")
            .change_context_lazy(|| UnrecoverableError)?,
    };
    
    service.execute(None, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_create_category() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    
    Ok(())
}


async fn rename_category(id: CategoryId, framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.category_command_service();
    
    let cmd = CategoryCommand::Rename {
        new: CategoryName::new("test2")
            .change_context_lazy(|| UnrecoverableError)?,
    };
    
    service.execute(id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_rename_category() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    let create_event = extract_first_event(&framework).await?;
    
    let CategoryEvent::Created { id, .. } = create_event else {
        return Err(Report::new(UnrecoverableError)
            .attach_printable("Event is not a Created event"));
    };
    
    rename_category(id, &framework).await?;
    
    Ok(())
}

async fn delete_category(id: CategoryId, framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.category_command_service();
    
    let cmd = CategoryCommand::Delete;
    
    service.execute(id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_delete_category() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    let create_event = extract_first_event(&framework).await?;
    
    let CategoryEvent::Created { id, .. } = create_event else {
        return Err(Report::new(UnrecoverableError)
            .attach_printable("Event is not a Created event"));
    };
    
    delete_category(id, &framework).await?;
    
    Ok(())
}

async fn add_product_to_category(
    category_id: CategoryId,
    product_id: ProductId,
    framework: &TestFramework
) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.category_command_service();
    
    let cmd = CategoryCommand::AddProduct {
        id: product_id,
    };
    
    service.execute(category_id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_add_product_to_category() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    let create_event = extract_first_event(&framework).await?;
    
    let CategoryEvent::Created { id, .. } = create_event else {
        return Err(Report::new(UnrecoverableError)
            .attach_printable("Event is not a Created event"));
    };
    
    add_product_to_category(id, ProductId::default(), &framework).await?;
    
    Ok(())
}

async fn remove_product_from_category(
    category_id: CategoryId,
    product_id: ProductId,
    framework: &TestFramework
) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.category_command_service();
    
    let cmd = CategoryCommand::RemoveProduct {
        id: product_id,
    };
    
    service.execute(category_id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_remove_product_from_category() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    let create_event = extract_first_event(&framework).await?;
    
    let CategoryEvent::Created { id, .. } = create_event else {
        return Err(Report::new(UnrecoverableError)
            .attach_printable("Event is not a Created event"));
    };
    
    let product = ProductId::default();
    
    add_product_to_category(id, product, &framework).await?;
    remove_product_from_category(id, product, &framework).await?;
    
    Ok(())
}

async fn change_ordering_product_in_category(
    category_id: CategoryId,
    new: BTreeMap<i64, ProductId>,
    framework: &TestFramework
) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.category_command_service();
    
    let cmd = CategoryCommand::ChangeProductOrdering { new };
    
    service.execute(category_id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_change_ordering_product_in_category() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    let create_event = extract_first_event(&framework).await?;
    
    let CategoryEvent::Created { id, .. } = create_event else {
        return Err(Report::new(UnrecoverableError)
            .attach_printable("Event is not a Created event"));
    };
    
    let product_1 = ProductId::default();
    let product_2 = ProductId::default();
    let product_3 = ProductId::default();
    
    add_product_to_category(id, product_1, &framework).await?;
    add_product_to_category(id, product_2, &framework).await?;
    add_product_to_category(id, product_3, &framework).await?;
    
    let mut new = BTreeMap::new();
    new.insert(3, product_1);
    new.insert(2, product_2);
    new.insert(1, product_3);
    
    change_ordering_product_in_category(id, new, &framework).await?;
    
    Ok(())
}

#[tokio::test]
async fn test_all() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    create_category(&framework).await?;
    
    let create_event = extract_first_event(&framework).await?;
    
    let CategoryEvent::Created { id, .. } = create_event else {
        return Err(Report::new(UnrecoverableError)
            .attach_printable("Event is not a Created event"));
    };
    
    rename_category(id, &framework).await?;
    
    let product_1 = ProductId::default();
    let product_2 = ProductId::default();
    let product_3 = ProductId::default();
    let product_4 = ProductId::default();
    
    add_product_to_category(id, product_1, &framework).await?;
    add_product_to_category(id, product_2, &framework).await?;
    add_product_to_category(id, product_3, &framework).await?;
    add_product_to_category(id, product_4, &framework).await?;
    
    remove_product_from_category(id, product_1, &framework).await?;
    
    let mut new = BTreeMap::new();
    new.insert(3, product_2);
    new.insert(2, product_3);
    new.insert(1, product_4);
    
    change_ordering_product_in_category(id, new, &framework).await?;
    
    delete_category(id, &framework).await?;
    
    Ok(())
}