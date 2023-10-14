use reqwest::header::HeaderValue;

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct AuthApiKey {
    pub api_key: String,
}

impl AuthApiKey {
    /// Construct a new `AuthApiKey`.
    pub fn new(api_key: &str) -> Self {
        AuthApiKey { api_key: api_key.into() }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> HeaderValue {
        let mut bearer = String::from("Bearer ");
        bearer.push_str(&self.api_key);
        let header_val = HeaderValue::from_str(&bearer).unwrap();
        return header_val
    }
}
