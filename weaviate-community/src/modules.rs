use reqwest::Url;
use std::error::Error;
use std::sync::Arc;
use crate::collections::error::ModuleError;

///
#[derive(Debug)]
pub struct Modules {
    endpoint: Url,
    client: Arc<reqwest::Client>,
}

impl Modules {
    ///
    pub(super) fn new(url: &Url, client: Arc<reqwest::Client>) -> Result<Self, Box<dyn Error>> {
        let endpoint = url.join("/v1/modules/")?;
        Ok(Modules { endpoint, client })
    }

    /// WIP
    pub async fn contextionary_get_concept(
        &self,
        concept: &str
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let mut endpoint = String::from("text2vec-contextionary/concepts/");
        endpoint.push_str(concept);
        let endpoint = self.endpoint.join(&endpoint)?;
        let res = self.client.get(endpoint).send().await?;

        match res.status() {
            reqwest::StatusCode::OK => {
                let res: serde_json::Value = res.json().await?;
                Ok(res)
            },
            _ => Err(self.get_err_msg("text2vec-contextionary concepts", res).await),
        }
    }

    /// WIP
    pub async fn contextionary_extend(
        &self,
        concept: serde_json::Value
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        let endpoint = self.endpoint.join("text2vec-contextionary/concepts/weaviate")?;
        let res = self
            .client
            .post(endpoint)
            .json(&concept)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let res: serde_json::Value = res.json().await?;
                Ok(res)
            },
            _ => Err(self.get_err_msg("text2vec-contextionary extend", res).await),
        }
    }

    /// Get the error message for the endpoint
    ///
    /// Made to reduce the boilerplate error message building
    async fn get_err_msg(
        &self,
        endpoint: &str,
        res: reqwest::Response
    ) -> Box<ModuleError> {
        let status_code = res.status();
        let msg: Result<serde_json::Value, reqwest::Error> = res.json().await;
        let r_str: String;
        if let Ok(json) = msg {
            r_str = format!(
                "Status code `{}` received when calling {} endpoint. Response: {}",
                status_code,
                endpoint,
                json,
            );
        } else {
            r_str = format!(
                "Status code `{}` received when calling {} endpoint.",
                status_code,
                endpoint
            );
        }
        Box::new(ModuleError(r_str))
    }
}

#[cfg(test)]
mod tests {
}
