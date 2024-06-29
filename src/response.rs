use std::pin::Pin;

use crate::errors::OllamaError;

pub type OllamaStream<T> = Pin<Box<dyn tokio_stream::Stream<Item = Result<T, OllamaError>>>>;
