use serde::{Deserialize, Serialize};

pub enum BackupBackends {
    S3,
    GCS,
    AZURE,
    FILESYSTEM,
}

impl BackupBackends {
    pub fn value(&self) -> &str {
        match self {
            BackupBackends::S3 => "s3",
            BackupBackends::GCS => "gcs",
            BackupBackends::AZURE => "azure",
            BackupBackends::FILESYSTEM => "filesystem",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupCreateRequest {
    pub id: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupRestoreRequest {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BackupStatus {
    STARTED,
    SUCCESS,
    FAILED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupStatusResponse {
    pub backend: String,
    pub id: String,
    pub path: Option<String>,
    pub status: BackupStatus,
}
