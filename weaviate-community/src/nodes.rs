use reqwest::Url;
use std::error::Error;

pub struct _Nodes {
    _endpoint: Url,
}

impl _Nodes {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/nodes")?;
        Ok(_Nodes { _endpoint })
    }
}
