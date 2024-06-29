use async_stream::stream;
use serde::Deserialize;
use tokio_stream::StreamExt;

use crate::response::OllamaStream;

use super::message::{Message, MessageBuilder};

pub enum ChatResponse {
    NonStream(ChatResponseInner),
    Stream(ChatResponseStream),
}

pub type ChatResponseStream = OllamaStream<ChatResponseInner>;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChatResponseInner {
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

impl ChatResponse {
    pub async fn as_non_stream(self) -> ChatResponseInner {
        match self {
            Self::NonStream(inner) => inner,
            Self::Stream(mut s) => {
                let mut message_content = String::default();
                let mut parts = vec![];
                while let Some(Ok(item)) = s.next().await {
                    let content = match &item.message {
                        Some(msg) => msg.content.as_str(),
                        None => "",
                    };
                    message_content.push_str(content);
                    parts.push(item);
                }

                let mut last = parts.pop().unwrap_or_default();
                last.created_at = parts.first().cloned().unwrap_or_default().created_at;
                last.message = Some(
                    MessageBuilder::default()
                        .role("assistant")
                        .content(message_content)
                        .build()
                        .unwrap(),
                );

                last
            }
        }
    }

    pub fn as_stream(self) -> ChatResponseStream {
        match self {
            Self::NonStream(item) => {
                let s = stream! {
                    yield Ok(item);
                };
                Box::pin(s)
            }
            Self::Stream(s) => s,
        }
    }
}
