/// All modules (contextionary) associated type components
use serde::{Deserialize, Serialize};

/// The expected response format when received from /v1/modules/text2vec-contextionary/concepts/{}
/// successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContextionaryConcept {
    pub individual_words: Vec<IndividualWords>,
}

/// Forms part of the expected response format when received from 
/// /v1/modules/text2vec-contextionary/concepts/{} successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualWords {
    pub info: Option<ContextionaryConceptInfo>,
    pub present: Option<bool>,
    pub word: String,
    pub concatenated_word: Option<ConcatenatedWord>,
}

/// Forms part of the expected response format when received from 
/// /v1/modules/text2vec-contextionary/concepts/{} successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContextionaryConceptInfo {
    pub nearest_neighbors: Vec<IndividualWord>,
    pub vector: Vec<f64>,
}

/// Forms part of the expected response format when received from 
/// /v1/modules/text2vec-contextionary/concepts/{} successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct IndividualWord {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub distance: Option<f64>,
    pub word: String,
}

/// Forms part of the expected response format when received from 
/// /v1/modules/text2vec-contextionary/concepts/{} successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConcatenatedWords {
    concatenated_word: ConcatenatedWord,
}

/// Forms part of the expected response format when received from 
/// /v1/modules/text2vec-contextionary/concepts/{} successfully.
///
/// This shouldn't be something you create yourself, as it is returned by the appropriate
/// endpoint when deserialized.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConcatenatedWord {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    single_words: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    concatenated_vector: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    concatenated_nearest_neighbors: Option<Vec<IndividualWord>>,
}

/// ContextionaryExtension object for extending contextionary
#[derive(Serialize, Deserialize, Debug)]
pub struct ContextionaryExtension {
    pub concept: String,
    pub definition: String,
    pub weight: f64,
}

impl ContextionaryExtension {
    /// Create a new ContextionaryExtension object
    ///
    /// # Example
    /// ```no_run
    /// use weaviate_community::collections::modules::ContextionaryExtension;
    ///
    /// let ext = ContextionaryExtension::new("concept", "description", 1.0);
    /// ```
    pub fn new(concept: &str, definition: &str, weight: f64) -> ContextionaryExtension {
        ContextionaryExtension { concept: concept.into(), definition: definition.into(), weight }
    }
}
