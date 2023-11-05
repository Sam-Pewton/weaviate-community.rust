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
        .auth_secret(Some(AuthApiKey::new("your-key")))
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
    let my_class = Class::builder("Article", "All article information").build();
    let res = client.schema.create_class(&my_class).await?;

    // Update a class in the schema
    let my_class = Class::builder("Article", "Updated information").build();
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
async fn objects_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
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
    let res = client.backups.restore(BackupBackends::FILESYSTEM, "my-backup", req, true).await?;

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
async fn batch_endpoints(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
    // Batch add objects
    todo!();

    // Batch delete objects
    todo!();

    // Batch add references
    todo!();

    // Batch delete references
    todo!();


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
async fn querying(client: WeaviateClient) -> Result<(), Box<dyn Error>> {
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

# Roadmap
- Comprehensive unit test update
- SI test update
- CI/CD update
- Classification endpoints
- Improvements to the GraphQL query system
- Module system for interacting with enabled modules
- External auth keys in client (for OpenAI, HuggingFace, etc.)
- Embedded functionality
- Create full schema in one command
- gRPC (longer term)

# Contributing
Any bug reports and feature requests welcome on [GitHub][github-url]

# License
This project is licensed under the [MIT License][mit-url] open-source license.
