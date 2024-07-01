use async_stream::stream;
use async_trait::async_trait;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio_stream::{Stream, StreamExt};

use super::message::Message;
use crate::{
    chat_completion::message::Role,
    errors::OllamaError,
    response::{OllamaStream, StreamHandler},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatResponse {
    /// The model name.
    pub model: String,

    pub created_at: String,

    pub done: bool,

    pub message: Option<Message>,

    /// Time spent generating the response.
    pub total_duration: Option<usize>,

    /// Time spent in nanoseconds loading the model.
    pub load_duration: Option<usize>,

    /// Number of tokens in the prompt.
    pub prompt_eval_count: Option<usize>,

    /// Time spent in nanoseconds evaluating the prompt.
    pub prompt_eval_duration: Option<usize>,

    /// Number of tokens in the response.
    pub eval_count: Option<usize>,

    /// Time in nanoseconds spent generating the response.
    pub eval_duration: Option<usize>,
}

#[async_trait]
impl StreamHandler for ChatResponse {
    async fn adapt_stream(
        mut input: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + Sync + 'static,
    ) -> OllamaStream<Self> {
        let adapted_stream = stream! {
            while let Some(item) = input.next().await {
                match item {
                    Ok(inner) => {
                        match serde_json::from_slice(&inner) {
                            Ok(inner) => yield Ok(inner),
                            Err(e) => yield Err(OllamaError::InvalidResponse(e.to_string())),
                        }
                    },
                    Err(e) => yield Err(OllamaError::StreamError(e.to_string()))
                }
            }
        };

        Box::pin(adapted_stream)
    }

    async fn stream_to_response(
        input: impl Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send + Sync + 'static,
    ) -> Result<Self, OllamaError> {
        let mut adapted_stream = Self::adapt_stream(input).await;
        let mut stream_items = vec![];
        let mut content = String::default();
        while let Some(Ok(item)) = adapted_stream.next().await {
            let msg = match &item.message {
                Some(m) => &m.content,
                None => "",
            };
            content += msg;
            stream_items.push(item);
        }

        let mut last = stream_items.pop().unwrap();
        last.message = Some(Message {
            role: Role::Assistant,
            content,
            images: None,
        });

        if let Some(first) = stream_items.first() {
            last.created_at = first.created_at.clone();
        }

        Ok(last)
    }
}
