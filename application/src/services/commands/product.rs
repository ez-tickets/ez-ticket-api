use crate::errors::ApplicationError;
use error_stack::{Report, ResultExt};
use kernel::commands::ProductCommand;
use kernel::entities::{Price, Product, ProductDescription, ProductId, ProductName, Stock};
use kernel::repositories::{DependOnProductRepository, ProductRepository};

pub trait DependOnProductCommandService: 'static + Sync + Send {
    type ProductCommandService: ProductCommandService;
    fn product_command_service(&self) -> &Self::ProductCommandService;
}

#[async_trait::async_trait]
pub trait ProductCommandService: 'static + Sync + Send
where
    Self:
        DependOnProductRepository
{
    async fn execute(&self, id: Option<ProductId>, cmd: ProductCommand) -> Result<(), Report<ApplicationError>> {
        match cmd {
            ProductCommand::Register { name, desc, price, stock } => {
                let name = ProductName::new(name);
                let desc = ProductDescription::new(desc);
                let price = Price::new(price);
                let stock = Stock::new(stock);
                
                let id = ProductId::default();
                
                let product = Product::create(id, name, desc, stock, price);
                
                self.product_repository()
                    .create(&product)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            ProductCommand::UpdateName { name } => {
                let id = id.ok_or(Report::new(ApplicationError::Require { data: "id" }))?;
                
                let name = ProductName::new(name);
                
                let mut product = self.product_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
                
                product.substitute(|prod| {
                    *prod.name = name;
                });
                
                self.product_repository()
                    .update(&id, &product)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            ProductCommand::UpdateDescription { desc } => {
                let id = id.ok_or(Report::new(ApplicationError::Require { data: "id" }))?;

                let desc = ProductDescription::new(desc);

                let mut product = self.product_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;

                product.substitute(|prod| {
                    *prod.description = desc;
                });

                self.product_repository()
                    .update(&id, &product)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            ProductCommand::StockIn { amount } => {
                let id = id.ok_or(Report::new(ApplicationError::Require { data: "id" }))?;

                let amount = Stock::new(amount);

                let mut product = self.product_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;

                product.substitute(|prod| {
                    prod.stock.in_stock(amount);
                });

                self.product_repository()
                    .update(&id, &product)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            ProductCommand::StockOut { amount } => {
                let id = id.ok_or(Report::new(ApplicationError::Require { data: "id" }))?;

                let mut product = self.product_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;

                product.substitute(|prod| {
                    prod.stock.bring_out(amount);
                });

                self.product_repository()
                    .update(&id, &product)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            ProductCommand::UpdatePrice { price } => {
                let id = id.ok_or(Report::new(ApplicationError::Require { data: "id" }))?;

                let price = Price::new(price);

                let mut product = self.product_repository()
                    .find_by_id(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;

                product.substitute(|prod| {
                    *prod.price = price;
                });

                self.product_repository()
                    .update(&id, &product)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
            ProductCommand::Delete => {
                let id = id.ok_or(Report::new(ApplicationError::Require { data: "id" }))?;

                self.product_repository()
                    .delete(&id)
                    .await
                    .change_context_lazy(|| ApplicationError::Driver)?;
            }
        }
        
        Ok(())
    }
}