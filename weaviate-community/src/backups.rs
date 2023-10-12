use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::backups::{
    BackupBackends,
    BackupCreateRequest,
    BackupRestoreRequest,
    BackupStatusResponse,
    BackupStatus, 
    BackupResponse,
};
use crate::collections::error::BackupError;

/// All backup related endpoints and functionality described in
/// [Weaviate meta API documentation](https://weaviate.io/developers/weaviate/api/rest/backups)
pub struct Backups {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Backups {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/backups/")?;
        Ok(Backups { endpoint, client })
    }

    /// Create a new backup
    ///
    /// # Examples
    /// Creating a backup to the filesystem, waiting for completion
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::backups::{BackupBackends, BackupCreateRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080")?;
    ///     let my_request = BackupCreateRequest { 
    ///         id: "doc-test-backup".into(),
    ///         include: None, 
    ///         exclude: None
    ///     };
    ///     let res = client.backups.create(
    ///         &BackupBackends::FILESYSTEM,
    ///         &my_request,
    ///         true
    ///     ).await?;
    ///     println!("{:#?}", res);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        backend: &BackupBackends,
        backup_request: &BackupCreateRequest,
        wait_for_completion: bool,
    ) -> Result<BackupResponse, Box<dyn Error>> {
        let endpoint = self.endpoint.join(backend.value())?;
        let payload = serde_json::to_value(&backup_request)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let mut res: BackupResponse = res.json().await?;
                if wait_for_completion {
                    let complete = self.wait_for_completion(&backend, &backup_request.id).await?;
                    res.status = complete;
                }
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

    /// Get the status of a backup
    ///
    /// # Examples
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::backups::BackupBackends;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080")?;
    ///     let res = client.backups.get_backup_status(
    ///         &BackupBackends::FILESYSTEM,
    ///         "doc-test-backup",
    ///     ).await?;
    ///     println!("{:#?}", res);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_backup_status(
        &self,
        backend: &BackupBackends,
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


    /// Restore a backup
    ///
    /// # Examples
    /// Restore a backup from the filesystem, waiting for completion
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::backups::{BackupBackends, BackupRestoreRequest};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080")?;
    ///     let my_request = BackupRestoreRequest { 
    ///         include: None, 
    ///         exclude: None
    ///     };
    ///     let res = client.backups.restore(
    ///         &BackupBackends::FILESYSTEM,
    ///         "doc-test-backup",
    ///         &my_request,
    ///         true
    ///     ).await?;
    ///     println!("{:#?}", res);
    ///     Ok(())
    /// }
    /// ```
    pub async fn restore(
        &self,
        backend: &BackupBackends,
        backup_id: &str,
        backup_request: &BackupRestoreRequest,
        wait_for_completion: bool,
    ) -> Result<BackupResponse, Box<dyn Error>> {
        let mut endpoint: String = backend.value().into();
        endpoint.push_str("/");
        endpoint.push_str(&backup_id.to_string());
        endpoint.push_str("/restore");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&backup_request)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let mut res: BackupResponse = res.json().await?;
                if wait_for_completion {
                    let complete = self.wait_for_completion(&backend, &backup_id).await?;
                    res.status = complete;
                }
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

    /// Wait for a backup to complete before returning
    async fn wait_for_completion(
        &self, 
        backend: &BackupBackends, 
        backup_id: &str
    ) -> Result<BackupStatus, Box<dyn Error>> {
        loop {
            let res = self.get_backup_status(backend, backup_id).await;
            let status = res?;
            if status.status == BackupStatus::SUCCESS {
                break;
            } else if status.status == BackupStatus::FAILED {
                return Err(
                    Box::new(
                        BackupError(
                            format!(
                                "backup status FAILED",
                            )
                        )
                    )
                )
            }
        }
        Ok(BackupStatus::SUCCESS)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        WeaviateClient,
        collections::{
            backups::{
                BackupBackends, 
                BackupCreateRequest, 
                BackupRestoreRequest
            }, 
            objects::Object
        }
    };

    fn test_backup_create_req() -> BackupCreateRequest {
        BackupCreateRequest { id: "this-is-a-test1".into(), include: None, exclude: None }
    }

    fn test_backup_restore_req() -> BackupRestoreRequest {
        BackupRestoreRequest { include: None, exclude: None }
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

    // commented out to avoid breaking other tests when restore is executing. Will use in SI test
    #[tokio::test]
    async fn test_create_backup() {
        let obj = test_object("BackupTest");
        let client = WeaviateClient::new("http://localhost:8080").unwrap();
        let _ = client.objects.create(&obj, None).await;

        // create
        //let c_req = test_backup_create_req();
        //let res = client.backups.create(&BackupBackends::FILESYSTEM, &c_req, true).await;
        //println!("{:#?}", res.unwrap());

        // restore
        //let r_req = test_backup_restore_req();
        //let res = client.backups.restore(&BackupBackends::FILESYSTEM, &c_req.id, &r_req, true).await;
        //println!("{:#?}", res);
    }
}
