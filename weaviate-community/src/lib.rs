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
mod utils;
pub use self::backups::_Backups;
pub use self::batch::Batch;
pub use self::classification::_Classification;
pub use self::meta::Meta;
pub use self::modules::_Modules;
pub use self::nodes::Nodes;
pub use self::objects::Objects;
pub use self::oidc::Oidc;
pub use self::schema::Schema;
use std::sync::Arc;

use reqwest::Url;
use std::error::Error;

pub struct WeaviateClient {
    pub base_url: Url,
    client: Arc<reqwest::Client>,
    pub schema: Schema,
    pub objects: Objects,
    pub batch: Batch,
    pub backups: _Backups,
    pub classification: _Classification,
    pub meta: Meta,
    pub nodes: Nodes,
    pub oidc: Oidc,
    pub modules: _Modules,
}

impl WeaviateClient {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        let client = Arc::new(reqwest::Client::new());
        let schema = Schema::new(&base, Arc::clone(&client))?;
        let objects = Objects::new(&base, Arc::clone(&client))?;
        let batch = Batch::new(&base, Arc::clone(&client))?;
        let backups = _Backups::new(&base, Arc::clone(&client))?;
        let classification = _Classification::new(&base, Arc::clone(&client))?;
        let meta = Meta::new(&base, Arc::clone(&client))?;
        let nodes = Nodes::new(&base, Arc::clone(&client))?;
        let oidc = Oidc::new(&base, Arc::clone(&client))?;
        let modules = _Modules::new(&base, Arc::clone(&client))?;
        Ok(WeaviateClient {
            base_url: base,
            client,
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

    /// Determine if the application is ready to receive traffic.
    ///
    /// https://weaviate.io/developers/weaviate/api/rest/well-known#liveness
    ///
    /// GET /v1/.well-known/live
    ///
    /// Endpoint returns HTTP status code 200 if the application is able to respond to HTTP
    /// requests.
    ///
    /// # Returns
    /// * Ok(bool) => True if 200, False otherwise
    ///
    /// # Errors
    /// When there is a reqwest error
    pub async fn is_live(&self) -> Result<bool, Box<dyn Error>> {
        let endpoint = self.base_url.join("/v1/.well-known/live")?;
        let resp = self.client.get(endpoint).send().await?;
        match resp.status() {
            reqwest::StatusCode::OK => Ok(true),
            _ => Ok(false),
        }
    }

    /// Determine if the application is ready to receive traffic.
    ///
    /// https://weaviate.io/developers/weaviate/api/rest/well-known#readiness
    ///
    /// GET /v1/.well-known/ready
    ///
    /// Endpoint returns HTTP status code 200 if the application is able to respond to HTTP
    /// requests, and 503 if the application is not able to serve traffic. If other horizontal
    /// replicas of Weaviate are available and they are capable of receiving traffic, all traffic
    /// should be redirected there instead.
    ///
    /// # Returns
    /// * bool => True if 200, False otherwise
    ///
    /// # Errors
    /// When there is a reqwest error
    pub async fn is_ready(&self) -> Result<bool, Box<dyn Error>> {
        let endpoint = self.base_url.join("/v1/.well-known/ready")?;
        let resp = self.client.get(endpoint).send().await?;
        match resp.status() {
            reqwest::StatusCode::OK => Ok(true),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the is_live endpoint returns true when it is expected to.
    #[tokio::test]
    async fn test_is_live_true() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let res = client.is_live().await;
        assert!(res.unwrap())
    }

    /// Test that the is_live endpoint returns false when it is expected to.
    #[tokio::test]
    async fn test_is_live_false() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let _res = client.is_live().await;
    }

    /// Test that the is_ready endpoint returns true when it is expected to.
    #[tokio::test]
    async fn test_is_ready_true() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let res = client.is_ready().await;
        assert!(res.unwrap())
    }

    /// Test that the is_ready endpoint returns false when it is expected to.
    #[tokio::test]
    async fn test_is_ready_false() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let _res = client.is_ready().await;
    }
}
