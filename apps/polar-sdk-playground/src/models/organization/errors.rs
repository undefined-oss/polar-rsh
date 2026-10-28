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
