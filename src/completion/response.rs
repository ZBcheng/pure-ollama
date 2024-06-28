use std::pin::Pin;

use async_stream::stream;
use serde::Deserialize;
use tokio_stream::StreamExt;

use crate::errors::OllamaError;

pub type GenerateResponseStream =
    Pin<Box<dyn tokio_stream::Stream<Item = Result<GenerateResponseInner, OllamaError>>>>;

pub enum GenerateResponse {
    NonStream(GenerateResponseInner),
    Stream(GenerateResponseStream),
}

impl GenerateResponse {
    pub async fn as_non_stream(self) -> GenerateResponseInner {
        match self {
            Self::NonStream(inner) => inner,
            Self::Stream(mut s) => {
                let mut response = String::default();
                let mut parts = vec![];
                while let Some(Ok(item)) = s.next().await {
                    response.push_str(&item.response);
                    parts.push(item);
                }

                let mut last = parts.pop().unwrap_or_default();
                last.created_at = parts.first().cloned().unwrap_or_default().created_at;
                last.response = response;

                last
            }
        }
    }

    pub fn as_stream(self) -> GenerateResponseStream {
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

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GenerateResponseInner {
    /// The model name.
    pub model: String,

    pub created_at: String,

    pub done: bool,

    /// Empty if the response was streamed,
    /// if not streamed, this will contain the full response.
    pub response: String,

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

    /// An encoding of the conversation used in this response,
    /// this can be sent in the next request to keep a conversational memory.
    pub context: Option<Vec<usize>>,
}
