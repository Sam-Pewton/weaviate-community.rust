use crate::collections::error::QueryError;
use crate::collections::objects::{ConsistencyLevel, Object, OrderBy, MultiObjects};
use reqwest::Url;
use std::{error::Error, sync::Arc};
use uuid::Uuid;

/// All objects endpoints and functionality described in
/// [Weaviate objects API documentation](https://weaviate.io/developers/weaviate/api/rest/objects)
///
#[derive(Debug)]
pub struct Objects {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Objects {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/objects/")?;
        Ok(Objects { endpoint, client })
    }

    /// List the data objects.
    ///
    /// # Parameters
    /// class_name: the name of the class to search for
    /// limit: 
    /// offset: cannot be used with after, should be used in conjunction with limit
    /// after: MUST be used with class, cannot be used with offset or sort, should be used with
    ///          limit
    /// include: has a list of allowed values, including classification, vector,
    ///            featureProjection, and other module-specific additional properties
    /// sort: can be a comma separated list of strings (corresponding to properties)
    /// order: `asc` or `desc` should be used with sort
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let res = client
    ///         .objects
    ///         .list(
    ///             Some("MyClass"),
    ///             None,
    ///             None,
    ///             None,
    ///             None,
    ///             None,
    ///             None,
    ///         ).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        class_name: Option<&str>,
        limit: Option<u64>,
        offset: Option<u64>,
        after: Option<&str>,
        include: Option<&str>,
        sort: Option<Vec<&str>>,
        order: Option<OrderBy>,
        ) -> Result<MultiObjects, Box<dyn Error>> {
        let mut endpoint = self.endpoint.clone();

        // Add the query params when they are present
        if let Some(c) = class_name {
            endpoint.query_pairs_mut().append_pair("class", c);
        }
        if let Some(l) = limit {
            endpoint
                .query_pairs_mut()
                .append_pair("limit", &l.to_string());
        }
        if let Some(o) = offset {
            endpoint
                .query_pairs_mut()
                .append_pair("offset", &o.to_string());
            // Raise an err if after is some
            if after.is_some() {
                return Err(Box::new(QueryError(
                            "'after' must be None when 'offset' is Some".into(),
                            )));
            }
        }
        if let Some(a) = after {
            endpoint.query_pairs_mut().append_pair("after", a);
            if after.is_none() {
                return Err(Box::new(QueryError(
                            "'class' must be Some when 'after' is Some".into(),
                            )));
            }
            // raise an error if offset or sort are some
            if offset.is_some() {
                return Err(Box::new(QueryError(
                            "'offset' must be None when 'after' is Some".into(),
                            )));
            }
            if sort.is_some() {
                return Err(Box::new(QueryError(
                            "'sort' must be None when 'after' is Some".into(),
                            )));
            }
        }
        if let Some(i) = include {
            endpoint.query_pairs_mut().append_pair("include", i);
        }
        if let Some(s) = sort {
            let values = s.join(",");
            endpoint.query_pairs_mut().append_pair("sort", &values);
        }
        if let Some(o) = order {
            endpoint.query_pairs_mut().append_pair("order", o.value());
        }

        let res = self.client.get(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: MultiObjects = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(QueryError(format!(
                            "status code {} received when calling get class endpoint.",
                            res.status()
                            )))),
        }
    }

    /// Create a new data object. The provided meta-data and schema values are validated.
    ///
    /// When inserting a large number of objects, it is more efficient to use the `batch` insert
    /// methods.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::objects::Object;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let properties = serde_json::json!({
    ///         "name": "Jodi Kantor",
    ///     });
    ///     let new = Object {
    ///         class: "Publication".into(),
    ///         properties,
    ///         id: None,
    ///         vector: None,
    ///         tenant: None,
    ///         creation_time_unix: None,
    ///         last_update_time_unix: None,
    ///         vector_weights: None,
    ///     };
    ///     let res = client.objects.create(
    ///         &new,
    ///         None
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        new_object: &Object,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<Object, Box<dyn Error>> {
        let mut endpoint = self.endpoint.clone();
        if let Some(x) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", x.value());
        }
        let payload = serde_json::to_value(&new_object)?;

        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Object = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling create class endpoint.",
                res.status()
            )))),
        }
    }

    /// Collect an individual data object given it's UUID.
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
    ///     let res = client
    ///         .objects
    ///         .get("TestListObject", &uuid, None, None, None).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get(
        &self,
        class_name: &str,
        id: &Uuid,
        include: Option<&str>,
        consistency_level: Option<ConsistencyLevel>,
        tenant_key: Option<&str>,
    ) -> Result<Object, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_key {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }
        if let Some(i) = include {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("include", i);
        }

        let res = self.client.get(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Object = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling get class endpoint.",
                res.status()
            )))),
        }
    }

    /// Check if a data object exists without returning the object itself.
    ///
    /// This works the same as the `get` method, but uses `HEAD` HTTP method.
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
    ///     let res = client
    ///         .objects
    ///         .exists("TestListObject", &uuid, None, None).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn exists(
        &self,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }

        let res = self.client.head(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(true)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling exists (class) endpoint.",
                res.status()
            )))),
        }
    }

    /// Updates the given property values of the data object.
    ///
    /// To replace all property values, use the `replace` method.
    ///
    /// Note that if the class is configured with a vectorizer, Weaviate will only compute a new
    /// vector for an updated object if the update changes the underlying text to be vectorized.
    /// 
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
    ///     let properties = serde_json::json!({
    ///         "name": "new name",
    ///     });
    ///     let res = client
    ///         .objects
    ///         .update(&properties, "Article", &uuid, None).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn update(
        &self,
        properties: &serde_json::Value,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        let res = self.client.patch(endpoint).json(&properties).send().await?;
        match res.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(true)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling update class endpoint.",
                res.status()
            )))),
        }
    }

    /// Replaces all property values of the data object.
    ///
    /// Use the `update` method if only modifying some properties.
    ///
    /// Note that if the class is configured with a vectorizer, Weaviate will only compute a new
    /// vector for an updated object if the update changes the underlying text to be vectorized.
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
    ///     let properties = serde_json::json!({
    ///         "properties": {
    ///             "name": "Jodi Kantor",
    ///         }
    ///     });
    ///     let res = client
    ///         .objects
    ///         .replace(&properties, "Publication", &uuid, None).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn replace(
        &self,
        properties: &serde_json::Value,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<Object, Box<dyn Error>> {
        let payload = serde_json::json!({
            "class": class_name,
            "id": id,
            "properties": properties
        });
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }

        let res = self.client.put(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Object = res.json().await?;
                Ok(res)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling update class endpoint.",
                res.status()
            )))),
        }
    }

    /// Delete an individual data object from Weaviate.
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
    ///     let res = client
    ///         .objects
    ///         .delete("Article", &uuid, None, None)
    ///         .await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete(
        &self,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }

        let res = self.client.delete(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(true)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling update class endpoint.",
                res.status()
            )))),
        }
    }

    /// Validate an object's schema and metadata without creating it.
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let properties = serde_json::json!({
    ///         "name": "New York Times"
    ///     });
    ///     let uuid = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    ///     let res = client.objects.validate("Publication", properties, &uuid).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn validate(
        &self,
        class_name: &str,
        properties: serde_json::Value,
        id: &Uuid,
    ) -> Result<bool, Box<dyn Error>> {
        let payload = serde_json::json!({
            "class": class_name,
            "id": id.to_string(),
            "properties": properties
        });
        let endpoint = self.endpoint.join("validate")?;

        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(true)
            }
            _ => Err(Box::new(QueryError(format!(
                "status code {} received when calling update class endpoint.",
                res.status()
            )))),
        }
    }

    /// Add a reference to the array of cross-references of the given property in the source object
    /// specified by its class name and id.
    ///
    /// More on cross-references can be found [here](https://weaviate.io/developers/weaviate/config-refs/datatypes#datatype-cross-reference)
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn reference_add(
        &self,
        from_class_name: &str,
        from_uuid: &Uuid,
        from_property_name: &str,
        to_class_name: &str,
        to_uuid: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::json!({
            "beacon": format!("weaviate://localhost/{}/{}", to_class_name, to_uuid),
        });
        let mut endpoint: String = from_class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&from_uuid.to_string());
        endpoint.push_str("/references/");
        endpoint.push_str(&from_property_name.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }
        println!("{:?}", payload);
        let res = self.client.post(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    /// Update all references in a specified property of an object specified by its class name and 
    /// id.
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn reference_update(
        &self,
        from_class_name: &str,
        from_uuid: &Uuid,
        from_property_name: &str,
        to_class_names: Vec<&str>,
        to_uuids: Vec<&Uuid>,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut beacons = Vec::new();
        for (class_name, id) in to_class_names.iter().zip(to_uuids.iter()) {
                beacons.push(serde_json::json!({
                    "beacon": format!("weaviate://localhost/{}/{}", class_name, id)
                })
            );
        }

        let payload = serde_json::json!(beacons);
        let mut endpoint: String = from_class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&from_uuid.to_string());
        endpoint.push_str("/references/");
        endpoint.push_str(&from_property_name.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }
        println!("{:?}", payload);

        let res = self.client.put(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    /// Delete the single reference that is given in the body from the list of references that the
    /// specified property of a given object has, if it exists in the list. Will return true both
    /// when the reference existed, and when it didn't.
    ///
    /// # Example
    /// ```
    /// ```
    pub async fn reference_delete(
        &self,
        from_class_name: &str,
        from_uuid: &Uuid,
        from_property_name: &str,
        to_class_name: &str,
        to_uuid: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::json!({
            "beacon": format!("weaviate://localhost/{}/{}", to_class_name, to_uuid),
        });
        let mut endpoint: String = from_class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&from_uuid.to_string());
        endpoint.push_str("/references/");
        endpoint.push_str(&from_property_name.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }
        println!("{:?}", payload);
        let res = self.client.delete(endpoint).json(&payload).send().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{WeaviateClient, collections::{objects::{Object, ConsistencyLevel}, auth::AuthApiKey}};

    fn test_object(class_name: &str, id: Option<Uuid>) -> Object {
        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        Object {
            class: class_name.into(),
            properties,
            id,
            vector: None,
            tenant: None,
            creation_time_unix: None,
            last_update_time_unix: None,
            vector_weights: None,
        }
    }

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn mock_get(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
    ) -> mockito::Mock {
        server.mock("GET", endpoint)
            .with_status(status_code)
            .create()
    }

    #[tokio::test]
    async fn test_list_ok() {
        //let (mut mock_server, client) = get_test_harness();

        let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
        let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
        let object = test_object("TestListObject", Some(uuid.clone()));
        let res = client
            .objects
            .create(&object, Some(ConsistencyLevel::ALL))
            .await;

        println!("{:?}", res);

        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        let uuiid = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
        let res = client.objects.validate("TestListObject", properties, &uuiid).await;

        println!("{:?}", res);
        //assert_eq!(200, res.unwrap().status());

        let res = client
            .objects
            .list(
                Some("TestListObject"),
                Some(10),
                None,
                None,
                None,
                None,
                None,
                )
            .await;

        println!("{:?}", res);
        //assert_eq!(
        //    "TestListObject",
        //    res.unwrap().json::<serde_json::Value>().await.unwrap()["objects"][0]["class"]
        //);

        let res = client
            .objects
            .delete("TestListObject", &uuid, None, None)
            .await;


        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn test_list_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_create_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_create_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_get_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_get_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_exists_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_exists_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_update_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_update_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_replace_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_replace_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_delete_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_delete_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_validate_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_validate_err() {
        let (mut mock_server, client) = get_test_harness();
    }
}
