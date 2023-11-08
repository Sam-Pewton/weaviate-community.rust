use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::{
    batch::{BatchAddObjects, BatchDeleteRequest, BatchDeleteResponse, BatchAddReferencesResponse},
    error::BatchError,
    objects::{ConsistencyLevel, MultiObjects, References},
};

/// All batch related endpoints and functionality described in
/// [Weaviate meta API documentation](https://weaviate.io/developers/weaviate/api/rest/batch)
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

    /// Batch add objects.
    ///
    /// # Parameters
    /// - objects: the objects to add
    /// - consistency_level: the consistency level to use
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::objects::{Object, MultiObjects, ConsistencyLevel};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None)?;
    ///
    ///     let author_uuid = Uuid::parse_str("36ddd591-2dee-4e7e-a3cc-eb86d30a4303").unwrap();
    ///     let article_a_uuid = Uuid::parse_str("6bb06a43-e7f0-393e-9ecf-3c0f4e129064").unwrap();
    ///     let article_b_uuid = Uuid::parse_str("b72912b9-e5d7-304e-a654-66dc63c55b32").unwrap();
    ///
    ///     let article_a = Object::builder("Article", serde_json::json!({}))
    ///         .with_id(article_a_uuid.clone())
    ///         .build();
    ///
    ///     let article_b = Object::builder("Article", serde_json::json!({}))
    ///         .with_id(article_b_uuid.clone())
    ///         .build();
    ///
    ///     let author = Object::builder("Author", serde_json::json!({}))
    ///         .with_id(author_uuid.clone())
    ///         .build();
    ///
    ///     let res = client.batch.objects_batch_add(
    ///         MultiObjects::new(vec![article_a, article_b, author]), Some(ConsistencyLevel::ALL)
    ///     ).await;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Batch delete objects.
    ///
    /// # Parameters
    /// - request_body: the config to use for deletion
    /// - consistency_level: the consistency level to use
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::objects::{Object, MultiObjects, ConsistencyLevel};
    /// use weaviate_community::collections::batch::{BatchDeleteRequest, MatchConfig};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let req = BatchDeleteRequest::builder(
    ///         MatchConfig::new(
    ///             "Article",
    ///             serde_json::json!({
    ///                 "operator": "Like",
    ///                 "path": ["id"],
    ///                 "valueText": "*4*",
    ///             })
    ///         )
    ///     ).build();
    ///
    ///     let res = client.batch.objects_batch_delete(req, Some(ConsistencyLevel::ALL)).await;
    ///
    ///     Ok(())
    /// }
    /// ```
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

    /// Batch add references.
    ///
    /// Note that the consistency_level and tenant_name in the `Reference` items contained within
    /// the `References` input bare no effect on this method and will be ignored.
    ///
    /// # Parameters
    /// - references: the references to add
    /// - consistency_level: the consistency level to use
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::objects::{Reference, References, ConsistencyLevel};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///
    ///     let author_uuid = Uuid::parse_str("36ddd591-2dee-4e7e-a3cc-eb86d30a4303").unwrap();
    ///     let article_a_uuid = Uuid::parse_str("6bb06a43-e7f0-393e-9ecf-3c0f4e129064").unwrap();
    ///     let article_b_uuid = Uuid::parse_str("b72912b9-e5d7-304e-a654-66dc63c55b32").unwrap();
    ///
    ///     let references = References::new(vec![
    ///         Reference::new(
    ///             "Author",
    ///             &author_uuid,
    ///             "wroteArticles",
    ///             "Article",
    ///             &article_a_uuid,
    ///         ),
    ///         Reference::new(
    ///             "Author",
    ///             &author_uuid,
    ///             "wroteArticles",
    ///             "Article",
    ///             &article_b_uuid,
    ///         ),
    ///     ]);
    ///
    ///     let res = client.batch.references_batch_add(
    ///         references,
    ///         Some(ConsistencyLevel::ALL)
    ///     ).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn references_batch_add(
        &self,
        references: References,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<BatchAddReferencesResponse, Box<dyn Error>> {
        let mut converted: Vec<serde_json::Value> = Vec::new();
        for reference in references.0 {
            let new_ref = serde_json::json!({
                "from": format!(
                    "weaviate://localhost/{}/{}/{}",
                    reference.from_class_name,
                    reference.from_uuid,
                    reference.from_property_name
                ),
                "to": format!(
                    "weaviate://localhost/{}/{}",
                    reference.to_class_name,
                    reference.to_uuid
                ),
            });
            converted.push(new_ref);
        }
        let payload = serde_json::json!(converted);
        
        let mut endpoint = self.endpoint.join("references")?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }

        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                println!("{:#?}", res);
                let res: BatchAddReferencesResponse = res.json().await?;
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
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        WeaviateClient,
        collections::{batch::{
            BatchDeleteRequest,
            MatchConfig,
            BatchAddObject,
            BatchDeleteResponse,
            BatchDeleteResult,
            ResultStatus,
            GeneralStatus,
        }, objects::{Reference, References}},
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

    fn test_references() -> References {
        let uuid = Uuid::parse_str("36ddd591-2dee-4e7e-a3cc-eb86d30a4303").unwrap();
        let uuid2 = Uuid::parse_str("6bb06a43-e7f0-393e-9ecf-3c0f4e129064").unwrap();
        let uuid3 = Uuid::parse_str("b72912b9-e5d7-304e-a654-66dc63c55b32").unwrap();
        References::new(vec![
            Reference::new(
                "Test",
                &uuid,
                "testProp",
                "Other",
                &uuid2,
            ),
            Reference::new(
                "Test",
                &uuid,
                "testProp",
                "Other",
                &uuid3,
            ),
        ])
    }

    fn test_add_references_response() -> String {
        serde_json::to_string(&serde_json::json!([{
            "result": {
                "errors": {
                    "error": [
                        {
                            "message": "test"
                        }
                    ]
                },
                "status": "FAILED"
            }
        }])).unwrap()
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
        let refs = test_references();
        let res_str = test_add_references_response();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_post(&mut mock_server, "/v1/batch/references", 200, &res_str);
        let res = client.batch.references_batch_add(refs, None).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_references_batch_add_err() {
        let refs = test_references();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_post(&mut mock_server, "/v1/batch/references", 500, "");
        let res = client.batch.references_batch_add(refs, None).await;
        mock.assert();
        assert!(res.is_err());
    }
}
