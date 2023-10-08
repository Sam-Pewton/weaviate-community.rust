use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

pub struct _Backups {
    _endpoint: Url,
    _client: Arc<reqwest::Client>,
}

impl _Backups {
    pub(super) fn new(url: &Url, _client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/backups")?;
        Ok(_Backups { _endpoint, _client })
    }
}
