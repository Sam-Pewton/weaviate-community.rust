/// All meta associated type components
use serde::{Deserialize, Serialize};

/// The Metadata struct used to contain all of the results returned from the get_meta endpoint.
///
/// There should never be a need for this to be created manually.
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub hostname: String,
    pub modules: serde_json::Value,
    pub version: String,
}
