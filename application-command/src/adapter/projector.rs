use nitinol::projection::Projector;

pub trait DependOnProjector: 'static + Sync + Send {
    fn projector(&self) -> &Projector;
}