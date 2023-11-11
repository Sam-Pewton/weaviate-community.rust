/// All classification associated type components
use serde::{Deserialize, Serialize};

/// A new ClassificationRequest used to make classification requests
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationRequest {
    #[serde(rename = "type")]
    pub classification_type: ClassificationType,
    pub class: String,
    pub classify_properties: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on_properties: Option<Vec<String>>,
    pub filters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
}

impl ClassificationRequest {
    /// Create a new builder for the ClassificationRequest.
    ///
    /// This is the same as `ClassificationRequestBuilder::new()`.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequest;
    ///
    /// let builder = ClassificationRequest::builder();
    /// ```
    pub fn builder() -> ClassificationRequestBuilder {
        ClassificationRequestBuilder::default()
    }
}

/// Builder for the ClassificationRequest
#[derive(Debug, Default)]
pub struct ClassificationRequestBuilder {
    pub classification_type: ClassificationType,
    pub class: String,
    pub classify_properties: Vec<String>,
    pub based_on_properties: Option<Vec<String>>,
    pub filters: serde_json::Value,
    pub settings: Option<serde_json::Value>,
}

impl ClassificationRequestBuilder {
    /// Create a new builder for the ClassificationRequest.
    ///
    /// This is the same as `ClassificationRequestBuilder::new()`.
    ///
    /// The resulting object will have no populated fields. These need filling out in accordance
    /// with the required classification type using the builder methods.
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequestBuilder;
    ///
    /// let builder = ClassificationRequestBuilder::new();
    /// ```
    pub fn new() -> ClassificationRequestBuilder {
        ClassificationRequestBuilder::default()
    }

    /// Add a value to the `classification_type` property of the ClassificationRequest.
    ///
    /// # Parameters
    /// - classification_type: the classification_type to use for the property
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::{
    ///     ClassificationRequestBuilder,
    ///     ClassificationType
    /// };
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_type(ClassificationType::KNN);
    /// ```
    pub fn with_type(
        mut self,
        classification_type: ClassificationType,
    ) -> ClassificationRequestBuilder {
        self.classification_type = classification_type;
        self
    }

    /// Add a value to the `class` property of the ClassificationRequest.
    ///
    /// # Parameters
    /// - class_name: the name of the class to run the classification on
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequestBuilder;
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_class("Article");
    /// ```
    pub fn with_class(mut self, class_name: &str) -> ClassificationRequestBuilder {
        self.class = class_name.into();
        self
    }

    /// Add a value to the `classify_properties` property of the ClassificationRequest.
    ///
    /// # Parameters
    /// - classify_properties: the properties to classify
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequestBuilder;
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_classify_properties(vec!["hasPopularity"]);
    /// ```
    pub fn with_classify_properties(
        mut self,
        classify_properties: Vec<&str>,
    ) -> ClassificationRequestBuilder {
        let classify_properties = classify_properties
            .iter()
            .map(|field| field.to_string())
            .collect();
        self.classify_properties = classify_properties;
        self
    }

    /// Add a value to the `based_on_properties` property of the ClassificationRequest.
    ///
    /// # Parameters
    /// - based_on_properties: the 'based on' properties to classify against
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequestBuilder;
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_based_on_properties(vec!["summary"]);
    /// ```
    pub fn with_based_on_properties(
        mut self,
        based_on_properties: Vec<&str>,
    ) -> ClassificationRequestBuilder {
        let based_on_properties = based_on_properties
            .iter()
            .map(|field| field.to_string())
            .collect();
        self.based_on_properties = Some(based_on_properties);
        self
    }

    /// Add a value to the `filters` property of the ClassificationRequest.
    ///
    /// # Parameters
    /// - filters: the filters for the classifier to use when retrieving results
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequestBuilder;
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_filters(serde_json::json!(
    ///         {"path": ["wordCount"], "operator": "GreaterThan", "valueInt": 100}
    ///     ));
    /// ```
    pub fn with_filters(mut self, filters: serde_json::Value) -> ClassificationRequestBuilder {
        self.filters = filters;
        self
    }

    /// Add a value to the `settings` property of the ClassificationRequest.
    ///
    /// # Parameters
    /// - settings: the settings for the classifier
    ///
    /// # Example
    /// ```rust
    /// use weaviate_community::collections::classification::ClassificationRequestBuilder;
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_settings(serde_json::json!({"k": 3}));
    /// ```
    pub fn with_settings(mut self, settings: serde_json::Value) -> ClassificationRequestBuilder {
        self.settings = Some(settings);
        self
    }

    /// Build the ClassificationRequest from the ClassificationRequestBuilder
    ///
    /// # Example
    /// Using ClassificationRequestBuilder
    /// ```rust
    /// use weaviate_community::collections::classification::{
    ///     ClassificationRequestBuilder,
    ///     ClassificationType
    /// };
    ///
    /// let builder = ClassificationRequestBuilder::new()
    ///     .with_type(ClassificationType::KNN)
    ///     .with_class("Article")
    ///     .with_classify_properties(vec!["hasPopularity"])
    ///     .with_based_on_properties(vec!["summary"])
    ///     .with_filters(serde_json::json!(
    ///         {"path": ["wordCount"], "operator": "GreaterThan", "valueInt": 100}
    ///     ))
    ///     .with_settings(serde_json::json!({"k": 3}))
    ///     .build();
    /// ```
    ///
    /// Using ClassificationRequest
    /// ```rust
    /// use weaviate_community::collections::classification::{
    ///     ClassificationRequest,
    ///     ClassificationType
    /// };
    ///
    /// let builder = ClassificationRequest::builder()
    ///     .with_type(ClassificationType::KNN)
    ///     .with_class("Article")
    ///     .with_classify_properties(vec!["hasPopularity"])
    ///     .with_based_on_properties(vec!["summary"])
    ///     .with_filters(serde_json::json!(
    ///         {"path": ["wordCount"], "operator": "GreaterThan", "valueInt": 100}
    ///     ))
    ///     .with_settings(serde_json::json!({"k": 3}))
    ///     .build();
    /// ```
    pub fn build(self) -> ClassificationRequest {
        ClassificationRequest {
            classification_type: self.classification_type,
            class: self.class,
            classify_properties: self.classify_properties,
            based_on_properties: self.based_on_properties,
            filters: self.filters,
            settings: self.settings,
        }
    }
}

/// Types of classification available
#[derive(Serialize, Deserialize, Debug, Default)]
pub enum ClassificationType {
    #[default]
    #[serde(rename = "knn")]
    KNN,
    #[serde(rename = "zeroshot")]
    ZEROSHOT,
}

/// Response received from the classification
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationResponse {
    pub id: String,
    pub class: String,
    pub classify_properties: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub based_on_properties: Option<Vec<String>>,
    pub status: String,
    pub meta: ClassificationMetadata,
    #[serde(rename = "type")]
    pub classification_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    pub filters: serde_json::Value,
}

/// Metadata for the Classification
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationMetadata {
    pub started: String,
    pub completed: String,
    pub count: u64,
    pub count_succeeded: u64,
    pub count_failed: u64,
}
