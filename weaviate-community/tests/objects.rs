use std::str::FromStr;
use uuid::Uuid;
use weaviate_community::collections::objects::{ConsistencyLevel, Object};
use weaviate_community::collections::auth::AuthApiKey;
use weaviate_community::WeaviateClient;

fn test_object(class_name: &str, id: Option<Uuid>) -> Object {
    let properties = serde_json::json!({
        "name": "test",
        "number": 123,
    });
    Object {
        class: class_name.into(),
        properties,
        id,
        vector: None,
        tenant: None,
        creation_time_unix: None,
        last_update_time_unix: None,
        vector_weights: None,
    }
}

#[tokio::test]
async fn test_list_objects() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd303").unwrap();
    let object = test_object("TestListObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());

    let res = client
        .objects
        .list(
            Some("TestListObject"),
            Some(10),
            None,
            None,
            None,
            None,
            None,
            )
        .await;
    assert_eq!(
        "TestListObject",
        res.unwrap().json::<serde_json::Value>().await.unwrap()["objects"][0]["class"]
        );

    let res = client
        .objects
        .delete("TestListObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}

#[tokio::test]
async fn test_get_object() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd202").unwrap();
    let object = test_object("TestGetObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());

    let res = client
        .objects
        .get("TestGetObject", &uuid, None, None, None)
        .await;

    assert_eq!(
        "TestGetObject",
        res.unwrap().json::<serde_json::Value>().await.unwrap()["class"]
        );

    let res = client
        .objects
        .delete("TestGetObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}

#[tokio::test]
async fn test_delete_object() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd967").unwrap();
    let object = test_object("TestDeleteObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());
    let res = client
        .objects
        .delete("TestDeleteObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}

#[tokio::test]
async fn test_exists_object() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd555").unwrap();
    let object = test_object("TestExistsObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());

    // exists
    let res = client
        .objects
        .exists(
            "TestExistsObject",
            &Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd555").unwrap(),
            None,
            None,
            )
        .await;
    assert_eq!(204, res.unwrap().status());

    // doesnt
    let res = client
        .objects
        .exists(
            "TestExistsObject",
            &Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd556").unwrap(),
            None,
            None,
            )
        .await;
    assert_eq!(404, res.unwrap().status());

    // Delete it
    let res = client
        .objects
        .delete("TestExistsObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}

#[tokio::test]
async fn test_create_object() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd178").unwrap();
    let object = test_object("TestCreateObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());

    //let res = client.objects.validate("TestClass2", serde_json::json!({"name": "test4"}), Uuid::from_str("de22d1b8-3b95-4e94-96d5-9a2b60fbd965").unwrap()).await;
    //println!("{:?}", res.unwrap());

    //println!("{:?}", res.unwrap().json::<serde_json::Value>().await);
    let res = client
        .objects
        .delete("TestCreateObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}

#[tokio::test]
async fn test_replace_object() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd666").unwrap();
    let object = test_object("TestReplaceObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());

    let test = serde_json::json!({
        "class": "TestReplaceObject",
        "id": uuid.clone(),
        "properties": {
            "name": "updated",
            "number": 987
        }
    });
    // note that if you drop a field, it will be dropped in the actual object too
    let res = client
        .objects
        .replace(
            test,
            &object.class,
            &uuid.clone(),
            Some(ConsistencyLevel::ALL),
            )
        .await;
    assert_eq!(200, res.unwrap().status());

    let res = client
        .objects
        .replace(
            serde_json::json!({}),
            &object.class,
            &uuid.clone(),
            Some(ConsistencyLevel::ALL),
            )
        .await;
    assert_eq!(422, res.unwrap().status());

    // Delete the object
    let res = client
        .objects
        .delete("TestReplaceObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}

#[tokio::test]
async fn test_update_object() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd444").unwrap();
    let object = test_object("TestUpdateObject", Some(uuid.clone()));
    let res = client
        .objects
        .create(&object, Some(ConsistencyLevel::ALL))
        .await;
    assert_eq!(200, res.unwrap().status());

    // Updates
    let test = serde_json::json!({
        "class": "TestUpdateObject",
        "id": uuid.clone(),
        "properties": {
            "name": "updated",
        }
    });
    let res = client
        .objects
        .update(
            test,
            &object.class,
            &uuid.clone(),
            Some(ConsistencyLevel::ALL),
            )
        .await;
    assert_eq!(204, res.unwrap().status());

    // Doesn't
    let res = client
        .objects
        .update(
            serde_json::json!({}),
            "test",
            &uuid.clone(),
            Some(ConsistencyLevel::ALL),
            )
        .await;
    assert_eq!(404, res.unwrap().status());

    // Delete the object
    let res = client
        .objects
        .delete("TestUpdateObject", &uuid, None, None)
        .await;
    assert_eq!(204, res.unwrap().status());
}
