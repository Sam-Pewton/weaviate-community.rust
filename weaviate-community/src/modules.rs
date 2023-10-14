use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub struct _Modules {
    _endpoint: Url,
    _client: Arc<reqwest::Client>,
}

impl _Modules {
    pub(super) fn new(url: &Url, _client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/modules")?;
        Ok(_Modules { _endpoint, _client })
    }
}
