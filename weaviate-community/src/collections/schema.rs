/// All schema associated type components
/// https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage for multiple classes.
#[derive(Serialize, Deserialize, Debug)]
pub struct Classes {
    pub classes: Vec<Class>,
}

impl Classes {
    /// Create a new Classes object
    ///
    /// # Parameters
    /// - classes: the classes to encapsulate
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{Class, Classes};
    ///
    /// let classes = Classes::new(
    ///     vec![
    ///         Class::builder("Article", "Class for storing article data").build(),
    ///         Class::builder("Journal", "Class for storing journal data").build()
    ///     ]
    /// );
    /// ```
    pub fn new(classes: Vec<Class>) -> Classes {
        Classes {
            classes
        }
    }
}

/// Full class definition and configuration options.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub class: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub properties: Option<Properties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_vector_index_type")]
    pub vector_index_type: Option<VectorIndexType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector_index_config: Option<VectorIndexConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vectorizer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub module_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inverted_index_config: Option<InvertedIndexConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharding_config: Option<ShardingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multi_tenancy_config: Option<MultiTenancyConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub replication_config: Option<ReplicationConfig>,
}

impl Class {
    /// Create a new builder for the class object.
    ///
    /// This is the same as `ClassBuilder::new()`.
    ///
    /// # Parameters
    /// - class_name: the name of the class
    /// - description: the description of the class
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::Class;
    ///
    /// let builder = Class::builder("Article", "Class for storing article data");
    /// ```
    pub fn builder(class_name: &str, description: &str) -> ClassBuilder {
        ClassBuilder::new(class_name, description)
    }
}

/// ClassBuilder for building new classes
#[derive(Default)]
pub struct ClassBuilder {
    pub class: String,
    pub description: String,
    pub properties: Option<Properties>,
    pub vector_index_type: Option<VectorIndexType>,
    pub vector_index_config: Option<VectorIndexConfig>,
    pub vectorizer: Option<String>,
    pub module_config: Option<String>,
    pub inverted_index_config: Option<InvertedIndexConfig>,
    pub sharding_config: Option<ShardingConfig>,
    pub multi_tenancy_config: Option<MultiTenancyConfig>,
    pub replication_config: Option<ReplicationConfig>,
}

impl ClassBuilder {
    /// Create a new builder for the class object.
    ///
    /// This is the same as `Class::builder()`.
    ///
    /// # Parameters
    /// - class_name: the name of the class
    /// - description: the description of the class
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ClassBuilder;
    ///
    /// let builder = ClassBuilder::new("Article", "Class for storing article data");
    /// ```
    pub fn new(class: &str, description: &str) -> ClassBuilder {
        ClassBuilder {
            class: class.into(),
            description: description.into(),
            properties: None,
            vector_index_type: None,
            vector_index_config: None,
            vectorizer: None,
            module_config: None,
            inverted_index_config: None,
            sharding_config: None,
            multi_tenancy_config: None,
            replication_config: None,
        }
    }

    /// Add a value to the optional `properties` value of the class.
    ///
    /// # Parameters
    /// - properties: the properties to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{
    ///     ClassBuilder,
    ///     Properties,
    ///     Property
    /// };
    ///
    /// let properties = Properties(vec![Property::builder("title", vec!["text"]).build()]);
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_properties(properties);
    /// ```
    pub fn with_properties(mut self, properties: Properties) -> ClassBuilder {
        self.properties = Some(properties);
        self
    }

    /// Add a value to the optional `vector_index_type` value of the class.
    ///
    /// The vector_index_type defaults to `hnsw` if unset.
    ///
    /// # Parameters
    /// - vector_index_type: the vector_index_type to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{
    ///     ClassBuilder,
    ///     VectorIndexType
    /// };
    ///
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_vector_index_type(VectorIndexType::HNSW);
    /// ```
    pub fn with_vector_index_type(mut self, vector_index_type: VectorIndexType) -> ClassBuilder {
        self.vector_index_type = Some(vector_index_type);
        self
    }

    /// Add a value to the optional `vector_index_config` value of the class.
    ///
    /// # Parameters
    /// - vector_index_config: the vector_index_config to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{
    ///     ClassBuilder,
    ///     VectorIndexConfig
    /// };
    ///
    /// let config = VectorIndexConfig::builder().build();
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_vector_index_config(config);
    /// ```
    pub fn with_vector_index_config(
        mut self,
        vector_index_config: VectorIndexConfig
    ) -> ClassBuilder {
        self.vector_index_config = Some(vector_index_config);
        self
    }

    /// Add a value to the optional `vectorizer` value of the class.
    ///
    /// Defaults to the vectorizer set in the database configuration if unset.
    ///
    /// # Parameters
    /// - vectorizer: the vectorizer to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ClassBuilder;
    ///
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_vectorizer("none");
    /// ```
    pub fn with_vectorizer(mut self, vectorizer: &str) -> ClassBuilder {
        self.vectorizer = Some(vectorizer.into());
        self
    }

    /// Add a value to the optional `module_config` value of the class.
    ///
    /// This parameter needs re-evaluating
    ///
    /// # Parameters
    /// - module_config: the module_config to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ClassBuilder;
    ///
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_module_config("");
    /// ```
    pub fn with_module_config(mut self, module_config: &str) -> ClassBuilder {
        self.module_config = Some(module_config.into());
        self
    }

    /// Add a value to the optional `inverted_index_config` value of the class.
    ///
    /// # Parameters
    /// - inverted_index_config: the inverted_index_config to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ClassBuilder, InvertedIndexConfig};
    ///
    /// let config = InvertedIndexConfig::builder().build();
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_inverted_index_config(config);
    /// ```
    pub fn with_inverted_index_config(
        mut self,
        inverted_index_config: InvertedIndexConfig,
    ) -> ClassBuilder {
        self.inverted_index_config = Some(inverted_index_config);
        self
    }

    /// Add a value to the optional `sharding_config` value of the class.
    ///
    /// # Parameters
    /// - sharding_config: the sharding_config to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ClassBuilder, ShardingConfig};
    ///
    /// let config = ShardingConfig::builder().build();
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_sharding_config(config);
    /// ```
    pub fn with_sharding_config(mut self, sharding_config: ShardingConfig) -> ClassBuilder {
        self.sharding_config = Some(sharding_config);
        self
    }

    /// Add a value to the optional `multi_tenancy_config` value of the class.
    ///
    /// # Parameters
    /// - multi_tenancy_config: the multi_tenancy_config to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ClassBuilder, MultiTenancyConfig};
    ///
    /// let config = MultiTenancyConfig::new(true);
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_multi_tenancy_config(config);
    /// ```
    pub fn with_multi_tenancy_config(
        mut self,
        multi_tenancy_config: MultiTenancyConfig,
    ) -> ClassBuilder {
        self.multi_tenancy_config = Some(multi_tenancy_config);
        self
    }

    /// Add a value to the optional `replication_config` value of the class.
    ///
    /// # Parameters
    /// - replication_config: the replication_config to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ClassBuilder, ReplicationConfig};
    ///
    /// let config = ReplicationConfig::new(3);
    /// let builder = ClassBuilder::new("Article", "Class for storing article data")
    ///     .with_replication_config(config);
    /// ```
    pub fn with_replication_config(
        mut self,
        replication_config: ReplicationConfig
    ) -> ClassBuilder {
        self.replication_config = Some(replication_config);
        self
    }

    /// Build the Class from the ClassBuilder
    ///
    /// # Example
    /// Using ClassBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::ClassBuilder;
    ///
    /// let class = ClassBuilder::new("Article", "Class for storing article data").build();
    /// ```
    ///
    /// Using Class
    /// ```rust
    /// use weaviate_community::collections::schema::Class;
    ///
    /// let class = Class::builder("Article", "Class for storing article data").build();
    /// ```
    pub fn build(self) -> Class {
        Class {
            class: self.class,
            description: self.description,
            properties: self.properties,
            vector_index_type: self.vector_index_type,
            vector_index_config: self.vector_index_config,
            vectorizer: self.vectorizer,
            module_config: self.module_config,
            inverted_index_config: self.inverted_index_config,
            sharding_config: self.sharding_config,
            multi_tenancy_config: self.multi_tenancy_config,
            replication_config: self.replication_config,
        }
    }
}

/// Strict definitions of Vector Index types.
///
/// Currently Weaviate only supports HNSW.
#[derive(Serialize, Deserialize, Debug)]
pub enum VectorIndexType {
    #[serde(rename = "hnsw")]
    HNSW,
}

/// Controls default for Class vector_index_type
fn default_vector_index_type() -> Option<VectorIndexType> {
    Some(VectorIndexType::HNSW)
}

/// Wrapper for multiple properties
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Properties(pub Vec<Property>);

/// Configuration options for a property
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub data_type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tokenization: Option<Tokenization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub module_config: Option<HashMap<String, HashMap<String, bool>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_filterable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_searchable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inverted_index_config: Option<InvertedIndexConfig>,
}

impl Property {
    /// Create a new builder for the property object.
    ///
    /// This is the same as `PropertyBuilder::new()`.
    ///
    /// When creating cross-references, a property can have multiple data types.
    ///
    /// # Parameters
    /// - name: the name of the property
    /// - data_type: the data type of the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::Property;
    ///
    /// let builder = Property::builder("title", vec!["text"]);
    /// ```
    pub fn builder(name: &str, data_type: Vec<&str>) -> PropertyBuilder {
        PropertyBuilder::new(name, data_type)
    }
}

/// PropertyBuilder for building new properties
#[derive(Default)]
pub struct PropertyBuilder {
    pub name: String,
    pub data_type: Vec<String>,
    pub description: Option<String>,
    pub tokenization: Option<Tokenization>,
    pub module_config: Option<HashMap<String, HashMap<String, bool>>>,
    pub index_filterable: Option<bool>,
    pub index_searchable: Option<bool>,
    pub inverted_index_config: Option<InvertedIndexConfig>,
}

impl PropertyBuilder {
    /// Create a new builder for the property object.
    ///
    /// This is the same as `Property::builder()`.
    ///
    /// When creating cross-references, a property can have multiple data types.
    ///
    /// # Parameters
    /// - name: the name of the property
    /// - data_type: the data type of the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PropertyBuilder;
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"]);
    /// ```
    pub fn new(name: &str, data_type: Vec<&str>) -> PropertyBuilder {
        let data_type = data_type.iter().map(|field| field.to_string()).collect();
        PropertyBuilder {
            name: name.into(),
            data_type,
            description: None,
            tokenization: None,
            module_config: None,
            index_filterable: None,
            index_searchable: None,
            inverted_index_config: None,
        }
    }

    /// Add a value to the optional `description` value of the property.
    ///
    /// # Parameters
    /// - description: the description of the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PropertyBuilder;
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"])
    ///     .with_description("The title of the article");
    /// ```
    pub fn with_description(mut self, description: &str) -> PropertyBuilder {
        self.description = Some(description.into());
        self
    }

    /// Add a value to the optional `tokenization` value of the property.
    ///
    /// # Parameters
    /// - tokenization: the tokenization to use for the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{PropertyBuilder, Tokenization};
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"])
    ///     .with_tokenization(Tokenization::WORD);
    /// ```
    pub fn with_tokenization(mut self, tokenization: Tokenization) -> PropertyBuilder {
        self.tokenization = Some(tokenization);
        self
    }

    /// Add a value to the optional `module_config` value of the property.
    ///
    /// This needs to be revisited.
    ///
    /// # Parameters
    /// - module_config: the module_config to use for the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PropertyBuilder;
    /// use std::collections::HashMap;
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"]);
    /// ```
    pub fn with_module_config(
        mut self,
        module_config: HashMap<String, HashMap<String, bool>>,
    ) -> PropertyBuilder {
        self.module_config = Some(module_config);
        self
    }

    /// Add a value to the optional `index_filterable` value of the property.
    ///
    /// # Parameters
    /// - index_filterable: the index_filterable to use for the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PropertyBuilder;
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"])
    ///     .with_index_filterable(true);
    /// ```
    pub fn with_index_filterable(mut self, index_filterable: bool) -> PropertyBuilder {
        self.index_filterable = Some(index_filterable);
        self
    }

    /// Add a value to the optional `index_searchable` value of the property.
    ///
    /// # Parameters
    /// - index_searchable: the index_searchable to use for the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PropertyBuilder;
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"])
    ///     .with_index_searchable(true);
    /// ```
    pub fn with_index_searchable(mut self, index_searchable: bool) -> PropertyBuilder {
        self.index_searchable = Some(index_searchable);
        self
    }

    /// Add a value to the optional `inverted_index_config` value of the property.
    ///
    /// # Parameters
    /// - inverted_index_config: the inverted_index_config to use for the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{PropertyBuilder, InvertedIndexConfig};
    ///
    /// let config = InvertedIndexConfig::builder().build();
    /// let builder = PropertyBuilder::new("title", vec!["text"])
    ///     .with_inverted_index_config(config);
    /// ```
    pub fn with_inverted_index_config(
        mut self,
        inverted_index_config: InvertedIndexConfig,
    ) -> PropertyBuilder {
        self.inverted_index_config = Some(inverted_index_config);
        self
    }

    /// Build the Property from the PropertyBuilder
    ///
    /// # Example
    /// Using PropertyBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::PropertyBuilder;
    ///
    /// let builder = PropertyBuilder::new("title", vec!["text"]).build();
    /// ```
    ///
    /// Using Property
    /// ```rust
    /// use weaviate_community::collections::schema::Property;
    ///
    /// let builder = Property::builder("title", vec!["text"]).build();
    /// ```
    pub fn build(self) -> Property {
        Property {
            name: self.name,
            data_type: self.data_type,
            description: self.description,
            tokenization: self.tokenization,
            module_config: self.module_config,
            index_filterable: self.index_filterable,
            index_searchable: self.index_searchable,
            inverted_index_config: self.inverted_index_config,
        }
    }
}

/// Configuration options for VectorIndexConfig
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VectorIndexConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub distance: Option<DistanceMetric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ef: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ef_construction: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_connections: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dynamic_ef_min: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dynamic_ef_max: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dynamic_ef_factor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub vector_cache_max_objects: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub flat_search_cut_off: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cleanup_interval_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pq: Option<PqConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub skip: Option<bool>,
}

impl VectorIndexConfig {
    /// Create a new builder for the VectorIndexConfig object.
    ///
    /// This is the same as `VectorIndexConfigBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfig;
    ///
    /// let builder = VectorIndexConfig::builder();
    /// ```
    pub fn builder() -> VectorIndexConfigBuilder {
        VectorIndexConfigBuilder::default()
    }
}

/// VectorIndexConfigBuilder for building a new VectorIndexConfig
#[derive(Default)]
pub struct VectorIndexConfigBuilder {
    pub distance: Option<DistanceMetric>,
    pub ef: Option<i64>,
    pub ef_construction: Option<u64>,
    pub max_connections: Option<u64>,
    pub dynamic_ef_min: Option<i64>,
    pub dynamic_ef_max: Option<i64>,
    pub dynamic_ef_factor: Option<i64>,
    pub vector_cache_max_objects: Option<u64>,
    pub flat_search_cut_off: Option<u64>,
    pub cleanup_interval_seconds: Option<u64>,
    pub pq: Option<PqConfig>,
    pub skip: Option<bool>,
}

impl VectorIndexConfigBuilder {
    /// Create a new builder for the VectorIndexConfig object.
    ///
    /// This is the same as `VectorIndexConfig::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new();
    /// ```
    pub fn new() -> VectorIndexConfigBuilder {
        VectorIndexConfigBuilder {
            distance: None,
            ef: None,
            ef_construction: None,
            max_connections: None,
            dynamic_ef_min: None,
            dynamic_ef_max: None,
            dynamic_ef_factor: None,
            vector_cache_max_objects: None,
            flat_search_cut_off: None,
            cleanup_interval_seconds: None,
            pq: None,
            skip: None,
        }
    }

    /// Add a value to the optional `distance` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - distance: the distance to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{VectorIndexConfigBuilder, DistanceMetric};
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_distance(DistanceMetric::COSINE);
    /// ```
    pub fn with_distance(mut self, distance: DistanceMetric) -> VectorIndexConfigBuilder {
        self.distance = Some(distance);
        self
    }

    /// Add a value to the optional `ef` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - ef: the ef to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_ef(10);
    /// ```
    pub fn with_ef(mut self, ef: i64) -> VectorIndexConfigBuilder {
        self.ef = Some(ef);
        self
    }

    /// Add a value to the optional `ef_construction` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - ef_construction: the ef_construction to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_ef_construction(5);
    /// ```
    pub fn with_ef_construction(mut self, ef_construction: u64) -> VectorIndexConfigBuilder {
        self.ef_construction = Some(ef_construction);
        self
    }

    /// Add a value to the optional `max_connections` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - max_connections: the max_connections to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_max_connections(5);
    /// ```
    pub fn with_max_connections(mut self, max_connections: u64) -> VectorIndexConfigBuilder {
        self.max_connections = Some(max_connections);
        self
    }

    /// Add a value to the optional `dynamic_ef_min` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - dynamic_ef_min: the dynamic_ef_min to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_dynamic_ef_min(5);
    /// ```
    pub fn with_dynamic_ef_min(mut self, dynamic_ef_min: i64) -> VectorIndexConfigBuilder {
        self.dynamic_ef_min = Some(dynamic_ef_min);
        self
    }

    /// Add a value to the optional `dynamic_ef_max` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - dynamic_ef_max: the dynamic_ef_max to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_dynamic_ef_max(10);
    /// ```
    pub fn with_dynamic_ef_max(mut self, dynamic_ef_max: i64) -> VectorIndexConfigBuilder {
        self.dynamic_ef_max = Some(dynamic_ef_max);
        self
    }

    /// Add a value to the optional `dynamic_ef_factor` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - dynamic_ef_factor: the dynamic_ef_factor to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_dynamic_ef_factor(3);
    /// ```
    pub fn with_dynamic_ef_factor(mut self, dynamic_ef_factor: i64) -> VectorIndexConfigBuilder {
        self.dynamic_ef_factor = Some(dynamic_ef_factor);
        self
    }

    /// Add a value to the optional `vector_cache_max_objects` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - vector_cache_max_objects: the vector_cache_max_objects to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_vector_cache_max_objects(3);
    /// ```
    pub fn with_vector_cache_max_objects(
        mut self,
        vector_cache_max_objects: u64
    ) -> VectorIndexConfigBuilder {
        self.vector_cache_max_objects = Some(vector_cache_max_objects);
        self
    }

    /// Add a value to the optional `flat_search_cut_off` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - flat_search_cut_off: the flat_search_cut_off to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_flat_search_cut_off(3);
    /// ```
    pub fn with_flat_search_cut_off(
        mut self,
        flat_search_cut_off: u64
    ) -> VectorIndexConfigBuilder {
        self.flat_search_cut_off = Some(flat_search_cut_off);
        self
    }

    /// Add a value to the optional `cleanup_interval_seconds` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - cleanup_interval_seconds: the cleanup_interval_seconds to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_cleanup_interval_seconds(3);
    /// ```
    pub fn with_cleanup_interval_seconds(
        mut self,
        cleanup_interval_seconds: u64
    ) -> VectorIndexConfigBuilder {
        self.cleanup_interval_seconds = Some(cleanup_interval_seconds);
        self
    }

    /// Add a value to the optional `pq` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - pq: the pq config to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{
    ///     VectorIndexConfigBuilder,
    ///     PqConfig
    /// };
    ///
    /// let pq_config = PqConfig::builder().build();
    /// let builder = VectorIndexConfigBuilder::new().with_pq(pq_config);
    /// ```
    pub fn with_pq(mut self, pq: PqConfig) -> VectorIndexConfigBuilder {
        self.pq = Some(pq);
        self
    }

    /// Add a value to the optional `skip` value of the VectorIndexConfig.
    ///
    /// # Parameters
    /// - skip: the skip to use for the vector index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let builder = VectorIndexConfigBuilder::new().with_skip(true);
    /// ```
    pub fn with_skip(mut self, skip: bool) -> VectorIndexConfigBuilder {
        self.skip = Some(skip);
        self
    }

    /// Build the VectorIndexConfig from the VectorIndexConfigBuilder
    ///
    /// # Example
    /// Using VectorIndexConfigBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfigBuilder;
    ///
    /// let config = VectorIndexConfigBuilder::new().build();
    /// ```
    ///
    /// Using VectorIndexConfig
    /// ```rust
    /// use weaviate_community::collections::schema::VectorIndexConfig;
    ///
    /// let config = VectorIndexConfig::builder().build();
    /// ```
    pub fn build(self) -> VectorIndexConfig {
        VectorIndexConfig {
            distance: self.distance,
            ef: self.ef,
            ef_construction: self.ef_construction,
            max_connections: self.max_connections,
            dynamic_ef_min: self.dynamic_ef_min,
            dynamic_ef_max: self.dynamic_ef_max,
            dynamic_ef_factor: self.dynamic_ef_factor,
            vector_cache_max_objects: self.vector_cache_max_objects,
            flat_search_cut_off: self.flat_search_cut_off,
            cleanup_interval_seconds: self.cleanup_interval_seconds,
            pq: self.pq,
            skip: self.skip,
        }
    }
}

/// The configuration options for pq
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PqConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub training_limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub segments: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub centroids: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub encoder: Option<EncoderConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bit_compression: Option<bool>,
}

impl PqConfig {
    /// Create a new builder for the PqConfig object.
    ///
    /// This is the same as `PqConfigBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new();
    /// ```
    pub fn builder() -> PqConfigBuilder {
        PqConfigBuilder::default()
    }
}

/// PqConfigBuilder for building a new PqConfig
#[derive(Default)]
pub struct PqConfigBuilder {
    pub enabled: Option<bool>,
    pub training_limit: Option<u64>,
    pub segments: Option<u64>,
    pub centroids: Option<u64>,
    pub encoder: Option<EncoderConfig>,
    pub bit_compression: Option<bool>,
}

impl PqConfigBuilder {
    /// Create a new builder for the PqConfig object.
    ///
    /// This is the same as `PqConfig::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new();
    /// ```
    pub fn new() -> PqConfigBuilder {
        PqConfigBuilder {
            enabled: None,
            training_limit: None,
            segments: None,
            centroids: None,
            encoder: None,
            bit_compression: None,
        }
    }

    /// Add a value to the optional `enabled` value of the PqConfig.
    ///
    /// # Parameters
    /// - enabled: the enabled value to use for the pq config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new().with_enabled(true);
    /// ```
    pub fn with_enabled(mut self, enabled: bool) -> PqConfigBuilder {
        self.enabled = Some(enabled);
        self
    }

    /// Add a value to the optional `training_limit` value of the PqConfig.
    ///
    /// # Parameters
    /// - training_limit: the training_limit value to use for the pq config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new().with_training_limit(100);
    /// ```
    pub fn with_training_limit(mut self, training_limit: u64) -> PqConfigBuilder {
        self.training_limit = Some(training_limit);
        self
    }

    /// Add a value to the optional `segments` value of the PqConfig.
    ///
    /// # Parameters
    /// - segments: the segments value to use for the pq config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new().with_segments(100);
    /// ```
    pub fn with_segments(mut self, segments: u64) -> PqConfigBuilder {
        self.segments = Some(segments);
        self
    }

    /// Add a value to the optional `centroids` value of the PqConfig.
    ///
    /// # Parameters
    /// - centroids: the centroids value to use for the pq config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new().with_centroids(20);
    /// ```
    pub fn with_centroids(mut self, centroids: u64) -> PqConfigBuilder {
        self.centroids = Some(centroids);
        self
    }

    /// Add a value to the optional `encoder` value of the PqConfig.
    ///
    /// # Parameters
    /// - encoder: the encoder config to use for the pq config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{
    ///     PqConfigBuilder,
    ///     EncoderConfig,
    ///     EncoderType
    /// };
    ///
    /// let encoder_config = EncoderConfig::builder(EncoderType::KMEANS).build();
    /// let builder = PqConfigBuilder::new().with_encoder(encoder_config);
    /// ```
    pub fn with_encoder(mut self, encoder: EncoderConfig) -> PqConfigBuilder {
        self.encoder = Some(encoder);
        self
    }

    /// Add a value to the optional `bit_compression` value of the PqConfig.
    ///
    /// # Parameters
    /// - bit_compression: the bit compression value to use for the pq config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let builder = PqConfigBuilder::new().with_bit_compression(true);
    /// ```
    pub fn with_bit_compression(mut self, bit_compression: bool) -> PqConfigBuilder {
        self.bit_compression = Some(bit_compression);
        self
    }

    /// Build the PqConfig from the PqConfigBuilder
    ///
    /// # Example
    /// Using PqConfigBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfigBuilder;
    ///
    /// let config = PqConfigBuilder::new().build();
    /// ```
    ///
    /// Using PqConfig
    /// ```rust
    /// use weaviate_community::collections::schema::PqConfig;
    ///
    /// let config = PqConfig::builder().build();
    /// ```
    pub fn build(self) -> PqConfig {
        PqConfig {
            enabled: self.enabled,
            training_limit: self.training_limit,
            segments: self.segments,
            centroids: self.centroids,
            encoder: self.encoder,
            bit_compression: self.bit_compression,
        }
    }
}

/// The configuration options for an encoder
///
/// - distribution
/// - encoder_type
#[derive(Serialize, Deserialize, Debug)]
pub struct EncoderConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub distribution: Option<Distribution>,

    #[serde(rename = "type")]
    pub encoder_type: EncoderType,
}

impl EncoderConfig {
    /// Create a new builder for the EncoderConfig object.
    ///
    /// This is the same as `EncoderConfigBuilder::new()`.
    ///
    /// # Parameters
    /// - encoder_type: the encoder type to use for the encoder config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{EncoderConfig, EncoderType};
    ///
    /// let builder = EncoderConfig::builder(EncoderType::KMEANS);
    /// ```
    pub fn builder(encoder_type: EncoderType) -> EncoderConfigBuilder {
        EncoderConfigBuilder::new(encoder_type)
    }
}

/// PqConfigBuilder for building a new PqConfig
pub struct EncoderConfigBuilder {
    pub distribution: Option<Distribution>,
    pub encoder_type: EncoderType,
}

impl EncoderConfigBuilder {
    /// Create a new builder for the EncoderConfig object.
    ///
    /// This is the same as `EncoderConfig::builder()`.
    ///
    /// # Parameters
    /// - encoder_type: the encoder type to use for the encoder config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{EncoderConfigBuilder, EncoderType};
    ///
    /// let builder = EncoderConfigBuilder::new(EncoderType::KMEANS);
    /// ```
    pub fn new(encoder_type: EncoderType) -> EncoderConfigBuilder {
        EncoderConfigBuilder {
            distribution: None,
            encoder_type,
        }
    }

    /// Add a value to the optional `distribution` value of the EncoderConfig.
    ///
    /// # Parameters
    /// - distribution: the distribution to use for the encoder config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{
    ///     EncoderConfigBuilder,
    ///     EncoderType,
    ///     Distribution
    /// };
    ///
    /// let builder = EncoderConfigBuilder::new(EncoderType::KMEANS)
    ///     .with_distribution(Distribution::LOGNORMAL);
    /// ```
    pub fn with_distribution(mut self, distribution: Distribution) -> EncoderConfigBuilder {
        self.distribution = Some(distribution);
        self
    }

    /// Build the PqConfig from the PqConfigBuilder
    ///
    /// # Example
    /// Using PqConfigBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::{EncoderConfigBuilder, EncoderType};
    ///
    /// let config = EncoderConfigBuilder::new(EncoderType::KMEANS).build();
    /// ```
    ///
    /// Using PqConfig
    /// ```rust
    /// use weaviate_community::collections::schema::{EncoderConfig, EncoderType};
    ///
    /// let config = EncoderConfig::builder(EncoderType::KMEANS).build();
    /// ```
    pub fn build(self) -> EncoderConfig {
        EncoderConfig {
            distribution: self.distribution,
            encoder_type: self.encoder_type,
        }
    }
}

/// Strict definitions of distributions.
///
/// Currently, Weaviate only allows log-normal and normal for kmeans
#[derive(Serialize, Deserialize, Debug)]
pub enum Distribution {
    #[serde(rename = "log-normal")]
    LOGNORMAL,
    #[serde(rename = "normal")]
    NORMAL,
}

/// Strict definitions of encoders.
///
/// Currently only supports KMeans and Tile
#[derive(Serialize, Deserialize, Debug)]
pub enum EncoderType {
    #[serde(rename = "kmeans")]
    KMEANS,
    #[serde(rename = "tile")]
    TILE,
}

/// Strict definitions of distance metrics.
///
/// Currently only supports the following:
/// - Cosine
/// - Dot
/// - L2 squared
/// - Hamming
/// - Manhattan
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DistanceMetric {
    #[serde(rename = "cosine")]
    COSINE,
    #[serde(rename = "dot")]
    DOT,
    #[serde(rename = "l2-squared")]
    L2SQUARED,
    #[serde(rename = "hamming")]
    HAMMING,
    #[serde(rename = "manhattan")]
    MANHATTAN,
}

/// The configuration options for ShardingConfig.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShardingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub virtual_per_physical: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub desired_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub actual_count: Option<u64>, // this could be problematic, it is read only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub desired_virtual_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub actual_virtual_count: Option<u64>, // this could be problematic, it is read only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ShardingKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<ShardingStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ShardingFunction>,
}

impl ShardingConfig {
    /// Create a new builder for the ShardingConfig object.
    ///
    /// This is the same as `ShardingConfigBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfig;
    ///
    /// let builder = ShardingConfig::builder();
    /// ```
    pub fn builder() -> ShardingConfigBuilder {
        ShardingConfigBuilder::default()
    }
}

/// ShardingConfigBuilder for building a new ShardingConfig
#[derive(Default)]
pub struct ShardingConfigBuilder {
    pub virtual_per_physical: Option<u64>,
    pub desired_count: Option<u64>,
    pub actual_count: Option<u64>, // this could be problematic, it is read only
    pub desired_virtual_count: Option<u64>,
    pub actual_virtual_count: Option<u64>, // this could be problematic, it is read only
    pub key: Option<ShardingKey>,
    pub strategy: Option<ShardingStrategy>,
    pub function: Option<ShardingFunction>,
}

impl ShardingConfigBuilder {
    /// Create a new builder for the ShardingConfig object.
    ///
    /// This is the same as `ShardingConfig::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let builder = ShardingConfigBuilder::new();
    /// ```
    pub fn new() -> ShardingConfigBuilder {
        ShardingConfigBuilder {
            virtual_per_physical: None,
            desired_count: None,
            actual_count: None,
            desired_virtual_count: None,
            actual_virtual_count: None,
            key: None,
            strategy: None,
            function: None,
        }
    }

    /// Add a value to the optional `virtual_per_physical` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - virtual_per_physical: the virtual per physical to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_virtual_per_physical(10);
    /// ```
    pub fn with_virtual_per_physical(
        mut self, 
        virtual_per_physical: u64
    ) -> ShardingConfigBuilder {
        self.virtual_per_physical = Some(virtual_per_physical);
        self
    }

    /// Add a value to the optional `desired_count` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - desired_count: the desired count to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_desired_count(10);
    /// ```
    pub fn with_desired_count(
        mut self, 
        desired_count: u64
    ) -> ShardingConfigBuilder {
        self.desired_count = Some(desired_count);
        self
    }

    /// Add a value to the optional `actual_count` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - actual_count: the actual count to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_actual_count(10);
    /// ```
    pub fn with_actual_count(
        mut self, 
        actual_count: u64
    ) -> ShardingConfigBuilder {
        self.actual_count = Some(actual_count);
        self
    }

    /// Add a value to the optional `desired_virtual_count` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - desired_virtual_count: the desired virtual count to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_desired_virtual_count(10);
    /// ```
    pub fn with_desired_virtual_count(
        mut self, 
        desired_virtual_count: u64
    ) -> ShardingConfigBuilder {
        self.desired_virtual_count = Some(desired_virtual_count);
        self
    }

    /// Add a value to the optional `actual_virtual_count` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - actual_virtual_count: the actual virtual count to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_actual_virtual_count(10);
    /// ```
    pub fn with_actual_virtual_count(
        mut self, 
        actual_virtual_count: u64
    ) -> ShardingConfigBuilder {
        self.actual_virtual_count = Some(actual_virtual_count);
        self
    }

    /// Add a value to the optional `key` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - key: the sharding key to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ShardingConfigBuilder, ShardingKey};
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_key(ShardingKey::_ID);
    /// ```
    pub fn with_key(
        mut self, 
        key: ShardingKey
    ) -> ShardingConfigBuilder {
        self.key = Some(key);
        self
    }

    /// Add a value to the optional `strategy` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - strategy: the sharding strategy to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ShardingConfigBuilder, ShardingStrategy};
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_strategy(ShardingStrategy::HASH);
    /// ```
    pub fn with_strategy(
        mut self, 
        strategy: ShardingStrategy
    ) -> ShardingConfigBuilder {
        self.strategy = Some(strategy);
        self
    }

    /// Add a value to the optional `function` value of the ShardingConfig.
    ///
    /// # Parameters
    /// - function: the sharding function to use for the sharding config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{ShardingConfigBuilder, ShardingFunction};
    ///
    /// let builder = ShardingConfigBuilder::new()
    ///     .with_function(ShardingFunction::MURMUR3);
    /// ```
    pub fn with_function(
        mut self, 
        function: ShardingFunction
    ) -> ShardingConfigBuilder {
        self.function = Some(function);
        self
    }

    /// Build the PqConfig from the PqConfigBuilder
    ///
    /// # Example
    /// Using PqConfigBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfigBuilder;
    ///
    /// let config = ShardingConfigBuilder::new().build();
    /// ```
    ///
    /// Using PqConfig
    /// ```rust
    /// use weaviate_community::collections::schema::ShardingConfig;
    ///
    /// let config = ShardingConfig::builder().build();
    /// ```
    pub fn build(self) -> ShardingConfig {
        ShardingConfig {
            virtual_per_physical: self.virtual_per_physical,
            desired_count: self.desired_count,
            actual_count: self.actual_count,
            desired_virtual_count: self.desired_virtual_count,
            actual_virtual_count: self.actual_virtual_count,
            key: self.key,
            strategy: self.strategy,
            function: self.function,
        }
    }
}

/// Strict definitions of sharding keys.
///
/// The default will usually be _ID, unless MultiTenancy is enabled, where the
/// default will be an empty string.
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingKey {
    #[serde(rename = "_id")]
    _ID,
    #[serde(rename = "")]
    MultiTenancyEnabled,
}

/// Strict definitions of sharding strategies.
///
/// The default will usually be HASH, unless MultiTenancy is enabled, where the
/// default will be an empty string.
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingStrategy {
    #[serde(rename = "hash")]
    HASH,
    #[serde(rename = "")]
    MultiTenancyEnabled,
}

/// Strict definitions of sharding functions.
///
/// The default will usually be MURMUR3, unless MultiTenancy is enabled, where the
/// default will be an empty string.
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingFunction {
    #[serde(rename = "murmur3")]
    MURMUR3,
    #[serde(rename = "")]
    MultiTenancyEnabled,
}

/// MultiTenancyConfig holds the configuration options for multi tenancy.
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiTenancyConfig {
    pub enabled: bool,
}

impl MultiTenancyConfig {
    /// Create a new MultiTenancyConfig
    ///
    /// # Parameters
    /// - enabled: to enable multi tenancy or not
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::MultiTenancyConfig;
    ///
    /// let config = MultiTenancyConfig::new(true);
    /// ```
    pub fn new(enabled: bool) -> MultiTenancyConfig {
        MultiTenancyConfig { enabled }
    }
}

/// The configuration options for InvertedIndexConfig
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InvertedIndexConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stopwords: Option<StopwordsConfig>, // revisit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_timestamps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_null_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_property_length: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bm25: Option<Bm25>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cleanup_interval_seconds: Option<u64>,
}

impl InvertedIndexConfig {
    /// Create a new builder for the InvertedIndexConfig object.
    ///
    /// This is the same as `InvertedIndexConfigBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfig;
    ///
    /// let builder = InvertedIndexConfig::builder();
    /// ```
    pub fn builder() -> InvertedIndexConfigBuilder {
        InvertedIndexConfigBuilder::default()
    }
}

/// InvertedIndexConfigBuilder for building a new InvertedIndexConfig
#[derive(Default)]
pub struct InvertedIndexConfigBuilder {
    pub stopwords: Option<StopwordsConfig>, // revisit
    pub index_timestamps: Option<bool>,
    pub index_null_state: Option<bool>,
    pub index_property_length: Option<bool>,
    pub bm25: Option<Bm25>,
    pub cleanup_interval_seconds: Option<u64>,
}

impl InvertedIndexConfigBuilder {
    /// Create a new builder for the InvertedIndexConfig object.
    ///
    /// This is the same as `InvertedIndexConfig::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfigBuilder;
    ///
    /// let builder = InvertedIndexConfigBuilder::new();
    /// ```
    pub fn new() -> InvertedIndexConfigBuilder {
        InvertedIndexConfigBuilder {
            stopwords: None,
            index_timestamps: None,
            index_null_state: None,
            index_property_length: None,
            bm25: None,
            cleanup_interval_seconds: None,
        }
    }

    /// Add a value to the optional `stopwords` value of the InvertedIndexConfig.
    ///
    /// # Parameters
    /// - stopwords: the stopwords to use for the inverted index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{InvertedIndexConfigBuilder, StopwordsConfig};
    ///
    /// let stopwords = StopwordsConfig::builder().build();
    /// let builder = InvertedIndexConfigBuilder::new().with_stopwords(stopwords);
    /// ```
    pub fn with_stopwords(
        mut self, 
        stopwords: StopwordsConfig
    ) -> InvertedIndexConfigBuilder {
        self.stopwords = Some(stopwords);
        self
    }

    /// Add a value to the optional `index_timestamps` value of the InvertedIndexConfig.
    ///
    /// # Parameters
    /// - index_timestamps: the index timestamps setting to use for the inverted index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfigBuilder;
    ///
    /// let builder = InvertedIndexConfigBuilder::new().with_index_timestamps(true);
    /// ```
    pub fn with_index_timestamps(mut self, index_timestamps: bool) -> InvertedIndexConfigBuilder {
        self.index_timestamps = Some(index_timestamps);
        self
    }

    /// Add a value to the optional `index_null_state` value of the InvertedIndexConfig.
    ///
    /// # Parameters
    /// - index_null_state: the index null state setting to use for the inverted index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfigBuilder;
    ///
    /// let builder = InvertedIndexConfigBuilder::new().with_index_null_state(true);
    /// ```
    pub fn with_index_null_state(mut self, index_null_state: bool) -> InvertedIndexConfigBuilder {
        self.index_null_state = Some(index_null_state);
        self
    }

    /// Add a value to the optional `index_property_length` value of the InvertedIndexConfig.
    ///
    /// # Parameters
    /// - index_property_length: the index prop state setting to use for the inverted index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfigBuilder;
    ///
    /// let builder = InvertedIndexConfigBuilder::new().with_index_property_length(true);
    /// ```
    pub fn with_index_property_length(
        mut self,
        index_property_length: bool
    ) -> InvertedIndexConfigBuilder {
        self.index_property_length = Some(index_property_length);
        self
    }

    /// Add a value to the optional `bm25` value of the InvertedIndexConfig.
    ///
    /// # Parameters
    /// - bm25: the bm25 config to use for the inverted index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{InvertedIndexConfigBuilder, Bm25};
    ///
    /// let bm25 = Bm25::new(10.0, 10.0);
    /// let builder = InvertedIndexConfigBuilder::new().with_bm25(bm25);
    /// ```
    pub fn with_bm25(mut self, bm25: Bm25) -> InvertedIndexConfigBuilder {
        self.bm25 = Some(bm25);
        self
    }

    /// Add a value to the optional `cleanup_interval_seconds` value of the InvertedIndexConfig.
    ///
    /// # Parameters
    /// - cleanup_interval_seconds: the cleanup_interval_seconds for the inverted index config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfigBuilder;
    ///
    /// let builder = InvertedIndexConfigBuilder::new().with_cleanup_interval_seconds(60);
    /// ```
    pub fn with_cleanup_interval_seconds(
        mut self,
        cleanup_interval_seconds: u64
    ) -> InvertedIndexConfigBuilder {
        self.cleanup_interval_seconds = Some(cleanup_interval_seconds);
        self
    }

    /// Build the InvertedIndexConfig from the InvertedIndexConfigBuilder
    ///
    /// # Example
    /// Using InvertedIndexConfigBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfigBuilder;
    ///
    /// let config = InvertedIndexConfigBuilder::new().build();
    /// ```
    ///
    /// Using InvertedIndexConfig
    /// ```rust
    /// use weaviate_community::collections::schema::InvertedIndexConfig;
    ///
    /// let config = InvertedIndexConfig::builder().build();
    /// ```
    pub fn build(self) -> InvertedIndexConfig {
        InvertedIndexConfig {
            stopwords: self.stopwords,
            index_timestamps: self.index_timestamps,
            index_null_state: self.index_null_state,
            index_property_length: self.index_property_length,
            bm25: self.bm25,
            cleanup_interval_seconds: self.cleanup_interval_seconds,
        }
    }
}

/// The configuration options for Stopwords.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopwordsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub preset: Option<StopwordPreset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub additions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub removals: Option<Vec<String>>,
}

impl StopwordsConfig {
    /// Create a new builder for the StopwordsConfig object.
    ///
    /// This is the same as `StopwordsConfigBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::StopwordsConfig;
    ///
    /// let builder = StopwordsConfig::builder();
    /// ```
    pub fn builder() -> StopwordsConfigBuilder {
        StopwordsConfigBuilder::default()
    }
}

/// StopwordsConfigBuilder for building a new StopwordsConfig
#[derive(Default)]
pub struct StopwordsConfigBuilder {
    pub preset: Option<StopwordPreset>,
    pub additions: Option<Vec<String>>,
    pub removals: Option<Vec<String>>,
}

impl StopwordsConfigBuilder {
    /// Create a new builder for the StopwordsConfig object.
    ///
    /// This is the same as `StopwordsConfig::builder()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::StopwordsConfigBuilder;
    ///
    /// let builder = StopwordsConfigBuilder::new();
    /// ```
    pub fn new() -> StopwordsConfigBuilder {
        StopwordsConfigBuilder {
            preset: None,
            additions: None,
            removals: None,
        }
    }

    /// Add a value to the optional `preset` value of the StopwordsConfig.
    ///
    /// # Parameters
    /// - preset: the preset for the stopwords config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{StopwordsConfigBuilder, StopwordPreset};
    ///
    /// let builder = StopwordsConfigBuilder::new().with_preset(StopwordPreset::EN);
    /// ```
    pub fn with_preset(
        mut self,
        preset: StopwordPreset
    ) -> StopwordsConfigBuilder {
        self.preset = Some(preset);
        self
    }

    /// Add a value to the optional `additions` value of the StopwordsConfig.
    ///
    /// # Parameters
    /// - additions: the additions for the stopwords config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::StopwordsConfigBuilder;
    ///
    /// let builder = StopwordsConfigBuilder::new().with_additions(vec!["word"]);
    /// ```
    pub fn with_additions(
        mut self,
        additions: Vec<&str>
    ) -> StopwordsConfigBuilder {
        let additions = additions.iter().map(|field| field.to_string()).collect();
        self.additions = Some(additions);
        self
    }

    /// Add a value to the optional `removals` value of the StopwordsConfig.
    ///
    /// # Parameters
    /// - removals: the removals for the stopwords config
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::StopwordsConfigBuilder;
    ///
    /// let builder = StopwordsConfigBuilder::new().with_removals(vec!["word"]);
    /// ```
    pub fn with_removals(
        mut self,
        removals: Vec<&str>
    ) -> StopwordsConfigBuilder {
        let removals = removals.iter().map(|field| field.to_string()).collect();
        self.removals = Some(removals);
        self
    }

    /// Build the StopwordsConfig from the StopwordsConfigBuilder.
    ///
    /// # Example
    /// Using StopwordsConfigBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::StopwordsConfigBuilder;
    ///
    /// let config = StopwordsConfigBuilder::new().build();
    /// ```
    ///
    /// Using StopwordsConfig
    /// ```rust
    /// use weaviate_community::collections::schema::StopwordsConfig;
    ///
    /// let config = StopwordsConfig::builder().build();
    /// ```
    pub fn build(self) -> StopwordsConfig {
        StopwordsConfig {
            preset: self.preset,
            additions: self.additions,
            removals: self.removals,
        }
    }
}

/// Strict definitions of Stopword presets.
///
/// Weaviate supports EN and NONE
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StopwordPreset {
    #[serde(rename = "en")]
    EN,
    #[serde(rename = "none")]
    NONE,
}

/// The configuration options for the ReplicationConfig
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicationConfig {
    pub factor: u64,
}

impl ReplicationConfig {
    /// Create a new ReplicationConfig
    ///
    /// # Parameters
    /// - factor: the replication factor
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::ReplicationConfig;
    ///
    /// let config = ReplicationConfig::new(3);
    /// ```
    pub fn new(factor: u64) -> ReplicationConfig {
        ReplicationConfig { factor }
    }
}

/// Tenants struct for encapsulating multiple tenants.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenants {
    pub tenants: Vec<Tenant>,
}

impl Tenants {
    /// Create a new Tenants object
    ///
    /// # Parameters
    /// - tenants: the tenants to encapsulate
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{Tenants, Tenant};
    ///
    /// let config = Tenants::new(
    ///     vec![
    ///         Tenant::builder("abcde").build(),
    ///         Tenant::builder("fghij").build(),
    ///     ]
    /// );
    /// ```
    pub fn new(tenants: Vec<Tenant>) -> Tenants {
        Tenants { tenants }
    }
}

/// The configuration options for a Tenant.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenant {
    pub name: String,
    #[serde(default = "default_activity_status")]
    pub activity_status: Option<ActivityStatus>,
}

/// Default activity status for a tenant
///
/// The default activity status for a tenant is HOT.
fn default_activity_status() -> Option<ActivityStatus> {
    Some(ActivityStatus::HOT)
}

impl Tenant {
    /// Create a new builder for the Tenant object.
    ///
    /// This is the same as `TenantBuilder::new()`.
    ///
    /// # Parameters
    /// - name: the name of the tenant
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::Tenant;
    ///
    /// let builder = Tenant::builder("abcde");
    /// ```
    pub fn builder(name: &str) -> TenantBuilder {
        TenantBuilder::new(name)
    }
}

/// TenantBuilder for building a new Tenant
pub struct TenantBuilder {
    name: String,
    activity_status: Option<ActivityStatus>,
}

impl TenantBuilder {
    /// Create a new builder for the Tenant object.
    ///
    /// This is the same as `Tenant::builder()`.
    ///
    /// # Parameters
    /// - name: the name of the tenant
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::TenantBuilder;
    ///
    /// let builder = TenantBuilder::new("abcde");
    /// ```
    pub fn new(name: &str) -> TenantBuilder {
        TenantBuilder {
            name: name.into(),
            activity_status: None,
        }
    }

    /// Add a value to the optional `activity_status` value of the Tenant.
    ///
    /// # Parameters
    /// - activity_status: the activity_status of the tenant
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{TenantBuilder, ActivityStatus};
    ///
    /// let builder = TenantBuilder::new("abcde").with_activity_status(ActivityStatus::HOT);
    /// ```
    pub fn with_activity_status(
        mut self,
        activity_status: ActivityStatus
    ) -> TenantBuilder {
        self.activity_status = Some(activity_status);
        self
    }

    /// Build the Tenant from the TenantBuilder.
    ///
    /// # Example
    /// Using TenantBuilder
    /// ```rust
    /// use weaviate_community::collections::schema::TenantBuilder;
    ///
    /// let config = TenantBuilder::new("abcde").build();
    /// ```
    ///
    /// Using Tenant
    /// ```rust
    /// use weaviate_community::collections::schema::Tenant;
    ///
    /// let config = Tenant::builder("abcde").build();
    /// ```
    pub fn build(self) -> Tenant {
        Tenant {
            name: self.name,
            activity_status: self.activity_status,
        }
    }
}

/// Strict definitions of ActivityStatus of a tenant.
///
/// The activity status of a tenant can either be `hot` or `cold`.
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityStatus {
    HOT,
    COLD,
}

/// The configuration options for BM25.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Bm25 {
    pub b: f64,
    pub k1: f64,
}

impl Bm25 {
    /// Create a new Bm25 object
    ///
    /// # Parameters
    /// - b: the b value to set
    /// - k1: the k1 value to set
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::Bm25;
    ///
    /// let config = Bm25::new(10.0, 10.0);
    /// ```
    pub fn new(b: f64, k1: f64) -> Bm25 {
        Bm25 { b, k1 }
    }
}

/// Strict definitions of tokenization methods.
///
/// Weaviate supports the following tokenization methods:
/// - Word
/// - Lowercase
/// - Whitespace
/// - Field
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Tokenization {
    #[serde(rename = "word")]
    WORD,
    #[serde(rename = "lowercase")]
    LOWERCASE,
    #[serde(rename = "whitespace")]
    WHITESPACE,
    #[serde(rename = "field")]
    FIELD,
}


/// Shards struct to hold multiple shards
#[derive(Serialize, Deserialize, Debug)]
pub struct Shards {
    pub shards: Vec<Shard>,
}

impl Shards {
    /// Create a new Shards object
    ///
    /// # Parameters
    /// - shards: the shards to encapsulate
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{Shards, Shard, ShardStatus};
    ///
    /// let shards = Shards::new(
    ///     vec![
    ///         Shard::new("abcd", ShardStatus::READY),
    ///         Shard::new("efgh", ShardStatus::READONLY),
    ///     ]
    /// );
    /// ```
    pub fn new(shards: Vec<Shard>) -> Shards {
        Shards { shards }
    }
}

/// Shard struct to define the name and status of a shard.
///
/// Generally this wouldn't be created by a user, it would be automatically created in the schema,
/// and readable by the user.
#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub name: String,
    pub status: ShardStatus,
}

impl Shard {
    /// Create a new Shard object
    ///
    /// # Parameters
    /// - name: the name of the shard
    /// - status: the status of the shard
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::schema::{Shard, ShardStatus};
    ///
    /// let shard = Shard::new("abcd", ShardStatus::READY);
    /// ```
    pub fn new(name: &str, status: ShardStatus) -> Shard {
        Shard { name: name.into(), status}
    }
}

/// Strict definitions of ShardStatus.
///
/// Weaviate supports READONLY and READY shard status.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ShardStatus {
    READONLY,
    READY,
}
