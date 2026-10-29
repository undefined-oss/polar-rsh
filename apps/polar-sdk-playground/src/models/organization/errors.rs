use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CannotCreateOrganizationErrorKind {
    CannotCreateOrganizationError,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{detail}")]
pub struct CannotCreateOrganizationError {
    pub error: CannotCreateOrganizationErrorKind,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
pub enum DisputeAutoAcceptNotEnabledKind {
    DisputeAutoAcceptNotEnabled,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{detail}")]
pub struct DisputeAutoAcceptNotEnabled {
    pub error: DisputeAutoAcceptNotEnabledKind,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
pub enum SsoEnforcementRequiresConnectionKind {
    #[serde(rename = "SSOEnforcementRequiresConnection")]
    SsoEnforcementRequiresConnection,
}

#[derive(Debug, Deserialize, thiserror::Error)]
#[error("{detail}")]
pub struct SsoEnforcementRequiresConnection {
    pub error: SsoEnforcementRequiresConnectionKind,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum OrganizationUpdateForbidden {
    NotPermitted(crate::models::common::errors::NotPermitted),
    DisputeAutoAcceptNotEnabled(DisputeAutoAcceptNotEnabled),
}

use crate::{
    error::{BuildRequestError, RequestFailure},
    models::common::errors::{HttpValidationError, NotPermitted, ResourceNotFound},
};

/// Errors returned by the get organization operation.
#[derive(Debug, thiserror::Error)]
pub enum GetOrganizationError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

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
}

pub(super) type GetOrganizationErrors = (RequestFailure, ResourceNotFound, HttpValidationError);

impl<T> From<eros::ErrorUnion<GetOrganizationErrors>> for GetOrganizationError<T> {
    fn from(error: eros::ErrorUnion<GetOrganizationErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
                RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
                RequestFailure::ReadBody(error) => Self::ReadBody(error),
                RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
                RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E3::B(error) => Self::NotFound(error),
            eros::E3::C(error) => Self::Validation(error),
        }
    }
}
impl<T> GetOrganizationError<T> {
    pub(super) fn from_failure(error: RequestFailure) -> Self {
        let union: eros::ErrorUnion<GetOrganizationErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}

/// Errors returned by the list organization operation.
#[derive(Debug, thiserror::Error)]
pub enum ListOrganizationsError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),

    #[error("validation failed: {0}")]
    Validation(#[source] HttpValidationError),
}

pub(super) type ListOrganizationsErrors = (RequestFailure, HttpValidationError);

impl<T> From<eros::ErrorUnion<ListOrganizationsErrors>> for ListOrganizationsError<T> {
    fn from(error: eros::ErrorUnion<ListOrganizationsErrors>) -> Self {
        match error.into_enum() {
            eros::E2::A(error) => match error {
                RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
                RequestFailure::ReadBody(error) => Self::ReadBody(error),
                RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
                RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E2::B(error) => Self::Validation(error),
        }
    }
}
impl<T> ListOrganizationsError<T> {
    pub(super) fn from_failure(error: RequestFailure) -> Self {
        let union: eros::ErrorUnion<ListOrganizationsErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}

/// Errors returned by the create organization operation.
#[derive(Debug, thiserror::Error)]
pub enum CreateOrganizationError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),

    #[error("cannot create organization: {0}")]
    CannotCreateOrganization(#[source] CannotCreateOrganizationError),

    #[error("validation failed: {0}")]
    Validation(#[source] HttpValidationError),
}

pub(super) type CreateOrganizationErrors =
    (RequestFailure, CannotCreateOrganizationError, HttpValidationError);

impl<T> From<eros::ErrorUnion<CreateOrganizationErrors>> for CreateOrganizationError<T> {
    fn from(error: eros::ErrorUnion<CreateOrganizationErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
                RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
                RequestFailure::ReadBody(error) => Self::ReadBody(error),
                RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
                RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E3::B(error) => Self::CannotCreateOrganization(error),
            eros::E3::C(error) => Self::Validation(error),
        }
    }
}
impl<T> CreateOrganizationError<T> {
    pub(super) fn from_failure(error: RequestFailure) -> Self {
        let union: eros::ErrorUnion<CreateOrganizationErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}

/// Errors returned by the update organization operation.
#[derive(Debug, thiserror::Error)]
pub enum UpdateOrganizationError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),

    #[error("operation not permitted: {0}")]
    NotPermitted(#[source] NotPermitted),

    #[error("dispute auto-accept is not enabled: {0}")]
    DisputeAutoAcceptNotEnabled(#[source] DisputeAutoAcceptNotEnabled),

    #[error("resource not found: {0}")]
    NotFound(#[source] ResourceNotFound),

    #[error("SSO enforcement requires a connection: {0}")]
    SsoEnforcementRequiresConnection(#[source] SsoEnforcementRequiresConnection),

    #[error("validation failed: {0}")]
    Validation(#[source] HttpValidationError),
}

pub(super) type UpdateOrganizationErrors = (
    RequestFailure,
    NotPermitted,
    DisputeAutoAcceptNotEnabled,
    ResourceNotFound,
    SsoEnforcementRequiresConnection,
    HttpValidationError,
);

impl<T> From<eros::ErrorUnion<UpdateOrganizationErrors>> for UpdateOrganizationError<T> {
    fn from(error: eros::ErrorUnion<UpdateOrganizationErrors>) -> Self {
        match error.into_enum() {
            eros::E6::A(error) => match error {
                RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
                RequestFailure::ReadBody(error) => Self::ReadBody(error),
                RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
                RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E6::B(error) => Self::NotPermitted(error),
            eros::E6::C(error) => Self::DisputeAutoAcceptNotEnabled(error),
            eros::E6::D(error) => Self::NotFound(error),
            eros::E6::E(error) => Self::SsoEnforcementRequiresConnection(error),
            eros::E6::F(error) => Self::Validation(error),
        }
    }
}
impl<T> UpdateOrganizationError<T> {
    pub(super) fn from_failure(error: RequestFailure) -> Self {
        let union: eros::ErrorUnion<UpdateOrganizationErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
