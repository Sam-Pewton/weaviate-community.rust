/// All nodes associated type components
use serde::{Deserialize, Serialize};

/// Nodes wrapper to encapsulate multiple Node items
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiNodes {
    pub nodes: Vec<Node>,
}

/// The expected response format when received from /v1/nodes successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub batch_stats: Option<BatchStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub git_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shards: Option<NodeShards>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stats: Option<NodeStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status: Option<NodeStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version: Option<String>,
}

/// The BatchStats of the node.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BatchStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rate_per_second: Option<u64>,
}

/// NodeShards wrapper to encapsulate multiple NodeShard items
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeShards(Vec<NodeShard>);

/// The NodeShard definitions of a Shard in the node.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeShard {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    object_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    vector_indexing_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    vector_queue_length: Option<u64>,
}

/// The NodeStats of the node.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    object_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    shard_count: Option<u64>,
}

/// The NodeStatus of the node.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub enum NodeStatus {
    HEALTHY,
    UNHEALTHY,
    UNAVAILABLE,
    INDEXING,
}
