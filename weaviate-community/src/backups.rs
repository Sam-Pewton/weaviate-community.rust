use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::backups::{BackupBackends, BackupCreateRequest, BackupRestoreRequest, BackupStatusResponse};
use crate::collections::error::BackupError;

pub struct Backups {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Backups {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/backups/")?;
        Ok(Backups { endpoint, client })
    }

    /// TODO wait for completion flag
    pub async fn create(
        &self,
        backend: BackupBackends,
        backup_request: BackupCreateRequest
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let endpoint = self.endpoint.join(backend.value())?;
        let payload = serde_json::to_value(&backup_request)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(res)
            }
            _ => {
                Err(Box::new(BackupError(format!(
                    "status code {} received.",
                res.status()
                ))))
            }
        }
    }

    pub async fn get_backup_status(
        &self,
        backend: BackupBackends,
        backup_id: &str,
    ) -> Result<BackupStatusResponse, Box<dyn Error>> {
        let mut endpoint: String = backend.value().into();
        endpoint.push_str("/");
        endpoint.push_str(&backup_id.to_string());
        let endpoint = self.endpoint.join(&endpoint)?;
        let res = self.client.get(endpoint).send().await?;
        let res: BackupStatusResponse = res.json().await?;
        Ok(res)
    }

    /// TODO wait for completion flag
    pub async fn restore(
        &self,
        backend: BackupBackends,
        backup_id: &str,
        backup_request: BackupRestoreRequest
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint: String = backend.value().into();
        endpoint.push_str("/");
        endpoint.push_str(&backup_id.to_string());
        endpoint.push_str("/restore");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&backup_request)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(res)
            }
            _ => {
                Err(Box::new(BackupError(format!(
                    "status code {} received.",
                    res.status()
                ))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{WeaviateClient, collections::{backups::{BackupBackends, BackupCreateRequest, BackupStatus}, objects::Object}};

    fn test_backup_req() -> BackupCreateRequest {
        BackupCreateRequest { id: "this-is-a-test4".into(), include: None, exclude: None }
    }

    fn test_object(class_name: &str) -> Object {
        Object {
            class: class_name.into(),
            properties: serde_json::json!({}),
            id: None,
            vector: None,
            tenant: None,
            creation_time_unix: None,
            last_update_time_unix: None,
            vector_weights: None,
        }
    }

    #[tokio::test]
    async fn test_create_backup() {
        let obj = test_object("BackupTest");
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let _ = client.objects.create(&obj, None).await;
        let b_req = test_backup_req();
        let res = client.backups.create(BackupBackends::FILESYSTEM, b_req).await;
        //println!("{:#?}", res.unwrap().json::<serde_json::Value>().await);

        loop {
            let res = client.backups.get_backup_status(
                BackupBackends::FILESYSTEM,
                "this-is-a-test4"
            ).await;
            let test = res.unwrap();
            if test.status == BackupStatus::SUCCESS {
                break;
            } else if test.status == BackupStatus::FAILED {
                break;
            }
        }
        println!("{:#?}", res);
    }
}
