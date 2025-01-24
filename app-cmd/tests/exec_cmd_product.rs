use nitinol::Event;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use app_cmd::services::product::{DependOnProductCommandService, ProductCommandService};
use kernel::entities::product::{ProductDesc, ProductId, ProductName, ProductPrice};
use kernel::io::commands::ProductCommand;
use kernel::io::events::ProductEvent;

include!("./test_framework.rs");

//noinspection RsTraitImplOrphanRules
impl DependOnProductCommandService for TestFramework {
    type ProductCommandService = Self;
    fn product_command_service(&self) -> &Self::ProductCommandService {
        self
    }
}

fn setup_logging() {
    tracing_subscriber::registry()
        .with(EnvFilter::new("trace"))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn extract_first_event(framework: &TestFramework) -> Result<ProductEvent, Report<UnrecoverableError>> {
    let create_event = framework.journal()
        .read_all_by_event::<ProductEvent>()
        .await
        .change_context_lazy(|| UnrecoverableError)?
        .first()
        .ok_or(Report::new(UnrecoverableError).attach_printable("No event found"))
        .map(|payload| ProductEvent::from_bytes(&payload.bytes))?
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(create_event)
}

async fn register_product(framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.product_command_service();
    
    let cmd = ProductCommand::Register {
        name: ProductName::new("test"),
        desc: ProductDesc::new("test desc"),
        price: ProductPrice::new(100).change_context_lazy(|| UnrecoverableError)?,
        image: vec![],
    };
    
    service.execute(None, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_register_product() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    
    let framework = TestFramework::new()?;
    
    register_product(&framework).await?;
    
    Ok(())
}

async fn rename_product(id: ProductId, framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.product_command_service();
    
    let cmd = ProductCommand::RenameProductName {
        new: ProductName::new("test 2"),
    };
    
    service.execute(id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_rename_product() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    register_product(&framework).await?;
    
    let event = extract_first_event(&framework).await?;
    let ProductEvent::Registered { id, .. } = event else {
        return Err(Report::new(UnrecoverableError).attach_printable("Unexpected event"));
    };
    
    rename_product(id, &framework).await?;
    
    Ok(())
}

async fn edit_product_desc(id: ProductId, framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.product_command_service();
    
    let cmd = ProductCommand::EditProductDesc {
        new: ProductDesc::new("test desc 2"),
    };
    
    service.execute(id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_edit_product_desc() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    register_product(&framework).await?;
    
    let event = extract_first_event(&framework).await?;
    let ProductEvent::Registered { id, .. } = event else {
        return Err(Report::new(UnrecoverableError).attach_printable("Unexpected event"));
    };
    
    edit_product_desc(id, &framework).await?;
    
    Ok(())
}

async fn change_product_price(id: ProductId, framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.product_command_service();
    
    let cmd = ProductCommand::ChangeProductPrice {
        new: ProductPrice::new(200).change_context_lazy(|| UnrecoverableError)?,
    };
    
    service.execute(id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_change_product_price() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    register_product(&framework).await?;
    
    let event = extract_first_event(&framework).await?;
    let ProductEvent::Registered { id, .. } = event else {
        return Err(Report::new(UnrecoverableError).attach_printable("Unexpected event"));
    };
    
    change_product_price(id, &framework).await?;
    
    Ok(())
}

async fn delete_product(id: ProductId, framework: &TestFramework) -> Result<(), Report<UnrecoverableError>> {
    let service = framework.product_command_service();
    
    let cmd = ProductCommand::Delete;
    
    service.execute(id, cmd).await
        .change_context_lazy(|| UnrecoverableError)?;
    
    Ok(())
}

#[tokio::test]
async fn test_delete_product() -> Result<(), Report<UnrecoverableError>> {
    setup_logging();
    let framework = TestFramework::new()?;
    
    register_product(&framework).await?;
    
    let event = extract_first_event(&framework).await?;
    let ProductEvent::Registered { id, .. } = event else {
        return Err(Report::new(UnrecoverableError).attach_printable("Unexpected event"));
    };
    
    delete_product(id, &framework).await?;
    
    Ok(())
}