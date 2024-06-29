use derive_builder::Builder;
use serde::Serialize;

use crate::{
    format::Format,
    options::{GetOptionsBuilder, Options, OptionsBuilder, OptionsConstructor},
};

/// Ollama API Doc
/// Modelfile: https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values

#[derive(Debug, Clone, Default, Builder, Serialize)]
pub struct CompletionRequest {
    /// Required parameters

    /// The model name.
    #[builder(setter(into))]
    pub model: String,

    /// The prompt to generate a response for.
    #[builder(setter(into))]
    pub prompt: String,

    /// A a list of base64-encoded images (for multimodal models such as llava).
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,

    /// Advanced parameters (optional)

    /// The format to return a response in. Currently the only accepted value is json.
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

    /// System message to (overrides what is defined in the Modelfile).
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// he prompt template to use (overrides what is defined in the Modelfile).
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// The context parameter returned from a previous request to /generate,
    /// this can be used to keep a short conversational memory.
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<usize>>,

    /// If false the response will be returned as a single response object,
    /// rather than a stream of objects.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// If true no formatting will be applied to the prompt.
    /// You may choose to use the raw parameter if you are
    /// specifying a full templated prompt in your request to the API.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,

    /// Kontrols how long the model will stay loaded into
    /// memory following the request (default: 5m).
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<usize>,
}

impl GetOptionsBuilder for CompletionRequestBuilder {
    fn get_options_builder<'a>(&'a mut self) -> &'a mut OptionsBuilder {
        &mut self.options
    }
}

impl OptionsConstructor for CompletionRequestBuilder {}
