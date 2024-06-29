use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Builder)]
#[builder(derive(PartialEq, Eq))]
pub struct Message {
    /// The role of the message, either system, user or assistant.
    #[builder(setter(into))]
    pub role: Role,

    /// The content of the message.
    #[builder(setter(into))]
    pub content: String,

    /// A list of images to include in the message (for multimodal models such as llava).
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "system" => Role::System,
            "user" => Role::User,
            "assistant" => Role::Assistant,
            other => panic!("unknown role: {other}"),
        }
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        let role = match self {
            Self::Assistant => "assistant",
            Self::System => "system",
            Self::User => "user",
        };

        String::from(role)
    }
}

impl PartialEq for MessageBuilderError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::UninitializedField(self_err) => {
                if let Self::UninitializedField(other_err) = other {
                    return self_err == other_err;
                }
            }
            Self::ValidationError(self_err) => {
                if let Self::ValidationError(other_err) = other {
                    return self_err == other_err;
                }
            }
        }
        return false;
    }
}

impl Eq for MessageBuilderError {}
