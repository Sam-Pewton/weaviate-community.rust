use reqwest::Url;
use std::error::Error;

pub struct Objects {
    endpoint: Url,
}

impl Objects {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/objects")?;
        Ok(Objects { endpoint })
    }
}
