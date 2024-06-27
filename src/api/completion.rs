use bytes::Bytes;
use derive_builder::Builder;
use serde::Serialize;

use super::options::{GetOptionsBuilder, Options, OptionsConstructor};

/// Ollama API Doc
/// Modelfile: https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values

#[derive(Debug, Clone, Builder, Serialize)]
pub struct GenerateRequest {
    /// Required parameters

    /// The model name.
    pub model: String,

    /// The prompt to generate a response for.
    pub prompt: String,

    /// A a list of base64-encoded images (for multimodal models such as llava).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<String>,

    /// Advanced parameters (optional)

    /// The format to return a response in. Currently the only accepted value is json.
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,

    /// Additional model parameters listed in the documentation for the Modelfile
    /// such as temperature.
    #[builder(setter(strip_option))]
    #[builder(field(
        ty = "super::options::OptionsBuilder",
        build = r#"Some(self.options.build().unwrap())"#
    ))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,

    /// System message to (overrides what is defined in the Modelfile).
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// he prompt template to use (overrides what is defined in the Modelfile).
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// The context parameter returned from a previous request to /generate,
    /// this can be used to keep a short conversational memory.
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Bytes>,

    /// If false the response will be returned as a single response object,
    /// rather than a stream of objects.
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// If true no formatting will be applied to the prompt.
    /// You may choose to use the raw parameter if you are
    /// specifying a full templated prompt in your request to the API.
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,

    /// Kontrols how long the model will stay loaded into
    /// memory following the request (default: 5m).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<usize>,
}

impl GetOptionsBuilder for GenerateRequestBuilder {
    fn get_options_builder<'a>(&'a mut self) -> &'a mut super::options::OptionsBuilder {
        &mut self.options
    }
}

impl OptionsConstructor for GenerateRequestBuilder {}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub enum Format {
    #[default]
    #[serde(rename = "json")]
    JSON,
}

impl ToString for Format {
    fn to_string(&self) -> String {
        String::from("json")
    }
}
