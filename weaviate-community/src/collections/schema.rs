/// All schema associated type components
/// https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/////////////////////////////////////////////////////////////////////////////////////////// Classes

/// Storage for multiple classes
#[derive(Serialize, Deserialize, Debug)]
pub struct Classes {
    pub classes: Vec<Class>,
}

/// Full class definition and configuration options
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
    pub fn builder() -> ClassBuilder {
        ClassBuilder::default()
    }
}

/// ClassBuilder, the preferred way to create a new Class
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

    pub fn properties(mut self, properties: Properties) -> ClassBuilder {
        self.properties = Some(properties);
        self
    }

    pub fn vector_index_type(mut self, vector_index_type: VectorIndexType) -> ClassBuilder {
        self.vector_index_type = Some(vector_index_type);
        self
    }

    pub fn vector_index_config(mut self, vector_index_config: VectorIndexConfig) -> ClassBuilder {
        self.vector_index_config = Some(vector_index_config);
        self
    }

    pub fn vectorizer(mut self, vectorizer: String) -> ClassBuilder {
        self.vectorizer = Some(vectorizer);
        self
    }

    pub fn module_config(mut self, module_config: String) -> ClassBuilder {
        self.module_config = Some(module_config);
        self
    }

    pub fn inverted_index_config(
        mut self,
        inverted_index_config: InvertedIndexConfig,
    ) -> ClassBuilder {
        self.inverted_index_config = Some(inverted_index_config);
        self
    }

    pub fn sharding_config(mut self, sharding_config: ShardingConfig) -> ClassBuilder {
        self.sharding_config = Some(sharding_config);
        self
    }

    pub fn multi_tenancy_config(
        mut self,
        multi_tenancy_config: MultiTenancyConfig,
    ) -> ClassBuilder {
        self.multi_tenancy_config = Some(multi_tenancy_config);
        self
    }

    pub fn replication_config(mut self, replication_config: ReplicationConfig) -> ClassBuilder {
        self.replication_config = Some(replication_config);
        self
    }

    /// Build the Class from the ClassBuilder
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

/////////////////////////////////////////////////////////////////////////////////// VectorIndexType

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

///////////////////////////////////////////////////////////////////////////////////////// Propeties
/// Wrapper for multiple properties
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Properties(Vec<Property>);

/// Configuration options for a property
///
/// - name
/// - data_type
/// - description
/// - tokenization
/// - module_config
/// - index_filterable
/// - index_searchable
/// - inverted_index_config
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
    pub fn builder() -> PropertyBuilder {
        PropertyBuilder::default()
    }
}

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
    pub fn new(name: &str, data_type: Vec<String>) -> PropertyBuilder {
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

    pub fn description(mut self, description: &str) -> PropertyBuilder {
        self.description = Some(description.into());
        self
    }
    pub fn tokenization(mut self, tokenization: Tokenization) -> PropertyBuilder {
        self.tokenization = Some(tokenization);
        self
    }
    pub fn module_config(
        mut self,
        module_config: HashMap<String, HashMap<String, bool>>,
    ) -> PropertyBuilder {
        self.module_config = Some(module_config);
        self
    }
    pub fn index_filterable(mut self, index_filterable: bool) -> PropertyBuilder {
        self.index_filterable = Some(index_filterable);
        self
    }
    pub fn index_searchable(mut self, index_searchable: bool) -> PropertyBuilder {
        self.index_searchable = Some(index_searchable);
        self
    }
    pub fn inverted_index_config(
        mut self,
        inverted_index_config: InvertedIndexConfig,
    ) -> PropertyBuilder {
        self.inverted_index_config = Some(inverted_index_config);
        self
    }

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

///////////////////////////////////////////////////////////////////////////////// VectorIndexConfig

/// Configuration options for VectorIndexConfig
///
/// - distance
/// - ef
/// - ef_construction
/// - max_connections
/// - dynamic_ef_min
/// - dynamic_ef_max
/// - dynamic_ef_factor
/// - vector_cache_max_objects
/// - flat_search_cut_off
/// - cleanup_interval_seconds
/// - pq
/// - skip
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
    pub fn builder() -> VectorIndexConfigBuilder {
        VectorIndexConfigBuilder::default()
    }
}

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

    // TODO
}

/// The configuration options for pq
///
/// - enabled
/// - training_limit
/// - segments
/// - centroids
/// - encoder
/// - bit_compression
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
    pub fn builder() -> PqConfigBuilder {
        PqConfigBuilder::default()
    }
}

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

    // TODO
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
#[derive(Serialize, Deserialize, Debug)]
pub enum EncoderType {
    #[serde(rename = "kmeans")]
    KMEANS,
    #[serde(rename = "tile")]
    TILE,
}

/// Strict definitions of distance metrics.
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
///
/// - virtual_per_physical
/// - desired_count
/// - actual_count
/// - desired_virtual_count
/// - actual_virtual_count
/// - key
/// - strategy
/// - function
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

/// Strict definitions of sharding keys.
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingKey {
    #[serde(rename = "_id")]
    _ID,
    #[serde(rename = "")]
    MultiTenancyEnabled,
}

/// Strict definitions of sharding strategies.
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingStrategy {
    #[serde(rename = "hash")]
    HASH,
    #[serde(rename = "")]
    MultiTenancyEnabled,
}

/// Strict definitions of sharding functions.
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingFunction {
    #[serde(rename = "murmur3")]
    MURMUR3,
    #[serde(rename = "")]
    MultiTenancyEnabled,
}

/// The configuration options for multi tenancy.
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiTenancyConfig {
    pub enabled: bool,
}

/// The configuration options for InvertedIndexConfig
///
/// - stopwords
/// - index_timestamps
/// - index_null_state
/// - index_property_length
/// - bm25
/// - cleanup_interval_seconds
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

/// The configuration options for Stopwords.
///
/// - preset             => the stopword preset to use
/// - additions  => a vector of strings to add to the preset
/// - removals   => a vector of strings to remove from the preset
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

/// Strict definitions of Stopword presets.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StopwordPreset {
    #[serde(rename = "en")]
    EN,
    #[serde(rename = "none")]
    NONE,
}

/// The configuration options for the ReplicationConfig
///
/// - factor  =>
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicationConfig {
    pub factor: u64,
}

/// The configuration options for a Tenant.
///
/// - name             => The name of the tenant
/// - activity_status  => The activity status of the tenant.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenant {
    pub name: String,

    #[serde(default = "default_activity_status")]
    pub activity_status: Option<ActivityStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenants {
    pub tenants: Vec<Tenant>,
}

/// Default activity status for a tenant
fn default_activity_status() -> Option<ActivityStatus> {
    Some(ActivityStatus::HOT)
}

/// Strict definitions of ActivityStatus of a tenant.
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityStatus {
    HOT,
    COLD,
}

/// The configuration options for BM25.
///
/// - b   =>
/// - k1  =>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Bm25 {
    pub b: f64,
    pub k1: f64,
}

/// Strict definitions of tokenization methods.
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

///////////////////////////////////////////////////////////////////////////////////////////// SHARD

#[derive(Serialize, Deserialize, Debug)]
pub struct Shards {
    pub shards: Vec<Shard>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shard {
    pub name: String,
    pub status: ShardStatus,
}

/// Strict definitions of ShardStatus.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ShardStatus {
    READONLY,
    READY,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let x: Class = ClassBuilder::new("TestBuilder", "This is a test")
            .replication_config(ReplicationConfig { factor: 3 })
            .build();
        //println!("{:#?}", x);
    }
}
