use nitinol::process::registry::ProcessSystem;

pub trait DependOnProcessRegistry: 'static + Sync + Send {
    type ProcessRegistry: ProcessSystem;
    fn process_registry(&self) -> Self::ProcessRegistry;
}
