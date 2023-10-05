use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

///
/// Belong to schema
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub class: String,
    pub description: String,
    pub properties: Option<Vec<Property>>,
    #[serde(default = "default_vector_index_type")]
    pub vector_index_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector_index_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vectorizer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub module_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inverted_index_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sharding_config: Option<ShardingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multi_tenancy_config: Option<MultiTenancyConfig>,
}

fn default_vector_index_type() -> Option<String> {
    Some("hsnw".to_string())
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub data_type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tokenization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub module_config: Option<HashMap<String, HashMap<String, bool>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_filterable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_searchable: Option<bool>,
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShardingConfig {}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiTenancyConfig {
    pub enabled: bool,
}

///
/// Belongs to schema
///
pub enum ShardStatus {
    READONLY,
    READY,
}

impl ShardStatus {
    pub fn value(&self) -> &str {
        match self {
            ShardStatus::READONLY => "READONLY",
            ShardStatus::READY => "READY",
        }
    }
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Tenant {
    pub name: String,
    #[serde(default = "default_activity_status")]
    pub activity_status: Option<ActivityStatus>,
}

fn default_activity_status() -> Option<ActivityStatus> {
    Some(ActivityStatus::HOT)
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityStatus {
    HOT,
    COLD,
}

impl ActivityStatus {
    pub fn value(&self) -> &str {
        match self {
            ActivityStatus::HOT => "HOT",
            ActivityStatus::COLD => "COLD",
        }
    }
}

///
/// Belongs to schema
///
pub enum ConsistencyLevel {
    ONE,
    QUORUM,
    ALL,
}

impl ConsistencyLevel {
    pub fn value(&self) -> &str {
        match self {
            ConsistencyLevel::ONE => "ONE",
            ConsistencyLevel::QUORUM => "QUORUM",
            ConsistencyLevel::ALL => "ALL",
        }
    }
}

///
/// Belongs to objects
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
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
}

///
/// Belongs to Objects
///
pub enum OrderBy {
    ASC,
    DESC,
}

impl OrderBy {
    pub fn value(&self) -> &str {
        match self {
            OrderBy::ASC => "asc",
            OrderBy::DESC => "desc",
        }
    }
}

///
/// Belongs to Objects
///
#[derive(Debug)]
pub struct QueryError(pub String);
impl std::error::Error for QueryError {}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid query parameters passed: {}", self.0)
    }
}

//impl std::fmt::Debug for QueryError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        write!(f, "Invalid query parameters passed: {}", self.message)
//    }
//}
