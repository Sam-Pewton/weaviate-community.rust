/// https://weaviate.io/developers/weaviate/api/rest/well-known
use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

use crate::collections::error::NotConfiguredError;
use crate::collections::oidc::OidcResponse;

#[derive(Debug)]
pub struct Oidc {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Oidc {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/.well-known")?;
        Ok(Oidc { endpoint, client })
    }

    /// Get OIDC information if OpenID Connect (OIDC) authentication is enabled. The endpoint
    /// redirects to the token issued if one is configured.
    ///
    /// The redirect will return the following fields:
    /// - href      => The reference to the client
    /// - cliendID  => The ID of the client
    ///
    /// # Examples
    ///
    /// GET /v1/.well-known/openid-configuration
    /// ```
    /// ```
    pub async fn get_open_id_configuration(&self) -> Result<OidcResponse, Box<dyn Error>> {
        let endpoint = self.endpoint.join("/openid-configuration")?;
        let resp = self.client.get(endpoint).send().await?;
        match resp.status() {
            reqwest::StatusCode::OK => {
                let parsed: OidcResponse = resp.json::<OidcResponse>().await?;
                Ok(parsed)
            }
            _ => {
                return Err(Box::new(NotConfiguredError(
                    "OIDC is not configured or is unavailable".into(),
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{WeaviateClient, AuthApiKey};

    #[tokio::test]
    async fn test_get_open_id_configuration() {
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let _res = client.oidc.get_open_id_configuration().await;
    }
}
