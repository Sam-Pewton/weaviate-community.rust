use reqwest::Url;
use std::error::Error;

pub struct Modules {
    endpoint: Url,
}

impl Modules {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/modules")?;
        Ok(Modules { endpoint })
    }
}
