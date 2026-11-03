//! Convert OpenAPI definitions into template inputs.

use crate::templates::client::Server;

mod domains;
pub mod enums;
pub mod models;
pub mod newtypes;
pub mod operations;
pub mod pagination;
mod references;
pub mod uuid_ids;

pub struct ClientInput {
    pub version: String,
    pub servers: Vec<Server>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error(
        "/servers/{index}/x-polar-environment: expected a string, but the extension is missing"
    )]
    MissingEnvironment { index: usize },
    #[error("/servers/{index}/x-polar-environment: expected a string, got {value}")]
    InvalidEnvironment { index: usize, value: serde_json::Value },
    #[error("{location}: {message}")]
    InvalidPagination { location: String, message: String },
    #[error("enum {name}: {message}")]
    InvalidEnum { name: String, message: String },
    #[error("domain resolution: {message}")]
    InvalidDomain { message: String },
    #[error("{location}: {message}")]
    InvalidUuidId { location: String, message: String },
    #[error("model resolution: {0}")]
    InvalidModel(String),
    #[error("operation resolution: {0}")]
    InvalidOperation(String),
}

impl TryFrom<&oas3::Spec> for ClientInput {
    type Error = ConversionError;

    fn try_from(spec: &oas3::Spec) -> Result<Self, Self::Error> {
        let servers = spec
            .servers
            .iter()
            .enumerate()
            .map(|(index, server)| {
                // oas3 strips the `x-` prefix from extension keys.
                let value = server
                    .extensions
                    .get("polar-environment")
                    .ok_or(ConversionError::MissingEnvironment { index })?;
                let environment = value.as_str().ok_or_else(|| {
                    ConversionError::InvalidEnvironment { index, value: value.clone() }
                })?;

                Ok(Server {
                    url: server.url.clone(),
                    description: server.description.clone(),
                    environment: environment.to_owned(),
                })
            })
            .collect::<Result<Vec<_>, ConversionError>>()?;

        Ok(Self { version: spec.info.version.clone(), servers })
    }
}
