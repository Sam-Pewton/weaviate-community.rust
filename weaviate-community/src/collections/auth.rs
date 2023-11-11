use reqwest::header::{HeaderName, HeaderValue};

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct AuthApiKey {
    pub api_key: String,
}

impl AuthApiKey {
    /// Construct a new `AuthApiKey`.
    pub fn new(api_key: &str) -> Self {
        AuthApiKey {
            api_key: api_key.into(),
        }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> HeaderValue {
        let mut bearer = String::from("Bearer ");
        bearer.push_str(&self.api_key);
        let header_val = HeaderValue::from_str(&bearer).unwrap();
        return header_val;
    }
}

/// The `AuthApiKey` can be used to attach a bearer token to a `WeaviateClient`.
#[derive(Debug)]
pub struct ApiKey {
    pub api_header: String,
    pub api_key: String,
}

impl ApiKey {
    /// Construct a new `AuthApiKey`.
    pub fn new(api_header: &str, api_key: &str) -> Self {
        ApiKey {
            api_header: api_header.into(),
            api_key: api_key.into(),
        }
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_name(&self) -> HeaderName {
        let header_name = HeaderName::from_bytes(self.api_header.as_bytes()).unwrap();
        return header_name;
    }

    /// Retrieve the `reqwest::header::HeaderValue` for an Authorization header.
    pub fn get_header_value(&self) -> HeaderValue {
        let header_val = HeaderValue::from_str(&self.api_key).unwrap();
        return header_val;
    }
}
