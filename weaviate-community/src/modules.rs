use reqwest::Url;
use std::error::Error;

pub struct _Modules {
    _endpoint: Url,
}

impl _Modules {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let _endpoint = url.join("/v1/modules")?;
        Ok(_Modules { _endpoint })
    }
}
