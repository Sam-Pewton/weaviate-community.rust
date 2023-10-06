//! # weaviate-client-community
//!
//! The `weaviate-client-community` crate...
//!
mod backups;
mod batch;
mod classification;
pub mod collections;
mod meta;
mod modules;
mod nodes;
mod objects;
mod oidc;
mod schema;
pub use self::backups::_Backups;
pub use self::batch::_Batch;
pub use self::classification::_Classification;
pub use self::collections::Class;
pub use self::meta::Meta;
pub use self::modules::_Modules;
pub use self::nodes::_Nodes;
pub use self::objects::Objects;
pub use self::oidc::_OIDC;
pub use self::schema::Schema;

use reqwest::Url;
use std::error::Error;

pub struct Client {
    pub base_url: Url,
    pub schema: Schema,
    pub objects: Objects,
    pub batch: _Batch,
    pub backups: _Backups,
    pub classification: _Classification,
    pub meta: Meta,
    pub nodes: _Nodes,
    pub oidc: _OIDC,
    pub modules: _Modules,
}

impl Client {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        let schema = Schema::new(&base)?;
        let objects = Objects::new(&base)?;
        let batch = _Batch::new(&base)?;
        let backups = _Backups::new(&base)?;
        let classification = _Classification::new(&base)?;
        let meta = Meta::new(&base)?;
        let nodes = _Nodes::new(&base)?;
        let oidc = _OIDC::new(&base)?;
        let modules = _Modules::new(&base)?;
        Ok(Client {
            base_url: base,
            schema,
            objects,
            batch,
            backups,
            classification,
            meta,
            nodes,
            oidc,
            modules,
        })
    }
}

#[cfg(test)]
mod tests {
    //use crate::schema::Class;

    //use super::*;

    //#[tokio::test]
    //async fn it_works() {
    //    let class = Class {
    //        class: "Test".into(),
    //        description: "Test".into(),
    //        properties: None,
    //        vector_index_type: None,
    //        vector_index_config: None,
    //        vectorizer: None,
    //        module_config: None,
    //        inverted_index_config: None,
    //        sharding_config: None,
    //        multi_tenancy_config: None,
    //    };
    //    let client = Client::new("http://localhost:8080").unwrap();
    //    client.schema.create_class(class).await;
    //    let test = client.schema.get(None).await;
    //    //let test = client.schema.get(Some("Embeddings")).await;
    //    //let test = client.schema.get_single_class("Embeddings").await;
    //    println!("{:#?}", test);
    //    //assert_eq!("http://localhost:8080", client.base_url);
    //}
}
