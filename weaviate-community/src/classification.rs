use reqwest::Url;
use std::error::Error;

pub struct Classification {
    endpoint: Url,
}

impl Classification {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/classification")?;
        Ok(Classification { endpoint })
    }
}
