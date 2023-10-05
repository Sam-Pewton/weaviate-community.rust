use crate::collections::{ConsistencyLevel, Object};
use reqwest::Url;
use std::error::Error;

/// All schema related endpoints and functionality described in
/// [Weaviate schema API documentation](https://weaviate.io/developers/weaviate/api/rest/objects)
///
pub struct Objects {
    endpoint: Url,
    client: reqwest::Client,
}

impl Objects {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/objects")?;
        Ok(Objects {
            endpoint,
            client: reqwest::Client::new(),
        })
    }

    ///
    ///
    ///
    pub async fn list(
        &self,
        _class_name: Option<&str>,
        _limit: Option<u64>,
        _offset: Option<u64>,
        _after: Option<&str>,
        _include: Option<&str>,
        _sort: Option<&str>,
        _order: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let res = self.client.get(self.endpoint.clone()).send().await?;
        Ok(res)
    }

    ///
    ///
    ///
    pub async fn create(
        &self,
        new_object: &Object,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let c_level = match consistency_level {
            Some(x) => {
                let mut query = "?consistency_level=".to_string();
                query.push_str(x.value());
                query
            }
            None => "".to_string(),
        };
        let endpoint = self.endpoint.join(&c_level)?;
        let payload = serde_json::to_value(&new_object)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    pub async fn batch_create(&self) {
        todo!();
    }

    pub async fn get(&self) {
        todo!();
    }

    pub async fn exists(&self) {
        todo!();
    }

    pub async fn update(&self) {
        todo!();
    }

    pub async fn delete(&self) {
    }

    pub async fn validate(&self) {
    }

    // cross references?
}

#[cfg(test)]
mod tests {
    //use crate::collections::{ConsistencyLevel, Object};
    use crate::collections::Object;
    //use crate::Client;

    fn _test_object() -> Object {
        let properties = serde_json::json!({
            "name": "test"
        });
        Object {
            class: "TestClass2".into(),
            properties,
            id: None,
            vector: None,
            tenant: None,
        }
    }

    #[tokio::test]
    async fn test_create_object() {
        //let client = Client::new("http://localhost:8080").unwrap();
        //let object = test_object();
        //let _res = client.objects.create(&object, Some(ConsistencyLevel::ALL)).await;
        //
        //let res = client.objects.list(None, None, None, None, None, None, None).await;

        //println!("{:?}", res);
    }
}
