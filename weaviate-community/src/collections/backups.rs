/// All backup associated type components
use serde::{Deserialize, Serialize};

/// Strict definitions of the different backends available for backups.
///
/// Weaviate supports S3, GCS, AZURE, and FILESYSTEM shard status.
#[derive(Serialize, Deserialize, Debug)]
pub enum BackupBackends {
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "gcs")]
    GCS,
    #[serde(rename = "azure")]
    AZURE,
    #[serde(rename = "filesystem")]
    FILESYSTEM,
}

impl BackupBackends {
    /// Retrieve the string value associated to the BackupBackends enum types.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupBackends;
    ///
    /// let s3 = BackupBackends::S3.value();
    /// ```
    pub fn value(&self) -> &str {
        match self {
            BackupBackends::S3 => "s3",
            BackupBackends::GCS => "gcs",
            BackupBackends::AZURE => "azure",
            BackupBackends::FILESYSTEM => "filesystem",
        }
    }
}

/// BackupCreateRequest struct defining the options for the json payload required to create a new
/// backup.
#[derive(Serialize, Deserialize, Debug)]
pub struct BackupCreateRequest {
    pub id: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl BackupCreateRequest {
    /// Create a new builder for the BackupCreateRequest object.
    ///
    /// This is the same as `BackupCreateRequestBuilder::new()`.
    ///
    /// # Parameters
    /// - id: the id of the BackupCreateRequest
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupCreateRequest;
    ///
    /// let builder = BackupCreateRequest::builder("my-backup");
    /// ```
    pub fn builder(id: &str) -> BackupCreateRequestBuilder {
        BackupCreateRequestBuilder::new(id)
    }
}

/// BackupCreateRequestBuilder for building new BackupCreateRequests
pub struct BackupCreateRequestBuilder {
    pub id: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl BackupCreateRequestBuilder {
    /// Create a new builder for the BackupCreateRequest object.
    ///
    /// This is the same as `BackupCreateRequest::builder()`.
    ///
    /// # Parameters
    /// - id: the id of the BackupCreateRequest
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupCreateRequestBuilder;
    ///
    /// let builder = BackupCreateRequestBuilder::new("my-backup");
    /// ```
    pub fn new(id: &str) -> BackupCreateRequestBuilder {
        BackupCreateRequestBuilder { 
            id: id.into(),
            include: None,
            exclude: None
        }
    }

    /// Add a value to the optional `include` value of the BackupCreateRequest.
    ///
    /// # Parameters
    /// - include: the item to include in the backup
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupCreateRequestBuilder;
    ///
    /// let builder = BackupCreateRequestBuilder::new("my-backup").with_include(vec!["Article"]);
    /// ```
    pub fn with_include(mut self, include: Vec<&str>) -> BackupCreateRequestBuilder {
        let include = include.iter().map(|field| field.to_string()).collect();
        self.include = Some(include);
        self
    }

    /// Add a value to the optional `exclude` value of the BackupCreateRequest.
    ///
    /// # Parameters
    /// - exclude: the item to exclude in the backup
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupCreateRequestBuilder;
    ///
    /// let builder = BackupCreateRequestBuilder::new("my-backup").with_exclude(vec!["Article"]);
    /// ```
    pub fn with_exclude(mut self, exclude: Vec<&str>) -> BackupCreateRequestBuilder {
        let exclude = exclude.iter().map(|field| field.to_string()).collect();
        self.exclude = Some(exclude);
        self
    }

    /// Build the BackupCreateRequest from the BackupCreateRequestBuilder
    ///
    /// # Example
    /// Using BackupCreateRequestBuilder
    /// ```rust
    /// use weaviate_community::collections::backups::BackupCreateRequestBuilder;
    ///
    /// let builder = BackupCreateRequestBuilder::new("new-id").build();
    /// ```
    ///
    /// Using BackupCreateRequest
    /// ```rust
    /// use weaviate_community::collections::backups::BackupCreateRequest;
    ///
    /// let builder = BackupCreateRequest::builder("new-id").build();
    /// ```
    pub fn build(self) -> BackupCreateRequest {
        BackupCreateRequest {
            id: self.id,
            include: self.include,
            exclude: self.exclude,
        }
    }
}

/// BackupRestoreRequest struct defining the options for the json payload required to restore a
/// backup.
#[derive(Serialize, Deserialize, Debug)]
pub struct BackupRestoreRequest {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl BackupRestoreRequest {
    /// Create a new builder for the BackupRestoreRequest object.
    ///
    /// This is the same as `BackupRestoreRequestBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupRestoreRequest;
    ///
    /// let builder = BackupRestoreRequest::builder();
    /// ```
    pub fn builder() -> BackupRestoreRequestBuilder {
        BackupRestoreRequestBuilder::default()
    }
}

/// BackupRestoreRequestBuilder for building new BackupRestoreRequests
#[derive(Default)]
pub struct BackupRestoreRequestBuilder {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

impl BackupRestoreRequestBuilder {
    /// Create a new builder for the BackupRestoreRequest object.
    ///
    /// This is the same as `BackupRestoreRequest::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupRestoreRequestBuilder;
    ///
    /// let builder = BackupRestoreRequestBuilder::new();
    /// ```
    pub fn new() -> BackupRestoreRequestBuilder {
        BackupRestoreRequestBuilder { include: None, exclude: None }
    }
     
    /// Add a value to the optional `include` value of the BackupCreateRequest.
    ///
    /// # Parameters
    /// - include: the item to include in the backup
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupRestoreRequestBuilder;
    ///
    /// let builder = BackupRestoreRequestBuilder::new().with_include(vec!["Article"]);
    /// ```
    pub fn with_include(mut self, include: Vec<&str>) -> BackupRestoreRequestBuilder {
        let include = include.iter().map(|field| field.to_string()).collect();
        self.include = Some(include);
        self
    }

    /// Add a value to the optional `exclude` value of the BackupRestoreRequest.
    ///
    /// # Parameters
    /// - exclude: the item to exclude from the backup
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::backups::BackupRestoreRequestBuilder;
    ///
    /// let builder = BackupRestoreRequestBuilder::new().with_exclude(vec!["Article"]);
    /// ```
    pub fn with_exclude(mut self, exclude: Vec<&str>) -> BackupRestoreRequestBuilder {
        let exclude = exclude.iter().map(|field| field.to_string()).collect();
        self.exclude = Some(exclude);
        self
    }

    /// Build the BackupRestoreRequest from the BackupRestoreRequestBuilder
    ///
    /// # Example
    /// Using BackupRestoreRequestBuilder
    /// ```rust
    /// use weaviate_community::collections::backups::BackupRestoreRequestBuilder;
    ///
    /// let builder = BackupRestoreRequestBuilder::new().build();
    /// ```
    ///
    /// Using BackupRestoreRequest
    /// ```rust
    /// use weaviate_community::collections::backups::BackupRestoreRequest;
    ///
    /// let builder = BackupRestoreRequest::builder().build();
    /// ```
    pub fn build(self) -> BackupRestoreRequest {
        BackupRestoreRequest {
            include: self.include,
            exclude: self.exclude,
        }
    }
}

/// Strict definitions of the different backup status' available for backups.
///
/// Weaviate supports STARTED, SUCCESS, FAILED, TRANSFERRING, and TRANSFERRED.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum BackupStatus {
    STARTED,
    SUCCESS,
    FAILED,
    TRANSFERRING,
    TRANSFERRED,
}

/// The general status response for backup status.
///
/// You shouldn't need to ever create this struct - it is just what the response from the backup
/// endpoints is deserialized into.
#[derive(Serialize, Deserialize, Debug)]
pub struct BackupStatusResponse {
    pub backend: String,
    pub id: String,
    pub path: Option<String>,
    pub status: BackupStatus,
}


/// The general backup response.
///
/// You shouldn't need to ever create this struct - it is just what the response from the backup
/// endpoints is deserialized into.
#[derive(Serialize, Deserialize, Debug)]
pub struct BackupResponse {
    pub backend: BackupBackends,
    pub classes: Vec<String>,
    pub id: String,
    pub path: String,
    pub status: BackupStatus,
}
