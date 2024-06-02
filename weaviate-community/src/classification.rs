use reqwest::Url;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::collections::{
    classification::{ClassificationRequest, ClassificationResponse},
    error::ClassificationError,
};

/// All classification related endpoints and functionality described in
/// [Weaviate meta API documentation](https://weaviate.io/developers/weaviate/api/rest/classification)
#[derive(Debug)]
pub struct Classification {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Classification {
    /// Create a new instance of the Classification endpoint struct. Should only be done by the 
    /// parent client.
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/classifications/")?;
        Ok(Classification { endpoint, client })
    }

    /// Schedule a new classification
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::classification::{
    ///     ClassificationRequest,
    ///     ClassificationType
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///
    ///     let req = ClassificationRequest::builder()
    ///         .with_type(ClassificationType::KNN)
    ///         .with_class("Article")
    ///         .with_based_on_properties(vec!["summary"])
    ///         .with_classify_properties(vec!["hasPopularity"])
    ///         .with_filters(serde_json::json!({
    ///             "trainingSetWhere": {
    ///                 "path": ["wordCount"],
    ///                 "operator": "GreaterThan",
    ///                 "valueInt": 100
    ///             }
    ///         }))
    ///         .with_settings(serde_json::json!({
    ///             "k": 3
    ///         }))
    ///         .build();
    ///
    ///     let res = client.classification.schedule(req).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn schedule(
        &self,
        request: ClassificationRequest,
    ) -> Result<ClassificationResponse, Box<dyn Error>> {
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&request)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::CREATED => {
                let res: ClassificationResponse = res.json().await?;
                Ok(res)
            }
            _ => Err(self.get_err_msg("schedule classification", res).await)
        }
    }

    /// Get the status of a classification
    ///
    /// # Example
    /// ```no_run
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let uuid = Uuid::parse_str("00037775-1432-35e5-bc59-443baaef7d80")?;
    ///     let client = WeaviateClient::builder("http://localhost:8080").build()?;
    ///
    ///     let res = client.classification.get(uuid).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self, id: Uuid) -> Result<ClassificationResponse, Box<dyn Error>> {
        let endpoint = self.endpoint.join(&id.to_string())?;
        let res = self.client.get(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: ClassificationResponse = res.json().await?;
                Ok(res)
            }
            _ => Err(self.get_err_msg("get classification", res).await)
        }
    }

    /// Get the error message for the endpoint
    ///
    /// Made to reduce the boilerplate error message building
    async fn get_err_msg(
        &self,
        endpoint: &str,
        res: reqwest::Response
    ) -> Box<ClassificationError> {
        let status_code = res.status();
        let msg: Result<serde_json::Value, reqwest::Error> = res.json().await;
        let r_str: String;
        if let Ok(json) = msg {
            r_str = format!(
                "Status code `{}` received when calling {} endpoint. Response: {}",
                status_code,
                endpoint,
                json,
            );
        } else {
            r_str = format!(
                "Status code `{}` received when calling {} endpoint.",
                status_code,
                endpoint
            );
        }
        Box::new(ClassificationError(r_str))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::{
        WeaviateClient,
        collections::classification::{ClassificationRequest, ClassificationType}
    };

    async fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new_async().await;
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn test_classification_req() -> ClassificationRequest {
        ClassificationRequest::builder()
            .with_class("Test")
            .with_type(ClassificationType::KNN)
            .with_based_on_properties(vec!["testProp"])
            .with_classify_properties(vec!["hasPopularity"])
            .with_filters(serde_json::json!({
                "path": ["testPropTwo"],
                "operator": "GreaterThan",
                "valueInt": 100
            }))
            .with_settings(serde_json::json!({"k": 3}))
            .build()
    }

    async fn mock_post(
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

    async fn mock_get(
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
    async fn test_classification_schedule_ok() {}

    #[tokio::test]
    async fn test_classification_schedule_err() {
        let req = test_classification_req();
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_post(&mut mock_server, "/v1/classifications/", 401, "").await;
        let res = client.classification.schedule(req).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_classification_get_ok() {}

    #[tokio::test]
    async fn test_classification_get_err() {
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/classifications/");
        url.push_str(&uuid.to_string());
        let (mut mock_server, client) = get_test_harness().await;
        let mock = mock_get(&mut mock_server, &url, 401, "").await;
        let res = client.classification.get(uuid).await;
        mock.assert();
        assert!(res.is_err());
    }
}
