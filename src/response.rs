use std::marker::PhantomData;

use async_trait::async_trait;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::{
    errors::OllamaError,
    stream_handler::{OllamaStream, StreamHandler},
};

pub struct OllamaResponse<T> {
    response: reqwest::Response,
    _marker: PhantomData<T>,
}

impl<T> OllamaResponse<T>
where
    T: DeserializeOwned,
{
    #[inline]
    pub fn raw_response(self) -> reqwest::Response {
        self.response
    }

    pub async fn response(self) -> Result<T, OllamaError> {
        match self.response.text().await {
            Ok(inner) => match serde_json::from_str(&inner) {
                Ok(content) => Ok(content),
                Err(e) => Err(OllamaError::ParseError(e.to_string())),
            },
            Err(e) => Err(OllamaError::OllamaError(e.to_string())),
        }
    }
}

impl<T> OllamaResponse<T>
where
    T: StreamHandler + DeserializeOwned,
{
    pub async fn stream(self) -> Result<OllamaStream<T>, OllamaError> {
        let adapted_stream = T::adapt_stream(self.response.bytes_stream()).await;
        Ok(adapted_stream)
    }

    pub async fn as_stream(self) -> Result<OllamaStream<T>, OllamaError> {
        let raw_stream = self.response.bytes_stream();
        let adapted_stream = T::adapt_stream(raw_stream).await;
        Ok(adapted_stream)
    }

    pub async fn as_response(self) -> Result<T, OllamaError> {
        let raw_stream = self.response.bytes_stream();
        let response = T::stream_to_response(raw_stream).await?;
        Ok(response)
    }
}

#[async_trait]
pub trait ResponseValidator {
    async fn valid(&self) -> Result<reqwest::Response, OllamaError>;
}

pub async fn check_response_valid(
    response: Result<reqwest::Response, reqwest::Error>,
) -> Result<reqwest::Response, OllamaError> {
    match response {
        Ok(r) => check_status_ok(r).await,
        Err(e) => Err(OllamaError::RequestError(e.to_string())),
    }
}

async fn check_status_ok(response: reqwest::Response) -> Result<reqwest::Response, OllamaError> {
    if response.status() != StatusCode::OK {
        let err_msg = response.text().await.unwrap_or_default().to_string();
        return Err(OllamaError::OllamaError(err_msg));
    }
    Ok(response)
}

impl<T> From<reqwest::Response> for OllamaResponse<T> {
    fn from(value: reqwest::Response) -> Self {
        OllamaResponse {
            response: value,
            _marker: PhantomData::default(),
        }
    }
}
