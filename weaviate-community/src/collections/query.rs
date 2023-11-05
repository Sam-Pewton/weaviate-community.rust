/// GraphQL related structures.
///
/// Contains the ability to generate the following queries:
/// - Get
/// - Aggregate
/// - Explore
///
/// Also contains the ability to create a raw query from a string.
///
/// This is currently incomplete - most of the data types for the different options are Strings,
/// which I want to enforce to the expected values a little better. Mainly just getting the
/// structure and ability to run completed now.
///
/// There are also some places I need to return an error from which I am yet to do.
///
/// I've also not had a chance to test a lot of the functionality, so lots will be broken like the
/// near_text or near_image as I have not implemented the `encoding` functionality yet.
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// RawQuery struct to hold a custom `raw` query.
#[derive(Serialize, Deserialize, Debug)]
pub struct RawQuery {
    pub query: String,
}

impl RawQuery {
    /// Retrieve a raw GraphQL query.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::RawQuery;
    ///
    /// let my_query_str = "{
    ///   Get {
    ///     JeopardyQuestion {
    ///       question
    ///       answer
    ///       points
    ///     }
    ///   }
    /// }";
    ///
    /// let query = RawQuery::new(my_query_str);
    /// ```
    pub fn new(query: &str) -> Self {
        RawQuery { query: query.into() }
    }
}

/// AggregatorQuery struct to hold an Aggregate query.
#[derive(Serialize, Deserialize, Debug)]
pub struct AggregateQuery {
    pub query: String,
}

impl AggregateQuery {
    /// Create a new `AggregateBuilder` for the GraphQL `AggregateQuery`.
    ///
    /// This is the same as `AggregateBuilder::new()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateQuery;
    ///
    /// let query = AggregateQuery::builder("Article");
    /// ```
    pub fn builder(class_name: &str) -> AggregateBuilder {
        AggregateBuilder::new(class_name)
    }
}

/// The builder for the `AggregateQuery`.
#[derive(Serialize, Deserialize, Debug)]
pub struct AggregateBuilder {
    pub class_name: String,
    pub object_limit: Option<u32>,
    pub meta_count: Option<bool>,
    pub fields: Option<Vec<String>>,
    pub where_clause: Option<String>,
    pub group_by: Option<String>,
    pub near: Option<String>,
    pub tenant: Option<String>,
    pub limit: Option<u32>,
}

impl AggregateBuilder {
    /// Create a new AggregateBuilder item.
    ///
    /// This is the same as `AggregateQuery::builder()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query_builder = AggregateBuilder::new("Article");
    /// ```
    pub fn new(class_name: &str) -> Self {
        AggregateBuilder {
            class_name: class_name.into(),
            object_limit: None,
            meta_count: None,
            fields: None,
            where_clause: None,
            group_by: None,
            near: None,
            tenant: None,
            limit: None,
        }
    }

    /// Set the `objectLimit: <value>` as a filter to the query to limit the vector search results
    /// used within the aggregation.
    ///
    /// Should only be set in conjunction when used in conjunction with a `near` filter (for
    /// example, `with_near_text()`
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query_builder = AggregateBuilder::new("Article")
    ///     .with_object_limit(1);
    /// ```
    pub fn with_object_limit(mut self, value: u32) -> AggregateBuilder {
        self.object_limit = Some(value);
        self
    }

    /// Add `meta{count}` to the body of the query when called.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query_builder = AggregateBuilder::new("Article")
    ///     .with_meta_count();
    /// ```
    pub fn with_meta_count(mut self) -> AggregateBuilder {
        self.meta_count = Some(true);
        self
    }

    /// Appends the specified fields in the aggregate query body.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query_builder = AggregateBuilder::new("Article")
    ///     .with_fields(vec!["wordCount { mean }"]);
    /// ```
    pub fn with_fields(mut self, fields: Vec<&str>) -> AggregateBuilder {
        let fields = fields.iter().map(|field| field.to_string()).collect();
        self.fields = Some(fields);
        self
    }

    /// Set the `where` filter in the aggregate query.
    ///
    /// # Example -> todo
    /// ```
    /// ```
    pub fn with_where(mut self, where_clause: &str) -> AggregateBuilder {
        self.where_clause = Some(where_clause.into());
        self
    }

    /// Set the `group_by` filter in the aggregate query.
    ///
    /// This may also require `groupedBy {...}` to be specified in with_fields().
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query_builder = AggregateBuilder::new("Article")
    ///     .with_group_by_filter("[\"inPublication\"]")
    ///     .with_fields(vec!["groupedBy {value path}"]);
    /// ```
    pub fn with_group_by_filter(mut self, group_by: &str) -> AggregateBuilder {
        self.group_by = Some(group_by.into());
        self
    }

    /// Set the `nearText` filter in the aggregate query. This filter can be used with text modules
    /// (text2vec).
    ///
    /// Note that the `autocorrect` field is only available with the `text-spellcheck` Weaviate
    /// module.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_text(mut self, near_text: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_text.into());
        self
    }

    /// Set the `nearVector` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_vector(mut self, near_vector: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_vector.into());
        self
    }

    /// Set the `nearObject` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_object(mut self, near_object: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_object.into());
        self
    }

    /// Set the `nearImage` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_image(mut self, near_image: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_image.into());
        self
    }

    /// Set the `nearAudio` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_audio(mut self, near_audio: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_audio.into());
        self
    }

    /// Set the `nearVideo` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_video(mut self, near_video: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_video.into());
        self
    }

    /// Set the `nearDepth` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_depth(mut self, near_depth: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_depth.into());
        self
    }

    /// Set the `nearThermal` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_thermal(mut self, near_thermal: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_thermal.into());
        self
    }

    /// Set the `nearIMU` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_imu(mut self, near_imu: &str) -> AggregateBuilder {
        if self.near.is_some() {
            // raise an error here, can only have one near filter
        }
        self.near = Some(near_imu.into());
        self
    }

    /// Set the `tenant` filter in the aggregate query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_tenant(mut self, tenant: &str) -> AggregateBuilder {
        self.tenant = Some(tenant.into());
        self
    }

    /// Set the `limit` filter in the aggregate query.
    ///
    /// Limits the number of results that are returned.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query_builder = AggregateBuilder::new("Article")
    ///     .with_limit(1);
    /// ```
    pub fn with_limit(mut self, limit: u32) -> AggregateBuilder {
        self.limit = Some(limit);
        self
    }
    
    /// Build the `AggregateQuery` to use within within a GraphQL Aggregate request.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::AggregateBuilder;
    ///
    /// let query = AggregateBuilder::new("Article")
    ///     .with_fields(vec!["wordCount {count maximum mean median minimum mode sum type}"])
    ///     .build();
    /// ```
    ///
    /// ```
    /// use weaviate_community::collections::query::AggregateQuery;
    ///
    /// let query = AggregateQuery::builder("Article")
    ///     .with_meta_count()
    ///     .with_fields(vec!["wordCount {count maximum mean median minimum mode sum type}"])
    ///     .build();
    /// ```
    ///
    /// Both examples will create the following AggregateQuery:
    /// ```text
    /// AggregateQuery {
    ///   query: "{
    ///     Aggregate {
    ///       Article
    ///       {
    ///         meta {count}
    ///         wordCount {count maximum mean median minimum mode sum type}
    ///       }
    ///     }
    ///   }"
    /// }
    /// ```
    pub fn build(&self) -> AggregateQuery {
        // Path
        let mut query = String::from("{\n");
        query.push_str("  Aggregate {\n");
        query.push_str(format!("    {} \n", self.class_name).as_str());

        // Filters
        if self.contains_filter() {
            query.push_str("    (\n");
            if let Some(where_clause) = &self.where_clause {
                query.push_str(format!("      where: {}\n", where_clause).as_str());
            }
            if let Some(group_by) = &self.group_by {
                query.push_str(format!("      groupBy: {}\n", group_by).as_str());
            }
            if let Some(near) = &self.where_clause {
                query.push_str(format!("      near: {}\n", near).as_str());
            }
            if let Some(object_limit) = &self.object_limit {
                query.push_str(format!("      objectLimit: {}\n", object_limit).as_str());
            }
            if let Some(tenant) = &self.tenant {
                query.push_str(format!("      tenant: {}\n", tenant).as_str());
            }
            if let Some(limit) = &self.limit {
                query.push_str(format!("      limit: {}\n", limit).as_str());
            }
            query.push_str("    )\n");
        }

        // Body
        query.push_str("    {\n");
        if let Some(_) = &self.meta_count {
            query.push_str("      meta{count}\n");
        }

        if let Some(fields) = &self.fields {
            query.push_str(format!("      {}\n", fields.join(" ")).as_str());
        }
        query.push_str("    }\n");
        query.push_str("  }\n");
        query.push_str("}");
        AggregateQuery { query }
    }

    /// Check if the query contains a filter.
    fn contains_filter(&self) -> bool {
        match
            self.where_clause.is_some() ||
            self.group_by.is_some() ||
            self.near.is_some() ||
            self.object_limit.is_some() ||
            self.tenant.is_some() ||
            self.limit.is_some()
        {
            true => true,
            false => false,
        }
    }
}

/// ExploreQuery struct to hold an Explore query.
#[derive(Serialize, Deserialize, Debug)]
pub struct ExploreQuery {
    pub query: String,
}

impl ExploreQuery {
    /// Create a new `ExploreBuilder` for the GraphQL `ExploreQuery`.
    ///
    /// This is the same as `ExploreBuilder::new()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::ExploreQuery;
    ///
    /// let query = ExploreQuery::builder();
    /// ```
    pub fn builder() -> ExploreBuilder {
        ExploreBuilder::new()
    }
}

/// The builder for the `ExploreQuery`
#[derive(Serialize, Deserialize, Debug)]
pub struct ExploreBuilder {
    limit: Option<u32>,
    near_text: Option<String>,
    near_vector: Option<String>,
    fields: Option<Vec<String>>,
}

impl ExploreBuilder {
    /// Create a new ExploreBuilder item.
    ///
    /// This is the same as `ExploreQuery::builder()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::ExploreBuilder;
    ///
    /// let query_builder = ExploreBuilder::new();
    /// ```
    pub fn new() -> Self {
        ExploreBuilder {
            limit: None,
            near_text: None,
            near_vector: None,
            fields: None,
        }
    }

    /// Appends the specified fields in the explore query body.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::ExploreBuilder;
    ///
    /// let query_builder = ExploreBuilder::new()
    ///     .with_fields(vec!["beacon", "certainty", "className"]);
    /// ```
    pub fn with_fields(mut self, fields: Vec<&str>) -> ExploreBuilder {
        let fields = fields.iter().map(|field| field.to_string()).collect();
        self.fields = Some(fields);
        self
    }

    /// Sets the `limit` in the explore query filters.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::ExploreBuilder;
    ///
    /// let query_builder = ExploreBuilder::new()
    ///     .with_limit(1);
    /// ```
    pub fn with_limit(mut self, limit: u32) -> ExploreBuilder {
        self.limit = Some(limit);
        self
    }

    /// Sets the `nearText` value in the explore query filters.
    ///
    /// One of either `with_near_text` or `with_near_vector` must be set in the query at point of
    /// build.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_text(mut self, near_text: &str) -> ExploreBuilder {
        self.near_text = Some(near_text.into());
        self
    }

    /// Sets the `nearVector` value in the explore query filters.
    ///
    /// One of either `with_near_text` or `with_near_vector` must be set in the query at point of
    /// build.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_vector(mut self, near_vector: &str) -> ExploreBuilder {
        self.near_vector = Some(near_vector.into());
        self
    }

    /// Build the `ExploreQuery` to use within within a GraphQL Explore request.
    ///
    /// # Examples -> todo: need to add a nearVector or nearText
    /// ```no_run
    /// use weaviate_community::collections::query::ExploreBuilder;
    ///
    /// let query = ExploreBuilder::new().build();
    /// ```
    ///
    /// ```no_run
    /// use weaviate_community::collections::query::ExploreQuery;
    ///
    /// let query = ExploreQuery::builder().build();
    /// ```
    ///
    /// Both examples will create the following ExploreQuery:
    /// ```text
    /// ```
    pub fn build(&self) -> ExploreQuery {
        if self.near_text.is_none() && self.near_vector.is_none() {
            // raise an error, one is required. TBD if other near fields can be used
        }

        // Path
        let mut query = String::from("{\n");
        query.push_str("  Explore\n");

        // Filters
        query.push_str("  (\n");
        if let Some(limit) = &self.limit {
            query.push_str(format!("    limit: {}\n", limit).as_str());
        }
        if let Some(near_text) = &self.near_text {
            query.push_str(format!("    nearText: {}\n", near_text).as_str());
        }
        if let Some(near_vector) = &self.near_vector {
            query.push_str(format!("    nearVector: {}\n", near_vector).as_str());
        }
        query.push_str("  )\n");

        // Body
        query.push_str("  {\n");
        if let Some(fields) = &self.fields {
            query.push_str(format!("    {}\n", fields.join(" ")).as_str());
        }
        query.push_str("  }\n");
        query.push_str("}");

        ExploreQuery { query }
    }
}

/// GetQuery struct to hold a Get query.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetQuery {
    pub query: String,
}

impl GetQuery {
    /// Create a new `GetBuilder` for the GraphQL `GetQuery`.
    ///
    /// This is the same as `GetBuilder::new()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetQuery;
    ///
    /// let query = GetQuery::builder("Article", vec!["author"]);
    /// ```
    pub fn builder(class_name: &str, properties: Vec<&str>) -> GetBuilder {
        GetBuilder::new(class_name, properties)
    }
}

/// The builder for the `GetQuery`
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBuilder {
    pub class_name: String,
    pub properties: Vec<String>,
    pub additional: Option<Vec<String>>,
    pub where_clause: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub after: Option<Uuid>, // cant use with where, near<media>, bm25, hybrid, etc
    pub near_text: Option<String>,
    pub near_vector: Option<String>,
    pub near_image: Option<String>,
    pub near_object: Option<String>,
    pub near_video: Option<String>,
    pub near_audio: Option<String>,
    pub near_thermal: Option<String>,
    pub near_imu: Option<String>,
    pub near_depth: Option<String>,
    pub sort: Option<String>,
    pub bm25: Option<String>,
    pub hybrid: Option<String>,
    pub group_by: Option<String>,
    pub tenant: Option<String>,
    pub autocut: Option<u32>,
    pub ask: Option<String>,
}

impl GetBuilder {
    /// Create a new GetBuilder item.
    ///
    /// This is the same as `GetQuery::builder()`.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// );
    /// ```
    pub fn new(class_name: &str, properties: Vec<&str>) -> GetBuilder {
        GetBuilder {
            class_name: class_name.into(),
            properties: properties.iter().map(|prop| prop.to_string()).collect(),
            limit: None,
            offset: None,
            additional: None,
            tenant: None,
            autocut: None,
            after: None,
            sort: None,
            where_clause: None,
            near_text: None,
            near_vector: None,
            near_image: None,
            near_object: None,
            near_video: None,
            near_audio: None,
            near_thermal: None,
            near_imu: None,
            near_depth: None,
            hybrid: None,
            bm25: None,
            ask: None,
            group_by: None,
        }
    }

    /// Sets the `limit` in the get query filters.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// ).with_limit(1);
    /// ```
    pub fn with_limit(mut self, limit: u32) -> GetBuilder {
        self.limit = Some(limit);
        self
    }

    /// Set the `offset` in the get query filters.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// )
    ///     .with_limit(1)
    ///     .with_offset(1);
    /// ```
    pub fn with_offset(mut self, offset: u32) -> GetBuilder {
        self.offset = Some(offset);
        self
    }

    /// Specify the `_additional` properties to retrieve in the query result.
    ///
    /// Note that the additional properties are properties that cannot be specified in the regular
    /// properties field, such as the `vector`, or the object UUID (`id`). More `_additional`
    /// properties are described [here](https://weaviate.io/developers/weaviate/api/graphql/additional-properties).
    ///
    /// Cross referenced properties should be specified in the regular properties field (in the
    /// `new` method).
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new("JeopardyQuestion", vec![])
    ///     .with_additional(vec!["vector"]);
    /// ```
    pub fn with_additional(mut self, additional: Vec<&str>) -> GetBuilder {
        let additional = additional.iter().map(|item| item.to_string()).collect();
        self.additional = Some(additional);
        self
    }

    /// Specify the `tenant` in the get query filter.
    ///
    /// For classes that have multi-tenancy enabled, the tenant parameter must be specified in each
    /// query.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new("JeopardyQuestion", vec!["answer"])
    ///     .with_tenant("tenantA");
    /// ```
    pub fn with_tenant(mut self, tenant: &str) -> GetBuilder {
        self.tenant = Some(tenant.into());
        self
    }

    /// Specify the `autocut` search filter in the get query.
    ///
    /// The `autocut` filter is an argument that can be added to class objects retrieved by the
    /// `near<media>`, `bm25`, and `hybrid` operators.
    ///
    /// More information on `autocut` can be found [here](https://weaviate.io/developers/weaviate/api/graphql/additional-operators#autocut)
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new("JeopardyQuestion", vec!["question", "answer"])
    ///     .with_hybrid("{query: \"food\"}")
    ///     .with_autocut(1)
    ///     .build();
    /// ```
    pub fn with_autocut(mut self, autocut: u32) -> GetBuilder {
        self.autocut = Some(autocut);
        self
    }

    /// Specify the `after` search filter in the get query.
    ///
    /// The `after` operator can be used to sequentially retrieve class objects from Weaviate.
    ///
    /// More information on `after` can be found [here](https://weaviate.io/developers/weaviate/api/graphql/additional-operators#cursor-with-after)
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_after(mut self, after: Uuid) -> GetBuilder {
        self.after = Some(after);
        self
    }

    /// Specify the `sort` search filter in the get query.
    ///
    /// Any primitive property types can be sorted, such as `text`, `string`, `number`, or `int`.
    ///
    /// When a query has a natural order (e.g. because of a near<media> vector search), adding a
    /// sort operator will override that order.
    ///
    /// More on sorting in Weaviate can be found [here](https://weaviate.io/developers/weaviate/api/graphql/additional-operators#sorting)
    pub fn with_sort(mut self, sort: &str) -> GetBuilder {
        self.sort = Some(sort.into());
        self
    }

    /// Specify conditionals to add to the `where` search filter in the get query.
    ///
    /// More information on conditionals can be found [here](https://weaviate.io/developers/weaviate/api/graphql/filters)
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_where(mut self, where_clause: &str) -> GetBuilder {
        self.where_clause = Some(where_clause.into());
        self
    }

    /// Set the `nearText` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_text(mut self, near_text: &str) -> GetBuilder {
        self.near_text = Some(near_text.into());
        self
    }

    /// Set the `nearVector` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_vector(mut self, near_vector: &str) -> GetBuilder {
        self.near_vector = Some(near_vector.into());
        self
    }

    /// Set the `nearObject` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_object(mut self, near_object: &str) -> GetBuilder {
        self.near_object = Some(near_object.into());
        self
    }

    /// Set the `nearImage` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_image(mut self, near_image: &str) -> GetBuilder {
        self.near_image = Some(near_image.into());
        self
    }

    /// Set the `nearVideo` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_video(mut self, near_video: &str) -> GetBuilder {
        self.near_video = Some(near_video.into());
        self
    }

    /// Set the `nearAudio` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_audio(mut self, near_audio: &str) -> GetBuilder {
        self.near_audio = Some(near_audio.into());
        self
    }

    /// Set the `nearThermal` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_thermal(mut self, near_thermal: &str) -> GetBuilder {
        self.near_thermal = Some(near_thermal.into());
        self
    }

    /// Set the `nearIMU` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_imu(mut self, near_imu: &str) -> GetBuilder {
        self.near_imu = Some(near_imu.into());
        self
    }

    /// Set the `nearDepth` filter in the get query.
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_near_depth(mut self, near_depth: &str) -> GetBuilder {
        self.near_depth = Some(near_depth.into());
        self
    }

    /// Specify the `hybrid` search filter in the get query.
    ///
    /// The `hybrid` operator produces results based on a weighted combination of results from a
    /// keyword (bm25) search and a vector (near<media>) search.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new("JeopardyQuestion", vec!["question", "answer"])
    ///     .with_hybrid("{query: \"food\"}")
    ///     .with_limit(3)
    ///     .build();
    /// ```
    ///
    /// This will generate the following GetQuery:
    /// ```text
    /// GetQuery {
    ///   query: "{
    ///     Get {
    ///       JeopardyQuestion
    ///       (
    ///         limit: 3
    ///         hybrid: {query: "food"]
    ///       )
    ///       {
    ///         question
    ///         answer
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    pub fn with_hybrid(mut self, hybrid: &str) -> GetBuilder {
        self.hybrid = Some(hybrid.into());
        self
    }

    /// Specify the `bm25` search filter in the get query.
    ///
    /// To use BM25 search, you must provide a search string as a minimum.
    ///
    /// More on Keyword (BM25) search can be found [here](https://weaviate.io/developers/weaviate/search/bm25)
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query_builder = GetBuilder::new("JeopardyQuestion", vec!["question", "answer"])
    ///     .with_bm25("{query: \"food\"}")
    ///     .with_limit(3)
    ///     .build();
    /// ```
    ///
    /// This will generate the following GetQuery:
    /// ```text
    /// GetQuery {
    ///   query: "{
    ///     Get {
    ///       JeopardyQuestion
    ///       (
    ///         limit: 3
    ///         bm25: {query: "food"]
    ///       )
    ///       {
    ///         question
    ///         answer
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    /// and would look for objects containing the keyword `food` anywhere in the object if ran.
    pub fn with_bm25(mut self, bm25: &str) -> GetBuilder {
        self.bm25 = Some(bm25.into());
        self
    }

    /// Specify the `groupBy` in the get query filters.
    ///
    /// To use `groupBy`:
    /// - Provide the property by which the results should be grouped,
    /// - The maximum number of groups, and
    /// - The maximum number of objects per group
    ///
    /// # Example
    /// ```
    /// ```
    pub fn with_group_by(mut self, group_by: &str) -> GetBuilder {
        self.group_by = Some(group_by.into());
        self
    }

    ///
    pub fn with_ask(mut self, ask: &str) -> GetBuilder {
        self.ask = Some(ask.into());
        self
    }
    
    /// Build the `GetQuery` to use within within a GraphQL Get request.
    ///
    /// # Example
    /// ```
    /// use weaviate_community::collections::query::GetBuilder;
    ///
    /// let query = GetBuilder::new(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// ).build();
    /// ```
    ///
    /// ```
    /// use weaviate_community::collections::query::GetQuery;
    ///
    /// let query = GetQuery::builder(
    ///     "JeopardyQuestion",
    ///     vec!["question", "answer", "points"]
    /// ).build();
    /// ```
    ///
    /// Both examples will create the following GetQuery:
    /// ```text
    /// GetQuery {
    ///   query: "{
    ///     Get {
    ///       JeopardyQuestion
    ///       {
    ///         question
    ///         answer
    ///         points
    ///       }
    ///     }
    ///   }"
    /// }
    /// ```
    pub fn build(&self) -> GetQuery {

        // Path
        let mut query = String::from("{\n");
        query.push_str("  Get {\n");
        query.push_str(format!("    {} \n", self.class_name).as_str());

        // Filters
        if self.contains_filter() {
            query.push_str("    (\n");
            if let Some(where_clause) = &self.where_clause {
                query.push_str(format!("      where: {}\n", where_clause).as_str());
            }
            if let Some(limit) = &self.limit {
                query.push_str(format!("      limit: {}\n", limit).as_str());
            }
            if let Some(offset) = &self.offset {
                query.push_str(format!("      offset: {}\n", offset).as_str());
            }
            if let Some(near_text) = &self.near_text {
                query.push_str(format!("      nearText: {}\n", near_text).as_str());
            }
            if let Some(near_vector) = &self.near_vector {
                query.push_str(format!("      nearVector: {}\n", near_vector).as_str());
            }
            if let Some(near_object) = &self.near_object {
                query.push_str(format!("      nearObject: {}\n", near_object).as_str());
            }
            if let Some(near_image) = &self.near_image {
                query.push_str(format!("      nearImage: {}\n", near_image).as_str());
            }
            if let Some(near_audio) = &self.near_audio {
                query.push_str(format!("      nearAudio: {}\n", near_audio).as_str());
            }
            if let Some(near_video) = &self.near_video {
                query.push_str(format!("      nearVideo: {}\n", near_video).as_str());
            }
            if let Some(near_thermal) = &self.near_thermal {
                query.push_str(format!("      nearThermal: {}\n", near_thermal).as_str());
            }
            if let Some(near_imu) = &self.near_imu {
                query.push_str(format!("      nearIMU: {}\n", near_imu).as_str());
            }
            if let Some(near_depth) = &self.near_depth {
                query.push_str(format!("      nearDepth: {}\n", near_depth).as_str());
            }
            if let Some(bm25) = &self.bm25 {
                query.push_str(format!("      bm25: {}\n", bm25).as_str());
            }
            if let Some(hybrid) = &self.hybrid {
                query.push_str(format!("      hybrid: {}\n", hybrid).as_str());
            }
            if let Some(group_by) = &self.group_by {
                query.push_str(format!("      group_by: {}\n", group_by).as_str());
            }
            if let Some(after) = &self.after {
                query.push_str(format!("      after: {}\n", after).as_str());
            }
            if let Some(tenant) = &self.tenant {
                query.push_str(format!("      tenant: {}\n", tenant).as_str());
            }
            if let Some(autocut) = &self.autocut {
                query.push_str(format!("      autocut: {}\n", autocut).as_str());
            }

            if let Some(sort) = &self.sort {
                query.push_str(format!("      sort: {}\n", sort).as_str());
            }
            if let Some(ask) = &self.ask {
                query.push_str(format!("      ask: {}\n", ask).as_str());
            }
            query.push_str("    )\n");
        }

        // Body
        query.push_str("    {\n");
        query.push_str(format!("      {}\n", self.properties.join(" ")).as_str());

        if let Some(additional) = &self.additional {
            query.push_str("      _additional {\n");
            query.push_str(format!("        {}\n", additional.join(" ")).as_str());
            query.push_str("      }\n");

        }
        query.push_str("    }\n");
        query.push_str("  }\n");
        query.push_str("}");
        GetQuery { query }
    }

    /// Check if the query contains a filter.
    fn contains_filter(&self) -> bool {
        match
            self.limit.is_some() || 
            self.offset.is_some() || 
            self.after.is_some() || 
            self.autocut.is_some() || 
            self.tenant.is_some() ||
            self.where_clause.is_some() ||
            self.near_text.is_some() ||
            self.near_vector.is_some() ||
            self.near_image.is_some() ||
            self.near_object.is_some() ||
            self.hybrid.is_some() ||
            self.bm25.is_some() ||
            self.sort.is_some() ||
            self.ask.is_some()
        {
            true => true,
            false => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GetBuilder;

    #[test]
    fn test_get_query_builder() {
        let query = GetBuilder::new(
            "JeopardyQuestion", 
            vec![
                "question".into(),
                "answer".into(),
                "points".into(),
                "hasCategory { ... on JeopardyCategory { title }}".into()
            ])
            .with_limit(1)
            .with_offset(1);
        //println!("{}", query.build());
    }
}
