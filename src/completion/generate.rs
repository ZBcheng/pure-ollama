use async_stream::stream;
use reqwest::StatusCode;
use tokio_stream::{Stream, StreamExt};

use super::{
    request::GenerateRequest,
    response::{GenerateResponse, GenerateResponseInner},
};
use crate::errors::OllamaError;

pub async fn generate(req: GenerateRequest) -> Result<GenerateResponse, OllamaError> {
    let url = format!("xxx/api/generate");
    let resp = reqwest::Client::default()
        .post(url)
        .json(&req)
        .send()
        .await
        .unwrap();

    match req.stream {
        Some(false) => handle_non_stream(resp).await,
        _ => handle_stream(resp.bytes_stream()).await,
    }
}

async fn handle_non_stream(resp: reqwest::Response) -> Result<GenerateResponse, OllamaError> {
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

    match serde_json::from_str::<GenerateResponseInner>(&inner) {
        Ok(content) => Ok(GenerateResponse::NonStream(content)),
        Err(e) => Err(OllamaError::InvalidResponse(e.to_string())),
    }
}

async fn handle_stream(
    mut stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin + 'static,
) -> Result<GenerateResponse, OllamaError> {
    let resp = stream! {
        while let Some(Ok(item)) = stream.next().await {
            match serde_json::from_slice::<GenerateResponseInner>(&item) {
                Ok(inner) => yield Ok(inner),
                Err(e) => yield Err(OllamaError::InvalidResponse(e.to_string())),
            }
        }
    };

    let response = GenerateResponse::Stream(Box::pin(resp));
    Ok(response)
}
