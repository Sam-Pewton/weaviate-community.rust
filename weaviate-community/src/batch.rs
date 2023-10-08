use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

pub struct _Batch {
    _endpoint: Url,
    _client: Arc<reqwest::Client>,
}

impl _Batch {
    pub(super) fn new(url: &Url, _client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/batch")?;
        Ok(_Batch { _endpoint, _client })
    }
}
