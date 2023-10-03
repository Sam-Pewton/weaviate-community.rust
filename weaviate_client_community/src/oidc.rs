use reqwest::Url;
use std::error::Error;

pub struct OIDC {
    endpoint: Url,
}

impl OIDC {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/.well-known")?;
        Ok(OIDC { endpoint })
    }
}
