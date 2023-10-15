use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::{
    batch::{BatchAddObject, BatchDeleteRequest, BatchDeleteResponse},
    error::BatchError,
    objects::{ConsistencyLevel, Objects},
};

#[derive(Debug)]
pub struct Batch {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Batch {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/batch/")?;
        Ok(Batch { endpoint, client })
    }

    pub async fn objects_batch_add(
        &self,
        objects: Objects,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<Vec<BatchAddObject>, Box<dyn Error>> {
        let mut endpoint = self.endpoint.join("objects")?;
        if let Some(x) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", x.value());
        }
        let payload = serde_json::to_value(&objects)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Vec<BatchAddObject> = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(BatchError(format!(
                "status code {} received.",
                res.status()
            )))),
        }
    }

    pub async fn objects_batch_delete(
        &self,
        request_body: BatchDeleteRequest,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<BatchDeleteResponse, Box<dyn Error>> {
        let mut endpoint = self.endpoint.join("objects")?;
        if let Some(x) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", x.value());
        }
        let payload = serde_json::to_value(&request_body)?;
        let res = self.client.delete(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: BatchDeleteResponse = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(BatchError(format!(
                "status code {} received.",
                res.status()
            )))),
        }
    }

    pub async fn references_batch_add() -> Result<reqwest::Response, Box<dyn Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::WeaviateClient;

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
    async fn test_objects_batch_add_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_objects_batch_add_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_objects_batch_delete_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_objects_batch_delete_err() {
        let (mut mock_server, client) = get_test_harness();
    }
}
