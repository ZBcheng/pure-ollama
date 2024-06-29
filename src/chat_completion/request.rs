use derive_builder::Builder;
use serde::Serialize;

use crate::{
    format::Format,
    options::{GetOptionsBuilder, Options, OptionsConstructor},
};

use super::message::Message;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Builder)]
pub struct ChatCompletionRequest {
    /// The model name.
    #[builder(setter(into))]
    pub model: String,

    /// The messages of the chat, this can be used to keep a chat memory.
    pub messages: Vec<Message>,

    /// The format to return a response in.
    /// Currently the only accepted value is json.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,

    /// Additional model parameters listed in the documentation for the Modelfile
    /// such as temperature.
    #[builder(setter(strip_option))]
    #[builder(field(
        ty = "crate::options::OptionsBuilder",
        build = r#"self.options.build().unwrap()"#
    ))]
    #[serde(skip_serializing_if = "crate::options::Options::is_default")]
    pub options: Options,

    /// If false the response will be returned as a single response object,
    /// rather than a stream of objects.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Kontrols how long the model will stay loaded into
    /// memory following the request (default: 5m).
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<usize>,
}

impl GetOptionsBuilder for ChatCompletionRequestBuilder {
    fn get_options_builder<'a>(&'a mut self) -> &'a mut crate::options::OptionsBuilder {
        &mut self.options
    }
}

impl OptionsConstructor for ChatCompletionRequestBuilder {}
