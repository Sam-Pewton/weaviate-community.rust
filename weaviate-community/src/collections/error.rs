/// All custom errors
use std::{
    error::Error,
    fmt::{
        Display,
        Result,
        Formatter,
    },
};

/// Custom QueryError, used when there was a mismatch in expected query parameters for endpoints.
#[derive(Debug)]
pub struct QueryError(pub String);

impl Error for QueryError {}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Invalid query parameters passed: {}", self.0)
    }
}

/// Custom NotConfiguredError, used when trying to retrieve about a configuration that is not
/// active.
#[derive(Debug)]
pub struct NotConfiguredError(pub String);
impl Error for NotConfiguredError {}

impl Display for NotConfiguredError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Invalid query parameters passed: {}", self.0)
    }
}
