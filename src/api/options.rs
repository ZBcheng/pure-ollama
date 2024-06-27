use derive_builder::Builder;
use serde::Serialize;

/// Ollama API Doc
/// https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values

#[derive(Debug, Clone, Default, Builder, Serialize)]
pub struct Options {
    /// Enable Mirostat sampling for controlling perplexity.
    /// (default: 0, 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirostat: Option<i32>,

    /// Influences how quickly the algorithm responds to feedback from the generated text.
    /// A lower learning rate will result in slower adjustments, while a higher learning
    /// rate will make the algorithm more responsive.
    /// (Default: 0.1).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirostat_eta: Option<f32>,

    /// Controls the balance between coherence and diversity of the output.
    /// A lower value will result in more focused and coherent text.
    /// (Default: 5.0).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirostat_tau: Option<f32>,

    /// Sets the size of the context window used to generate the next token.
    /// (Default: 2048).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<i32>,

    /// Sets how far back for the model to look back to prevent repetition.
    /// (Default: 64, 0 = disabled, -1 = num_ctx).
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_last_n: Option<i32>,

    /// Sets how strongly to penalize repetitions. A higher value (e.g., 1.5)
    /// will penalize repetitions more strongly, while a lower value (e.g., 0.9)
    /// will be more lenient.
    /// (Default: 1.1)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_penalty: Option<f32>,

    /// The temperature of the model. Increasing the temperature will make
    /// the model answer more creatively.
    /// (Default: 0.8)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<i32>,

    /// Sets the random number seed to use for generation. Setting this to a
    /// specific number will make the model generate the same text for the same prompt.
    /// (Default: 0)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,

    /// Sets the stop sequences to use. When this pattern is encountered the LLM
    /// will stop generating text and return. Multiple stop patterns may be set by
    /// specifying multiple separate stop parameters in a modelfile.
    #[builder(setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,

    /// Tail free sampling is used to reduce the impact of less probable tokens
    /// from the output. A higher value (e.g., 2.0) will reduce the impact more,
    /// while a value of 1.0 disables this setting.
    /// (default: 1)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfs_z: Option<f32>,

    /// Maximum number of tokens to predict when generating text.
    /// (Default: 128, -1 = infinite generation, -2 = fill context)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<i32>,

    /// Reduces the probability of generating nonsense. A higher value (e.g. 100)
    /// will give more diverse answers, while a lower value (e.g. 10) will be more conservative.
    /// (Default: 40)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// Works together with top-k. A higher value (e.g., 0.95) will lead to more diverse text,
    /// while a lower value (e.g., 0.5) will generate more focused and conservative text.
    /// (Default: 0.9)
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

pub trait GetOptionsBuilder {
    fn get_options_builder<'a>(&'a mut self) -> &'a mut OptionsBuilder;
}

pub trait OptionsConstructor: GetOptionsBuilder {
    fn mirostat(&mut self, mirostat: i32) -> &mut Self {
        self.get_options_builder().mirostat(mirostat);
        self
    }

    fn mirostat_eta(&mut self, mirostat_eta: f32) -> &mut Self {
        self.get_options_builder().mirostat_eta(mirostat_eta);
        self
    }

    fn mirostat_tau(&mut self, mirostat_tau: f32) -> &mut Self {
        self.get_options_builder().mirostat_tau(mirostat_tau);
        self
    }

    fn num_ctx(&mut self, num_ctx: i32) -> &mut Self {
        self.get_options_builder().num_ctx(num_ctx);
        self
    }

    fn repeat_last_n(&mut self, repeat_last_n: i32) -> &mut Self {
        self.get_options_builder().repeat_last_n(repeat_last_n);
        self
    }

    fn repeat_penalty(&mut self, repeat_penalty: f32) -> &mut Self {
        self.get_options_builder().repeat_penalty(repeat_penalty);
        self
    }

    fn temperature(&mut self, temperature: i32) -> &mut Self {
        self.get_options_builder().temperature(temperature);
        self
    }

    fn seed(&mut self, seed: i32) -> &mut Self {
        self.get_options_builder().seed(seed);
        self
    }

    fn stop(&mut self, stop: impl ToString) -> &mut Self {
        self.get_options_builder().stop(stop.to_string());
        self
    }

    fn tfs_z(&mut self, tfs_z: f32) -> &mut Self {
        self.get_options_builder().tfs_z(tfs_z);
        self
    }

    fn num_predict(&mut self, num_predict: i32) -> &mut Self {
        self.get_options_builder().num_predict(num_predict);
        self
    }

    fn top_k(&mut self, top_k: i32) -> &mut Self {
        self.get_options_builder().top_k(top_k);
        self
    }

    fn top_p(&mut self, top_p: f32) -> &mut Self {
        self.get_options_builder().top_p(top_p);
        self
    }
}
