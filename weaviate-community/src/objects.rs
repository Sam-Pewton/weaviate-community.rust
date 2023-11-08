use crate::collections::error::QueryError;
use crate::collections::objects::{ConsistencyLevel, Object, MultiObjects, ObjectListParameters, Reference};
use reqwest::Url;
use std::{error::Error, sync::Arc};
use uuid::Uuid;

/// All objects endpoints and functionality described in
/// [Weaviate objects API documentation](https://weaviate.io/developers/weaviate/api/rest/objects)
#[derive(Debug)]
pub struct Objects {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Objects {
    /// Create a new Objects endpoint orchestrator for the client.
    ///
    /// Should not be done manually.
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/objects/")?;
        Ok(Objects { endpoint, client })
    }

    /// List the data objects.
    ///
    /// # Parameters
    /// - parameters: the ObjectListParameters to use in the request.
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::WeaviateClient;
    /// use weaviate_community::collections::objects::ObjectListParameters;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///
    ///     //let params = ObjectListParameters::builder().with_class_name("MyClass").build();
    ///     let params = ObjectListParameters::new();
    ///     let res = client.objects.list(params).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn list(
        &self,
        parameters: ObjectListParameters
    ) -> Result<MultiObjects, Box<dyn Error>> {
        let mut endpoint = self.endpoint.clone();

        // Add the query params when they are present
        if let Some(c) = &parameters.class_name {
            endpoint.query_pairs_mut().append_pair("class", &c);
        }
        if let Some(l) = &parameters.limit {
            endpoint
                .query_pairs_mut()
                .append_pair("limit", &l.to_string());
        }
        if let Some(o) = &parameters.offset {
            endpoint
                .query_pairs_mut()
                .append_pair("offset", &o.to_string());
            // Raise an err if after is some
            if parameters.after.is_some() {
                return Err(
                    Box::new(
                        QueryError(
                            "'after' must be None when 'offset' is Some".into(),
                        )
                    )
                );
            }
        }
        if let Some(a) = &parameters.after {
            endpoint.query_pairs_mut().append_pair("after", &a);
            if parameters.after.is_none() {
                return Err(
                    Box::new(
                        QueryError(
                            "'class' must be Some when 'after' is Some".into(),
                        )
                    )
                );
            }
            // raise an error if offset or sort are some
            if parameters.offset.is_some() {
                return Err(
                    Box::new(
                        QueryError(
                            "'offset' must be None when 'after' is Some".into(),
                        )
                    )
                );
            }
            if parameters.sort.is_some() {
                return Err(
                    Box::new(
                        QueryError(
                            "'sort' must be None when 'after' is Some".into(),
                        )
                    )
                );
            }
        }
        if let Some(i) = parameters.include {
            endpoint.query_pairs_mut().append_pair("include", &i);
        }
        if let Some(s) = parameters.sort {
            let values = s.join(",");
            endpoint.query_pairs_mut().append_pair("sort", &values);
        }
        if let Some(o) = parameters.order {
            let values = o.join(",");
            endpoint.query_pairs_mut().append_pair("order", &values);
        }

        let res = self.client.get(endpoint).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: MultiObjects = res.json().await?;
                Ok(res)
            }
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling list objects endpoint.",
                        res.status()
                        )
                    )
                )
            ),
        }
    }

    /// Create a new data object. The provided meta-data and schema values are validated.
    ///
    /// When inserting a large number of objects, it is more efficient to use the `batch` insert
    /// methods.
    ///
    /// # Parameters
    /// - new_object: the new object to create
    /// - consistency_level: the consistency_level of the new object
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
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling create object endpoint.",
                        res.status()
                    ))
                )
            ),
        }
    }

    /// Collect an individual data object given it's UUID.
    ///
    /// # Parameters
    /// - class_name: the name of the class that the object belongs to
    /// - id: the uuid of the object
    /// - include: extra fields to include (classification, vector)
    /// - consistency_level: the consistency_level of the object
    /// - tenant_key: the tenant that the object is associated with
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
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling get object endpoint.",
                        res.status()
                    ))
                )
            ),
        }
    }

    /// Check if a data object exists without returning the object itself.
    ///
    /// This works the same as the `get` method, but uses `HEAD` HTTP method.
    ///
    /// # Parameters
    /// - class_name: the class name of the object to check for
    /// - id: the uuid of the object
    /// - consistency_level: the consistency_level of the object
    /// - tenant_name: the name of the tenant the object is associated with
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
            _ => Err(
                Box::new(
                    QueryError(
                        format!(
                            "status code {} received when calling exists (object) endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    /// Updates the given property values of the data object.
    ///
    /// To replace all property values, use the `replace` method.
    ///
    /// Note that if the class is configured with a vectorizer, Weaviate will only compute a new
    /// vector for an updated object if the update changes the underlying text to be vectorized.
    ///
    /// # Parameters
    /// - properties: the properties to update the object with
    /// - class_name: the name of the class the object belongs to
    /// - id: the uuid of the object
    /// - consistency_level: the consistency_level of the object
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
            _ => Err(
                Box::new(
                    QueryError(
                        format!(
                            "status code {} received when calling update object endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    /// Replaces all property values of the data object.
    ///
    /// Use the `update` method if only modifying some properties.
    ///
    /// Note that if the class is configured with a vectorizer, Weaviate will only compute a new
    /// vector for an updated object if the update changes the underlying text to be vectorized.
    ///
    /// # Parameters
    /// - properties: the properties to replace with
    /// - class_name: the name of the class the object belongs to
    /// - id: the uuid of the object to replace
    /// - consistency_level: the consistency_level of the object
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
            _ => Err(
                Box::new(
                    QueryError(
                        format!(
                            "status code {} received when calling update class endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    /// Delete an individual data object from Weaviate.
    ///
    /// # Parameters
    /// - class_name: the name of the class the object belongs to
    /// - id: the uuid of the object to delete
    /// - consistency_level: the consistency_level of the object
    /// - tenant_name: the name of the tenant the object is associated to
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
            _ => Err(
                Box::new(
                    QueryError(
                        format!(
                            "status code {} received when calling delete object endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    /// Validate an object's schema and metadata without creating it.
    ///
    /// # Parameters
    /// - class_name: the name of the class you want to validate against
    /// - properties: the properties you want to validate
    /// - id: the uuid you want to set the new object
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
    ///     let res = client.objects.validate("Publication", &properties, &uuid).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn validate(
        &self,
        class_name: &str,
        properties: &serde_json::Value,
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
            _ => Err(
                Box::new(
                    QueryError(
                        format!(
                            "status code {} received when calling validate object endpoint.",
                            res.status()
                        )
                    )
                )
            ),
        }
    }

    /// Add a reference to the array of cross-references of the given property in the source object
    /// specified by its class name and id.
    ///
    /// More on cross-references can be found [here](https://weaviate.io/developers/weaviate/config-refs/datatypes#datatype-cross-reference)
    ///
    /// # Parameters
    /// - from_class_name: the class to add the beacon to
    /// - from_uuid: the uuid of the object to add the beacon to
    /// - from_property_name: the name of the property the beacon should be added to
    /// - to_class_name: the name of the class to beacon to
    /// - to_uuid: the uuid of the object you want to create a beacon to
    /// - consistency_level: the consistency level to set
    /// - tenant_name: the name of the tenant the `from_uuid` belongs to
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    ///     let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    ///     let res = client.objects.reference_add(
    ///         "JeopardyQuestion", 
    ///         &uuid1,
    ///         "hasCategory", 
    ///         "JeopardyCategory",
    ///         &uuid2,
    ///         None,
    ///         None
    ///     ).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn reference_add(
        &self,
        reference: Reference,
    ) -> Result<bool, Box<dyn Error>> {
        let payload = serde_json::json!({
            "beacon": format!("weaviate://localhost/{}/{}", reference.to_class_name, reference.to_uuid),
        });
        let mut endpoint: String = reference.from_class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&reference.from_uuid.to_string());
        endpoint.push_str("/references/");
        endpoint.push_str(&reference.from_property_name.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = reference.consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = reference.tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", &t);
        }

        let res = self.client.post(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(true)
            }
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling create object reference endpoint.",
                        res.status()
                    ))
                )
            ),
        }
    }

    /// Update all references in a specified property of an object specified by its class name and 
    /// id.
    ///
    /// Requires the same length of to_class_names as to_uuids as input.
    ///
    /// # Parameters
    /// - from_class_name: the class that has the beacons
    /// - from_uuid: the uuid of the object to update the beacons of
    /// - from_property_name: the name of the property containing the beacons
    /// - to_class_names: the names of the classes to beacon to
    /// - to_uuids: the uuids of the objects you want to update the beacons to
    /// - consistency_level: the consistency level to set
    /// - tenant_name: the name of the tenant the `from_uuid` belongs to
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    ///     let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    ///     let res = client.objects.reference_update(
    ///         "JeopardyQuestion", 
    ///         &uuid1,
    ///         "hasCategory", 
    ///         vec!["JeopardyCategory"],
    ///         vec![&uuid2],
    ///         None,
    ///         None
    ///     ).await;
    ///
    ///     Ok(())
    /// }
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
    ) -> Result<Object, Box<dyn Error>> {

        if to_class_names.len() != to_uuids.len() {
            return Err(Box::new(QueryError(
                "to_class_names.len() must equal to_uuids.len().".into()
            )))
        }

        // Match the class names to the id's in the beacon format
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

        let res = self.client.put(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: Object = res.json().await?;
                Ok(res)
            }
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling update object reference endpoint.",
                        res.status()
                    ))
                )
            ),
        }
    }

    /// Delete the single reference that is given in the body from the list of references that the
    /// specified property of a given object has, if it exists in the list. Will return true both
    /// when the reference existed, and when it didn't.
    ///
    /// # Parameters
    /// - from_class_name: the class that has the beacons
    /// - from_uuid: the uuid of the object to update the beacons of
    /// - from_property_name: the name of the property containing the beacons
    /// - to_class_name: the names of the class to remove beacon to
    /// - to_uuid: the uuid of the object you want to remove the beacon to
    /// - consistency_level: the consistency level to set
    /// - tenant_name: the name of the tenant the `from_uuid` belongs to
    ///
    /// # Example
    /// ```
    /// use uuid::Uuid;
    /// use weaviate_community::WeaviateClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
    ///     let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    ///     let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    ///     let res = client.objects.reference_delete(
    ///         "JeopardyQuestion", 
    ///         &uuid1,
    ///         "hasCategory", 
    ///         "JeopardyCategory",
    ///         &uuid2,
    ///         None,
    ///         None
    ///     ).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn reference_delete(
        &self,
        reference: Reference
    ) -> Result<bool, Box<dyn Error>> {
        let payload = serde_json::json!({
            "beacon": format!("weaviate://localhost/{}/{}", reference.to_class_name, reference.to_uuid),
        });
        let mut endpoint: String = reference.from_class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&reference.from_uuid.to_string());
        endpoint.push_str("/references/");
        endpoint.push_str(&reference.from_property_name.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = reference.consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = reference.tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", &t);
        }

        let res = self.client.delete(endpoint).json(&payload).send().await?;
        match res.status() {
            reqwest::StatusCode::NO_CONTENT => {
                Ok(true)
            }
            _ => Err(
                Box::new(
                    QueryError(format!(
                        "status code {} received when calling delete class reference endpoint.",
                        res.status()
                    ))
                )
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{
        WeaviateClient, 
        collections::objects::{Object, ObjectListParameters, MultiObjects, Reference}
    };

    fn test_object(class_name: &str) -> Object {
        let properties = serde_json::json!({
            "name": "test",
            "number": 123,
        });
        Object::builder(class_name, properties).build()
    }

    fn test_objects(class_name: &str) -> MultiObjects {
        MultiObjects::new(vec![test_object(class_name), test_object(class_name)])
    }

    fn test_reference(uuid: &Uuid, uuid_2: &Uuid) -> Reference {
        Reference::new(
            "Test",
            uuid,
            "testProperty",
            "TestTwo",
            uuid_2,
        )
    }

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn mock_post(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("POST", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    fn mock_put(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("PUT", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    fn mock_patch(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("PATCH", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    fn mock_head(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("HEAD", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    fn mock_get(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
        body: &str
    ) -> mockito::Mock {
        server.mock("GET", endpoint)
            .with_status(status_code)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    fn mock_delete(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
    ) -> mockito::Mock {
        server.mock("DELETE", endpoint)
            .with_status(status_code)
            .create()
    }

    #[tokio::test]
    async fn test_list_ok() {
        let (mut mock_server, client) = get_test_harness();
        let objects = test_objects("Test");
        let objects_str = serde_json::to_string(&objects).unwrap();
        let mock = mock_get(&mut mock_server, "/v1/objects/", 200, &objects_str);
        let res = client.objects.list(ObjectListParameters::new()).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(objects.objects[0].class, res.unwrap().objects[0].class);
    }

    #[tokio::test]
    async fn test_list_err() {
        let (mut mock_server, client) = get_test_harness();
        let mock = mock_get(&mut mock_server, "/v1/objects/", 422, "");
        let res = client.objects.list(ObjectListParameters::new()).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_create_ok() {
        let (mut mock_server, client) = get_test_harness();
        let object = test_object("Test");
        let object_str = serde_json::to_string(&object).unwrap();
        let mock = mock_post(&mut mock_server, "/v1/objects/", 200, &object_str);
        let res = client.objects.create(&object, None).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(object.class, res.unwrap().class);
    }

    #[tokio::test]
    async fn test_create_err() {
        let (mut mock_server, client) = get_test_harness();
        let object = test_object("Test");
        let mock = mock_post(&mut mock_server, "/v1/objects/", 422, "");
        let res = client.objects.create(&object, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_get_ok() {
        let (mut mock_server, client) = get_test_harness();
        let object = test_object("Test");
        let object_str = serde_json::to_string(&object).unwrap();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_get(&mut mock_server, &url, 200, &object_str);
        let res = client.objects.get("Test", &uuid, None, None, None).await;
        mock.assert();
        assert!(res.is_ok());
        assert_eq!(object.class, res.unwrap().class);
    }

    #[tokio::test]
    async fn test_get_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_get(&mut mock_server, &url, 422, "");
        let res = client.objects.get("Test", &uuid, None, None, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_exists_ok() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_head(&mut mock_server, &url, 204, "");
        let res = client.objects.exists("Test", &uuid, None, None).await;
        mock.assert();
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn test_exists_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_head(&mut mock_server, &url, 422, "");
        let res = client.objects.exists("Test", &uuid, None, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_update_ok() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_patch(&mut mock_server, &url, 204, "");
        let res = client.objects.update(&serde_json::json![{}], "Test", &uuid, None).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_update_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_patch(&mut mock_server, &url, 422, "");
        let res = client.objects.update(&serde_json::json![{}], "Test", &uuid, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_replace_ok() {
        let (mut mock_server, client) = get_test_harness();
        let object = test_object("Test");
        let object_str = serde_json::to_string(&object).unwrap();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_put(&mut mock_server, &url, 200, &object_str);
        let res = client.objects.replace(&serde_json::json![{}], "Test", &uuid, None).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_replace_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_put(&mut mock_server, &url, 422, "");
        let res = client.objects.replace(&serde_json::json![{}], "Test", &uuid, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_delete_ok() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_delete(&mut mock_server, &url, 204);
        let res = client.objects.delete("Test", &uuid, None, None).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_delete_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        let mock = mock_delete(&mut mock_server, &url, 404);
        let res = client.objects.delete("Test", &uuid, None, None).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_validate_ok() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mock = mock_post(&mut mock_server, "/v1/objects/validate", 200, "");
        let res = client.objects.validate("Test", &serde_json::json![{}], &uuid).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_validate_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let mock = mock_post(&mut mock_server, "/v1/objects/validate", 404, "");
        let res = client.objects.validate("Test", &serde_json::json![{}], &uuid).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_reference_add_ok() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        url.push_str("/references/testProperty");
        let mock = mock_post(&mut mock_server, &url, 200, "");
        let res = client.objects.reference_add(
            test_reference(&uuid, &uuid_2)
        ).await;
        mock.assert();
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn test_reference_add_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        url.push_str("/references/testProperty");
        let mock = mock_post(&mut mock_server, &url, 404, "");
        let res = client.objects.reference_add(
            test_reference(&uuid, &uuid_2)
        ).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_reference_update_ok() {
        let (mut mock_server, client) = get_test_harness();
        let object = test_object("Test");
        let object_str = serde_json::to_string(&object).unwrap();
        let uuid = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        url.push_str("/references/testProperty");
        let mock = mock_put(&mut mock_server, &url, 200, &object_str);
        let res = client.objects.reference_update(
            "Test",
            &uuid,
            "testProperty",
            vec!["TestTwo"],
            vec![&uuid_2],
            None,
            None,
        ).await;
        mock.assert();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_reference_update_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        url.push_str("/references/testProperty");
        let mock = mock_put(&mut mock_server, &url, 404, "");
        let res = client.objects.reference_update(
            "Test",
            &uuid,
            "testProperty",
            vec!["TestTwo"],
            vec![&uuid_2],
            None,
            None,
        ).await;
        mock.assert();
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_reference_delete_ok() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        url.push_str("/references/testProperty");
        let mock = mock_delete(&mut mock_server, &url, 204);
        let res = client.objects.reference_delete(
            test_reference(&uuid, &uuid_2)
        ).await;
        mock.assert();
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn test_reference_delete_err() {
        let (mut mock_server, client) = get_test_harness();
        let uuid = Uuid::new_v4();
        let uuid_2 = Uuid::new_v4();
        let mut url = String::from("/v1/objects/Test/");
        url.push_str(&uuid.to_string());
        url.push_str("/references/testProperty");
        let mock = mock_delete(&mut mock_server, &url, 404);
        let res = client.objects.reference_delete(
            test_reference(&uuid, &uuid_2)
        ).await;
        mock.assert();
        assert!(res.is_err());
    }

}
