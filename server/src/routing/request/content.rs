use serde::Deserialize;
use kernel::entities::ImageId;

#[derive(Deserialize)]
pub struct ImageFindById {
    pub id: ImageId
}