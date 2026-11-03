use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomerPortalLicenseKeysActivateError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),
    #[error("failed to build request: {0}")]
    BuildRequest(#[source] crate::v2026_10::error::BuildRequestError),
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
    /// License key is revoked, disabled, or expired, does not support
    /// activations, or has reached its activation limit. Use /validate for
    /// licenses without activations.
    #[error(
        "{message}: {0}",
        message = "License key is revoked, disabled, or expired, does not support activations, or \
                   has reached its activation limit. Use /validate for licenses without \
                   activations."
    )]
    NotPermitted(#[source] crate::v2026_10::models::common::types::NotPermitted),
    /// License key not found.
    #[error("{message}: {0}", message = "License key not found.")]
    ResourceNotFound(#[source] crate::v2026_10::models::common::types::ResourceNotFound),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type CustomerPortalLicenseKeysActivateErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::NotPermitted,
    crate::v2026_10::models::common::types::ResourceNotFound,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomerPortalLicenseKeysActivateErrors>>
    for CustomerPortalLicenseKeysActivateError<T>
{
    fn from(error: eros::ErrorUnion<CustomerPortalLicenseKeysActivateErrors>) -> Self {
        match error.into_enum() {
            eros::E4::A(error) => match error {
                crate::v2026_10::error::RequestFailure::BuildRequest(error) => {
                    Self::BuildRequest(error)
                }
                crate::v2026_10::error::RequestFailure::ReadBody(error) => Self::ReadBody(error),
                crate::v2026_10::error::RequestFailure::DecodeResponse(error) => {
                    Self::DecodeResponse(error)
                }
                crate::v2026_10::error::RequestFailure::HttpStatus { status, body } => {
                    Self::HttpStatus { status, body }
                }
                crate::v2026_10::error::RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                crate::v2026_10::error::RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E4::B(error) => Self::NotPermitted(error),
            eros::E4::C(error) => Self::ResourceNotFound(error),
            eros::E4::D(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomerPortalLicenseKeysActivateError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomerPortalLicenseKeysActivateErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Activate a license key instance.
///
/// > This endpoint doesn't require authentication and can be safely used on a
/// > public
/// > client, like a desktop application or a mobile app.
/// > If you plan to validate a license key on a server, use the
/// > `/v1/license-keys/activate`
/// > endpoint instead.
pub struct CustomerPortalLicenseKeysActivate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) payload: crate::v2026_10::models::license_keys::types::LicenseKeyActivate,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomerPortalLicenseKeysActivate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = "v1/customer-portal/license-keys/activate".to_owned();
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::POST, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomerPortalLicenseKeysActivate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::license_keys::types::LicenseKeyActivationCreated,
        CustomerPortalLicenseKeysActivateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CustomerPortalLicenseKeysActivateError::from_failure(RequestFailure::BuildRequest(
                error,
            ))
        })?;
        let response = self
            .client
            .send(request)
            .await
            .map_err(CustomerPortalLicenseKeysActivateError::Transport)?;
        let result: eros::Result<_, CustomerPortalLicenseKeysActivateErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2026_10::models::license_keys::types::LicenseKeyActivationCreated,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::NotPermitted,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::ResourceNotFound,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::HttpValidationError,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                _ => Err(eros::ErrorUnion::new(RequestFailure::UnexpectedStatus {
                    status: response_code,
                    body: body_slice,
                })),
            }
        }
        .await;
        result.map_err(Into::into)
    }

    pub async fn send(
        self,
    ) -> Result<
        crate::v2026_10::models::license_keys::types::LicenseKeyActivationCreated,
        CustomerPortalLicenseKeysActivateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomerPortalLicenseKeysActivateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomerPortalLicenseKeysActivate<'a, ClientService>
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
        crate::v2026_10::models::license_keys::types::LicenseKeyActivationCreated,
        CustomerPortalLicenseKeysActivateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
