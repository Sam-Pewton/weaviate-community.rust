/// Custom QueryError, used when there was a mismatch in expected query parameters for endpoints.
#[derive(Debug)]
pub struct QueryError(pub String);
impl std::error::Error for QueryError {}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid query parameters passed: {}", self.0)
    }
}
