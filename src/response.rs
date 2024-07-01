use std::{marker::PhantomData, pin::Pin};

use async_trait::async_trait;
use bytes::Bytes;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use tokio_stream::Stream;

use crate::errors::OllamaError;

pub type OllamaStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, OllamaError>> + Send + Sync + 'static>>;

pub struct OllamaResponse<T> {
    response: reqwest::Response,
    _marker: PhantomData<T>,
}

impl<T> OllamaResponse<T>
where
    T: StreamHandler + DeserializeOwned,
{
    pub fn raw_response(self) -> reqwest::Response {
        self.response
    }

    pub async fn response(self) -> Result<T, OllamaError> {
        if self.response.status() != StatusCode::OK {
            let err_msg = self.response.text().await.unwrap_or_default().to_string();
            return Err(OllamaError::RequestError(err_msg));
        }

        match self.response.text().await {
            Ok(inner) => match serde_json::from_str(&inner) {
                Ok(content) => Ok(content),
                Err(e) => Err(OllamaError::ParseError(e.to_string())),
            },
            Err(e) => Err(OllamaError::OllamaError(e.to_string())),
        }
    }

    pub async fn stream(self) -> Result<OllamaStream<T>, OllamaError> {
        Ok(T::adapt_stream(self.response.bytes_stream()).await)
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

impl<T> From<reqwest::Response> for OllamaResponse<T> {
    fn from(value: reqwest::Response) -> Self {
        OllamaResponse {
            response: value,
            _marker: PhantomData::default(),
        }
    }
}
#[async_trait]
pub trait StreamHandler
where
    Self: Sized,
{
    async fn adapt_stream(
        input: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + Sync + 'static,
    ) -> OllamaStream<Self>;

    async fn stream_to_response(
        input: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + Sync + 'static,
    ) -> Result<Self, OllamaError>;
}
