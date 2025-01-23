#[allow(unused_imports)]
use error_stack::{Report, ResultExt};
#[allow(unused_imports)]
use nitinol::eventstream::EventStream;
#[allow(unused_imports)]
use nitinol::process::eventstream::EventStreamExtension;
#[allow(unused_imports)]
use nitinol::process::manager::ProcessManager;
#[allow(unused_imports)]
use nitinol::process::persistence::PersistenceExtension;
#[allow(unused_imports)]
use nitinol::projection::EventProjector;
#[allow(unused_imports)]
use nitinol::protocol::adapter::inmemory::InMemoryEventStore;
#[allow(unused_imports)]
use nitinol::protocol::io::ReadProtocol;
#[allow(unused_imports)]
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
        let stream = EventStream::default();
        let manager = ProcessManager::with_extension(|ext| {
            ext.install(PersistenceExtension::new(inmemory.clone()))?
                .install(EventStreamExtension::new(stream))
        }).change_context_lazy(|| UnrecoverableError)?;
        
        let projector = EventProjector::new(inmemory.clone());
        
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