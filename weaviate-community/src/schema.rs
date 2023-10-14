use crate::collections::error::SchemaError;
use crate::collections::schema::{
    Class, Classes, Property, Shard, ShardStatus, Shards, Tenant, Tenants,
};
use reqwest::Url;
use std::error::Error;
use std::sync::Arc;

/// All schema related endpoints and functionality described in
/// [Weaviate schema API documentation](https://weaviate.io/developers/weaviate/api/rest/schema)
#[derive(Debug)]
pub struct Schema {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Schema {
    /// Create a new Schema object. The schema object is intended to like inside the WeaviateClient
    /// and be called through the WeaviateClient.
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/schema/")?;
        Ok(Schema { endpoint, client })
    }

    /// Facilitates the retrieval of the configuration for a single class in the schema.
    ///
    /// GET /v1/schema/{class_name}
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None)?;
    ///     let response = client.schema.get_class("Library").await;
    ///     assert!(response.is_err());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_class(&self, class_name: &str) -> Result<Class, Box<dyn Error>> {
        let endpoint = self.endpoint.join(class_name)?;
        let res = self.client.get(endpoint).send().await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Class = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling get_class endpoint.",
                res.status()
            )))),
        }
    }

    /// Facilitates the retrieval of the full Weaviate schema.
    ///
    /// GET /v1/schema
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None)?;
    ///     let schema = client.schema.get().await?;
    ///     println!("{:#?}", &schema);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(&self) -> Result<Classes, Box<dyn Error>> {
        let res = self.client.get(self.endpoint.clone()).send().await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Classes = res.json().await?;
                //let res2 = res.json::<serde_json::Value>().await?;
                //let res: Classes = serde_json::from_value(res2)?;
                Ok(res)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling create_class endpoint.",
                res.status()
            )))),
        }
    }

    /// Create a new data object class in the schema.
    ///
    /// Note that from 1.5.0, creating a schema is optional, as Auto Schema is available. See for
    /// more info:
    /// [Weaviate auto-schema documentation](https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema)
    ///
    /// POST /v1/schema
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::schema::Class;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let class = Class {
    ///         class: "Library".into(),
    ///         description: "Library Class".into(),
    ///         properties: None,
    ///         vector_index_type: None,
    ///         vector_index_config: None,
    ///         vectorizer: None,
    ///         module_config: None,
    ///         inverted_index_config: None,
    ///         sharding_config: None,
    ///         multi_tenancy_config: None,
    ///         replication_config: None,
    ///     };
    ///
    ///     let client = WeaviateClient::new("http://localhost:8080", None)?;
    ///     let class = client.schema.create_class(&class).await?;
    ///     println!("{:#?}", &class);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_class(&self, class: &Class) -> Result<Class, Box<dyn Error>> {
        let payload = serde_json::to_value(&class).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&payload)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Class = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling create_class endpoint.",
                res.status()
            )))),
        }
    }

    ///
    /// Remove a class (and all data in the instances) from the schema.
    ///
    /// DELETE v1/schema/{class_name}
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let response = client.schema.delete("Library").await;
    /// }
    /// ```
    ///
    pub async fn delete(&self, class_name: &str) -> Result<bool, Box<dyn Error>> {
        let endpoint = self.endpoint.join(class_name)?;
        let res = self.client.delete(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(true),
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling delete endpoint.",
                res.status()
            )))),
        }
    }

    /// Update settings of an existing schema class.
    ///
    /// Use this endpoint to alter an existing class in the schema. Note that not all settings are
    /// mutable. If an error about immutable fields is returned and you still need to update this
    /// particular setting, you will have to delete the class (and the underlying data) and
    /// recreate. This endpoint cannot be used to modify properties.
    //  Instead, use POST /v1/schema/{ClassName}/properties (add_property method).
    //
    /// A typical use case for this endpoint is to update configuration, such as the
    /// vectorIndexConfig. Note that even in mutable sections, such as vectorIndexConfig,
    /// some fields may be immutable.
    ///
    /// You should attach a body to this PUT request with the entire new configuration of the class
    pub async fn update(&self, class: &Class) -> Result<Class, Box<dyn Error>> {
        let endpoint = self.endpoint.join(&class.class)?;
        let payload = serde_json::to_value(&class)?;
        let res = self.client.put(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Class = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling update endpoint.",
                res.status()
            )))),
        }
    }

    ///
    /// Add a property to an existing class in the schema.
    ///
    pub async fn add_property(
        &self,
        class_name: &str,
        property: &Property,
    ) -> Result<Property, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/properties");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&property)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Property = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling add_property endpoint.",
                res.status()
            )))),
        }
    }

    ///
    /// View all of the shards for a particular class.
    ///
    pub async fn get_shards(&self, class_name: &str) -> Result<Shards, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/shards");
        let endpoint = self.endpoint.join(&endpoint)?;
        let res = self.client.get(endpoint).send().await?;
        let shards = res.json::<Vec<Shard>>().await?;
        let shards = Shards { shards };
        Ok(shards)
    }

    ///
    /// Update shard status
    ///
    pub async fn update_class_shard(
        &self,
        class_name: &str,
        shard_name: &str,
        status: ShardStatus,
    ) -> Result<Shard, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/shards/");
        endpoint.push_str(shard_name);
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::json!({ "status": status });
        let res = self.client.put(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(Shard {
                name: shard_name.into(),
                status,
            }),
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling update class shard endpoint.",
                res.status()
            )))),
        }
    }

    ///
    /// List tenants
    ///
    pub async fn list_tenants(&self, class_name: &str) -> Result<Tenants, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let res = self.client.get(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let tenants = res.json::<Vec<Tenant>>().await?;
                let tenants = Tenants { tenants };
                Ok(tenants)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling list_tenants endpoint.",
                res.status()
            )))),
        }
    }

    ///
    /// Add tenant
    ///
    pub async fn add_tenants(
        &self,
        class_name: &str,
        tenants: &Tenants,
    ) -> Result<Tenants, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&tenants.tenants)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let tenants = res.json::<Vec<Tenant>>().await?;
                let tenants = Tenants { tenants };
                Ok(tenants)
            }
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling list_tenants endpoint.",
                res.status()
            )))),
        }
    }

    ///
    /// Remove tenants
    ///
    pub async fn remove_tenants(
        &self,
        class_name: &str,
        tenants: &Vec<&str>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&tenants)?;
        let res = self.client.delete(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => Ok(true),
            _ => Err(Box::new(SchemaError(format!(
                "status code {} received when calling remove_tenants endpoint.",
                res.status()
            )))),
        }
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
        tenants: &Tenants,
    ) -> Result<Tenants, Box<dyn Error>> {
        let mut endpoint = class_name.to_string();
        endpoint.push_str("/tenants");
        let endpoint = self.endpoint.join(&endpoint)?;
        let payload = serde_json::to_value(&tenants.tenants)?;
        let res = self.client.put(endpoint).json(&payload).send().await?;
        let tenants = res.json::<Vec<Tenant>>().await?;
        let tenants = Tenants { tenants };
        Ok(tenants)
    }
}

#[cfg(test)]
mod tests {
    // Tests currently require a weaviate instance to be running on localhost, as I have not yet
    // implemented anything to mock the database. In future, actual tests will run as integration
    // tests in a container as part of the CICD process.
    use crate::collections::schema::{
        ActivityStatus, Class, ClassBuilder, MultiTenancyConfig, Property, ShardStatus, Tenant,
        Tenants,
    };
    use crate::{WeaviateClient, AuthApiKey, WeaviateClientBuilder};

    /// Helper function for generating a testing class
    fn test_class(class_name: &str, enabled: bool) -> Class {
        ClassBuilder::new(class_name, "Test")
            .multi_tenancy_config(MultiTenancyConfig { enabled })
            .build()
    }

    /// Helper function for generating a testing property
    fn test_property(property_name: &str) -> Property {
        Property {
            name: property_name.into(),
            data_type: vec!["boolean".into()],
            description: Some("test property".into()),
            index_filterable: None,
            index_searchable: None,
            module_config: None,
            tokenization: None,
            inverted_index_config: None,
        }
    }

    /// Helper function for generating some test tenants, as shown on the weaviate API webpage.
    fn test_tenants() -> Tenants {
        Tenants {
            tenants: vec![
                Tenant {
                    name: "TENANT_A".into(),
                    activity_status: None,
                },
                Tenant {
                    name: "TENANT_B".into(),
                    activity_status: Some(ActivityStatus::COLD),
                },
            ],
        }
    }

    #[tokio::test]
    async fn test_create_single_class() {
        // Insert the class and get it from the schema
        let class = test_class("CreateSingle", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(&result.is_ok());
        assert_eq!(&result.unwrap().class, "CreateSingle");
        //let test = result.unwrap().json::<serde_json::Value>().await.unwrap();
        //let test2: Class = serde_json::from_value(test.clone()).unwrap();

        //assert_eq!(
        //    class.class,
        //    result.unwrap().json::<serde_json::Value>().await.unwrap()["class"]
        //);

        // Delete it to tidy up after ourselves
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_get_single_class_ok() {
        // Insert, to make sure it exists.
        let class = test_class("GetSingleClass", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        // get it
        let result = client.schema.get_class(&class.class).await;
        assert!(result.is_ok());
        assert_eq!(&result.unwrap().class, "GetSingleClass");

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_get_single_class_err() {
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.get_class("DOESNOTEXIST").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_all_classes() {
        // Insert, to make sure it exists.
        let class = test_class("GetAllClasses", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        let result = client.schema.get().await;
        assert!(result.is_ok());

        // There could be more than just one in the schema, depending on when the tests run.
        assert_ne!(&result.unwrap().classes.len(), &0);

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_update_single_class() {
        // Insert, to make sure it exists.
        let mut class = test_class("UpdateSingle", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        // Update it and make sure that it changed
        class.description = "Updated".into();
        let result = client.schema.update(&class).await;
        assert_eq!("Updated", result.unwrap().description);

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_add_property() {
        // Insert, to make sure it exists.
        let class = test_class("AddProperty", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        // Validate the property does not exist in the class schema
        let result = client.schema.get_class(&class.class).await;
        assert_eq!(None, result.unwrap().properties);

        // Update class with test property
        let property = test_property("TestProperty");

        // Update it and make sure that it changed
        let result = client.schema.add_property(&class.class, &property).await;
        assert_eq!("testProperty", result.unwrap().name);

        // Validate the property now exists in the class schema
        let result = client.schema.get_class(&class.class).await;
        assert!(result.unwrap().properties.is_some());

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_get_shards() {
        let class = test_class("GetShards", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        let shards = client.schema.get_shards(&class.class).await;
        assert_eq!(ShardStatus::READY, shards.unwrap().shards[0].status);

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_update_shard_status() {
        let class = test_class("UpdateShards", false);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        // Get the name of the shard
        let result = client.schema.get_shards(&class.class).await;
        let shards = result.unwrap();
        assert_eq!(1, shards.shards.len());
        assert_eq!(ShardStatus::READY, shards.shards[0].status);

        // Update the shard status
        let name = serde_json::to_string(&shards.shards[0].name)
            .unwrap()
            .clone();
        let name = name.trim_start_matches("\"");
        let name = name.trim_end_matches("\"");
        let result = client
            .schema
            .update_class_shard(&class.class, &name, ShardStatus::READONLY)
            .await;
        assert_eq!(ShardStatus::READONLY, result.unwrap().status);

        // Get the shard again
        let result = client.schema.get_shards(&class.class).await;
        let shards = result.unwrap();
        assert_eq!(1, shards.shards.len());
        assert_eq!(ShardStatus::READONLY, shards.shards[0].status);

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_list_tenants() {
        let class = test_class("ListTenants", true);
        let auth = AuthApiKey::new("test-key");
        let client = WeaviateClient::builder("http://localhost:8080").auth_secret(auth)
            .build().unwrap();
        let result = client.schema.create_class(&class).await;
        assert!(result.is_ok());

        let result = client.schema.list_tenants(&class.class).await;
        assert_eq!(0, result.unwrap().tenants.len());

        let mut tenants = test_tenants();
        let result = client.schema.add_tenants(&class.class, &tenants).await;
        assert_eq!(2, result.unwrap().tenants.len());

        let result = client.schema.list_tenants(&class.class).await;
        assert_eq!(2, result.unwrap().tenants.len());

        tenants.tenants[0].activity_status = Some(ActivityStatus::COLD);
        tenants.tenants[1].activity_status = Some(ActivityStatus::HOT);
        let result = client.schema.update_tenants(&class.class, &tenants).await;
        assert_eq!(2, result.unwrap().tenants.len());

        let result = client.schema.list_tenants(&class.class).await;
        assert_eq!(2, result.unwrap().tenants.len());

        let result = client
            .schema
            .remove_tenants(&class.class, &vec!["TENANT_A", "TENANT_B"])
            .await;
        assert!(result.is_ok());
        let result = client.schema.list_tenants(&class.class).await;
        assert_eq!(0, result.unwrap().tenants.len());

        // Delete it and make sure that it is gone
        let result = client.schema.delete(&class.class).await;
        assert!(result.unwrap());
    }
}
