use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum SubscriptionsCreateError<TransportError> {
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
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type SubscriptionsCreateErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<SubscriptionsCreateErrors>> for SubscriptionsCreateError<T> {
    fn from(error: eros::ErrorUnion<SubscriptionsCreateErrors>) -> Self {
        match error.into_enum() {
            eros::E2::A(error) => match error {
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
            eros::E2::B(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> SubscriptionsCreateError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<SubscriptionsCreateErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Create a subscription programmatically.
///
/// This endpoint only allows to create subscription on free products.
/// For paid products, use the checkout flow.
///
/// No initial order will be created and no confirmation email will be sent.
///
/// **Scopes**: `subscriptions:write`
pub struct SubscriptionsCreate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) payload:
        crate::v2027_01::models::subscriptions::unions::SubscriptionsCreateRequestUnion,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> SubscriptionsCreate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = "v1/subscriptions/".to_owned();
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::POST, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> SubscriptionsCreate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::subscriptions::types::Subscription,
        SubscriptionsCreateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            SubscriptionsCreateError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(SubscriptionsCreateError::Transport)?;
        let result: eros::Result<_, SubscriptionsCreateErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::CREATED => serde_json::from_slice::<
                    crate::v2027_01::models::subscriptions::types::Subscription,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
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
        crate::v2027_01::models::subscriptions::types::Subscription,
        SubscriptionsCreateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            SubscriptionsCreateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for SubscriptionsCreate<'a, ClientService>
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
        crate::v2027_01::models::subscriptions::types::Subscription,
        SubscriptionsCreateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
