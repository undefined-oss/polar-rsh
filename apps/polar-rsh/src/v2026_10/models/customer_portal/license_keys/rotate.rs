use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomerPortalLicenseKeysRotateError<TransportError> {
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
    /// License key cannot be rotated in its current status. Allowed statuses:
    /// disabled, granted.
    #[error(
        "{message}: {0}",
        message = "License key cannot be rotated in its current status. Allowed statuses: \
                   disabled, granted."
    )]
    RotateNotPermitted(#[source] crate::v2026_10::models::license_keys::types::RotateNotPermitted),
    /// Not authorized to manage license key.
    #[error("{message}: {0}", message = "Not authorized to manage license key.")]
    Unauthorized(#[source] crate::v2026_10::models::license_keys::types::Unauthorized),
    /// License key not found.
    #[error("{message}: {0}", message = "License key not found.")]
    ResourceNotFound(#[source] crate::v2026_10::models::common::types::ResourceNotFound),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type CustomerPortalLicenseKeysRotateErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::license_keys::types::RotateNotPermitted,
    crate::v2026_10::models::license_keys::types::Unauthorized,
    crate::v2026_10::models::common::types::ResourceNotFound,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomerPortalLicenseKeysRotateErrors>>
    for CustomerPortalLicenseKeysRotateError<T>
{
    fn from(error: eros::ErrorUnion<CustomerPortalLicenseKeysRotateErrors>) -> Self {
        match error.into_enum() {
            eros::E5::A(error) => match error {
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
            eros::E5::B(error) => Self::RotateNotPermitted(error),
            eros::E5::C(error) => Self::Unauthorized(error),
            eros::E5::D(error) => Self::ResourceNotFound(error),
            eros::E5::E(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomerPortalLicenseKeysRotateError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomerPortalLicenseKeysRotateErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Rotate a license key.
///
/// Generates a new key string for the same license key record. The previous
/// key string immediately stops validating. Status, usage, limits, expiry,
/// and activations are preserved.
///
/// **Scopes**: `customer_portal:write`
pub struct CustomerPortalLicenseKeysRotate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomerPortalLicenseKeysRotate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/customer-portal/license-keys/{}/rotate", self.id);
        self.client.build_request(http::Method::POST, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomerPortalLicenseKeysRotate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::license_keys::types::RotatedLicenseKey,
        CustomerPortalLicenseKeysRotateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CustomerPortalLicenseKeysRotateError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self
            .client
            .send(request)
            .await
            .map_err(CustomerPortalLicenseKeysRotateError::Transport)?;
        let result: eros::Result<_, CustomerPortalLicenseKeysRotateErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2026_10::models::license_keys::types::RotatedLicenseKey,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::BAD_REQUEST => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::license_keys::types::RotateNotPermitted,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNAUTHORIZED => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::license_keys::types::Unauthorized,
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
        crate::v2026_10::models::license_keys::types::RotatedLicenseKey,
        CustomerPortalLicenseKeysRotateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomerPortalLicenseKeysRotateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomerPortalLicenseKeysRotate<'a, ClientService>
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
        crate::v2026_10::models::license_keys::types::RotatedLicenseKey,
        CustomerPortalLicenseKeysRotateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
