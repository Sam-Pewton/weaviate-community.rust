use reqwest::Url;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

use crate::collections::{
    classification::{ClassificationRequest, ClassificationResponse},
    error::ClassificationError
};

#[derive(Debug)]
pub struct Classification {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Classification {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/classifications")?;
        Ok(Classification { endpoint, client })
    }

    pub async fn schedule(
        &self,
        request: ClassificationRequest,
    ) -> Result<ClassificationResponse, Box<dyn Error>> {
        let payload = serde_json::to_value(&request)?;
        let res = self.client.post(self.endpoint.clone()).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: ClassificationResponse = res.json().await?;
                Ok(res)
            }
            _ => Err(
                Box::new(
                    ClassificationError(
                        format!(
                            "status code {} received when calling schedule endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    pub async fn get(
        &self,
        id: Uuid,
    ) -> Result<ClassificationResponse, Box<dyn Error>> {
        let endpoint = self.endpoint.join(&id.to_string())?;
        let res = self.client.get(self.endpoint.clone()).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: ClassificationResponse = res.json().await?;
                Ok(res)
            }
            _ => Err(
                Box::new(
                    ClassificationError(
                        format!(
                            "status code {} received when calling schedule endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }
}
