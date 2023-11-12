# Weaviate Community

[![Crates.io][crates-badge]][crates-url]
[![CircleCI][circleci-badge]][circleci-url]
[![License][license-badge]][github-url]

[crates-badge]: https://img.shields.io/crates/v/weaviate-community.svg
[license-badge]: https://img.shields.io/badge/license-MIT-green.svg
[circleci-badge]: https://circleci.com/gh/Sam-Pewton/weaviate-community.rust.svg?style=shield
[crates-url]: https://crates.io/crates/weaviate-community
[circleci-url]: https://app.circleci.com/pipelines/github/Sam-Pewton/weaviate-community.rust
[github-url]: https://github.com/Sam-Pewton/weaviate-community.rust
[mit-url]: https://opensource.org/license/mit/
[rsdocs-url]: https://docs.rs/weaviate-community/0.1.0/weaviate_community/
[weaviate-url]: https://weaviate.io/developers/weaviate

Community client for handling Weaviate transactions written in Rust, for Rust.

More information on Weaviate can be found on the official [Weaviate][weaviate-url] webpage.


# Installation
Run the following in your project directory
```bash
cargo add weaviate-community
```

or add the following to your `Cargo.toml` file
```text
weaviate-community = "0.1.0"
```

Note: the latest version is not yet published to crates.io. Once all the features are implemented,
a new release version will be created.

# Documentation
The library reference documentation can be found [here][rsdocs-url]

# Usage
Below are some examples on how to interact with the Weaviate community Rust client.

## Creating a new WeaviateClient
```rust
use std::error::Error;
use weaviate_community::WeaviateClient;
use weaviate_community::collections::auth::AuthApiKey;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // With anonymous access
    let client = WeaviateClient::builder("http://localhost:8080")
        .build()?;
    
    // With Auth key
    let client = WeaviateClient::builder("http://localhost:8080")
        .with_auth_secret(AuthApiKey::new("your-key"))
        .build()?;

    // With multiple other API key (eg, OpenAI, JinaAI, ..)
    let client = WeaviateClient::builder("http://localhost:8080")
        .with_auth_secret(AuthApiKey::new("your-key"))
        .with_api_key("X-OpenAI-Api-Key", "abcdefg")
        .with_api_key("X-Jinaai-Api-Key", "hijklmn")
        .build()?;

    Ok(())
}
```

## Schema endpoints
```rust
use weaviate_community::collections::schema::{
    Class,
    Property,
    ShardStatus,
    Tenants,
    Tenant,
    ActivityStatus
};

async fn schema_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Get full schema
    let res = client.schema.get().await?;

    // Get the schema for a single class
    let res = client.schema.get_class("Article").await?;

    // Create a new class in the schema
    let my_class = Class::builder("Article").with_description("News article").build();
    let res = client.schema.create_class(&my_class).await?;

    // Update a class in the schema
    let my_class = Class::builder("Article").with_description("Updated information").build();
    let res = client.schema.update(&my_class).await?;

    // Add a property to a class
    let property = Property::builder("title", vec!["text"]).build();
    let res = client.schema.add_property("Article", &property).await?;

    // Get the shards for a class
    let res = client.schema.get_shards("Article").await?;

    // Update a class shard
    let res = client.schema.update_class_shard("Article", "abcdefg", ShardStatus::READONLY).await?;

    // List tenants for a class
    let res = client.schema.list_tenants("Article").await?;

    // Add tenants to a class
    let tenants = Tenants::new(vec![Tenant::builder("TENANT_B").build()]);
    let res = client.schema.add_tenants("Article", &tenants).await?;

    // Update tenants
    let tenants = Tenants::new(
        vec![
            Tenant::builder("TENANT_B").with_activity_status(ActivityStatus::COLD).build()
        ]
    );
    let res = client.schema.update_tenants("Article", &tenants).await?;

    // Remove tenants from a class
    let tenants = vec!["TENANT_B"];
    let res = client.schema.add_tenants("Article", &tenants).await?;

    // Delete a class from the schema
    let res = client.schema.delete("Article").await?;

    Ok(())
}
```

## Objects endpoints
```rust
use uuid::Uuid;
use weaviate_community::collections::objects::{Object, ObjectListParameters};

async fn objects_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // List every single object
    let res = client.objects.list(ObjectListParameters::new()).await?;

    // List all objects for a single class
    let params = ObjectListParameters::builder().with_class_name("Article").build();
    let res = client.objects.list(params).await?;

    // Create a new object
    let my_object = Object::builder("Article", serde_json::json![{}]).build();
    let res = client.objects.create(&my_object, None).await?;

    // Get an object based on its UUID
    let uuid = Uuid::new_v4();
    let res = client.objects.get("Article", uuid, None, None, None).await?;

    // Check if a data object exists
    let uuid = Uuid::new_v4();
    let res = client.objects.exists("Article", uuid, None, None).await?;

    // Update a data object
    let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303")?;
    let properties = serde_json::json!({
        "title": "new title",
    });
    let res = client.objects.update(&properties, "Article", &uuid, None).await?;

    // Replace a data object
    let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303")?;
    let properties = serde_json::json!({
        "properties": {
            "author": "Jodi Kantor",
        }
    });
    let res = client.objects.replace(&properties, "Publication", &uuid, None).await?;

    // Delete a data object
    let uuid = Uuid::parse_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303")?;
    let res = client.objects.delete("Article", &uuid, None, None).await?;

    // Validate a data object
    let properties = serde_json::json!({
        "name": "New York Times"
    });
    let uuid = Uuid::parse_str("12345678-1234-1234-1234-123456789012")?;
    let res = client.objects.validate("Publication", &properties, &uuid).await?;

    // Add a cross-reference
    let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012")?;
    let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef")?;
    let reference = Reference::new(
        "JeopardyQuestion", 
        &uuid1,
        "hasCategory", 
        "JeopardyCategory",
        &uuid2,
    );
    let res = client.objects.reference_add(reference).await?;

    // Update a cross-reference
    let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012")?;
    let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef")?;
    let res = client.objects.reference_update(
        "JeopardyQuestion", 
        &uuid1,
        "hasCategory", 
        vec!["JeopardyCategory"],
        vec![&uuid2],
        None,
        None
    ).await?;

    // Delete a cross-reference
    let uuid1 = Uuid::parse_str("12345678-1234-1234-1234-123456789012")?;
    let uuid2 = Uuid::parse_str("20ffc68d-986b-5e71-a680-228dba18d7ef")?;
    let reference = Reference::new(
        "JeopardyQuestion", 
        &uuid1,
        "hasCategory", 
        "JeopardyCategory",
        &uuid2,
    );
    let res = client.objects.reference_delete(reference).await?;

    Ok(())
}
```

## Backups endpoints
```rust
use weaviate_community::collections::backups::{
    BackupCreateRequest,
    BackupRestoreRequest,
    BackupBackends
};

async fn backups_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Create a new backup - with wait for completion
    let req = BackupCreateRequest::builder("my-backup").build();
    let res = client.backups.create(BackupBackends::FILESYSTEM, req, true).await?;

    // Create a new backup - without wait for completion
    let req = BackupCreateRequest::builder("my-backup").build();
    let res = client.backups.create(BackupBackends::FILESYSTEM, req, false).await?;

    // Get the status of a backup create
    let res = client.backups.get_backup_status(
        BackupBackends::FILESYSTEM,
        "my-backup",
        false
    ).await?;

    // Restore a backup - with wait for completion
    let req = BackupRestoreRequest::builder().build();
    let res = client.backups.restore(BackupBackends::FILESYSTEM, "my-backup", req, true).await?;

    // Restore a backup - without wait for completion
    let req = BackupRestoreRequest::builder().build();
    let res = client.backups.restore(BackupBackends::FILESYSTEM, "my-backup", req, false).await?;

    // Get the status of a backup restore
    let res = client.backups.get_backup_status(
        BackupBackends::FILESYSTEM,
        "my-backup",
        true
    ).await?;

    Ok(())
}
```

## Batch endpoints
```rust
use uuid::Uuid;
use weaviate_community::collections::objects::{
    Object, 
    MultiObject,
    Reference, 
    References, 
    ConsistencyLevel,
};
use weaviate_community::collections::batch::{BatchDeleteRequest, MatchConfig};
async fn batch_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Batch add objects
    let author_uuid = Uuid::parse_str("36ddd591-2dee-4e7e-a3cc-eb86d30a4303").unwrap();
    let article_a_uuid = Uuid::parse_str("6bb06a43-e7f0-393e-9ecf-3c0f4e129064").unwrap();
    let article_b_uuid = Uuid::parse_str("b72912b9-e5d7-304e-a654-66dc63c55b32").unwrap();

    let article_a = Object::builder("Article", serde_json::json!({}))
        .with_id(article_a_uuid.clone())
        .build();

    let article_b = Object::builder("Article", serde_json::json!({}))
        .with_id(article_b_uuid.clone())
        .build();

    let author = Object::builder("Author", serde_json::json!({}))
        .with_id(author_uuid.clone())
        .build();

    let res = client.batch.objects_batch_add(
        MultiObjects::new(vec![article_a, article_b, author]), Some(ConsistencyLevel::ALL)
    ).await;

    // Batch delete objects
    let req = BatchDeleteRequest::builder(
        MatchConfig::new(
            "Article",
            serde_json::json!({
                "operator": "Like",
                "path": ["id"],
                "valueText": "*4*",
            })
        )
    ).build();
    let res = client.batch.objects_batch_delete(req, Some(ConsistencyLevel::ALL)).await;

    // Batch add references
    let references = References::new(vec![
        Reference::new(
            "Author",
            &author_uuid,
            "wroteArticles",
            "Article",
            &article_a_uuid,
        ),
        Reference::new(
            "Author",
            &author_uuid,
            "wroteArticles",
            "Article",
            &article_b_uuid,
        ),
    ]);
    let res = client.batch.references_batch_add(references, Some(ConsistencyLevel::ALL)).await;
    
    Ok(())
}
```

## Meta endpoint
```rust
async fn meta_endpoint(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Get database metadata
    let res = client.meta.get_meta().await?;

    Ok(())
}
```

## Nodes endpoint
```rust
async fn nodes_endpoint(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Get the nodes status'
    let res = client.nodes.get_nodes_status().await?;

    Ok(())
}
```

## OIDC endpoint
```rust
async fn oidc_endpoint(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Get the OIDC config
    let res = client.oidc.get_open_id_configuration().await?;

    Ok(())
}
```

## Querying
```rust
use weaviate_community::collections::query::{
    GetQuery,
    AggregateQuery,
    ExploreQuery,
    RawQuery
};
async fn querying(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Get
    let query = GetQuery::builder(
        "JeopardyQuestion", 
        vec![
            "question",
            "answer",
            "points",
            "hasCategory { ... on JeopardyCategory { title }}"
        ])
        .with_limit(1)
        .with_additional(vec!["id"])
        .build();
    let res = client.query.get(query).await;

    // Aggregate
    let query = AggregateQuery::builder("Article")
        .with_meta_count()
        .with_fields(vec!["wordCount {count maximum mean median minimum mode sum type}"])
        .build();
    let res = client.query.aggregate(query).await;

    // Explore
    let query = ExploreQuery::builder()
        .with_limit(1)
        .with_near_vector("{vector: [-0.36840257,0.13973749,-0.28994447]}")
        .with_fields(vec!["beacon", "className", "certainty"])
        .build();
    let res = client.query.explore(query).await;

    // Raw
    let query = RawQuery::new("{ Get { JeopardyQuestion { question answer points } } }");
    let res = client.query.raw(query).await?;

    Ok(())
}
```

## Health endpoints
```rust
async fn health_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Check database is live
    let res = client.is_live().await?;

    // Check database is ready
    let res = client.is_ready().await?;

    Ok(())
}
```

## Classification endpoints
```rust
use uuid::Uuid;
use weaviate_community::collections::classification::{
    ClassificationRequest,
    ClassificationType
};
async fn classification_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Schedule a new classification
    let req = ClassificationRequest::builder()
        .with_type(ClassificationType::KNN)
        .with_class("Article")
        .with_based_on_properties(vec!["summary"])
        .with_classify_properties(vec!["hasPopularity"])
        .with_filters(serde_json::json!({
            "trainingSetWhere": {
                "path": ["wordCount"],
                "operator": "GreaterThan",
                "valueInt": 100
            }
        }))
        .with_settings(serde_json::json!({
            "k": 3
        }))
        .build();
    let res = client.classification.schedule(req).await?;

    // Get the status of a classification
    let uuid = Uuid::parse_str("00037775-1432-35e5-bc59-443baaef7d80")?;
    let res = client.classification.get(uuid).await?;

    Ok(())
}
```

# Roadmap
- SI test update
- Improvements to the GraphQL query system (and batch delete match config)
- Module system for interacting with enabled modules
- Create full schema in one command
- General improvements to try and remove as much serde_json in the deserialized objects..
- Embedded functionality
- gRPC (after beta testing for official clients)

# Contributing
Any bug reports and feature requests welcome on [GitHub][github-url]

# License
This project is licensed under the [MIT License][mit-url] open-source license.
