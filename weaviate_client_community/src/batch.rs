use reqwest::Url;
use std::error::Error;

pub struct Batch {
    endpoint: Url,
}

impl Batch {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/batch")?;
        Ok(Batch { endpoint })
    }
}
