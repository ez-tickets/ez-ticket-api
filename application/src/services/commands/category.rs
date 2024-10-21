use error_stack::{Report, ResultExt};
use kernel::commands::CategoryCommand;
use kernel::entities::{Category, CategoryId, CategoryName, CategoryOrdering, OrderingProduct, ProductId};
use kernel::repositories::{CategoryRepository, DependOnCategoryRepository};
use crate::errors::ApplicationError;

pub trait DependOnCategoryCommandService: 'static + Sync + Send {
    type CategoryCommandService: CategoryCommandService;
    fn category_command_service(&self) -> &Self::CategoryCommandService;
}

impl<T> CategoryCommandService for T 
where 
    T: DependOnCategoryRepository
{
}


#[async_trait::async_trait]
pub trait CategoryCommandService: 'static + Sync + Send 
where 
    Self: DependOnCategoryRepository
{
    async fn execute(&self, id: Option<CategoryId>, cmd: CategoryCommand) -> Result<(), Report<ApplicationError>> {
        match cmd {
            CategoryCommand::Create { name, ordering } => {
                let id = CategoryId::default();
                let name = CategoryName::new(name);
                let ordering = CategoryOrdering::new(ordering);
                let category = Category::create(id, name, ordering);
                
                self.category_repository()
                    .create(&category)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            CategoryCommand::UpdateName { name } => {
                let id = id.ok_or(ApplicationError::MissingId { entity: "Category" })?;
                let mut category = self.category_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                let name = CategoryName::new(name);
                
                category.substitute(|cat| {
                    *cat.name = name;
                });
                
                self.category_repository()
                    .update(&id, &category)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            CategoryCommand::UpdateOrdering { ordering } => {
                let id = id.ok_or(ApplicationError::MissingId { entity: "Category" })?;
                let mut category = self.category_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                let ordering = CategoryOrdering::new(ordering);
                
                category.substitute(|cat| {
                    *cat.ordering = ordering;
                });
                
                self.category_repository()
                    .update(&id, &category)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            CategoryCommand::Delete => {
                let id = id.ok_or(ApplicationError::MissingId { entity: "Category" })?;
                
                self.category_repository()
                    .delete(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            CategoryCommand::AddProduct { ordered, product_id } => {
                let id = id.ok_or(ApplicationError::MissingId { entity: "Category" })?;
                let mut category = self.category_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                let product_id = ProductId::new(product_id);
                
                category.substitute(|cat| {
                    cat.products.insert(OrderingProduct::new(ordered, product_id));
                });
                
                self.category_repository()
                    .update(&id, &category)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            CategoryCommand::UpdateProductOrdering { .. } => {
                todo!()
            }
        }
        Ok(())
    }
}