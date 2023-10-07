use crate::collections::objects::{ConsistencyLevel, Object, OrderBy, QueryError};
use reqwest::Url;
use std::error::Error;
use uuid::Uuid;

/// All objects endpoints and functionality described in
/// [Weaviate objects API documentation](https://weaviate.io/developers/weaviate/api/rest/objects)
///
pub struct Objects {
    endpoint: Url,
    client: reqwest::Client,
}

impl Objects {
    pub fn new(url: &Url) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/objects/")?;
        Ok(Objects {
            endpoint,
            client: reqwest::Client::new(),
        })
    }

    ///
    /// class
    /// limit
    /// offset -> cannot be used with after, should be used in conjunction with limit
    /// after -> MUST be used with class, cannot be used with offset or sort, should be used with
    ///          limit
    /// include -> has a list of allowed values, including classification, vector,
    ///            featureProjection, and other module-specific additional properties
    /// sort -> can be a comma separated list of strings (corresponding to properties)
    /// order -> `asc` or `desc` should be used with sort
    ///
    pub async fn list(
        &self,
        class_name: Option<&str>,
        limit: Option<u64>,
        offset: Option<u64>,
        after: Option<&str>,
        include: Option<&str>,
        sort: Option<Vec<&str>>,
        order: Option<OrderBy>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = self.endpoint.clone();
        if let Some(c) = class_name {
            endpoint.query_pairs_mut().append_pair("class", c);
        }
        if let Some(l) = limit {
            endpoint
                .query_pairs_mut()
                .append_pair("limit", &l.to_string());
        }
        if let Some(o) = offset {
            endpoint
                .query_pairs_mut()
                .append_pair("offset", &o.to_string());
            // Raise an err if after is some
            if after.is_some() {
                return Err(Box::new(QueryError(
                    "'after' must be None when 'offset' is Some".into(),
                )));
            }
            // warn if limit is none
        }
        if let Some(a) = after {
            endpoint.query_pairs_mut().append_pair("after", a);
            // raise err if class is none
            if after.is_none() {
                return Err(Box::new(QueryError(
                    "'class' must be Some when 'after' is Some".into(),
                )));
            }
            // raise an error if offset or sort are some
            if offset.is_some() {
                return Err(Box::new(QueryError(
                    "'offset' must be None when 'after' is Some".into(),
                )));
            }
            if sort.is_some() {
                return Err(Box::new(QueryError(
                    "'sort' must be None when 'after' is Some".into(),
                )));
            }
            // warn if limit is none
        }
        if let Some(i) = include {
            endpoint.query_pairs_mut().append_pair("include", i);
        }
        if let Some(s) = sort {
            let values = s.join(",");
            endpoint.query_pairs_mut().append_pair("sort", &values);
        }
        if let Some(o) = order {
            endpoint.query_pairs_mut().append_pair("order", o.value());
        }
        let res = self.client.get(endpoint).send().await?;
        Ok(res)
    }

    ///
    ///
    ///
    pub async fn create(
        &self,
        new_object: &Object,
        consistency_level: Option<ConsistencyLevel>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint = self.endpoint.clone();
        if let Some(x) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", x.value());
        }
        let payload = serde_json::to_value(&new_object)?;
        let res = self.client.post(endpoint).json(&payload).send().await?;
        Ok(res)
    }

    ///
    /// Collect an individual data object
    ///
    pub async fn get(
        &self,
        class_name: &str,
        id: &Uuid,
        include: Option<&str>,
        consistency_level: Option<ConsistencyLevel>,
        tenant_key: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_key {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }
        if let Some(i) = include {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("include", i);
        }

        let res = self.client.get(endpoint).send().await?;
        Ok(res)
    }

    pub async fn exists(
        &self,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }

        let res = self.client.head(endpoint).send().await?;
        Ok(res)
    }

    /// 
    /// Updates property values of the data object
    /// 
    pub async fn update(
        &self,
        properties: serde_json::Value,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        let res = self.client.patch(endpoint).json(&properties).send().await?;
        Ok(res)
    }

    /// 
    /// Replaces all property values of the data object
    /// 
    pub async fn replace(
        &self,
        properties: serde_json::Value,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        let res = self.client.put(endpoint).json(&properties).send().await?;
        Ok(res)
    }

    pub async fn delete(
        &self,
        class_name: &str,
        id: &Uuid,
        consistency_level: Option<ConsistencyLevel>,
        tenant_name: Option<&str>,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut endpoint: String = class_name.into();
        endpoint.push_str("/");
        endpoint.push_str(&id.to_string());
        let mut endpoint = self.endpoint.join(&endpoint)?;
        if let Some(cl) = consistency_level {
            endpoint
                .query_pairs_mut()
                .append_pair("consistency_level", &cl.value());
        }
        if let Some(t) = tenant_name {
            // multi tenancy must be enabled first
            endpoint.query_pairs_mut().append_pair("tenant", t);
        }

        let res = self.client.delete(endpoint).send().await?;
        Ok(res)
    }

    ///
    /// This method doesn't seem to work as intended. I can get a 200 response, but I can't see how
    /// it returns True/None. All responses are an empty string..
    ///
    pub async fn validate(
        &self,
        class_name: &str,
        properties: serde_json::Value,
        id: &Uuid,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::json!({
            "class": class_name,
            "id": id.to_string(),
            "properties": properties
        });
        let endpoint = self.endpoint.join("validate")?;
        println!("{:?}", payload);
        let res = self.client.post(endpoint).json(&payload).send().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use uuid::Uuid;

    use crate::Client;
    use crate::collections::objects::{Object, ConsistencyLevel};

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
        }
    }

    #[tokio::test]
    async fn test_list_objects() {
        let client = Client::new("http://localhost:8080").unwrap();
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
        let client = Client::new("http://localhost:8080").unwrap();
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
        let client = Client::new("http://localhost:8080").unwrap();
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
        let client = Client::new("http://localhost:8080").unwrap();
        let uuid = Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd555").unwrap();
        let object = test_object("TestExistsObject", Some(uuid.clone()));
        let res = client
            .objects
            .create(&object, Some(ConsistencyLevel::ALL))
            .await;
        assert_eq!(200, res.unwrap().status());

        // exists
        let res = client.objects.exists(
            "TestExistsObject",
            &Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd555").unwrap(),
            None,
            None,
        ).await;
        assert_eq!(204, res.unwrap().status());

        // doesnt
        let res = client.objects.exists(
            "TestExistsObject",
            &Uuid::from_str("ee22d1b8-3b95-4e94-96d5-9a2b60fbd556").unwrap(),
            None,
            None,
        ).await;
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
        let client = Client::new("http://localhost:8080").unwrap();
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
        let client = Client::new("http://localhost:8080").unwrap();
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
        let res = client.objects.replace(
            test,
            &object.class,
            &uuid.clone(),
            Some(ConsistencyLevel::ALL)
        ).await;
        assert_eq!(200, res.unwrap().status());

        let res = client.objects.replace(
            serde_json::json!({}),
            &object.class,
            &uuid.clone(),
            Some(ConsistencyLevel::ALL)
        ).await;
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
        let client = Client::new("http://localhost:8080").unwrap();
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
        let res = client.objects.update(
            test,
            &object.class,
            &uuid.clone(),
            Some(ConsistencyLevel::ALL)
        ).await;
        assert_eq!(204, res.unwrap().status());

        // Doesn't
        let res = client.objects.update(
            serde_json::json!({}),
            "test",
            &uuid.clone(),
            Some(ConsistencyLevel::ALL)
        ).await;
        assert_eq!(404, res.unwrap().status());

        // Delete the object
        let res = client
            .objects
            .delete("TestUpdateObject", &uuid, None, None)
            .await;
        assert_eq!(204, res.unwrap().status());
    }
}
