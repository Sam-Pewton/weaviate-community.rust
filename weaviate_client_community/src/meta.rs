use reqwest::Url;
use std::error::Error;

pub struct Meta {
    endpoint: Url,
}

impl Meta {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/meta")?;
        Ok(Meta { endpoint })
    }
}
