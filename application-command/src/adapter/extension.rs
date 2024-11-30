use nitinol::process::extension::Extensions;

pub trait DependOnProcessExtension: 'static + Sync + Send {
    fn process_extension(&self) -> Extensions;
}