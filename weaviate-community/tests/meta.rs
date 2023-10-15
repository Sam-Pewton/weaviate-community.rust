use weaviate_community::WeaviateClient;
use weaviate_community::collections::auth::AuthApiKey;

/// Test the get_meta endpoint
#[tokio::test]
async fn test_get_meta() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let res = client.meta.get_meta().await;
    assert_eq!(
        "http://[::]:8080",
        res.unwrap().hostname
        );
}
