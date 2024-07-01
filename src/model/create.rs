use async_stream::stream;
use async_trait::async_trait;
use bytes::Bytes;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tokio_stream::{Stream, StreamExt};

use crate::{
    errors::OllamaError,
    response::{OllamaStream, StreamHandler},
};

#[derive(Debug, Clone, Serialize, Builder)]
pub struct CreateModelRequest {
    /// Name of the model to create.
    #[builder(setter(into))]
    pub name: String,

    /// Contents of the Modelfile.
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modelfile: Option<String>,

    /// If false the response will be returned as a
    /// single response object, rather than a stream of objects.
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Path to the Modelfile.
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// A stream of JSON objects. Notice that the final JSON
/// object shows "status": "success" if the response is a stream.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateModelResponse {
    pub status: String,
}

#[async_trait]
impl StreamHandler for CreateModelResponse {
    async fn adapt_stream(
        mut input: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + Sync + 'static,
    ) -> OllamaStream<Self> {
        let adapted = stream! {
            while let Some(item) = input.next().await {
                match item {
                    Ok(inner) => {
                        match serde_json::from_slice(&inner) {
                            Ok(content) => yield Ok(content),
                            Err(e) => yield Err(OllamaError::InvalidResponse(e.to_string()))
                        }
                    },
                    Err(e) => yield Err(OllamaError::StreamError(e.to_string()))
                }
            }
        };

        Box::pin(adapted)
    }

    async fn stream_to_response(
        input: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + Sync + 'static,
    ) -> Result<Self, OllamaError> {
        let mut adapted_stream = Self::adapt_stream(input).await;

        let mut status = String::default();
        while let Some(Ok(item)) = adapted_stream.next().await {
            let line = item.status + "\n";
            status.push_str(&line);
        }

        Ok(Self { status })
    }
}
