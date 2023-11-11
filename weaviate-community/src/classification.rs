use reqwest::Url;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::collections::{
    classification::{ClassificationRequest, ClassificationResponse},
    error::ClassificationError,
};

#[derive(Debug)]
pub struct Classification {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Classification {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/classifications/")?;
        Ok(Classification { endpoint, client })
    }

    pub async fn schedule(
        &self,
        request: serde_json::Value,
        //request: ClassificationRequest,
    ) -> Result<(), Box<dyn Error>> {
        //) -> Result<serde_json::Value, Box<dyn Error>> {
        //) -> Result<ClassificationResponse, Box<dyn Error>> {
        //let payload = serde_json::to_value(&request)?;
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&request)
            .send()
            .await?;
        let res: serde_json::Value = res.json().await?;
        println!("{:#?}", res);
        Ok(())
        //match res.status() {
        //    reqwest::StatusCode::OK => {
        //        //let res: ClassificationResponse = res.json().await?;
        //        //let res: serde_json::Value = res.json().await?;
        //        //Ok(res)
        //        Ok(())
        //    }
        //    _ => Err(
        //        Box::new(
        //            ClassificationError(
        //                format!(
        //                    "status code {} received when calling schedule endpoint.",
        //                    res.status()
        //                )
        //            )
        //        )
        //    ),
        //}
    }

    pub async fn get(&self, id: Uuid) -> Result<ClassificationResponse, Box<dyn Error>> {
        let endpoint = self.endpoint.join(&id.to_string())?;
        let res = self.client.get(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: ClassificationResponse = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(ClassificationError(format!(
                "status code {} received when calling get classification endpoint.",
                res.status()
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        collections::{
            auth::AuthApiKey,
            classification::{ClassificationRequest, ClassificationType},
            objects::{MultiObjects, Object},
            schema::{Class, Properties, Property},
        },
        WeaviateClient,
    };

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn mock_post(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str,
    ) -> mockito::Mock {
        server
            .mock("POST", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
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
    async fn test_objects_batch_add_ok() {
        //let (mut mock_server, client) = get_test_harness();
        //let mock = mock_post(&mut mock_server, "/v1/batch/objects", 200, &res_str);
        //let res = client.batch.objects_batch_add(objects, None).await;
        //mock.assert();
        //assert!(res.is_ok());

        //let client = WeaviateClient::builder("http://localhost:8080").build().unwrap();

        // Create some objects
        //let article = Class::builder("Article", "an article")
        //    .with_properties(
        //        Properties::new(vec![
        //            Property::builder("title").build()
        //        ])
        //    )
        //    .build();

        //let client = WeaviateClient::builder("https://edu-demo.weaviate.network")
        //    .with_auth_secret(AuthApiKey::new("learn-weaviate"))
        //    .with_api_key(
        //        "X-OpenAI-Api-Key",
        //        "abcde"
        //    )
        //    .build()
        //    .unwrap();

        //let req = ClassificationRequest::builder()
        //    .with_type(ClassificationType::KNN)
        //    .with_class("Article")
        //    .with_based_on_properties(vec!["summary"])
        //    .with_classify_properties(vec!["hasPopularity"])
        //    .with_filters(serde_json::json!({
        //        "trainingSetWhere": {
        //            "path": ["wordCount"],
        //            "operator": "GreaterThan",
        //            "valueInt": 100
        //        }
        //    }))
        //    .with_settings(serde_json::json!({
        //        "k": 3
        //    }))
        //    .build();

        //let req2 = serde_json::json!({
        //    "class": "Article",
        //    "type": "knn",
        //    "settings": {
        //        "k": 3
        //    },
        //    "basedOnProperties": [
        //        "summary"
        //    ],
        //    "classifyProperties": [
        //        "hasPopularity"
        //    ],
        //    "filters": {
        //        "trainingSetWhere": {"path": ["wordCount"], "operator": "GreaterThan", "valueInt": 100}
        //    }
        //});

        //let res = client.classification.schedule(req).await;
        //let res = client.meta.get_meta().await;
        //let res = client.schema.get().await;
        //println!("{:#?}", res);
    }

    #[tokio::test]
    async fn test_objects_batch_add_err() {}
}
