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
pub use self::backups::Backups;
pub use self::batch::Batch;
pub use self::classification::_Classification;
pub use self::meta::Meta;
pub use self::modules::_Modules;
pub use self::nodes::Nodes;
pub use self::objects::Objects;
pub use self::oidc::Oidc;
pub use self::schema::Schema;
use std::sync::Arc;

use collections::auth::AuthApiKey;
use reqwest::Url;
use reqwest::header::HeaderMap;
use reqwest::header::AUTHORIZATION;
use std::error::Error;

/// An asynchronous `WeaviateClient` to interact with a Weaviate database.
#[derive(Debug)]
pub struct WeaviateClient {
    pub base_url: Url,
    client: Arc<reqwest::Client>,
    pub schema: Schema,
    pub objects: Objects,
    pub batch: Batch,
    pub backups: Backups,
    pub classification: _Classification,
    pub meta: Meta,
    pub nodes: Nodes,
    pub oidc: Oidc,
    pub modules: _Modules,
}

impl WeaviateClient {
    /// Construct a new `WeaviateClient`
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::auth::AuthApiKey;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let auth = AuthApiKey::new("test-key");
    ///     let client = WeaviateClient::new("http://localhost:8080", Some(auth))?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(url: &str, auth_client_secret: Option<AuthApiKey>) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        let mut client_builder = reqwest::Client::builder();

        // Add the authorization header to the client if it is present
        if let Some(auth) = auth_client_secret {
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                auth.get_header_value()
            );
            client_builder = client_builder.default_headers(headers);
        };

        // Each of the endpoint categories hold a strong ref to the main client.
        let client = Arc::new(client_builder.build()?);
        let schema = Schema::new(&base, Arc::clone(&client))?;
        let objects = Objects::new(&base, Arc::clone(&client))?;
        let batch = Batch::new(&base, Arc::clone(&client))?;
        let backups = Backups::new(&base, Arc::clone(&client))?;
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
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::auth::AuthApiKey;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let auth = AuthApiKey::new("test-key");
    ///     let client = WeaviateClient::builder("http://localhost:8080")
    ///         .auth_secret(auth)
    ///         .build()?;
    ///     let res = client.is_live().await;
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::auth::AuthApiKey;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let auth = AuthApiKey::new("test-key");
    ///     let client = WeaviateClient::builder("http://localhost:8080")
    ///         .auth_secret(auth)
    ///         .build()?;
    ///     let res = client.is_ready().await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn is_ready(&self) -> Result<bool, Box<dyn Error>> {
        let endpoint = self.base_url.join("/v1/.well-known/ready")?;
        let resp = self.client.get(endpoint).send().await?;
        match resp.status() {
            reqwest::StatusCode::OK => Ok(true),
            _ => Ok(false),
        }
    }

    /// Builder for the WeaviateClient
    ///
    /// # Examples
    /// Anonymous
    /// ```
    /// use weaviate_community::WeaviateClient;
    /// let client = WeaviateClient::builder("http://localhost:8080").build();
    /// ```
    ///
    /// Authenticated with API key
    /// ```
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::auth::AuthApiKey;
    ///
    /// let auth = AuthApiKey::new("your-key");
    /// let client = WeaviateClient::builder("http://localhost:8080").auth_secret(auth).build();
    /// ```
    pub fn builder(url: &str) -> WeaviateClientBuilder {
        WeaviateClientBuilder::new(url.into())
    }
}


/// A `WeaviateClientBuilder` can be used to create a new `WeaviateClient`.
#[derive(Default, Debug)]
pub struct WeaviateClientBuilder {
    pub base_url: String,
    pub auth_secret: Option<AuthApiKey>,
}

impl WeaviateClientBuilder {
    /// Construct a new `WeaviateClientBuilder`.
    ///
    /// This is the same as `WeaviateClient::builder()`.
    pub fn new(base_url: &str) -> WeaviateClientBuilder {
        WeaviateClientBuilder { base_url: base_url.into(), auth_secret: None }
    }

    /// Sets the authentication token to be used by the client.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClientBuilder;
    /// use weaviate_community::collections::auth::AuthApiKey;
    ///
    /// let auth = AuthApiKey::new("your-key");
    /// let client = WeaviateClientBuilder::new("http://localhost:8080")
    ///     .auth_secret(auth)
    ///     .build();
    /// ```
    pub fn auth_secret(mut self, auth_secret: AuthApiKey) -> WeaviateClientBuilder {
        self.auth_secret = Some(auth_secret);
        self
    }

    /// Build a `WeaviateClient` from the values set in the WeaviateClientBuilder.
    pub fn build(self) -> Result<WeaviateClient, Box<dyn Error>> {
        let client = WeaviateClient::new(&self.base_url, self.auth_secret)?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn mock_get(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
    ) -> mockito::Mock {
        server.mock("GET", endpoint)
            .with_status(status_code)
            .create()
    }

    #[tokio::test]
    async fn test_is_ready_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_is_ready_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_is_live_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_is_live_err() {
        let (mut mock_server, client) = get_test_harness();
    }
}
