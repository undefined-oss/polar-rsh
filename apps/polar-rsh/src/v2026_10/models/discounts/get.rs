use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum DiscountsGetError<TransportError> {
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
    /// Discount not found.
    #[error("{message}: {0}", message = "Discount not found.")]
    ResourceNotFound(#[source] crate::v2026_10::models::common::types::ResourceNotFound),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type DiscountsGetErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::ResourceNotFound,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<DiscountsGetErrors>> for DiscountsGetError<T> {
    fn from(error: eros::ErrorUnion<DiscountsGetErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
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
            eros::E3::B(error) => Self::ResourceNotFound(error),
            eros::E3::C(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> DiscountsGetError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<DiscountsGetErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Get a discount by ID.
///
/// **Scopes**: `discounts:read` `discounts:write`
pub struct DiscountsGet<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2026_10::models::common::ids::DiscountId,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> DiscountsGet<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/discounts/{}", self.id);
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> DiscountsGet<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::discounts::unions::Discount,
        DiscountsGetError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            DiscountsGetError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self.client.send(request).await.map_err(DiscountsGetError::Transport)?;
        let result: eros::Result<_, DiscountsGetErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2026_10::models::discounts::unions::Discount,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
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
        crate::v2026_10::models::discounts::unions::Discount,
        DiscountsGetError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| DiscountsGetError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for DiscountsGet<'a, ClientService>
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
        crate::v2026_10::models::discounts::unions::Discount,
        DiscountsGetError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
