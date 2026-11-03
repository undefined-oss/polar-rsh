use http_body_util::BodyExt;
use presence_rs::Presence;
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
    errors::{DisputeAutoAcceptNotEnabled, SsoEnforcementRequiresConnection},
    types::{
        Organization, OrganizationCustomerEmailSettings, OrganizationCustomerPortalSettings,
        OrganizationDetails, OrganizationDisputeSettingsUpdate, OrganizationFeatureSettingsUpdate,
        OrganizationSocialLink, OrganizationSubscriptionSettings,
    },
};
use crate::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
    models::common::{
        OrganizationId, OrganizationName,
        enums::{CountryAlpha2Input, PresentmentCurrency, TaxBehaviorOption},
        errors::{HttpValidationError, NotPermitted, ResourceNotFound},
    },
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationUpdate {
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub name: Presence<OrganizationName>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub avatar_url: Presence<Url>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub email: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub website: Presence<Url>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub socials: Presence<Vec<OrganizationSocialLink>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub details: Presence<OrganizationDetails>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub country: Presence<CountryAlpha2Input>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub feature_settings: Presence<OrganizationFeatureSettingsUpdate>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub subscription_settings: Presence<OrganizationSubscriptionSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_email_settings: Presence<OrganizationCustomerEmailSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_portal_settings: Presence<OrganizationCustomerPortalSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub dispute_settings: Presence<OrganizationDisputeSettingsUpdate>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub embed_hosts: Presence<Vec<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub default_presentment_currency: Presence<PresentmentCurrency>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub default_tax_behavior: Presence<TaxBehaviorOption>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub sso_enforced: Presence<bool>,
}

impl OrganizationUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, value: OrganizationName) -> Self {
        self.name = Presence::Some(value);
        self
    }

    pub fn avatar_url(mut self, value: Url) -> Self {
        self.avatar_url = Presence::Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Presence::Some(value.into());
        self
    }

    pub fn website(mut self, value: Url) -> Self {
        self.website = Presence::Some(value);
        self
    }

    pub fn socials(mut self, value: Vec<OrganizationSocialLink>) -> Self {
        self.socials = Presence::Some(value);
        self
    }

    pub fn details(mut self, value: OrganizationDetails) -> Self {
        self.details = Presence::Some(value);
        self
    }

    pub fn country(mut self, value: CountryAlpha2Input) -> Self {
        self.country = Presence::Some(value);
        self
    }

    pub fn feature_settings(mut self, value: OrganizationFeatureSettingsUpdate) -> Self {
        self.feature_settings = Presence::Some(value);
        self
    }

    pub fn subscription_settings(mut self, value: OrganizationSubscriptionSettings) -> Self {
        self.subscription_settings = Presence::Some(value);
        self
    }

    pub fn customer_email_settings(mut self, value: OrganizationCustomerEmailSettings) -> Self {
        self.customer_email_settings = Presence::Some(value);
        self
    }

    pub fn customer_portal_settings(mut self, value: OrganizationCustomerPortalSettings) -> Self {
        self.customer_portal_settings = Presence::Some(value);
        self
    }

    pub fn dispute_settings(mut self, value: OrganizationDisputeSettingsUpdate) -> Self {
        self.dispute_settings = Presence::Some(value);
        self
    }

    pub fn embed_hosts(mut self, value: Vec<String>) -> Self {
        self.embed_hosts = Presence::Some(value);
        self
    }

    pub fn default_presentment_currency(mut self, value: PresentmentCurrency) -> Self {
        self.default_presentment_currency = Presence::Some(value);
        self
    }

    pub fn default_tax_behavior(mut self, value: TaxBehaviorOption) -> Self {
        self.default_tax_behavior = Presence::Some(value);
        self
    }

    pub fn sso_enforced(mut self, value: bool) -> Self {
        self.sso_enforced = Presence::Some(value);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OrganizationUpdateForbidden {
    NotPermitted(crate::models::common::errors::NotPermitted),
    DisputeAutoAcceptNotEnabled(DisputeAutoAcceptNotEnabled),
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

pub struct UpdateOrganization<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: OrganizationId,
    pub(super) payload: OrganizationUpdate,
    pub(super) options: RequestOptions,
}

impl<'a, ClientService> UpdateOrganization<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/organizations/{}", self.id);
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::PATCH, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}

impl<'a, ClientService, Body> UpdateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<Organization, UpdateOrganizationError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            UpdateOrganizationError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(UpdateOrganizationError::Transport)?;
        let result: eros::Result<_, UpdateOrganizationErrors> = async {
            let status = response.status();
            let body = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();

            match status {
                http::StatusCode::OK => serde_json::from_slice(&body)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error = serde_json::from_slice::<OrganizationUpdateForbidden>(&body)
                        .map_err(|error| {
                            eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                        })?;
                    Err(match error {
                        OrganizationUpdateForbidden::NotPermitted(error) => {
                            eros::ErrorUnion::new(error)
                        }
                        OrganizationUpdateForbidden::DisputeAutoAcceptNotEnabled(error) => {
                            eros::ErrorUnion::new(error)
                        }
                    })
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::ResourceNotFound,
                    >(&body)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<SsoEnforcementRequiresConnection>(&body)
                        .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::HttpValidationError,
                    >(&body)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                _ => Err(eros::ErrorUnion::new(RequestFailure::UnexpectedStatus { status, body })),
            }
        }
        .await;
        result.map_err(Into::into)
    }

    pub async fn send(self) -> Result<Organization, UpdateOrganizationError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            UpdateOrganizationError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}

impl<'a, ClientService, Body> std::future::IntoFuture for UpdateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<Organization, UpdateOrganizationError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
