use weaviate_community::collections::auth::AuthApiKey;
use weaviate_community::collections::schema::{
    ActivityStatus, Class, ClassBuilder, MultiTenancyConfig, Property, ShardStatus, Tenant, Tenants,
};
use weaviate_community::WeaviateClient;

/// Helper function for generating a testing class
fn test_class(class_name: &str, enabled: bool) -> Class {
    ClassBuilder::new(class_name, "Test")
        .multi_tenancy_config(MultiTenancyConfig { enabled })
        .build()
}

/// Helper function for generating a testing property
fn test_property(property_name: &str) -> Property {
    Property {
        name: property_name.into(),
        data_type: vec!["boolean".into()],
        description: Some("test property".into()),
        index_filterable: None,
        index_searchable: None,
        module_config: None,
        tokenization: None,
        inverted_index_config: None,
    }
}

/// Helper function for generating some test tenants, as shown on the weaviate API webpage.
fn test_tenants() -> Tenants {
    Tenants {
        tenants: vec![
            Tenant {
                name: "TENANT_A".into(),
                activity_status: None,
            },
            Tenant {
                name: "TENANT_B".into(),
                activity_status: Some(ActivityStatus::COLD),
            },
        ],
    }
}

#[tokio::test]
async fn test_create_single_class() {
    // Insert the class and get it from the schema
    let class = test_class("CreateSingle2", false);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(&result.is_ok());
    assert_eq!(&result.unwrap().class, "CreateSingle2");

    // Delete it to tidy up after ourselves
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_get_single_class_err() {
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.get_class("DOESNOTEXIST").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_all_classes() {
    // Insert, to make sure it exists.
    let class = test_class("GetAllClasses", false);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(result.is_ok());

    let result = client.schema.get().await;
    assert!(result.is_ok());

    // There could be more than just one in the schema, depending on when the tests run.
    assert_ne!(&result.unwrap().classes.len(), &0);

    // Delete it and make sure that it is gone
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_update_single_class() {
    // Insert, to make sure it exists.
    let mut class = test_class("UpdateSingle", false);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(result.is_ok());

    // Update it and make sure that it changed
    class.description = "Updated".into();
    let result = client.schema.update(&class).await;
    assert_eq!("Updated", result.unwrap().description);

    // Delete it and make sure that it is gone
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_add_property() {
    // Insert, to make sure it exists.
    let class = test_class("AddProperty", false);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(result.is_ok());

    // Validate the property does not exist in the class schema
    let result = client.schema.get_class(&class.class).await;
    assert_eq!(None, result.unwrap().properties);

    // Update class with test property
    let property = test_property("TestProperty");

    // Update it and make sure that it changed
    let result = client.schema.add_property(&class.class, &property).await;
    assert_eq!("testProperty", result.unwrap().name);

    // Validate the property now exists in the class schema
    let result = client.schema.get_class(&class.class).await;
    assert!(result.unwrap().properties.is_some());

    // Delete it and make sure that it is gone
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_get_shards() {
    let class = test_class("GetShards", false);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(result.is_ok());

    let shards = client.schema.get_shards(&class.class).await;
    //assert_eq!(ShardStatus::READY, shards.unwrap().shards[0].status);
    println!("{:#?}", shards);

    // Delete it and make sure that it is gone
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_update_shard_status() {
    let class = test_class("UpdateShards", false);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(result.is_ok());

    // Get the name of the shard
    let result = client.schema.get_shards(&class.class).await;
    let shards = result.unwrap();
    assert_eq!(1, shards.shards.len());
    assert_eq!(ShardStatus::READY, shards.shards[0].status);

    // Update the shard status
    let name = serde_json::to_string(&shards.shards[0].name)
        .unwrap()
        .clone();
    let name = name.trim_start_matches("\"");
    let name = name.trim_end_matches("\"");
    let result = client
        .schema
        .update_class_shard(&class.class, &name, ShardStatus::READONLY)
        .await;
    assert_eq!(ShardStatus::READONLY, result.unwrap().status);

    // Get the shard again
    let result = client.schema.get_shards(&class.class).await;
    let shards = result.unwrap();
    assert_eq!(1, shards.shards.len());
    assert_eq!(ShardStatus::READONLY, shards.shards[0].status);

    // Delete it and make sure that it is gone
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_list_tenants() {
    let class = test_class("ListTenants2", true);
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::builder("http://localhost:8080")
        .auth_secret(auth)
        .build()
        .unwrap();
    let result = client.schema.create_class(&class).await;
    assert!(result.is_ok());

    let result = client.schema.list_tenants(&class.class).await;
    assert_eq!(0, result.unwrap().tenants.len());

    let mut tenants = test_tenants();
    let result = client.schema.add_tenants(&class.class, &tenants).await;
    assert_eq!(2, result.unwrap().tenants.len());

    let result = client.schema.list_tenants(&class.class).await;
    assert_eq!(2, result.unwrap().tenants.len());

    tenants.tenants[0].activity_status = Some(ActivityStatus::COLD);
    tenants.tenants[1].activity_status = Some(ActivityStatus::HOT);
    let result = client.schema.update_tenants(&class.class, &tenants).await;
    assert_eq!(2, result.unwrap().tenants.len());

    let result = client.schema.list_tenants(&class.class).await;
    assert_eq!(2, result.unwrap().tenants.len());

    let result = client
        .schema
        .remove_tenants(&class.class, &vec!["TENANT_A", "TENANT_B"])
        .await;
    assert!(result.is_ok());
    let result = client.schema.list_tenants(&class.class).await;
    assert_eq!(0, result.unwrap().tenants.len());

    // Delete it and make sure that it is gone
    let result = client.schema.delete(&class.class).await;
    assert!(result.unwrap());
}
