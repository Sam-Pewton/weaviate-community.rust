use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// All schema related endpoints and functionality described in the below Weaviate documentation.
///
/// https://weaviate.io/developers/weaviate/api/rest/schema
///
pub struct Schema {
    endpoint: Url,
    client: reqwest::Client,
}

impl Schema {
    ///
    /// Create a new Schema object. The schema object is intended to like inside the WeaviateClient
    /// and be called through the WeaviateClient, but can be created independently too.
    ///
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/schema/")?;
        Ok(Schema {
            endpoint,
            client: reqwest::Client::new(),
        })
    }

    ///
    /// Facilitates the retrieval of both the full Weaviate schema, and the retrieval of the
    /// configuration for a single class in the schema.
    ///
    /// GET /v1/schema
    /// ```
    /// use weaviate_community::Client;
    ///
    /// let client = Client::new("http://localhost:8080").unwrap();
    /// let response = client.schema.get(None);
    /// ```
    ///
    /// GET /v1/schema/{class_name}
    /// ```
    /// use weaviate_community::Client;
    ///
    /// let client = Client::new("http://localhost:8080").unwrap();
    /// let response = client.schema.get(Some("Library"));
    /// ```
    ///
    pub async fn get(&self, class_name: Option<&str>) -> Result<Response, Box<dyn Error>> {
        let endpoint = match class_name {
            Some(x) => self.endpoint.join(x)?,
            None => self.endpoint.clone(),
        };
        let resp = self.client.get(endpoint).send().await?;
        Ok(resp)
    }

    ///
    /// Create a new data object class in the schema.
    ///
    /// POST /v1/schema
    /// ```
    /// use weaviate_community::Client;
    /// //use weaviate_client_unofficial::schema::Class;
    ///
    /// //let class = Class {
    /// //    class: "Test".into(),
    /// //    description: "Test".into(),
    /// //    properties: None,
    /// //    vector_index_type: None,
    /// //    vector_index_config: None,
    /// //    vectorizer: None,
    /// //    module_config: None,
    /// //    inverted_index_config: None,
    /// //    sharding_config: None,
    /// //    multi_tenancy_config: None,
    /// //}
    ///
    /// let client = Client::new("http://localhost:8080").unwrap();
    /// //let response = client.schema.create_class(&class);
    /// ```
    ///
    pub async fn create_class(&self, class: &Class) -> Result<reqwest::Response, Box<dyn Error>> {
        let data = serde_json::to_value(&class).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&data)
            .send()
            .await?;
        Ok(res)
    }

    ///
    /// Remove a class (and all data in the instances) from the schema.
    ///
    /// DELETE v1/schema/{class_name}
    ///
    pub async fn delete(&self, class_name: &str) -> Result<reqwest::Response, Box<dyn Error>> {
        let endpoint = self.endpoint.join(class_name)?;
        let res = self.client.delete(endpoint).send().await?;
        Ok(res)
    }

    ///
    /// Update settings of an existing schema class.
    /// Use this endpoint to alter an existing class in the schema. Note that not all settings are
    /// mutable. If an error about immutable fields is returned and you still need to update this
    /// particular setting, you will have to delete the class (and the underlying data) and
    /// recreate. This endpoint cannot be used to modify properties. Instead, use
    /// POST /v1/schema/{ClassName}/properties. A typical use case for this endpoint is to update
    /// configuration, such as the vectorIndexConfig. Note that even in mutable sections,
    /// such as vectorIndexConfig, some fields may be immutable.
    ///
    /// You should attach a body to this PUT request with the entire new configuration of the class
    ///
    pub async fn update(&self, class: &Class) -> Result<reqwest::Response, Box<dyn Error>> {
        let endpoint = self.endpoint.join(&class.class)?;
        let data = serde_json::to_value(&class).unwrap();
        let res = self.client.put(endpoint).json(&data).send().await?;
        Ok(res)
    }

    ///
    ///
    ///
    pub async fn add_property() {
        todo!();
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub class: String,
    pub description: String,
    pub properties: Option<Vec<Property>>,
    #[serde(default = "default_vector_index_type")]
    pub vector_index_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector_index_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vectorizer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub module_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inverted_index_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sharding_config: Option<ShardingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multi_tenancy_config: Option<String>,
}

fn default_vector_index_type() -> Option<String> {
    Some("hsnw".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    name: String,
    description: String,
    data_type: Vec<String>,
    tokenization: String,
    module_config: Option<HashMap<String, HashMap<String, bool>>>,
    index_filterable: bool,
    index_searchable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShardingConfig {}

#[cfg(test)]
mod tests {
    // Tests currently require a weaviate instance to be running on localhost, as I have not yet
    // implemented anything to mock the database. In future, actual tests will run as integration
    // tests in a container as part of the CICD process.
    use super::*;
    use crate::Client;

    fn test_class(class_name: &str) -> Class {
        Class {
            class: class_name.into(),
            description: "Test".into(),
            properties: None,
            vector_index_type: None,
            vector_index_config: None,
            vectorizer: None,
            module_config: None,
            inverted_index_config: None,
            sharding_config: None,
            multi_tenancy_config: None,
        }
    }

    #[tokio::test]
    async fn test_create_single_class() {
        // Insert the class and get it from the schema
        let class = test_class("CreateSingle");
        let client = Client::new("http://localhost:8080").unwrap();
        let _ = client.schema.create_class(&class).await;
        let result = client.schema.get(Some(&class.class)).await;
        assert_eq!(
            class.class,
            result.unwrap().json::<serde_json::Value>().await.unwrap()["class"]
        );

        // Delete it to tidy up after ourselves
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }

    #[tokio::test]
    async fn test_delete_single_class() {
        // Insert, to make sure it exists.
        let class = test_class("DeleteSingle");
        let client = Client::new("http://localhost:8080").unwrap();
        let result = client.schema.create_class(&class).await;
        assert_eq!(200, result.unwrap().status());

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }

    #[tokio::test]
    async fn test_update_single_class() {
        // Insert, to make sure it exists.
        let mut class = test_class("UpdateSingle");
        let client = Client::new("http://localhost:8080").unwrap();
        let result = client.schema.create_class(&class).await;
        assert_eq!(200, result.as_ref().unwrap().status());
        assert_eq!(
            "Test",
            result.unwrap().json::<serde_json::Value>().await.unwrap()["description"]
        );

        // Update it and make sure that it changed
        class.description = "Updated".into();
        let result = client.schema.update(&class).await;
        assert_eq!(200, result.as_ref().unwrap().status());
        assert_eq!(
            "Updated",
            result.unwrap().json::<serde_json::Value>().await.unwrap()["description"]
        );

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }
}
