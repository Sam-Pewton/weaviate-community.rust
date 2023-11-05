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
    /// Using Class
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
