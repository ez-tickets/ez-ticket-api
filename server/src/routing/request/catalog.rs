use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCatalogBase {
    pub name: String,
    pub desc: String,
    pub base: i32,
}

#[derive(Debug)]
pub struct CreateCatalog {
    pub base: CreateCatalogBase,
    pub image: Vec<u8>,
}

pub struct RawCreateCatalog {
    pub base: Option<CreateCatalogBase>,
    pub image: Option<Vec<u8>>
}