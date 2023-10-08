/// All OIDC associated type components
use serde::{Deserialize, Serialize};

/// The expected response format when received from /v1/.well-known/openid-configuration
/// successfully.
///
/// Generally wouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct OidcResponse {
    href: String,
    #[serde(rename = "cliendID")]
    cliend_id: String,
}
