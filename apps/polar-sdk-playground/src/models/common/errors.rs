use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum NotPermittedKind {
    NotPermitted,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{detail}")]
pub struct NotPermitted {
    pub error: NotPermittedKind,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
pub enum ResourceNotFoundKind {
    ResourceNotFound,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{detail}")]
pub struct ResourceNotFound {
    pub error: ResourceNotFoundKind,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ValidationLocation {
    Field(String),
    Index(i64),
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<ValidationLocation>,
    pub msg: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub input: Option<serde_json::Value>,
    pub ctx: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("request validation failed")]
pub struct HttpValidationError {
    pub detail: Option<Vec<ValidationError>>,
}
