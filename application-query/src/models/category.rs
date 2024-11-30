use crate::errors::QueryError;
use async_trait::async_trait;
use error_stack::{Report, ResultExt};
use kernel::events::{CategoriesEvent, CategoryEvent};
use nitinol::projection::Projection;
use nitinol::resolver::{Mapper, ResolveMapping};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::convert::Infallible;
use uuid::Uuid;
use kernel::entities::CategoryId;
use crate::adaptor::DependOnEventQueryProjector;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Categories(pub BTreeSet<Category>);

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub ordering: i32,
}

impl Eq for Category {}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd<Self> for Category {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        Some(self.cmp(other)) 
    }
}

impl Ord for Category {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering.cmp(&other.ordering)
    }
}

impl ResolveMapping for Categories {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<CategoriesEvent>();
    }
}

#[async_trait]
impl Projection<CategoriesEvent> for Categories {
    type Rejection = Infallible;

    async fn apply(&mut self, event: CategoriesEvent) -> Result<(), Self::Rejection> {
        match event {
            CategoriesEvent::Added { id, ordering } => {
                self.0.insert(Category {
                    id: id.into(),
                    name: "".to_string(),
                    ordering,
                });
            }
            CategoriesEvent::Removed { id } => {
                self.0.retain(|category| category.id != id.into());
            }
            CategoriesEvent::Updated { new } => {
                self.0 = new.into_iter()
                    .map(|(order, id)| Category { 
                        id: id.into(), 
                        name: "".to_string(),
                        ordering: order
                    })
                    .collect()
            }
        }
        
        Ok(())
    }
}

impl ResolveMapping for Category {
    fn mapping(mapper: &mut Mapper<Self>) {
        mapper.register::<CategoryEvent>();
    }
}

#[async_trait]
impl Projection<CategoryEvent> for Category {
    type Rejection = Infallible;

    async fn apply(&mut self, event: CategoryEvent) -> Result<(), Self::Rejection> {
        if let CategoryEvent::UpdatedName { name, .. } 
             | CategoryEvent::Created { name, .. } = event {
            self.name = name.into();
        }
        Ok(())
    }
}


#[async_trait]
pub trait CategoryQueryService: 'static + Sync + Send 
where
    Self: DependOnEventQueryProjector
{
    async fn find_all_category(&self) -> Result<Categories, Report<QueryError>> {
        let (categories, _) = self.projector()
            .projection_with_resolved_events(Categories::default())
            .await
            .change_context_lazy(|| QueryError)?;

        let mut categories_new = BTreeSet::new();
        for category in categories.0.into_iter() {
            let (category, _) = self.projector()
                .projection_to_latest(CategoryId::new(category.id), (category, 0))
                .await
                .change_context_lazy(|| QueryError)?;
            categories_new.insert(category);
        }

        Ok(Categories(categories_new))
    }
}

pub trait DependOnCategoryQueryService: 'static + Sync + Send {
    type CategoryQueryService: CategoryQueryService;
    fn category_query_service(&self) -> &Self::CategoryQueryService;
}

impl<T> CategoryQueryService for T
where T: DependOnEventQueryProjector
{}