/// This enum represents every type of error that a cluster can throw
pub enum BufferError{
    /// Basic error containing only a string
    BasicError(String),
}

impl From<&str> for BufferError {
    fn from(error_message: &str) -> Self {
        BufferError::BasicError(error_message.to_string())
    }
}

impl From<String> for BufferError {
    fn from(error_message: String) -> Self {
        BufferError::BasicError(error_message)
    }
}

/// Implementing the Display trait to make the error printable
impl std::fmt::Display for BufferError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BufferError::BasicError(error_message) => write!(f, "{}", error_message),
        }
    }
}

impl From<BufferError> for String {
    fn from(err: BufferError) -> Self {
        match err {
            BufferError::BasicError(err) => err,
        }
    }
}

