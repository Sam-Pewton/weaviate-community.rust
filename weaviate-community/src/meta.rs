use reqwest::{Response, Url};
use std::error::Error;
use std::sync::Arc;

/// All meta related endpoints and functionality described in
/// [Weaviate meta API documentation](https://weaviate.io/developers/weaviate/api/rest/meta)
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
    ///
    /// ```
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = WeaviateClient::new("http://localhost:8080").unwrap();
    ///     let res = client.meta.get_meta().await;
    ///     println!("{:#?}", res.unwrap().json::<serde_json::Value>().await);
    /// }
    /// ```
    pub async fn get_meta(&self) -> Result<Response, Box<dyn Error>> {
        let res = self.client.get(self.endpoint.clone()).send().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::WeaviateClient;

    /// Test the get_meta endpoint
    #[tokio::test]
    async fn test_get_meta() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let res = client.meta.get_meta().await;
        assert_eq!(
            "http://[::]:8080",
            res.unwrap().json::<serde_json::Value>().await.unwrap()["hostname"]
        );
    }
}
