use kernel::entities::ProductId;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct CreateCatalogBase {
    pub name: String,
    pub desc: String,
    pub base: i32,
    pub main: BTreeMap<i32, ProductId>,
    
    #[serde(default)]
    pub subs: Option<BTreeMap<i32, ProductId>>,
    #[serde(default)]
    pub opts: Option<BTreeMap<i32, ProductId>>
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