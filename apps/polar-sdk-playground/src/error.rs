use crate::models::{
    common::errors::{HttpValidationError, NotPermitted, ResourceNotFound},
    organization::{
        CannotCreateOrganizationError, DisputeAutoAcceptNotEnabled,
        SsoEnforcementRequiresConnection,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum RequestError<TransportError> {
    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),

    #[error("resource not found: {0}")]
    NotFound(#[source] ResourceNotFound),

    #[error("validation failed: {0}")]
    Validation(#[source] HttpValidationError),

    #[error("cannot create organization: {0}")]
    CannotCreateOrganization(#[source] CannotCreateOrganizationError),

    #[error("operation not permitted: {0}")]
    NotPermitted(#[source] NotPermitted),

    #[error("dispute auto-accept is not enabled: {0}")]
    DisputeAutoAcceptNotEnabled(#[source] DisputeAutoAcceptNotEnabled),

    #[error("SSO enforcement requires a connection: {0}")]
    SsoEnforcementRequiresConnection(#[source] SsoEnforcementRequiresConnection),
}

#[derive(Debug, thiserror::Error)]
pub enum BuildRequestError {
    #[error("failed to construct request URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("failed to construct HTTP request: {0}")]
    Http(#[from] http::Error),

    #[error("failed to serialize request body: {0}")]
    Serialize(#[from] serde_json::Error),
}
