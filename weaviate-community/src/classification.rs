use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct _Classification {
    _endpoint: Url,
    _client: Arc<reqwest::Client>,
}

impl _Classification {
    pub(super) fn new(url: &Url, _client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/classification")?;
        Ok(_Classification { _endpoint, _client })
    }
}
