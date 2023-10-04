use reqwest::Url;
use std::error::Error;

pub struct _Meta {
    _endpoint: Url,
}

impl _Meta {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/meta")?;
        Ok(_Meta { _endpoint })
    }
}
