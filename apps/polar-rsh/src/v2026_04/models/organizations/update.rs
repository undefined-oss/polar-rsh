use http_body_util::BodyExt;

use crate::v2026_04::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum OrganizationsUpdateError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),
    #[error("failed to build request: {0}")]
    BuildRequest(#[source] crate::v2026_04::error::BuildRequestError),
    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),
    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),
    /// A documented error status without a response schema.
    #[error("HTTP error status: {status}")]
    HttpStatus { status: http::StatusCode, body: bytes::Bytes },
    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },
    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),
    /// You don't have the permission to update this organization, or dispute
    /// auto-accept isn't enabled for it.
    #[error(
        "{message}: {0}",
        message = "You don't have the permission to update this organization, or dispute \
                   auto-accept isn't enabled for it."
    )]
    NotPermitted(#[source] crate::v2026_04::models::common::types::NotPermitted),
    /// You don't have the permission to update this organization, or dispute
    /// auto-accept isn't enabled for it.
    #[error(
        "{message}: {0}",
        message = "You don't have the permission to update this organization, or dispute \
                   auto-accept isn't enabled for it."
    )]
    DisputeAutoAcceptNotEnabled(
        #[source] crate::v2026_04::models::organizations::types::DisputeAutoAcceptNotEnabled,
    ),
    /// Organization not found.
    #[error("{message}: {0}", message = "Organization not found.")]
    ResourceNotFound(#[source] crate::v2026_04::models::common::types::ResourceNotFound),
    /// Cannot enforce SSO without an enabled connection.
    #[error("{message}: {0}", message = "Cannot enforce SSO without an enabled connection.")]
    SsoEnforcementRequiresConnection(
        #[source] crate::v2026_04::models::organizations::types::SsoEnforcementRequiresConnection,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_04::models::common::types::HttpValidationError),
}
pub(super) type OrganizationsUpdateErrors = (
    crate::v2026_04::error::RequestFailure,
    crate::v2026_04::models::common::types::NotPermitted,
    crate::v2026_04::models::organizations::types::DisputeAutoAcceptNotEnabled,
    crate::v2026_04::models::common::types::ResourceNotFound,
    crate::v2026_04::models::organizations::types::SsoEnforcementRequiresConnection,
    crate::v2026_04::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<OrganizationsUpdateErrors>> for OrganizationsUpdateError<T> {
    fn from(error: eros::ErrorUnion<OrganizationsUpdateErrors>) -> Self {
        match error.into_enum() {
            eros::E6::A(error) => match error {
                crate::v2026_04::error::RequestFailure::BuildRequest(error) => {
                    Self::BuildRequest(error)
                }
                crate::v2026_04::error::RequestFailure::ReadBody(error) => Self::ReadBody(error),
                crate::v2026_04::error::RequestFailure::DecodeResponse(error) => {
                    Self::DecodeResponse(error)
                }
                crate::v2026_04::error::RequestFailure::HttpStatus { status, body } => {
                    Self::HttpStatus { status, body }
                }
                crate::v2026_04::error::RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                crate::v2026_04::error::RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E6::B(error) => Self::NotPermitted(error),
            eros::E6::C(error) => Self::DisputeAutoAcceptNotEnabled(error),
            eros::E6::D(error) => Self::ResourceNotFound(error),
            eros::E6::E(error) => Self::SsoEnforcementRequiresConnection(error),
            eros::E6::F(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> OrganizationsUpdateError<T> {
    pub(super) fn from_failure(error: crate::v2026_04::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<OrganizationsUpdateErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Update an organization.
///
/// **Scopes**: `organizations:write`
pub struct OrganizationsUpdate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2026_04::models::common::ids::OrganizationId,
    pub(super) payload: crate::v2026_04::models::organizations::types::OrganizationUpdate,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> OrganizationsUpdate<'a, ClientService> {
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
impl<'a, ClientService, Body> OrganizationsUpdate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_04::models::organizations::types::Organization,
        OrganizationsUpdateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            OrganizationsUpdateError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(OrganizationsUpdateError::Transport)?;
        let result: eros::Result<_, OrganizationsUpdateErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(
                    RequestFailure::ReadBody(error.into()),
                ))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => {
                    serde_json::from_slice::<
                        crate::v2026_04::models::organizations::types::Organization,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))
                }
                http::StatusCode::FORBIDDEN => {
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    enum ResponseError {
                        Payload0(crate::v2026_04::models::common::types::NotPermitted),
                        Payload1(
                            crate::v2026_04::models::organizations::types::DisputeAutoAcceptNotEnabled,
                        ),
                    }
                    let error = serde_json::from_slice::<ResponseError>(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(
                        match error {
                            ResponseError::Payload0(error) => {
                                eros::ErrorUnion::new(error)
                            }
                            ResponseError::Payload1(error) => {
                                eros::ErrorUnion::new(error)
                            }
                        },
                    )
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::common::types::ResourceNotFound,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::organizations::types::SsoEnforcementRequiresConnection,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::common::types::HttpValidationError,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                _ => {
                    Err(
                        eros::ErrorUnion::new(RequestFailure::UnexpectedStatus {
                            status: response_code,
                            body: body_slice,
                        }),
                    )
                }
            }
        }
            .await;
        result.map_err(Into::into)
    }

    pub async fn send(
        self,
    ) -> Result<
        crate::v2026_04::models::organizations::types::Organization,
        OrganizationsUpdateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            OrganizationsUpdateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for OrganizationsUpdate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<
        crate::v2026_04::models::organizations::types::Organization,
        OrganizationsUpdateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
