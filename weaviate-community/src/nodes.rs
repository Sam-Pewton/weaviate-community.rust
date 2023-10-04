use reqwest::Url;
use std::error::Error;

pub struct Nodes {
    endpoint: Url,
}

impl Nodes {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/nodes")?;
        Ok(Nodes { endpoint })
    }
}
