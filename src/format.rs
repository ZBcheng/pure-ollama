use serde::Serialize;

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
