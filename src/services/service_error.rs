/// This enum represents every type of error that a service can throw
pub enum ServiceError{
    /// Basic error containing only a string
    BasicError(String),
    /// Error thrown during request, here using the Reqwest library
    RequestError(reqwest::Error),
    /// Errors thrown during input/output operations, including tcp listeners
    IOError(std::io::Error),
    /// Errors that occur while parsing tcp requests
    EncodingError(std::str::Utf8Error)
}


impl From<std::str::Utf8Error> for ServiceError {
    fn from(error: std::str::Utf8Error) -> Self { ServiceError::EncodingError(error) }
}

impl From<std::io::Error> for ServiceError {
    fn from(error: std::io::Error) -> Self { ServiceError::IOError(error) }
}

impl From<reqwest::Error> for ServiceError {
    fn from(error: reqwest::Error) -> Self {
        ServiceError::RequestError(error)
    }
}

impl From<&str> for ServiceError {
    fn from(error_message: &str) -> Self {
        ServiceError::BasicError(String::from(error_message))
    }
}

/// Implementing the Display trait to make the error printable
impl std::fmt::Display for ServiceError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ServiceError::BasicError(error_message) => write!(f, "[ERROR] {}", error_message),
            ServiceError::RequestError(error) => write!(f, "[ERROR] {}", error.to_string()),
            ServiceError::IOError(error) => write!(f, "[ERROR] {}", error.to_string()),
            ServiceError::EncodingError(error) => write!(f, "[ERROR] {}", error.to_string())
        }
    }
}

impl From<ServiceError> for String {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::BasicError(err) => err,
            ServiceError::RequestError(err) => err.to_string(),
            ServiceError::IOError(err) => err.to_string(),
            ServiceError::EncodingError(err) => err.to_string(),
        }
    }
}

