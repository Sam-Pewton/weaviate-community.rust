use weaviate_community::{
    WeaviateClient,
    collections::{
        backups::{
            BackupBackends, 
            BackupCreateRequest, 
            BackupRestoreRequest
        }, 
        objects::Object
    },
    collections::auth::AuthApiKey
};

fn test_backup_create_req() -> BackupCreateRequest {
    BackupCreateRequest { id: "this-is-a-test1".into(), include: None, exclude: None }
}

fn test_backup_restore_req() -> BackupRestoreRequest {
    BackupRestoreRequest { include: None, exclude: None }
}

fn test_object(class_name: &str) -> Object {
    Object {
        class: class_name.into(),
        properties: serde_json::json!({}),
        id: None,
        vector: None,
        tenant: None,
        creation_time_unix: None,
        last_update_time_unix: None,
        vector_weights: None,
    }
}

// commented out to avoid breaking other tests when restore is executing. Will use in SI test
#[tokio::test]
async fn test_create_backup() {
    let obj = test_object("BackupTest");
    let auth = AuthApiKey::new("test-key");
    let client = WeaviateClient::new("http://localhost:8080", Some(auth)).unwrap();
    let _ = client.objects.create(&obj, None).await;

    // create
    //let c_req = test_backup_create_req();
    //let res = client.backups.create(&BackupBackends::FILESYSTEM, &c_req, true).await;
    //println!("{:#?}", res.unwrap());

    // restore
    //let r_req = test_backup_restore_req();
    //let res = client.backups.restore(&BackupBackends::FILESYSTEM, &c_req.id, &r_req, true).await;
    //println!("{:#?}", res);
}
