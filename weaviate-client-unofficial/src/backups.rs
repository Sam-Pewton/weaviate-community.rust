use reqwest::Url;
use std::error::Error;

pub struct Backups {
    endpoint: Url,
}

impl Backups {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/backups")?;
        Ok(Backups { endpoint })
    }
}
