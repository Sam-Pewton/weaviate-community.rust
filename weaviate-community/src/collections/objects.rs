use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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

///
/// Belongs to objects
///
#[derive(Serialize, Deserialize, Debug)]
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
