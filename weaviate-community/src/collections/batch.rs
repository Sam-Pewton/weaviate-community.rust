use crate::collections::objects::Object;
/// All batch associated type components
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A new BatchDeleteRequest used to make batch delete requests
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteRequest {
    #[serde(rename = "match")]
    pub matches: MatchConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub output: Option<Verbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dry_run: Option<bool>,
}

impl BatchDeleteRequest {
    /// Create a new builder for the BatchDeleteRequest object.
    ///
    /// This is the same as `BatchDeleteRequestBuilder::new()`.
    ///
    /// # Parameters
    /// - matches: the match config of the BatchDeleteRequest
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::batch::{BatchDeleteRequest, MatchConfig};
    ///
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    ///
    /// let builder = BatchDeleteRequest::builder(match_config);
    /// ```
    pub fn builder(matches: MatchConfig) -> BatchDeleteRequestBuilder {
        BatchDeleteRequestBuilder::new(matches)
    }
}

/// BatchDeleteRequestBuilder for building new BatchDeleteRequests
pub struct BatchDeleteRequestBuilder {
    pub matches: MatchConfig,
    pub output: Option<Verbosity>,
    pub dry_run: Option<bool>,
}

impl BatchDeleteRequestBuilder {
    /// Create a new builder for the BatchDeleteRequest object.
    ///
    /// This is the same as `BatchDeleteRequest::builder()`.
    ///
    /// # Parameters
    /// - matches: the match config of the BatchDeleteRequest
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::batch::{BatchDeleteRequestBuilder, MatchConfig};
    ///
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    ///
    /// let builder = BatchDeleteRequestBuilder::new(match_config);
    /// ```
    pub fn new(matches: MatchConfig) -> BatchDeleteRequestBuilder {
        BatchDeleteRequestBuilder {
            matches,
            output: None,
            dry_run: None,
        }
    }

    /// Add a value to the optional `output` value of the BatchDeleteRequest.
    ///
    /// # Parameters
    /// - output: the verbosity level of the batch request
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::batch::{
    ///     BatchDeleteRequestBuilder,
    ///     MatchConfig,
    ///     Verbosity
    /// };
    ///
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    ///
    /// let builder = BatchDeleteRequestBuilder::new(match_config)
    ///     .with_output(Verbosity::VERBOSE);
    /// ```
    pub fn with_output(mut self, output: Verbosity) -> BatchDeleteRequestBuilder {
        self.output = Some(output);
        self
    }

    /// Add a value to the optional `dry_run` value of the BatchDeleteRequest.
    ///
    /// # Parameters
    /// - dry_run: the dry_run flag to set in the batch request
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::batch::{
    ///     BatchDeleteRequestBuilder,
    ///     MatchConfig,
    /// };
    ///
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    ///
    /// let builder = BatchDeleteRequestBuilder::new(match_config)
    ///     .with_dry_run(true);
    /// ```
    pub fn with_dry_run(mut self, dry_run: bool) -> BatchDeleteRequestBuilder {
        self.dry_run = Some(dry_run);
        self
    }

    /// Build the BatchDeleteRequest from the BatchDeleteRequestBuilder
    ///
    /// # Example
    /// Using BatchDeleteRequestBuilder
    /// ```rust
    /// use weaviate_community::collections::batch::{BatchDeleteRequestBuilder, MatchConfig};
    ///
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    ///
    /// let builder = BatchDeleteRequestBuilder::new(match_config).build();
    /// ```
    ///
    /// Using BatchDeleteRequest
    /// ```rust
    /// use weaviate_community::collections::batch::{BatchDeleteRequest, MatchConfig};
    ///
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    ///
    /// let builder = BatchDeleteRequest::builder(match_config).build();
    /// ```
    pub fn build(self) -> BatchDeleteRequest {
        BatchDeleteRequest {
            matches: self.matches,
            output: self.output,
            dry_run: self.dry_run,
        }
    }
}

/// MatchConfig object outlining how to find the objects to be deleted.
///
/// Used explicitly in batch deletes.
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchConfig {
    pub class: String,
    #[serde(rename = "where")]
    pub match_where: serde_json::Value,
}

impl MatchConfig {
    /// Create a new MatchConfig
    ///
    /// To revisit to strict type the map
    ///
    /// ```rust
    /// use weaviate_community::collections::batch::MatchConfig;
    /// let map = serde_json::json!({
    ///     "operator": "NotEqual",
    ///     "path": ["name"],
    ///     "valueText": "aaa"
    /// });
    /// let match_config = MatchConfig::new("Article", map);
    /// ```
    pub fn new(class: &str, match_where: serde_json::Value) -> MatchConfig {
        MatchConfig {
            class: class.into(),
            match_where,
        }
    }
}

/// Strict definitions of the different verbosity levels available.
///
/// Weaviate supports MINIMAL and VERBOSE.
#[derive(Serialize, Deserialize, Debug)]
pub enum Verbosity {
    #[serde(rename = "minimal")]
    MINIMAL,
    #[serde(rename = "verbose")]
    VERBOSE,
}

/// The BatchDeleteResponse object resulting from a BatchDeleteRequest
///
/// You shouldn't need to create this yourself unless for asserting against.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteResponse {
    #[serde(rename = "match")]
    pub matches: MatchConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub output: Option<Verbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dry_run: Option<bool>,
    pub results: BatchDeleteResult,
}

/// The BatchDeleteResult of a BatchDeleteResponse
///
/// You shouldn't need to create this yourself unless for asserting against.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchDeleteResult {
    pub matches: u64,
    pub limit: u64,
    pub successful: u64,
    pub failed: u64,
    #[serde(default)]
    pub objects: Option<DeleteObjects>,
}

/// Container for multiple individual DeleteObject items
///
/// You shouldn't need to create this yourself.
#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteObjects(Vec<DeleteObject>);

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteObject {
    pub id: Uuid,
    pub status: GeneralStatus,
    #[serde(default)]
    pub errors: Option<BatchRequestErrors>,
}

/// Strict definitions of the different status levels available for batch requests.
///
/// Weaviate supports SUCCESS, FAILED, and DRYRUN.
#[derive(Serialize, Deserialize, Debug)]
pub enum GeneralStatus {
    SUCCESS,
    FAILED,
    DRYRUN,
}

/// The ResultStatus of a request
///
/// You shouldn't need to create this yourself unless for asserting against.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultStatus {
    pub status: GeneralStatus,
}

/// The errors received as a result of a failed request
///
/// You shouldn't need to create this yourself unless for asserting against.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRequestErrors {
    pub error: ErrorMessages,
}

/// Container for multiple individual DeleteObjectErrorMessage items
///
/// You shouldn't need to create this yourself.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessages(Vec<ErrorMessage>);

/// A single error message received as a result of a failed request
///
/// You shouldn't need to create this yourself unless for asserting against.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

/// Container for multiple individual BatchAddObject items
///
/// You shouldn't need to create this yourself.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchAddObjects(Vec<BatchAddObject>);

/// This is basically the same as the collections::objects variant of an Object,
/// however there is an extra field which Weaviate polls with a ResultStatus.
///
/// There should be no need to manually create this object, it forms part of the response from the
/// batch add endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchAddObject {
    pub class: String,
    pub properties: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creation_time_unix: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub last_update_time_unix: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector_weights: Option<u64>,
    pub result: ResultStatus,
}

impl BatchAddObject {
    /// Transform the BatchAddObject response item to an Object item.
    pub fn to_object(self) -> Object {
        Object {
            class: self.class,
            properties: self.properties,
            id: self.id,
            vector: self.vector,
            tenant: self.tenant,
            creation_time_unix: self.creation_time_unix,
            last_update_time_unix: self.last_update_time_unix,
            vector_weights: self.vector_weights,
            additional: None,
        }
    }
}

/// Wrapper for the response of the batch add response payload items for each beacon.
///
/// There should be no need to make this manually.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchAddReferencesResponse(pub Vec<BatchAddReferenceResponse>);

/// An individual item received as part of the batch add response payload.
///
/// There should be no need to make this manually.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchAddReferenceResponse {
    result: BatchAddReferenceResult,
}

/// The response field of the BatchAddReferenceResponse
///
/// There should be no need to make this manually.
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchAddReferenceResult {
    pub status: GeneralStatus,
    #[serde(default)]
    pub errors: Option<BatchRequestErrors>,
}
