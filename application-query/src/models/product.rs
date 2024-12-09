use crate::errors::QueryError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::events::ProductEvent;
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::adaptor::DependOnEventQueryProjector;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllProducts(pub Vec<Product>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
}

impl ResolveMapping for AllProducts {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<ProductEvent>();
    }
}

#[async_trait]
impl Projection<ProductEvent> for AllProducts {
    type Rejection = QueryError;

    async fn apply(&mut self, event: ProductEvent) -> Result<(), Self::Rejection> {
        match event {
            ProductEvent::Created { id, name } => {
                self.0.push(Product {
                    id: id.into(),
                    name: name.into(),
                });
            }
            ProductEvent::UpdatedName { id, name } => {
                if let Some(p) = self.0.iter_mut().find(|p| &p.id == id.as_ref()) { 
                    p.name = name.into();
                }
            }
            ProductEvent::Deleted { id } => {
                self.0.retain(|p| &p.id != id.as_ref());
            }
        }
        Ok(())
    }
}

#[async_trait]
pub trait ProductQueryService: 'static + Sync + Send 
where 
    Self: DependOnEventQueryProjector
{
    async fn find_all(&self) -> Result<AllProducts, Report<QueryError>> {
        let products = self.projector()
            .projection_with_resolved_events(AllProducts::default())
            .await
            .change_context_lazy(|| QueryError)?;
        Ok(products.0)
    }
}

pub trait DependOnProductQueryService: 'static + Sync + Send {
    type ProductQueryService: ProductQueryService;
    fn product_query_service(&self) -> &Self::ProductQueryService;
}

impl<T> ProductQueryService for T
where 
    T: DependOnEventQueryProjector
{}