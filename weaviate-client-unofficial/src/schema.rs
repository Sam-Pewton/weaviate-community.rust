use reqwest::Url;
use std::error::Error;

pub struct Schema {
    endpoint: Url,
}

impl Schema {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/schema/")?;
        Ok(Schema { endpoint })
    }

    async fn execute(&self, url: Url) -> Result<serde_json::Value, Box<dyn Error>> {
        let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        Ok(resp)
    }

    /// 
    /// Get the schema from Weaviate.
    ///
    /// GET /v1/schema
    ///
    pub async fn get(&self, class_name: Option<&str>) -> Result<serde_json::Value, Box<dyn Error>> {
        let endpoint = match class_name {
            Some(x) => self.endpoint.join(x)?,
            None =>  self.endpoint.clone(),
        };
        self.execute(endpoint).await
    }

    pub async fn create() {
        todo!();
    }

    pub async fn delete() {
        todo!();
    }

    pub async fn update() {
        todo!();
    }

    pub async fn add_property() {
        todo!();
    }
}
