// schema reference
// https://weaviate.io/developers/weaviate/config-refs/schema#auto-schema
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct VectorIndexConfig {} //todo

///
/// Optional setting to control the behaviour of class in a multi-node setting
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShardingConfig {
    virtual_per_physical: Option<u64>,
    desired_count: Option<u64>,
    actual_count: Option<u64>,             // this could be problematic, it is read only
    desired_virtual_count: Option<u64>,
    actual_virtual_count: Option<u64>,     // this could be problematic, it is read only
    key: Option<ShardingKey>,
    strategy: Option<ShardingStrategy>,
    function: Option<ShardingFunction>,
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
    stopwords: Option<String>, // revisit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    index_timestamps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    index_null_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    index_property_length: Option<bool>,
    bm25: Option<Bm25>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StopwordsConfig {
    preset: Option<StopwordPreset>,
    additions: Option<Vec<String>>,
    removals: Option<Vec<String>>,
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
    factor: u64,
}

///
/// Belongs to schema
///
#[derive(Serialize, Deserialize, Debug)]
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
    b: f64,
    k1: f64,
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
