//! # weaviate-community
//!
//! Community client for handling Weaviate vector database transactions written in Rust, for Rust.
//! More information on Weaviate can be found on the official Weaviate webpage.
mod backups;
mod batch;
mod classification;
pub mod collections;
mod meta;
mod modules;
mod nodes;
mod objects;
mod oidc;
mod query;
mod schema;
pub use self::backups::Backups;
pub use self::batch::Batch;
pub use self::classification::Classification;
pub use self::meta::Meta;
pub use self::modules::Modules;
pub use self::nodes::Nodes;
pub use self::objects::Objects;
pub use self::oidc::Oidc;
pub use self::query::Query;
pub use self::schema::Schema;
use collections::auth::{ApiKey, AuthApiKey};

use std::error::Error;
use std::sync::Arc;

use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::Url;

/// An asynchronous `WeaviateClient` to interact with a Weaviate database.
#[derive(Debug)]
pub struct WeaviateClient {
    pub base_url: Url,
    client: Arc<reqwest::Client>,
    pub schema: Schema,
    pub objects: Objects,
    pub batch: Batch,
    pub backups: Backups,
    pub classification: Classification,
    pub meta: Meta,
    pub nodes: Nodes,
    pub oidc: Oidc,
    pub modules: Modules,
    pub query: Query,
}

impl WeaviateClient {
    /// Construct a new `WeaviateClient`
    ///
    /// # Parameters
    /// - url: the root url for the client
    /// - auth_client_secret: the API authentication key
    ///
    /// # Example
    /// Using the WeaviateClient
    /// ```
    /// use std::collections::HashMap;
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::auth::{AuthApiKey, ApiKey};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let auth = AuthApiKey::new("test-key");
    ///     let client = WeaviateClient::new(
    ///         "http://localhost:8080",
    ///         Some(auth),
    ///         Some(vec![]),
    ///     )?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Using the WeaviateClientBuilder
    /// ```
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080")
    ///         .with_auth_secret("test-key")
    ///         .with_api_key("X-OpenAI-Api-Key", "your-key")
    ///         .build();
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        url: &str,
        auth_client_secret: Option<AuthApiKey>,
        api_keys: Option<Vec<ApiKey>>,
    ) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        let mut client_builder = reqwest::Client::builder();

        let mut headers = HeaderMap::new();

        // Add the authorization header to the client if it is present
        if let Some(auth) = auth_client_secret {
            headers.insert(AUTHORIZATION, auth.get_header_value());
        };

        // Add any of the other header keys to the client, for example, OpenAI
        if let Some(keys) = api_keys {
            for i in 0..keys.len() {
                headers.insert(
                    keys.get(i).unwrap().get_header_name(),
                    keys.get(i).unwrap().get_header_value(),
                );
            }
        }

        client_builder = client_builder.default_headers(headers);

        // Each of the endpoint categories hold a strong ref to the main client.
        let client = Arc::new(client_builder.build()?);
        let schema = Schema::new(&base, Arc::clone(&client))?;
        let objects = Objects::new(&base, Arc::clone(&client))?;
        let batch = Batch::new(&base, Arc::clone(&client))?;
        let backups = Backups::new(&base, Arc::clone(&client))?;
        let classification = Classification::new(&base, Arc::clone(&client))?;
        let meta = Meta::new(&base, Arc::clone(&client))?;
        let nodes = Nodes::new(&base, Arc::clone(&client))?;
        let oidc = Oidc::new(&base, Arc::clone(&client))?;
        let modules = Modules::new(&base, Arc::clone(&client))?;
        let query = Query::new(&base, Arc::clone(&client))?;

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
            query,
        })
    }

    /// Determine if the application is ready to receive traffic.
    ///
    /// More info on the liveness can be found [here](https://weaviate.io/developers/weaviate/api/rest/well-known#liveness)
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
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080")
    ///         .with_auth_secret("test-key")
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
    /// More info on the readiness can be found [here](https://weaviate.io/developers/weaviate/api/rest/well-known#readiness)
    ///
    /// GET /v1/.well-known/ready
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080")
    ///         .with_auth_secret("test-key")
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
    /// # Parameters
    /// - base_url: the root url for the client
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
    ///
    /// let client = WeaviateClient::builder("http://localhost:8080")
    ///     .with_auth_secret("your-key")
    ///     .build();
    /// ```
    pub fn builder(base_url: &str) -> WeaviateClientBuilder {
        WeaviateClientBuilder::new(base_url)
    }
}

/// A `WeaviateClientBuilder` can be used to create a new `WeaviateClient`.
#[derive(Default, Debug)]
pub struct WeaviateClientBuilder {
    pub base_url: String,
    pub auth_secret: Option<AuthApiKey>,
    pub api_keys: Vec<ApiKey>,
}

impl WeaviateClientBuilder {
    /// Construct a new `WeaviateClientBuilder`.
    ///
    /// # Parameters
    /// - base_url: the root url for the client
    ///
    /// This is the same as `WeaviateClient::builder()`.
    ///
    /// # Examples
    /// Anonymous
    /// ```
    /// use weaviate_community::WeaviateClientBuilder;
    /// let client = WeaviateClientBuilder::new("http://localhost:8080").build();
    /// ```
    ///
    /// Authenticated with API key
    /// ```
    /// use weaviate_community::WeaviateClientBuilder;
    ///
    /// let client = WeaviateClientBuilder::new("http://localhost:8080")
    ///     .with_auth_secret("your-key")
    ///     .build();
    /// ```
    pub fn new(base_url: &str) -> WeaviateClientBuilder {
        WeaviateClientBuilder {
            base_url: base_url.into(),
            auth_secret: None,
            api_keys: Vec::new(),
        }
    }

    /// Sets the authentication token to be used by the client.
    ///
    /// # Parameters
    /// - auth_secret: the AuthApiKey to set in the client
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClientBuilder;
    ///
    /// let client = WeaviateClientBuilder::new("http://localhost:8080")
    ///     .with_auth_secret("your-key")
    ///     .build();
    /// ```
    pub fn with_auth_secret(mut self, auth_secret: &str) -> WeaviateClientBuilder {
        self.auth_secret = Some(AuthApiKey::new(auth_secret));
        self
    }

    /// Sets a new api key to be used by the client.
    ///
    /// # Parameters
    /// - header: the header to set in the client
    /// - api_key: the api key
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClientBuilder;
    ///
    /// let client = WeaviateClientBuilder::new("http://localhost:8080")
    ///     .with_api_key("X-OpenAI-Api-Key", "abcdefg")
    ///     .build();
    /// ```
    pub fn with_api_key(mut self, header: &str, api_key: &str) -> WeaviateClientBuilder {
        self.api_keys.push(ApiKey {
            api_header: header.into(),
            api_key: api_key.into(),
        });
        self
    }

    /// Build a `WeaviateClient` from the values set in the WeaviateClientBuilder.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClientBuilder;
    ///
    /// let client = WeaviateClientBuilder::new("http://localhost:8080").build();
    /// ```
    pub fn build(self) -> Result<WeaviateClient, Box<dyn Error>> {
        let client = WeaviateClient::new(&self.base_url, self.auth_secret, Some(self.api_keys))?;
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
        body: &str,
    ) -> mockito::Mock {
        server
            .mock("GET", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    #[tokio::test]
    async fn test_is_ready_ok() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/v1/.well-known/ready", 200, "");
        let res = client.is_ready().await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_is_ready_err() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/v1/.well-known/ready", 503, "");
        let res = client.is_ready().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(false, res.unwrap());
    }

    #[tokio::test]
    async fn test_is_live_ok() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/v1/.well-known/live", 200, "");
        let res = client.is_live().await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_is_live_err() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/v1/.well-known/live", 404, "");
        let res = client.is_live().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(false, res.unwrap());
    }
}
