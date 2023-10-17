use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchConfig {
    pub class: String,
    #[serde(rename = "where")]
    pub match_where: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Verbosity {
    #[serde(rename = "minimal")]
    MINIMAL,
    #[serde(rename = "verbose")]
    VERBOSE,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchDeleteResult {
    pub matches: u64,
    pub limit: u64,
    pub successful: u64,
    pub failed: u64,
    #[serde(default)]
    pub objects: Option<Vec<DeleteObject>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteObject {
    pub id: Uuid,
    pub status: GeneralStatus,
    #[serde(default)]
    pub errors: Option<DeleteObjectErrors>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GeneralStatus {
    SUCCESS,
    FAILED,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultStatus {
    pub status: GeneralStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteObjectErrors {
    pub error: Vec<DeleteObjectErrorMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteObjectErrorMessage {
    pub message: String,
}

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
