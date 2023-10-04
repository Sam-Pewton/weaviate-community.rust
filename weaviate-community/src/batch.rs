use reqwest::Url;
use std::error::Error;

pub struct _Batch {
    _endpoint: Url,
}

impl _Batch {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/batch")?;
        Ok(_Batch { _endpoint })
    }
}
