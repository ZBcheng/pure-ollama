use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum OllamaError {
    #[error("Request Error: {0}")]
    RequestError(String),

    #[error("Invalid Response: {0}")]
    InvalidResponse(String),

    #[error("Parse Error: {0}")]
    ParseError(String),

    #[error("Ollama Error: {0}")]
    OllamaError(String),

    #[error("Stream Error: {0}")]
    StreamError(String),

    #[error("Invalid Parameter: {0}")]
    InvalidParameter(String),
}
