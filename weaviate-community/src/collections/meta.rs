use serde::{Deserialize, Serialize};

/// The Metadata struct used to contain all of the results returned from the get_meta endpoint.
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub hostname: String,
    pub modules: serde_json::Value,
    pub version: String,
}
