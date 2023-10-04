use crate::collections::{Class, Property, ShardStatus, Tenant};
use reqwest::{Response, Url};
use std::error::Error;

/// All schema related endpoints and functionality described in 
/// [Weaviate schema API documentation](https://weaviate.io/developers/weaviate/api/rest/schema)
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
    /// Note that from 1.5.0, creating a schema is optional, as Auto Schema is available. See for 
    /// more info:
    /// [Weaviate auto-schema documentation](https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema)
    ///
    /// POST /v1/schema
    /// ```
    /// use weaviate_community::Client;
    /// use weaviate_community::collections::Class;
    ///
    /// let class = Class {
    ///     class: "Library".into(),
    ///     description: "Library Class".into(),
    ///     properties: None,
    ///     vector_index_type: None,
    ///     vector_index_config: None,
    ///     vectorizer: None,
    ///     module_config: None,
    ///     inverted_index_config: None,
    ///     sharding_config: None,
    ///     multi_tenancy_config: None,
    /// };
    ///
    /// let client = Client::new("http://localhost:8080").unwrap();
    /// let response = client.schema.create_class(&class);
    /// ```
    ///
    pub async fn create_class(&self, class: &Class) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::to_value(&class).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&payload)
            .send()
            .await?;
        Ok(res)
    }

    ///
    /// Remove a class (and all data in the instances) from the schema.
    ///
    /// DELETE v1/schema/{class_name}
    /// ```
    /// use weaviate_community::Client;
    ///
    /// let client = Client::new("http://localhost:8080").unwrap();
    /// let response = client.schema.delete("Library");
    /// ```
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
        let payload = serde_json::to_value(&class)?;
        let res = self.client.put(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    ///
    /// Add a property to an existing class in the schema.
    ///
    pub async fn add_property(
        &self,
        class_name: &str,
        property: &Property,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/properties");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&property)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    ///
    /// View all of the shards for a particular class.
    ///
    pub async fn get_shards(&self, class_name: &str) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/shards");
        let endpoint = self.endpoint.join(&endpoint)?;
        let res = self.client.get(endpoint).send().await?;
        Ok(res)
    }

    ///
    /// Update shard status
    ///
    pub async fn update_class_shard(
        &self,
        class_name: &str,
        shard_name: &str,
        status: ShardStatus,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/shards/");
        endpoint.push_str(shard_name);
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::json!({
                "status": status.value()
        });
        let res = self.client.put(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    ///
    /// List tenants
    ///
    pub async fn list_tenants(
        &self,
        class_name: &str,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let res = self.client.get(endpoint).send().await?;
        Ok(res)
    }

    ///
    /// Add tenant
    ///
    pub async fn add_tenants(
        &self,
        class_name: &str,
        tenants: &Vec<Tenant>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&tenants)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    ///
    /// Remove tenants
    ///
    pub async fn remove_tenants(
        &self,
        class_name: &str,
        tenants: &Vec<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&tenants)?;
        let res = self.client.delete(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    ///
    /// Update tenants
    ///
    /// For updating tenants, both `name` and `activity_status` are required.
    ///
    /// Note that tenant activity status setting is only available from Weaviate v1.21
    ///
    pub async fn update_tenants(
        &self,
        class_name: &str,
        tenants: &Vec<Tenant>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&tenants)?;
        println!("{:?}", &payload);
        let res = self.client.put(endpoint).json(&payload).send().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    // Tests currently require a weaviate instance to be running on localhost, as I have not yet
    // implemented anything to mock the database. In future, actual tests will run as integration
    // tests in a container as part of the CICD process.
    use crate::collections::{
        ActivityStatus, Class, MultiTenancyConfig, Property, ShardStatus, Tenant,
    };
    use crate::Client;

    ///
    /// Helper function for generating a testing class
    ///
    fn test_class(class_name: &str, enabled: bool) -> Class {
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
            multi_tenancy_config: Some(MultiTenancyConfig { enabled }),
        }
    }

    ///
    /// Helper function for generating a testing property
    ///
    fn test_property(property_name: &str) -> Property {
        Property {
            name: property_name.into(),
            data_type: vec!["boolean".into()],
            description: Some("this is a test to see camel case".into()),
            index_filterable: None,
            index_searchable: None,
            module_config: None,
            tokenization: None,
        }
    }

    ///
    /// Helper function for generating some test tenants, as shown on the weaviate API webpage.
    ///
    fn test_tenants() -> Vec<Tenant> {
        vec![
            Tenant {
                name: "TENANT_A".into(),
                activity_status: None,
            },
            Tenant {
                name: "TENANT_B".into(),
                activity_status: Some(ActivityStatus::COLD),
            },
        ]
    }

    #[tokio::test]
    async fn test_create_single_class() {
        // Insert the class and get it from the schema
        let class = test_class("CreateSingle", false);
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
        let class = test_class("DeleteSingle", false);
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
        let mut class = test_class("UpdateSingle", false);
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

    #[tokio::test]
    async fn test_add_property() {
        // Insert, to make sure it exists.
        let class = test_class("AddProperty", false);
        let client = Client::new("http://localhost:8080").unwrap();
        let result = client.schema.create_class(&class).await;
        assert_eq!(200, result.as_ref().unwrap().status());

        // Validate the property does not exist in the class schema
        let result = client.schema.get(Some(&class.class)).await;
        assert_eq!(
            serde_json::Value::Null,
            result.unwrap().json::<serde_json::Value>().await.unwrap()["properties"]
        );

        // Update class with test property
        let property = test_property("TestProperty");

        // Update it and make sure that it changed
        let result = client.schema.add_property(&class.class, &property).await;
        assert_eq!(200, result.as_ref().unwrap().status());

        // Validate the property now exists in the class schema
        let result = client.schema.get(Some(&class.class)).await;
        assert_eq!(
            "testProperty",
            result.unwrap().json::<serde_json::Value>().await.unwrap()["properties"][0]["name"]
        );

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }

    #[tokio::test]
    async fn test_get_shards() {
        let class = test_class("GetShards", false);
        let client = Client::new("http://localhost:8080").unwrap();
        let result = client.schema.create_class(&class).await;
        assert_eq!(200, result.as_ref().unwrap().status());

        let result = client.schema.get_shards(&class.class).await;
        assert_eq!(200, result.as_ref().unwrap().status());
        assert_eq!(
            "READY",
            result.unwrap().json::<serde_json::Value>().await.unwrap()[0]["status"]
        );

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }

    #[tokio::test]
    async fn test_update_shard_status() {
        let class = test_class("UpdateShards", false);
        let client = Client::new("http://localhost:8080").unwrap();
        let result = client.schema.create_class(&class).await;
        assert_eq!(200, result.as_ref().unwrap().status());

        // Get the name of the shard
        let result = client.schema.get_shards(&class.class).await;
        assert_eq!(200, result.as_ref().unwrap().status());
        let shards = result.unwrap().json::<serde_json::Value>().await.unwrap();
        assert_eq!("READY", shards[0]["status"]);

        // Update the shard status
        let name = serde_json::to_string(&shards[0]["name"]).unwrap().clone();
        let name = name.trim_start_matches("\"");
        let name = name.trim_end_matches("\"");
        let _result = client
            .schema
            .update_class_shard(&class.class, &name, ShardStatus::READONLY)
            .await;
        //println!("{:?}", result.as_ref().unwrap().status());
        //assert_eq!(200, result.as_ref().unwrap().status());

        // Get the shard again
        let result = client.schema.get_shards(&class.class).await;
        assert_eq!(200, result.as_ref().unwrap().status());
        let _shards = result.unwrap().json::<serde_json::Value>().await.unwrap();
        //assert_eq!("READONLY", shards[0]["status"]);
        //println!("{:?}", shards[0]["status"]);

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }

    #[tokio::test]
    async fn test_list_tenants() {
        let class = test_class("ListTenants", true);
        let client = Client::new("http://localhost:8080").unwrap();
        let result = client.schema.create_class(&class).await;
        assert_eq!(200, result.as_ref().unwrap().status());

        let result = client.schema.list_tenants(&class.class).await;
        //assert_eq!(200, result.as_ref().unwrap().status());
        println!(
            "{:?}",
            result.unwrap().json::<serde_json::Value>().await.unwrap()
        );

        let mut tenants = test_tenants();
        let result = client.schema.add_tenants(&class.class, &tenants).await;
        assert_eq!(200, result.as_ref().unwrap().status());

        let result = client.schema.list_tenants(&class.class).await;
        //assert_eq!(200, result.as_ref().unwrap().status());
        println!(
            "{:?}",
            result.unwrap().json::<serde_json::Value>().await.unwrap()
        );

        tenants[0].activity_status = Some(ActivityStatus::COLD);
        tenants[1].activity_status = Some(ActivityStatus::COLD);
        let result = client.schema.update_tenants(&class.class, &tenants).await;
        //assert_eq!(200, result.as_ref().unwrap().status());
        println!(
            "{:?}",
            result.unwrap().json::<serde_json::Value>().await.unwrap()
        );

        let result = client.schema.list_tenants(&class.class).await;
        //assert_eq!(200, result.as_ref().unwrap().status());
        println!(
            "{:?}",
            result.unwrap().json::<serde_json::Value>().await.unwrap()
        );

        let result = client
            .schema
            .remove_tenants(&class.class, &vec!["TENANT_A", "TENANT_B"])
            .await;
        assert_eq!(200, result.as_ref().unwrap().status());
        let result = client.schema.list_tenants(&class.class).await;
        //assert_eq!(200, result.as_ref().unwrap().status());
        println!(
            "{:?}",
            result.unwrap().json::<serde_json::Value>().await.unwrap()
        );

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert_eq!(200, result.unwrap().status());
    }
}
