use reqwest::Url;
use std::error::Error;

pub struct _OIDC {
    _endpoint: Url,
}

impl _OIDC {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/.well-known")?;
        Ok(_OIDC { _endpoint })
    }
}
