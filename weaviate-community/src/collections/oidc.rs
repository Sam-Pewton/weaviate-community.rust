/// All OIDC associated type components
use serde::{Deserialize, Serialize};

/// The expected response format when received from /v1/.well-known/openid-configuration
/// successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct OidcResponse {
    pub href: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
}
