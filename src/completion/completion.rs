use async_stream::stream;
use reqwest::StatusCode;
use tokio_stream::{Stream, StreamExt};

use super::{
    request::CompletionRequest,
    response::{CompletionResponse, CompletionResponseInner},
};
use crate::errors::OllamaError;

/// Generate a response for a given prompt with a provided model. This is a streaming endpoint,
/// so there will be a series of responses. The final response object will include statistics and
/// additional data from the request.
pub async fn completion(request: CompletionRequest) -> Result<CompletionResponse, OllamaError> {
    let url = format!("xxx/api/generate");
    let resp = reqwest::Client::default()
        .post(url)
        .json(&request)
        .send()
        .await
        .unwrap();

    match request.stream {
        Some(false) => handle_non_stream(resp).await,
        _ => handle_stream(resp.bytes_stream()).await,
    }
}

async fn handle_non_stream(resp: reqwest::Response) -> Result<CompletionResponse, OllamaError> {
    let status = resp.status();
    if status != StatusCode::OK {
        let text = resp.text().await.unwrap_or_default();
        return Err(OllamaError::OllamaError(text));
    }

    let inner = resp.text().await;
    if let Some(err) = inner.as_ref().err() {
        return Err(OllamaError::DecodeError(err.to_string()));
    }

    let inner = inner.unwrap();

    match serde_json::from_str::<CompletionResponseInner>(&inner) {
        Ok(content) => Ok(CompletionResponse::NonStream(content)),
        Err(e) => Err(OllamaError::InvalidResponse(e.to_string())),
    }
}

async fn handle_stream(
    mut stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin + 'static,
) -> Result<CompletionResponse, OllamaError> {
    let resp = stream! {
        while let Some(Ok(item)) = stream.next().await {
            match serde_json::from_slice::<CompletionResponseInner>(&item) {
                Ok(inner) => yield Ok(inner),
                Err(e) => yield Err(OllamaError::InvalidResponse(e.to_string())),
            }
        }
    };

    let response = CompletionResponse::Stream(Box::pin(resp));
    Ok(response)
}
