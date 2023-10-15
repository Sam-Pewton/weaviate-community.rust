use weaviate_community::{
    collections::{
        batch::{BatchDeleteRequest, MatchConfig},
        objects::{Object, Objects},
    },
    WeaviateClient,
    collections::auth::AuthApiKey,
};
use uuid::Uuid;

fn test_objects(class_name: &str, uuid_one: &Uuid, uuid_two: &Uuid) -> Objects {
    let properties = serde_json::json!({
        "name": "test",
        "number": 123,
    });
    let properties2 = serde_json::json!({
        "name": "test2",
        "number": 456,
    });
    Objects {
        objects: vec![
            Object {
                class: class_name.into(),
                properties,
                id: Some(*uuid_one),
                vector: None,
                tenant: None,
                creation_time_unix: None,
                last_update_time_unix: None,
                vector_weights: None,
            },
            Object {
                class: class_name.into(),
                properties: properties2,
                id: Some(*uuid_two),
                vector: None,
                tenant: None,
                creation_time_unix: None,
                last_update_time_unix: None,
                vector_weights: None,
            },
        ],
    }
}

fn test_delete_objects(class_name: &str) -> BatchDeleteRequest {
    // this will eventually be defined with the graphql stuff later on
    let map = serde_json::json!({
        "operator": "NotEqual",
        "path": ["name"],
        "valueText": "aaa"
    });
    BatchDeleteRequest {
        matches: MatchConfig {
            class: class_name.into(),
            match_where: map,
        },
        dry_run: None,
        output: None,
    }
}

#[tokio::test]
async fn test_objects_batch_add_and_delete() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let uuid_one = Uuid::new_v4();
    let uuid_two = Uuid::new_v4();
    let objects = test_objects("TestObjectsBatchAdd", &uuid_one, &uuid_two);
    let res = client.batch.objects_batch_add(objects, None).await.unwrap();
    assert_eq!(&2, &res.len());

    let delete = test_delete_objects("TestObjectsBatchAdd");
    let res = client
        .batch
        .objects_batch_delete(delete, None)
        .await
        .unwrap();
    assert_eq!(&2, &res.results.successful);
}
