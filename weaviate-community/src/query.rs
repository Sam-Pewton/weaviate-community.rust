/// This is currently incomplete
use reqwest::Url;
use std::error::Error;
use std::sync::Arc;
use crate::collections::{
    error::GraphQLError,
    query::{
        ExploreQuery,
        AggregateQuery,
        GetQuery,
        RawQuery,
    },
};

///
#[derive(Debug)]
pub struct Query {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Query {
    ///
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/graphql")?;
        Ok(Query { endpoint, client })
    }

    ///
    pub async fn get(&self, query: GetQuery) -> Result<serde_json::Value, Box<dyn Error>> {
        let payload = serde_json::to_value(query).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&payload)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res = res.json::<serde_json::Value>().await?;
                Ok(res)
            }
            _ => Err(Box::new(GraphQLError(format!(
                "status code {} received when executing GraphQL Get.",
                res.status()
            )))),
        }
    }
    
    ///
    pub async fn aggregate(
        &self,
        query: AggregateQuery
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::to_value(query).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&payload)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(res)
            }
            _ => Err(Box::new(GraphQLError(format!(
                "status code {} received when executing GraphQL Aggregate.",
                res.status()
            )))),
        }
    }
    
    ///
    pub async fn explore(
        &self,
        query: ExploreQuery
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::to_value(query).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&payload)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(res)
            }
            _ => Err(Box::new(GraphQLError(format!(
                "status code {} received when executing GraphQL Explore.",
                res.status()
            )))),
        }
    }

    /// Execute a raw GraphQL query.
    pub async fn raw(
        &self,
        query: RawQuery,
    ) -> Result<reqwest::Response, Box<dyn Error>> {
        let payload = serde_json::to_value(query).unwrap();
        let res = self
            .client
            .post(self.endpoint.clone())
            .json(&payload)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                Ok(res)
            }
            _ => Err(Box::new(GraphQLError(format!(
                "status code {} received when executing GraphQL raw query.",
                res.status()
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    //use crate::collections::auth::AuthApiKey;
    //use crate::collections::query::GetBuilder;
    //use crate::WeaviateClient;

    //#[tokio::test]
    //async fn test_get() {
    //    let auth = AuthApiKey::new("learn-weaviate");
    //    let client = WeaviateClient::new("https://edu-demo.weaviate.network", Some(auth)).unwrap();
    //    let query = GetBuilder::new(
    //        "JeopardyQuestion", 
    //        vec![
    //            "question".into(),
    //            "answer".into(),
    //            "points".into(),
    //            "hasCategory { ... on JeopardyCategory { title }}".into()
    //        ])
    //        .with_limit(1)
    //        .with_additional(vec!["id"])
    //        .build();
    //    let _res = client.query.get(query).await;
    //    //println!("{:#?}", res);
    //}
}
