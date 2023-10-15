use crate::collections::error::QueryError;
use crate::collections::objects::{ConsistencyLevel, Object, OrderBy};
use reqwest::Url;
use std::{error::Error, sync::Arc};
use uuid::Uuid;

/// All objects endpoints and functionality described in
/// [Weaviate objects API documentation](https://weaviate.io/developers/weaviate/api/rest/objects)
///
#[derive(Debug)]
pub struct Objects {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Objects {
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/objects/")?;
        Ok(Objects { endpoint, client })
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
        consistency_level: Option<ConsistencyLevel>,
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
        consistency_level: Option<ConsistencyLevel>,
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
    use crate::WeaviateClient;

    fn get_test_harness() -> (mockito::ServerGuard, WeaviateClient) {
        let mock_server = mockito::Server::new();
        let mut host = "http://".to_string();
        host.push_str(&mock_server.host_with_port());
        let client = WeaviateClient::builder(&host).build().unwrap();
        (mock_server, client)
    }

    fn mock_get(
        server: &mut mockito::ServerGuard,
        endpoint: &str,
        status_code: usize,
    ) -> mockito::Mock {
        server.mock("GET", endpoint)
            .with_status(status_code)
            .create()
    }

    #[tokio::test]
    async fn test_list_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_list_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_create_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_create_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_get_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_get_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_exists_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_exists_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_update_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_update_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_replace_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_replace_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_delete_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_delete_err() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_validate_ok() {
        let (mut mock_server, client) = get_test_harness();
    }

    #[tokio::test]
    async fn test_validate_err() {
        let (mut mock_server, client) = get_test_harness();
    }
}
