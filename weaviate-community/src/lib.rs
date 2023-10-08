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
use std::sync::Arc;
pub use self::backups::_Backups;
pub use self::batch::_Batch;
pub use self::classification::_Classification;
pub use self::collections::schema::Class;
pub use self::meta::Meta;
pub use self::modules::_Modules;
pub use self::nodes::Nodes;
pub use self::objects::Objects;
pub use self::oidc::_OIDC;
pub use self::schema::Schema;

use reqwest::Url;
use std::error::Error;

pub struct Client {
    pub base_url: Url,
    _client: Arc<reqwest::Client>,
    pub schema: Schema,
    pub objects: Objects,
    pub batch: _Batch,
    pub backups: _Backups,
    pub classification: _Classification,
    pub meta: Meta,
    pub nodes: Nodes,
    pub oidc: _OIDC,
    pub modules: _Modules,
}

impl Client {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        let client = Arc::new(reqwest::Client::new());
        let schema = Schema::new(&base, Arc::clone(&client))?;
        let objects = Objects::new(&base, Arc::clone(&client))?;
        let batch = _Batch::new(&base, Arc::clone(&client))?;
        let backups = _Backups::new(&base, Arc::clone(&client))?;
        let classification = _Classification::new(&base, Arc::clone(&client))?;
        let meta = Meta::new(&base, Arc::clone(&client))?;
        let nodes = Nodes::new(&base, Arc::clone(&client))?;
        let oidc = _OIDC::new(&base, Arc::clone(&client))?;
        let modules = _Modules::new(&base, Arc::clone(&client))?;
        Ok(Client {
            base_url: base,
            _client: client,
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
}
