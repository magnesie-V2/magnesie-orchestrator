/// This enum represents every type of error that a cluster can throw
pub enum ClusterError{
    /// Basic error containing only a string
    BasicError(String),
}

impl From<&str> for ClusterError {
    fn from(error_message: &str) -> Self {
        ClusterError::BasicError(error_message.to_string())
    }
}

impl From<String> for ClusterError {
    fn from(error_message: String) -> Self {
        ClusterError::BasicError(error_message)
    }
}

/// Implementing the Display trait to make the error printable
impl std::fmt::Display for ClusterError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClusterError::BasicError(error_message) => write!(f, "[ERROR] {}", error_message),
        }
    }
}

impl From<ClusterError> for String {
    fn from(err: ClusterError) -> Self {
        match err {
            ClusterError::BasicError(err) => err,
        }
    }
}

