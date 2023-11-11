/// All objects associated type components
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Wrapper for multiple objects.
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiObjects {
    pub objects: Vec<Object>,
}

impl MultiObjects {
    /// Create a new MultiObjects object
    ///
    /// # Parameters
    /// - objects: the objects to encapsulate
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::{Object, MultiObjects};
    ///
    /// let object = Object::builder("Object", serde_json::json![{}]).build();
    /// let objects = MultiObjects::new(vec![object]);
    /// ```
    pub fn new(objects: Vec<Object>) -> MultiObjects {
        MultiObjects { objects }
    }
}

/// Object struct used for creating a new Object.
#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub class: String,
    pub properties: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tenant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creation_time_unix: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub last_update_time_unix: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector_weights: Option<u64>,
}

impl Object {
    /// Create a new builder for the Object.
    ///
    /// This is the same as `ObjectBuilder::new()`.
    ///
    /// # Parameters
    /// - class: the name of the class that the object belongs to.
    /// - properties: the properties associated with the new object.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::Object;
    ///
    /// let builder = Object::builder("Object", serde_json::json![{}]);
    /// ```
    pub fn builder(class: &str, properties: serde_json::Value) -> ObjectBuilder {
        ObjectBuilder::new(class, properties)
    }
}

/// The builder for an Object
///
/// Note that you should not adjust the creation_time_unix or the last_update_time_unix values.
pub struct ObjectBuilder {
    pub class: String,
    pub properties: serde_json::Value,
    pub id: Option<Uuid>,
    pub vector: Option<Vec<f64>>,
    pub tenant: Option<String>,
    pub creation_time_unix: Option<u64>,
    pub last_update_time_unix: Option<u64>,
    pub vector_weights: Option<u64>,
}

impl ObjectBuilder {
    /// Create a new builder for the Object.
    ///
    /// This is the same as `Object::builder()`.
    ///
    /// # Parameters
    /// - class: the name of the class that the object belongs to.
    /// - properties: the properties associated with the new object.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectBuilder;
    ///
    /// let builder = ObjectBuilder::new("Object", serde_json::json![{}]);
    /// ```
    pub fn new(class: &str, properties: serde_json::Value) -> ObjectBuilder {
        ObjectBuilder {
            class: class.into(),
            properties,
            id: None,
            vector: None,
            tenant: None,
            creation_time_unix: None,
            last_update_time_unix: None,
            vector_weights: None,
        }
    }

    /// Add a value to the optional `id` value of the object.
    ///
    /// # Parameters
    /// - id: the uuid to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectBuilder;
    /// use uuid::Uuid;
    ///
    /// let builder = ObjectBuilder::new("Object", serde_json::json![{}]).with_id(Uuid::new_v4());
    /// ```
    pub fn with_id(mut self, id: Uuid) -> ObjectBuilder {
        self.id = Some(id);
        self
    }

    /// Add a value to the optional `vector` value of the object.
    ///
    /// # Parameters
    /// - vector: the vector to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectBuilder;
    ///
    /// let builder = ObjectBuilder::new("Object", serde_json::json![{}])
    ///     .with_vector(vec![1.0, 1.0, 1.0]);
    /// ```
    pub fn with_vector(mut self, vector: Vec<f64>) -> ObjectBuilder {
        self.vector = Some(vector);
        self
    }

    /// Add a value to the optional `tenant` value of the object.
    ///
    /// # Parameters
    /// - tenant: the tenant to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectBuilder;
    ///
    /// let builder = ObjectBuilder::new("Object", serde_json::json![{}])
    ///     .with_tenant("TENANT_A");
    /// ```
    pub fn with_tenant(mut self, tenant: &str) -> ObjectBuilder {
        self.tenant = Some(tenant.into());
        self
    }

    /// Add a value to the optional `vector_weights` value of the object.
    ///
    /// # Parameters
    /// - vector_weights: the vector_weights to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectBuilder;
    ///
    /// let builder = ObjectBuilder::new("Object", serde_json::json![{}])
    ///     .with_vector_weights(10);
    /// ```
    pub fn with_vector_weights(mut self, vector_weights: u64) -> ObjectBuilder {
        self.vector_weights = Some(vector_weights);
        self
    }

    /// Build the Object from the ObjectBuilder
    ///
    /// # Example
    /// Using ObjectBuilder
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectBuilder;
    ///
    /// let object = ObjectBuilder::new("Object", serde_json::json![{}]).build();
    /// ```
    ///
    /// Using Object
    /// ```rust
    /// use weaviate_community::collections::objects::Object;
    ///
    /// let object = Object::builder("Object", serde_json::json![{}]).build();
    /// ```
    pub fn build(self) -> Object {
        Object {
            class: self.class,
            properties: self.properties,
            id: self.id,
            vector: self.vector,
            tenant: self.tenant,
            creation_time_unix: self.creation_time_unix,
            last_update_time_unix: self.last_update_time_unix,
            vector_weights: self.vector_weights,
        }
    }
}

/// Strict definitions for ordering queries.
///
/// The options available are ASC and DESC.
pub enum OrderBy {
    ASC,
    DESC,
}

impl OrderBy {
    /// Get the text value for a given OrderBy.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::OrderBy;
    ///
    /// let val = OrderBy::ASC.value();
    /// ```
    pub fn value(&self) -> &str {
        match self {
            OrderBy::ASC => "asc",
            OrderBy::DESC => "desc",
        }
    }
}

/// Strict definitions of consistency levels.
///
/// For more information on consistency levels in Weaviate, check out the replication architecture
/// documentation [here](https://weaviate.io/developers/weaviate/concepts/replication-architecture/consistency#tunable-read-consistency)
///
/// Tunable consistency strategies:
/// - QUORUM / QUORUM => balanced write and read latency
/// - ONE / ALL => fast write and slow read (optimized for write)
/// - ALL / ONE => slow write and fast read (optimized for read)
#[derive(Serialize, Deserialize, Debug)]
pub enum ConsistencyLevel {
    ONE,
    QUORUM,
    ALL,
}

impl ConsistencyLevel {
    /// Get the text value for a given ConsistencyLevel.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ConsistencyLevel;
    ///
    /// let val = ConsistencyLevel::ONE.value();
    /// ```
    pub fn value(&self) -> &str {
        match self {
            ConsistencyLevel::ONE => "ONE",
            ConsistencyLevel::QUORUM => "QUORUM",
            ConsistencyLevel::ALL => "ALL",
        }
    }
}

#[derive(Debug, Default)]
pub struct ObjectListParameters {
    pub class_name: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub after: Option<String>,
    pub include: Option<String>,
    pub sort: Option<Vec<String>>,
    pub order: Option<Vec<String>>,
}

impl ObjectListParameters {
    /// Create a new ObjectListParameters with all parameters set to None.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParameters;
    ///
    /// let builder = ObjectListParameters::new();
    /// ```
    pub fn new() -> ObjectListParameters {
        ObjectListParameters::default()
    }

    /// Create a new builder for the ObjectListParameters.
    ///
    /// This is the same as `ObjectListParametersBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParameters;
    ///
    /// let builder = ObjectListParameters::builder();
    /// ```
    pub fn builder() -> ObjectListParametersBuilder {
        ObjectListParametersBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ObjectListParametersBuilder {
    pub class_name: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub after: Option<String>,
    pub include: Option<String>,
    pub sort: Option<Vec<String>>,
    pub order: Option<Vec<String>>,
}

impl ObjectListParametersBuilder {
    /// Create a new builder for the ObjectListParameters.
    ///
    /// This is the same as `ObjectListParameters::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new();
    /// ```
    pub fn new() -> ObjectListParametersBuilder {
        ObjectListParametersBuilder::default()
    }

    /// Add a value to the optional `class_name` value to the parameters.
    ///
    /// # Parameters
    /// - class_name: the class_name to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new()
    ///     .with_class_name("Article");
    /// ```
    pub fn with_class_name(mut self, class_name: &str) -> ObjectListParametersBuilder {
        self.class_name = Some(class_name.into());
        self
    }

    /// Add a value to the optional `limit` value to the parameters.
    ///
    /// If not set, defaults to 25.
    ///
    /// # Parameters
    /// - limit: the limit value to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new()
    ///     .with_limit(25);
    /// ```
    pub fn with_limit(mut self, limit: u64) -> ObjectListParametersBuilder {
        self.limit = Some(limit);
        self
    }

    /// Add a value to the optional `offset` value to the parameters.
    ///
    /// Cannot be used with `after`.
    /// Should be used with `limit`.
    ///
    /// # Parameters
    /// - offset: the offset value to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new()
    ///     .with_offset(2);
    /// ```
    pub fn with_offset(mut self, offset: u64) -> ObjectListParametersBuilder {
        self.offset = Some(offset);
        self
    }

    /// Add a value to the optional `after` value to the parameters.
    ///
    /// Must be used in conjunction with `class`.
    /// Cannot be used with `offset` or `sort`.
    /// Should be used with `limit`.
    ///
    /// # Parameters
    /// - after: the after value to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new()
    ///     .with_after("dcfbe06f-fb69-48d7-9a13-e8e78e422486");
    /// ```
    pub fn with_after(mut self, after: &str) -> ObjectListParametersBuilder {
        self.after = Some(after.into());
        self
    }

    /// Add a value to the optional `include` value to the parameters.
    ///
    /// Allowed values include:
    /// - classification
    /// - vector
    /// - featureProjection
    /// and other module specific additional properties.
    ///
    /// # Parameters
    /// - include: the include value to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new().with_include("classification");
    /// ```
    pub fn with_include(mut self, include: &str) -> ObjectListParametersBuilder {
        self.include = Some(include.into());
        self
    }

    /// Add a value to the optional `sort` value to the parameters.
    ///
    /// # Parameters
    /// - sort: the sort value to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new().with_sort(vec!["title"]);
    /// ```
    pub fn with_sort(mut self, sort: Vec<&str>) -> ObjectListParametersBuilder {
        let sort = sort.iter().map(|field| field.to_string()).collect();
        self.sort = Some(sort);
        self
    }

    /// Add a value to the optional `order` value to the parameters.
    ///
    /// # Parameters
    /// - order: the order to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let builder = ObjectListParametersBuilder::new().with_order(vec!["asc"]);
    /// ```
    pub fn with_order(mut self, order: Vec<&str>) -> ObjectListParametersBuilder {
        let order = order.iter().map(|field| field.to_string()).collect();
        self.order = Some(order);
        self
    }

    /// Build the ObjectListParameters from the ObjectListParametersBuilder
    ///
    /// # Example
    /// Using ObjectListParametersBuilder
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParametersBuilder;
    ///
    /// let object = ObjectListParametersBuilder::new().build();
    /// ```
    ///
    /// Using ObjectListParameters
    /// ```rust
    /// use weaviate_community::collections::objects::ObjectListParameters;
    ///
    /// let object = ObjectListParameters::builder().build();
    /// ```
    pub fn build(self) -> ObjectListParameters {
        ObjectListParameters {
            class_name: self.class_name,
            limit: self.limit,
            offset: self.offset,
            after: self.after,
            include: self.include,
            sort: self.sort,
            order: self.order,
        }
    }
}

/// Wrapper for multiple references.
pub struct References(pub Vec<Reference>);

impl References {
    /// Create a new References wrapper.
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::{References, Reference};
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    /// let reference = Reference::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// );

    /// let references = References::new(
    ///    vec![reference]
    /// );
    /// ```
    pub fn new(references: Vec<Reference>) -> References {
        References(references)
    }
}

/// The Reference type for adding a reference between classes
pub struct Reference {
    pub from_class_name: String,
    pub from_uuid: Uuid,
    pub from_property_name: String,
    pub to_class_name: String,
    pub to_uuid: Uuid,
    pub consistency_level: Option<ConsistencyLevel>,
    pub tenant_name: Option<String>,
}

impl Reference {
    /// Create a new Reference with all optional parameters set to None.
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::Reference;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    /// let reference = Reference::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// );
    /// ```
    pub fn new(
        from_class_name: &str,
        from_uuid: &Uuid,
        from_property_name: &str,
        to_class_name: &str,
        to_uuid: &Uuid,
    ) -> Reference {
        Reference {
            from_class_name: from_class_name.into(),
            from_uuid: from_uuid.clone(),
            from_property_name: from_property_name.into(),
            to_class_name: to_class_name.into(),
            to_uuid: to_uuid.clone(),
            consistency_level: None,
            tenant_name: None,
        }
    }

    /// Create a new builder for the Reference.
    ///
    /// This is the same as `ReferenceBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::Reference;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    /// let reference = Reference::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// );
    /// ```
    pub fn builder(
        from_class_name: &str,
        from_uuid: &Uuid,
        from_property_name: &str,
        to_class_name: &str,
        to_uuid: &Uuid,
    ) -> ReferenceBuilder {
        ReferenceBuilder::new(
            from_class_name,
            from_uuid,
            from_property_name,
            to_class_name,
            to_uuid,
        )
    }
}

/// The builder for a Reference
pub struct ReferenceBuilder {
    pub from_class_name: String,
    pub from_uuid: Uuid,
    pub from_property_name: String,
    pub to_class_name: String,
    pub to_uuid: Uuid,
    pub consistency_level: Option<ConsistencyLevel>,
    pub tenant_name: Option<String>,
}

impl ReferenceBuilder {
    /// Create a new builder for the Reference.
    ///
    /// This is the same as `Reference::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::ReferenceBuilder;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    ///
    /// let reference = ReferenceBuilder::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// );
    /// ```
    pub fn new(
        from_class_name: &str,
        from_uuid: &Uuid,
        from_property_name: &str,
        to_class_name: &str,
        to_uuid: &Uuid,
    ) -> ReferenceBuilder {
        ReferenceBuilder {
            from_class_name: from_class_name.into(),
            from_uuid: from_uuid.clone(),
            from_property_name: from_property_name.into(),
            to_class_name: to_class_name.into(),
            to_uuid: to_uuid.clone(),
            consistency_level: None,
            tenant_name: None,
        }
    }

    /// Add a value to the optional `consistency_level` value to the reference.
    ///
    /// # Parameters
    /// - consistency_level: the consistency_level to set
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::ConsistencyLevel;
    /// use weaviate_community::collections::objects::ReferenceBuilder;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    /// let reference = ReferenceBuilder::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// ).with_consistency_level(ConsistencyLevel::ALL).build();
    /// ```
    pub fn with_consistency_level(
        mut self,
        consistency_level: ConsistencyLevel,
    ) -> ReferenceBuilder {
        self.consistency_level = Some(consistency_level);
        self
    }

    /// Add a value to the optional `tenant_name` value to the reference.
    ///
    /// # Parameters
    /// - tenant_name: the tenant_name to set
    ///
    /// # Example
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::ReferenceBuilder;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    /// let reference = ReferenceBuilder::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// ).with_tenant_name("TENANT_A").build();
    /// ```
    pub fn with_tenant_name(mut self, tenant_name: &str) -> ReferenceBuilder {
        self.tenant_name = Some(tenant_name.into());
        self
    }

    /// Build the Reference from the ReferenceBuilder
    ///
    /// # Example
    /// Using ReferenceBuilder
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::ReferenceBuilder;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    /// let reference = ReferenceBuilder::new(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// ).build();
    /// ```
    ///
    /// Using ObjectListParameters
    /// ```rust
    /// use uuid::Uuid;
    /// use weaviate_community::collections::objects::Reference;
    ///
    /// let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012").unwrap();
    /// let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef").unwrap();
    /// let reference = Reference::builder(
    ///     "JeopardyQuestion",
    ///     &uuid1,
    ///     "hasCategory",
    ///     "JeopardyCategory",
    ///     &uuid2,
    /// ).build();
    ///
    /// ```
    pub fn build(self) -> Reference {
        Reference {
            from_class_name: self.from_class_name,
            from_uuid: self.from_uuid,
            from_property_name: self.from_property_name,
            to_class_name: self.to_class_name,
            to_uuid: self.to_uuid,
            consistency_level: self.consistency_level,
            tenant_name: self.tenant_name,
        }
    }
}
