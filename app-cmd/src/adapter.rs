use nitinol::process::manager::ProcessManager;
use nitinol::projection::EventProjector;

pub trait DependOnProcessManager: 'static + Sync + Send {
    fn process_manager(&self) -> &ProcessManager;
}

pub trait DependOnEventProjector: 'static + Sync + Send {
    fn event_projector(&self) -> &EventProjector;
}

pub(crate) mod utils {
    use error_stack::{Report, ResultExt};
    use nitinol::process::{Process, Ref};
    use nitinol::process::manager::ProcessManager;
    use nitinol::projection::EventProjector;
    use nitinol::projection::resolver::ResolveMapping;
    use nitinol::ToEntityId;
    use crate::errors::ApplicationError;

    pub async fn find_or_replay<T>(
        id: impl ToEntityId, 
        manager: &ProcessManager, 
        projector: &EventProjector
    ) -> Result<Ref<T>, Report<ApplicationError>> 
        where T: Process + ResolveMapping
    {
        let id = id.to_entity_id();
        
        let Some(refs) = manager.find::<T>(id.clone()).await
            .change_context_lazy(|| ApplicationError::Process)? 
        else {
            let replay = projector.projection_to_latest::<T>(id.clone(), None).await
                .change_context_lazy(|| ApplicationError::Formation)?;
            let refs = manager.spawn(id, replay.0, replay.1).await
                .change_context_lazy(|| ApplicationError::Process)?;
            return Ok(refs)
        };
        
        Ok(refs)
    }
}