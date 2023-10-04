use reqwest::Url;
use std::error::Error;

pub struct _Objects {
    _endpoint: Url,
}

impl _Objects {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/objects")?;
        Ok(_Objects { _endpoint })
    }
}
