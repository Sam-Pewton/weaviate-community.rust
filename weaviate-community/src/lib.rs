//! # weaviate-client-community
//!
//! The `weaviate-client-community` crate...
//!
mod backups;
mod batch;
mod classification;
mod meta;
mod modules;
mod nodes;
mod objects;
mod oidc;
mod schema;
pub use self::backups::Backups;
pub use self::batch::Batch;
pub use self::classification::Classification;
pub use self::meta::Meta;
pub use self::modules::Modules;
pub use self::nodes::Nodes;
pub use self::objects::Objects;
pub use self::oidc::OIDC;
pub use self::schema::Schema;

use reqwest::Url;
use std::error::Error;

pub struct Client {
    pub base_url: Url,
    pub schema: Schema,
    pub objects: Objects,
    pub batch: Batch,
    pub backups: Backups,
    pub classification: Classification,
    pub meta: Meta,
    pub nodes: Nodes,
    pub oidc: OIDC,
    pub modules: Modules,
}

impl Client {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        let schema = Schema::new(&base)?;
        let objects = Objects::new(&base)?;
        let batch = Batch::new(&base)?;
        let backups = Backups::new(&base)?;
        let classification = Classification::new(&base)?;
        let meta = Meta::new(&base)?;
        let nodes = Nodes::new(&base)?;
        let oidc = OIDC::new(&base)?;
        let modules = Modules::new(&base)?;
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
    use crate::schema::Class;

    use super::*;

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
