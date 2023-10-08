use reqwest::{Response, Url};
use std::error::Error;
use std::sync::Arc;

/// All nodes related endpoints and functionality described in
/// [Weaviate nodes API documentation](https://weaviate.io/developers/weaviate/api/rest/nodes)
pub struct Nodes {
    /// The full URL to the Meta endpoint
    endpoint: Url,
    /// The sub-client which executes the requests - temporary
    client: Arc<reqwest::Client>,
}

impl Nodes {
    /// Create a new instance of the Nodes endpoint struct. Should only be done by the parent
    /// client.
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/nodes/")?;
        Ok(Nodes { endpoint, client })
    }

    /// Get the node status for all nodes in the Weaviate instance.
    ///
    /// # Return value
    ///
    /// * Full Response of get request, deserializable into an array of nodes containing the
    /// following fields:
    /// - name
    /// - status
    /// - version
    /// - gitHash
    /// - stats
    ///   - shardCount
    ///   - objectCount
    /// - shards
    ///   - name
    ///   - class
    ///   - objectCount
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
    ///     let res = client.nodes.get_nodes_status().await;
    ///     println!("{:#?}", res.unwrap().json::<serde_json::Value>().await);
    /// }
    /// ```
    pub async fn get_nodes_status(&self) -> Result<Response, Box<dyn Error>> {
        let res = self.client.get(self.endpoint.clone()).send().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::WeaviateClient;

    #[tokio::test]
    async fn test_get_nodes_status() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let res = client.nodes.get_nodes_status().await;
        let nodes = res.unwrap().json::<serde_json::Value>().await.unwrap();
        assert_eq!("weaviate1", nodes["nodes"][0]["name"]);
    }
}
