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
                Err(Box::new(NotConfiguredError(
                    "OIDC is not configured or is unavailable".into(),
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{WeaviateClient, collections::oidc::OidcResponse};

    fn test_oidc_response() -> OidcResponse {
        let response: OidcResponse = serde_json::from_value(
            serde_json::json!({
                "clientId": "wcs",
                "href": "https://auth.wcs.api.weaviate.io/auth/realms/SeMI/.well-known/openid-configuration"
            })
        ).unwrap();
        response
    }

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn mock_get(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("GET", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    #[tokio::test]
    async fn test_get_open_id_configuration_ok() {
        let resp = test_oidc_response();
        let resp_str = serde_json::to_string(&resp).unwrap();
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/openid-configuration", 200, &resp_str);
        let res = client.oidc.get_open_id_configuration().await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(resp.client_id, res.unwrap().client_id);
    }

    #[tokio::test]
    async fn test_get_open_id_configuration_err() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/openid-configuration", 404, "");
        let res = client.oidc.get_open_id_configuration().await;
        mock.assert();
        assert!(res.is_err());
    }
}
