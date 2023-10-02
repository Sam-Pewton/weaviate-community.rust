use reqwest::Url;
use std::error::Error;

pub struct Client {
    base_url: Url,
}

impl Client {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let base = Url::parse(url)?;
        Ok(Client { base_url: base })
    }

    async fn execute(&self, url: Url) -> Result<serde_json::Value, Box<dyn Error>> {
        let resp = reqwest::get(url).await?.json::<serde_json::Value>().await?;
        Ok(resp)
    }

    pub async fn get_schema(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        // GET /v1/schema
        let endpoint = self.base_url.join("/v1/schema")?;
        self.execute(endpoint).await
    }

    pub async fn create_class() {
        todo!();
    }

    pub async fn delete_class() {
        todo!();
    }

    pub async fn get_single_class(
        &self,
        class_name: &str,
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        // GET /v1/schema/{class_name}
        let endpoint = self.base_url.join("/v1/schema/")?.join(class_name)?;
        self.execute(endpoint).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client = Client::new("http://localhost:8080").unwrap();
        //let test = client.get_schema().await;
        let test = client.get_single_class("Embeddings").await;
        println!("{:#?}", test);
        //assert_eq!("http://localhost:8080", client.base_url);
    }
}
