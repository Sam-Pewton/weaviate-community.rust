use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::meta::Metadata;

/// All meta related endpoints and functionality described in
/// [Weaviate meta API documentation](https://weaviate.io/developers/weaviate/api/rest/meta)
#[derive(Debug)]
pub struct Meta {
    /// The full URL to the Meta endpoint
    endpoint: Url,
    /// The sub-client which executes the requests - temporary
    client: Arc<reqwest::Client>,
}

impl Meta {
    /// Create a new instance of the Meta endpoint struct. Should only be done by the parent
    /// client.
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/meta/")?;
        Ok(Meta { endpoint, client })
    }

    /// Get the metadata associated to the clients Weaviate instance.
    ///
    /// # Return value
    ///
    /// * Full Response of get request, deserializable into: hostname, version, module
    ///
    /// # Errors
    ///
    /// If the client is unable to execute get, an Err result is returned.
    ///
    /// # Examples
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let res = client.meta.get_meta().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_meta(&self) -> Result<Metadata, Box<dyn Error>> {
        let res = self.client.get(self.endpoint.clone()).send().await?;
        let res: Metadata = res.json().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{collections::meta::Metadata, WeaviateClient};

    async fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new_async().await;
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn test_metadata() -> Metadata {
        let data: Metadata = serde_json::from_value(serde_json::json!({
            "hostname": "http://[::]:8080",
            "modules": {
                "text2vec-contextionary": {
                  "version": "en0.16.0-v0.4.21",
                  "wordCount": 818072
                }
            },
            "version": "1.0.0"
        }))
        .unwrap();
        data
    }

    async fn mock_get(
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
    async fn test_get_meta_ok() {
        let (mut mock_server, client) = get_test_harness().await;
        let metadata = test_metadata();
        let metadata_str = serde_json::to_string(&metadata).unwrap();
        let mock = mock_get(&mut mock_server, "/v1/meta/", 200, &metadata_str).await;
        let res = client.meta.get_meta().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(res.unwrap().hostname, metadata.hostname);
    }

    #[tokio::test]
    async fn test_get_meta_err() {
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, "/v1/meta/", 404, "").await;
        let res = client.meta.get_meta().await;
        mock.assert();
        assert!(res.is_err());
    }
}
