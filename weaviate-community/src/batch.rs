use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::{
    batch::{BatchAddObject, BatchDeleteRequest, BatchDeleteResponse},
    error::BatchError,
    objects::{ConsistencyLevel, Objects},
};

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
    use crate::{
        collections::{
            batch::{BatchDeleteRequest, MatchConfig},
            objects::{Object, Objects},
        },
        WeaviateClient,
    };
    use uuid::Uuid;

    fn test_objects(class_name: &str, uuid_one: &Uuid, uuid_two: &Uuid) -> Objects {
        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        let properties2 = serde_json::json!({
            "name": "test2",
            "number": 456,
        });
        Objects {
            objects: vec![
                Object {
                    class: class_name.into(),
                    properties,
                    id: Some(*uuid_one),
                    vector: None,
                    tenant: None,
                    creation_time_unix: None,
                    last_update_time_unix: None,
                    vector_weights: None,
                },
                Object {
                    class: class_name.into(),
                    properties: properties2,
                    id: Some(*uuid_two),
                    vector: None,
                    tenant: None,
                    creation_time_unix: None,
                    last_update_time_unix: None,
                    vector_weights: None,
                },
            ],
        }
    }

    fn test_delete_objects(class_name: &str) -> BatchDeleteRequest {
        // this will eventually be defined with the graphql stuff later on
        let map = serde_json::json!({
            "operator": "NotEqual",
            "path": ["name"],
            "valueText": "aaa"
        });
        BatchDeleteRequest {
            matches: MatchConfig {
                class: class_name.into(),
                match_where: map,
            },
            dry_run: None,
            output: None,
        }
    }

    #[tokio::test]
    async fn test_objects_batch_add_and_delete() {
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let uuid_one = Uuid::new_v4();
        let uuid_two = Uuid::new_v4();
        let objects = test_objects("TestObjectsBatchAdd", &uuid_one, &uuid_two);
        let res = client.batch.objects_batch_add(objects, None).await.unwrap();
        assert_eq!(&2, &res.len());

        let delete = test_delete_objects("TestObjectsBatchAdd");
        let res = client
            .batch
            .objects_batch_delete(delete, None)
            .await
            .unwrap();
        assert_eq!(&2, &res.results.successful);
    }
}
