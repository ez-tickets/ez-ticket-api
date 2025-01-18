use app_cmd::services::category::DependOnCategoryCommandService;
use kernel::entities::category::CategoryId;
use kernel::io::commands::CategoryCommand;

include!("./test_framework.rs");

impl DependOnCategoryCommandService for TestFramework {
    type CategoryCommandService = Self;
    fn category_command_service(&self) -> &Self::CategoryCommandService {
        self
    }
}

#[tokio::test]
async fn test_category_command_service() -> Result<(), Report<UnrecoverableError>> {
    let framework = TestFramework::new()?;
    let service = framework.category_command_service();

    // let id = CategoryId::default();
    // let cmd = CategoryCommand::Create {
    //     name: "test".to_string(),
    // };
    // 
    // let result = service.execute(id, cmd).await;
    // assert!(result.is_ok());
    
    Ok(())
}