use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum OllamaError {
    #[error("Request Error: {0}")]
    RequestError(String),

    #[error("Invalid Response: {0}")]
    InvalidResponse(String),

    #[error("Decode Error: {0}")]
    DecodeError(String),

    #[error("Ollama Error: {0}")]
    OllamaError(String),
}

// impl From<reqwest::Error> for OllamaError {
//     fn from(value: reqwest::Error) -> Self {}
// }
