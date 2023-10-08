use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

pub struct _OIDC {
    _endpoint: Url,
    _client: Arc<reqwest::Client>,
}

impl _OIDC {
    pub(super) fn new(url: &Url, _client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/.well-known")?;
        Ok(_OIDC { _endpoint, _client })
    }
}
