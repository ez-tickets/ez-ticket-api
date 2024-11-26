use async_trait::async_trait;
use error_stack::Report;
use nitinol::Event;
use crate::errors::ApplicationError;

#[async_trait]
pub trait EventStore: 'static + Sync + Send {
    async fn write<E: Event>(&self, aggregate_id: &str, event: &E, seq: i64) -> Result<(), Report<ApplicationError>>;
}


pub trait DependOnEventStore: 'static + Sync + Send {
    type EventStore: EventStore;
    fn event_store(&self) -> &Self::EventStore;
}