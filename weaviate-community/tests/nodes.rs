use weaviate_community::collections::auth::AuthApiKey;
use weaviate_community::WeaviateClient;

#[tokio::test]
async fn test_get_nodes_status() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let res = client.nodes.get_nodes_status().await;
    let nodes = res.unwrap().json::<serde_json::Value>().await.unwrap();
    assert_eq!("weaviate1", nodes["nodes"][0]["name"]);
}
