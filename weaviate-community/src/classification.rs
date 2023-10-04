use reqwest::Url;
use std::error::Error;

pub struct _Classification {
    _endpoint: Url,
}

impl _Classification {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/classification")?;
        Ok(_Classification { _endpoint })
    }
}
