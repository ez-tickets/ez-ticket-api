use serde::Deserialize;
use kernel::entities::ImageId;

#[derive(Deserialize)]
pub struct ImageFindById {
    id: ImageId
}