use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CheckoutsClientGetError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),
    #[error("failed to build request: {0}")]
    BuildRequest(#[source] crate::v2027_01::error::BuildRequestError),
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
    /// The organization is not allowed to accept payments.
    #[error("{message}: {0}", message = "The organization is not allowed to accept payments.")]
    NotPermitted(#[source] crate::v2027_01::models::common::types::NotPermitted),
    /// Checkout session not found.
    #[error("{message}: {0}", message = "Checkout session not found.")]
    ResourceNotFound(#[source] crate::v2027_01::models::common::types::ResourceNotFound),
    /// The checkout session is expired.
    #[error("{message}: {0}", message = "The checkout session is expired.")]
    ExpiredCheckoutError(#[source] crate::v2027_01::models::checkouts::types::ExpiredCheckoutError),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type CheckoutsClientGetErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::NotPermitted,
    crate::v2027_01::models::common::types::ResourceNotFound,
    crate::v2027_01::models::checkouts::types::ExpiredCheckoutError,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CheckoutsClientGetErrors>> for CheckoutsClientGetError<T> {
    fn from(error: eros::ErrorUnion<CheckoutsClientGetErrors>) -> Self {
        match error.into_enum() {
            eros::E5::A(error) => match error {
                crate::v2027_01::error::RequestFailure::BuildRequest(error) => {
                    Self::BuildRequest(error)
                }
                crate::v2027_01::error::RequestFailure::ReadBody(error) => Self::ReadBody(error),
                crate::v2027_01::error::RequestFailure::DecodeResponse(error) => {
                    Self::DecodeResponse(error)
                }
                crate::v2027_01::error::RequestFailure::HttpStatus { status, body } => {
                    Self::HttpStatus { status, body }
                }
                crate::v2027_01::error::RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                crate::v2027_01::error::RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E5::B(error) => Self::NotPermitted(error),
            eros::E5::C(error) => Self::ResourceNotFound(error),
            eros::E5::D(error) => Self::ExpiredCheckoutError(error),
            eros::E5::E(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CheckoutsClientGetError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CheckoutsClientGetErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Get a checkout session by client secret.
pub struct CheckoutsClientGet<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) client_secret: String,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CheckoutsClientGet<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let encoded_client_secret = crate::v2027_01::path::encode_segment(&self.client_secret);
        let path = format!("v1/checkouts/client/{}", encoded_client_secret);
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CheckoutsClientGet<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::checkouts::types::CheckoutPublic,
        CheckoutsClientGetError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CheckoutsClientGetError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(CheckoutsClientGetError::Transport)?;
        let result: eros::Result<_, CheckoutsClientGetErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2027_01::models::checkouts::types::CheckoutPublic,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::NotPermitted,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::ResourceNotFound,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::GONE => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::checkouts::types::ExpiredCheckoutError,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::HttpValidationError,
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
        crate::v2027_01::models::checkouts::types::CheckoutPublic,
        CheckoutsClientGetError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CheckoutsClientGetError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for CheckoutsClientGet<'a, ClientService>
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
        crate::v2027_01::models::checkouts::types::CheckoutPublic,
        CheckoutsClientGetError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
