use crate::collections::error::NodesError;
use crate::collections::nodes::MultiNodes;
use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

/// All nodes related endpoints and functionality described in
/// [Weaviate nodes API documentation](https://weaviate.io/developers/weaviate/api/rest/nodes)
#[derive(Debug)]
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
    /// # Examples
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>>{
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///     let res = client.nodes.get_nodes_status().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_nodes_status(&self) -> Result<MultiNodes, Box<dyn Error>> {
        let res = self.client.get(self.endpoint.clone()).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: MultiNodes = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(NodesError(format!(
                "status code {} received when calling get_nodes_status endpoint.",
                res.status()
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{collections::nodes::MultiNodes, WeaviateClient};

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn test_nodes() -> MultiNodes {
        let nodes: MultiNodes = serde_json::from_value(serde_json::json!(
        {
            "nodes": [
              {
                "batchStats": {
                  "ratePerSecond": 0
                },
                "gitHash": "e6b37ce",
                "name": "weaviate-0",
                "shards": [
                  {
                    "class": "TestArticle",
                    "name": "nq1Bg9Q5lxxP",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  },
                  {
                    "class": "TestAuthor",
                    "name": "MINLtCghkdG8",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  }
                ],
                "stats": {
                  "objectCount": 0,
                  "shardCount": 2
                },
                "status": "HEALTHY",
                "version": "1.22.1"
              },
              {
                "batchStats": {
                  "ratePerSecond": 0
                },
                "gitHash": "e6b37ce",
                "name": "weaviate-1",
                "shards": [
                  {
                    "class": "TestArticle",
                    "name": "HuPocHE5w2LP",
                    "objectCount": 1,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  },
                  {
                    "class": "TestAuthor",
                    "name": "PeQjZRmK0xNB",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  }
                ],
                "stats": {
                  "objectCount": 1,
                  "shardCount": 2
                },
                "status": "HEALTHY",
                "version": "1.22.1"
              },
              {
                "batchStats": {
                  "ratePerSecond": 0
                },
                "gitHash": "e6b37ce",
                "name": "weaviate-2",
                "shards": [
                  {
                    "class": "TestArticle",
                    "name": "JTg39c7ZlFUX",
                    "objectCount": 0,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  },
                  {
                    "class": "TestAuthor",
                    "name": "W5ulmuJGDTxj",
                    "objectCount": 1,
                    "vectorIndexingStatus": "READY",
                    "vectorQueueLength": 0
                  }
                ],
                "stats": {
                  "objectCount": 1,
                  "shardCount": 2
                },
                "status": "HEALTHY",
                "version": "1.22.1"
              }
            ]
          }))
        .unwrap();
        nodes
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
    async fn test_get_nodes_status_ok() {
        let (mut mock_server, client) = get_test_harness();
        let nodes = test_nodes();
        let nodes_str = serde_json::to_string(&nodes).unwrap();
        let mock = mock_get(&mut mock_server, "/v1/nodes/", 200, &nodes_str);
        let res = client.nodes.get_nodes_status().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(res.unwrap().nodes.len(), nodes.nodes.len());
    }

    #[tokio::test]
    async fn test_get_nodes_status_err() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/v1/nodes/", 404, "");
        let res = client.nodes.get_nodes_status().await;
        mock.assert();
        assert!(res.is_err());
    }
}
