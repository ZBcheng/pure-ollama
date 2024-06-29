use async_stream::stream;
use reqwest::StatusCode;
use tokio_stream::{Stream, StreamExt};

use crate::errors::OllamaError;

use super::{
    request::ChatCompletionRequest,
    response::{ChatResponse, ChatResponseInner},
};

/// Generate the next message in a chat with a provided model. This is a streaming endpoint,
/// so there will be a series of responses. Streaming can be disabled using "stream": false.
/// The final response object will include statistics and additional data from the request.
pub async fn chat(request: ChatCompletionRequest) -> Result<ChatResponse, OllamaError> {
    let url = "http://localhost:11434/api/chat";
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

async fn handle_non_stream(resp: reqwest::Response) -> Result<ChatResponse, OllamaError> {
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

    match serde_json::from_str::<ChatResponseInner>(&inner) {
        Ok(content) => Ok(ChatResponse::NonStream(content)),
        Err(e) => Err(OllamaError::InvalidResponse(e.to_string())),
    }
}

async fn handle_stream(
    mut stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin + 'static,
) -> Result<ChatResponse, OllamaError> {
    let resp = stream! {
        while let Some(Ok(item)) = stream.next().await {
            match serde_json::from_slice::<ChatResponseInner>(&item) {
                Ok(inner) => yield Ok(inner),
                Err(e) => yield Err(OllamaError::InvalidResponse(e.to_string())),
            }
        }
    };

    let response = ChatResponse::Stream(Box::pin(resp));
    Ok(response)
}
