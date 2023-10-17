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
    use uuid::Uuid;

    use crate::{
        WeaviateClient,
        collections::batch::{
            BatchDeleteRequest,
            MatchConfig,
            BatchAddObject,
            BatchDeleteResponse,
            BatchDeleteResult,
            ResultStatus,
            GeneralStatus
        },
        collections::objects::{
            Objects,
            Object,
        }
    };

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn test_create_objects() -> Objects {
        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        Objects {
            objects: vec![
                Object {
                    class: "Test".into(),
                    properties,
                    id: Some(Uuid::new_v4()),
                    vector: None,
                    tenant: None,
                    creation_time_unix: None,
                    last_update_time_unix: None,
                    vector_weights: None,
                },
            ],
        }
    }

    fn test_batch_add_object_response() -> String {
        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        serde_json::to_string(&vec![BatchAddObject {
            class: "Test".into(),
            properties,
            id: None,
            vector: None,
            tenant: None,
            creation_time_unix: None,
            last_update_time_unix: None,
            vector_weights: None,
            result: ResultStatus { status: GeneralStatus::SUCCESS },
        }]).unwrap()
    }

    fn test_delete_objects() -> BatchDeleteRequest {
        // this will eventually be defined with the graphql stuff later on
        let map = serde_json::json!({
            "operator": "NotEqual",
            "path": ["name"],
            "valueText": "aaa"
        });
        BatchDeleteRequest {
            matches: MatchConfig {
                class: "Test".into(),
                match_where: map,
            },
            dry_run: None,
            output: None,
        }
    }

    fn test_delete_response() -> BatchDeleteResponse {
        let map = serde_json::json!({
            "operator": "NotEqual",
            "path": ["name"],
            "valueText": "aaa"
        });
        BatchDeleteResponse {
            matches: MatchConfig {
                class: "Test".into(),
                match_where: map
            },
            output: None,
            dry_run: None,
            results: BatchDeleteResult {
                matches: 0,
                limit: 1,
                successful: 1,
                failed: 0,
                objects: None,
            }
        }
    }

    fn mock_post(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("POST", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    fn mock_delete(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str,
    ) -> mockito::Mock {
        server.mock("DELETE", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    #[tokio::test]
    async fn test_objects_batch_add_ok() {
        let objects = test_create_objects();
        let res_str = test_batch_add_object_response();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_post(&mut mock_server, "/v1/batch/objects", 200, &res_str);
        let res = client.batch.objects_batch_add(objects, None).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_objects_batch_add_err() {
        let objects = test_create_objects();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_post(&mut mock_server, "/v1/batch/objects", 404, "");
        let res = client.batch.objects_batch_add(objects, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_objects_batch_delete_ok() {
        let req = test_delete_objects();
        let out = test_delete_response();
        let res_str = serde_json::to_string(&out).unwrap();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_delete(&mut mock_server, "/v1/batch/objects", 200, &res_str);
        let res = client.batch.objects_batch_delete(req, None).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_objects_batch_delete_err() {
        let req = test_delete_objects();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_delete(&mut mock_server, "/v1/batch/objects", 401, "");
        let res = client.batch.objects_batch_delete(req, None).await;
        mock.assert();
        assert!(res.is_err());
    }
}
