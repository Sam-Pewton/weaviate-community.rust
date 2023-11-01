# Weaviate Community

[![Crates.io][crates-badge]][crates-url]
[![CircleCI][circleci-badge][circleci-url]
[![License][license-badge][github-url]

[license-badge]: https://img.shields.io/badge/license-MIT-green.svg
[circleci-badge]: https://circleci.com/gh/Sam-Pewton/weaviate-community.rust.svg?style=shield
[crates-badge]: https://img.shields.io/crates/v/tokio.svg
[crates-url]: https://crates.io/crates/weaviate-community
[circleci-url]: https://app.circleci.com/pipelines/github/Sam-Pewton/weaviate-community.rust
[github-url]: https://github.com/Sam-Pewton/weaviate-community.rust
[mit-url]: https://opensource.org/license/mit/
[rsdocs-url]: https://docs.rs/weaviate-community/0.1.0/weaviate_community/
[weaviate-url]: https://weaviate.io/developers/weaviate

Community client for handling Weaviate transactions written in Rust, for Rust.

More information on Weaviate can be found on the official [Weaviate][weaviate-url] webpage.


# Installation
```bash
```
Note: the latest version is not yet published to crates.io. Once all the features are implemented,
a new release version will be created.

# Documentation
The library reference documentation can be found [here][rsdocs-url]

# Crate Features
TODO

# Usage
Below are some examples on how to interact with the Weaviate community Rust client.

## Creating a new WeaviateClient
```rust
use weaviate_community::WeaviateClient;
use weaviate_community::collections::auth::AuthApiKey;

// With anonymous access
let client = WeaviateClient::builder("http://localhost:8080").build();

// With Auth key
let client = WeaviateClient::builder("http://localhost:8080")
    .auth_secret(Some(AuthApiKey::new("your-key")))
    .build();
```

## Schema endpoints
```rust
todo!()
```

## Objects endpoints
```rust
todo!()
```

## Backups endpoints
```rust
todo!()
```

## Batch endpoints
```rust
todo!()
```

## Meta endpoint
```rust
todo!()
```

## Meta endpoint
```rust
todo!()
```

## Nodes endpoint
```rust
todo!()
```

## OIDC endpoint
```rust
todo!()
```

## Querying
```rust
todo!()
```

## Health endpoints
```rust
todo!()
```

# Roadmap
- Implementation of the builder pattern on most of the data types
- Comprehensive unit test update
- SI test update
- CI/CD update
- Classification endpoints
- Improvements to the GraphQL query system
- Module system for interacting with enabled modules
- External auth keys in client (for OpenAI, HuggingFace, etc.)

# Contributing
Any bug reports and feature requests welcome on [GitHub][github-url]

# License
This project is licensed under the [MIT License][mit-url] open-source license.
