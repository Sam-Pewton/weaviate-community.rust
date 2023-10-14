/// All objects associated type components
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Object struct used for deserializing the json response to a usable format.
///
/// # Parameters
///
/// * class: the name of the class the object belongs to
/// * properties: the properties associated to the object
/// * id: the UUID of the object
/// * vector: the vector associated to the object
/// * tenant:
/// * creation_time_unix: the creation time of the object. If constructing new object, this should
///       be set to None, for Weaviate to automatically assign.
/// * last_update_time_unix: the timestamp of which the object was last updated. This should be set
///       to None when constructing a new object.
/// * vector_weights: TODO
/// * used as a response
///
/// # Examples
///
/// Constructing a new object
/// ```no_run
/// use weaviate_community::collections::objects::{
///     Object,
///     ConsistencyLevel,
/// };
/// use weaviate_community::WeaviateClient;
///
/// #[tokio::main]
/// async fn main() {
///     // Define a new object to add to Weaviate
///     let new_class = Object {
///         class: "MyNewClass".into(),
///         properties: serde_json::json!({}),
///         id: None,  // let Weaviate auto assign a new UUID
///         vector: None,
///         tenant: None,
///         creation_time_unix: None,  // let Weaviate auto update
///         last_update_time_unix: None,  // let Weaviate auto update
///         vector_weights: None,
///     };
///
///     // Insert the new object into the database
///     let client = WeaviateClient::new("http://localhost:8080", None).unwrap();
///     let res = client.objects.create(&new_class, Some(ConsistencyLevel::ALL)).await;
/// }
/// ```
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

/// Wrapper for multiple objects.
#[derive(Serialize, Deserialize, Debug)]
pub struct Objects {
    pub objects: Vec<Object>,
}

/// Strict definitions for ordering queries.
///
/// # Examples
///
/// ```
/// use weaviate_community::collections::objects::OrderBy;
///
/// let ob = OrderBy::ASC;
/// println!("{}", ob.value());  // prints `asc`
/// ```
pub enum OrderBy {
    ASC,
    DESC,
}

impl OrderBy {
    /// Get the text value for a given OrderBy.
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
/// documentation (here)[https://weaviate.io/developers/weaviate/concepts/replication-architecture/consistency#tunable-read-consistency]
///
/// Tunable consistency strategies:
/// - QUORUM / QUORUM => balanced write and read latency
/// - ONE / ALL => fast write and slow read (optimized for write)
/// - ALL / ONE => slow write and fast read (optimized for read)
///
/// # Examples
///
/// ```
/// use weaviate_community::collections::objects::ConsistencyLevel;
///
/// let cl = ConsistencyLevel::ALL;
/// println!("{}", cl.value());  // prints `ALL`
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub enum ConsistencyLevel {
    ONE,
    QUORUM,
    ALL,
}

impl ConsistencyLevel {
    /// Get the text value for a given ConsistencyLevel.
    pub fn value(&self) -> &str {
        match self {
            ConsistencyLevel::ONE => "ONE",
            ConsistencyLevel::QUORUM => "QUORUM",
            ConsistencyLevel::ALL => "ALL",
        }
    }
}
