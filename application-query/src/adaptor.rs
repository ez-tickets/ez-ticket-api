use nitinol::projection::Projector;

pub trait DependOnEventQueryProjector: 'static + Sync + Send {
    fn projector(&self) -> &Projector;
}