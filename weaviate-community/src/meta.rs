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
    ///     let client = WeaviateClient::new("http://localhost:8080", None)?;
    ///     let res = client.meta.get_meta().await?;
    ///     println!("{:#?}", res);
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
    use crate::{WeaviateClient, AuthApiKey};

    /// Test the get_meta endpoint
    #[tokio::test]
    async fn test_get_meta() {
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let res = client.meta.get_meta().await;
        assert_eq!(
            "http://[::]:8080",
            res.unwrap().hostname
        );
    }
}
