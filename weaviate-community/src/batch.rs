use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::{
    batch::{BatchAddObjects, BatchDeleteRequest, BatchDeleteResponse},
    error::BatchError,
    objects::{ConsistencyLevel, MultiObjects},
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
        objects: MultiObjects,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<BatchAddObjects, Box<dyn Error>> {
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
                let res: BatchAddObjects = res.json().await?;
                Ok(res)
            }
            _ => Err(
                Box::new(
                    BatchError(
                        format!(
                            "status code {} received.",
                            res.status()
                        )
                    )
                )
            ),
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
            _ => Err(
                Box::new(
                    BatchError(
                        format!(
                            "status code {} received.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    pub async fn references_batch_add(
        references: References
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::json!({
            "beacon": format!("weaviate://localhost/{}/{}", reference.to_class_name, reference.to_uuid),
        });
        let mut endpoint: String = reference.from_class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&reference.from_uuid.to_string());
        endpoint.push_str("/references/");
        endpoint.push_str(&reference.from_property_name.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = reference.consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = reference.tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", &t);
        }

        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(true)
            }
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling create object reference endpoint.",
                        res.status()
                    ))
                )
            ),
        }
        todo!()
    }

    pub async fn references_batch_delete() -> Result<reqwest::Response, Box<dyn Error>> {
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
            MultiObjects,
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

    fn test_create_objects() -> MultiObjects {
        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        MultiObjects {
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
        BatchDeleteRequest::builder(MatchConfig::new("Test", map)).build()
    }

    fn test_delete_response() -> BatchDeleteResponse {
        let map = serde_json::json!({
            "operator": "NotEqual",
            "path": ["name"],
            "valueText": "aaa"
        });
        BatchDeleteResponse {
            matches: MatchConfig::new("Test", map),
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

    #[tokio::test]
    async fn test_references_batch_add_ok() {
    }

    #[tokio::test]
    async fn test_references_batch_add_err() {
    }

    #[tokio::test]
    async fn test_references_batch_delete_ok() {
    }

    #[tokio::test]
    async fn test_references_batch_delete_err() {
    }
}
