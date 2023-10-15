use weaviate_community::collections::auth::AuthApiKey;
use weaviate_community::WeaviateClient;

#[tokio::test]
async fn test_get_open_id_configuration() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let _res = client.oidc.get_open_id_configuration().await;
}
