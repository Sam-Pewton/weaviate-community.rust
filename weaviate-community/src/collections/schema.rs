// schema reference
// https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaConfig {
}

///
/// Full class definition
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub class: String,

    pub description: String,

    pub properties: Option<Properties>,

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
    #[serde(default)]
    pub sharding_config: Option<ShardingConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub multi_tenancy_config: Option<MultiTenancyConfig>,


    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub replication_config: Option<ReplicationConfig>,
}

/// 
/// Vector index types enum used to ensure valid types.
///
/// Currently Weaviate only supports HNSW.
///
#[derive(Serialize, Deserialize, Debug)]
pub enum VectorIndexType {
    #[serde(rename = "hnsw")]
    HNSW,
}

/// 
/// Controls default for Class vector_index_type
///
fn default_vector_index_type() -> Option<VectorIndexType> {
    Some(VectorIndexType::HNSW)
}

/// 
/// Wrapper for multiple properties
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Properties(Vec<Property>);

///
/// Full property definition
///
#[derive(Serialize, Deserialize, Debug)]
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

/// 
/// Vector index type specific settings
///
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EncoderConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub distribution: Option<Distribution>,
    #[serde(rename = "type")]
    pub encoder_type: EncoderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Distribution {
    #[serde(rename = "log-normal")]
    LOGNORMAL,
    #[serde(rename = "normal")]
    NORMAL,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EncoderType {
    #[serde(rename = "kmeans")]
    KMEANS,
    #[serde(rename = "tile")]
    TILE,
}

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


///
/// Optional setting to control the behaviour of class in a multi-node setting
///
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
    pub actual_count: Option<u64>,             // this could be problematic, it is read only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub desired_virtual_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub actual_virtual_count: Option<u64>,     // this could be problematic, it is read only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key: Option<ShardingKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub strategy: Option<ShardingStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub function: Option<ShardingFunction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingKey {
    #[serde(rename = "_id")]
    _ID,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingStrategy {
    #[serde(rename = "hash")]
    HASH,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShardingFunction {
    #[serde(rename = "murmur3")]
    MURMUR3,
}

///
/// Optional setting used for enabling multi-tenancy
///
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiTenancyConfig {
    pub enabled: bool,
}

///
///
///
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum StopwordPreset {
    #[serde(rename = "en")]
    EN,
    #[serde(rename = "none")]
    NONE,
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
pub enum ShardStatus {
    READONLY,
    READY,
}

///
///
///
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplicationConfig {
    pub factor: u64,
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tenant {
    pub name: String,
    #[serde(default = "default_activity_status")]
    pub activity_status: Option<ActivityStatus>,
}

fn default_activity_status() -> Option<ActivityStatus> {
    Some(ActivityStatus::HOT)
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
pub enum ActivityStatus {
    HOT,
    COLD,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bm25 {
    pub b: f64,
    pub k1: f64,
}


#[derive(Serialize, Deserialize, Debug)]
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
