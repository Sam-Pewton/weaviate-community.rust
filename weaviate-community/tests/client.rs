use weaviate_community::WeaviateClient;
use weaviate_community::collections::auth::AuthApiKey;

/// Test that the is_live endpoint returns true when it is expected to.
#[tokio::test]
async fn test_is_live_true() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::builder("http://localhost:8080")
        .auth_secret(auth)
        .build().unwrap();
    let res = client.is_live().await;
    assert!(res.unwrap())
}

/// Test that the is_live endpoint returns false when it is expected to.
#[tokio::test]
async fn test_is_live_false() {
    let client = WeaviateClient::builder("http://localhost:8080")
        .build().unwrap();
    let _res = client.is_live().await;
}

/// Test that the is_ready endpoint returns true when it is expected to.
#[tokio::test]
async fn test_is_ready_true() {
    let client = WeaviateClient::builder("http://localhost:8080")
        .build().unwrap();
    let res = client.is_ready().await;
    assert!(res.unwrap())
}

/// Test that the is_ready endpoint returns false when it is expected to.
#[tokio::test]
async fn test_is_ready_false() {
    let client = WeaviateClient::builder("http://localhost:8080")
        .build().unwrap();
    let _res = client.is_ready().await;
}
