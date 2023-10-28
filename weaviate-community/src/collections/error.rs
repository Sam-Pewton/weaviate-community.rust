/// All custom errors
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
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
        write!(f, "NotConfiguredError: {}", self.0)
    }
}

/// Custom BatchError, used when the request to a batch endpoint results in a statuscode that isn't
/// 200.
#[derive(Debug)]
pub struct BatchError(pub String);
impl Error for BatchError {}

impl Display for BatchError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "BatchError: {}", self.0)
    }
}

/// Custom SchemaError, used when the request to a schema endpoint results in a statuscode that
/// isn't 200.
#[derive(Debug)]
pub struct SchemaError(pub String);
impl Error for SchemaError {}

impl Display for SchemaError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "SchemaError: {}", self.0)
    }
}

/// Custom BackupError, used when the request to a schema endpoint results in a statuscode that
/// isn't 200.
#[derive(Debug)]
pub struct BackupError(pub String);
impl Error for BackupError {}

impl Display for BackupError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "BackupError: {}", self.0)
    }
}

/// Custom GraphQL, used when there was a mismatch in expected query parameters for endpoints.
#[derive(Debug)]
pub struct GraphQLError(pub String);

impl Error for GraphQLError {}

impl Display for GraphQLError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Error executing GraphQL query: {}", self.0)
    }
}
