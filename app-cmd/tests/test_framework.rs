use error_stack::{Report, ResultExt};
use nitinol::process::manager::ProcessManager;
use nitinol::process::persistence::PersistenceExtension;
use nitinol::projection::EventProjector;
use nitinol::protocol::adapter::inmemory::InMemoryEventStore;
use nitinol::protocol::io::ReadProtocol;
use app_cmd::adapter::{DependOnEventProjector, DependOnProcessManager};

#[derive(Debug, thiserror::Error)]
#[error("unrecoverable error")]
pub struct UnrecoverableError;

#[derive(Clone)]
pub struct TestFramework {
    manager: ProcessManager,
    projector: EventProjector,
    journal: InMemoryEventStore
}

impl TestFramework {
    pub fn new() -> Result<TestFramework, Report<UnrecoverableError>> {
        let inmemory = InMemoryEventStore::default();
        let manager = ProcessManager::with_extension(|ext| {
            ext.install(PersistenceExtension::new(inmemory.clone()))
        }).change_context_lazy(|| UnrecoverableError)?;
        
        let projector = EventProjector::new(ReadProtocol::new(inmemory.clone()));
        
        Ok(TestFramework { manager, projector, journal: inmemory })
    }
    
    pub fn journal(&self) -> ReadProtocol {
        ReadProtocol::new(self.journal.clone())
    }
}

impl DependOnProcessManager for TestFramework {
    fn process_manager(&self) -> &ProcessManager {
        &self.manager
    }
}

impl DependOnEventProjector for TestFramework {
    fn event_projector(&self) -> &EventProjector {
        &self.projector
    }
}