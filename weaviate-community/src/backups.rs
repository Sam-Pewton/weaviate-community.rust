use reqwest::Url;
use std::error::Error;

pub struct _Backups {
    _endpoint: Url,
}

impl _Backups {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/backups")?;
        Ok(_Backups { _endpoint })
    }
}
