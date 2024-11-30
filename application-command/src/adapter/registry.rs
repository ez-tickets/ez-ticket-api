use nitinol::process::registry::Registry;

pub trait DependOnProcessRegistry: 'static + Sync + Send {
    fn process_registry(&self) -> Registry;
}
