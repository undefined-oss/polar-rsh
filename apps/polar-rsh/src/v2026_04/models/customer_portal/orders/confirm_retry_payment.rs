use http_body_util::BodyExt;

use crate::v2026_04::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomerPortalOrdersConfirmRetryPaymentError<TransportError> {
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
    /// Order not found.
    #[error("{message}: {0}", message = "Order not found.")]
    ResourceNotFound(#[source] crate::v2026_04::models::common::types::ResourceNotFound),
    /// Payment already in progress.
    #[error("{message}: {0}", message = "Payment already in progress.")]
    PaymentAlreadyInProgress(
        #[source] crate::v2026_04::models::orders::types::PaymentAlreadyInProgress,
    ),
    /// Order not eligible for retry or payment confirmation failed.
    #[error(
        "{message}: {0}",
        message = "Order not eligible for retry or payment confirmation failed."
    )]
    OrderNotEligibleForRetry(
        #[source] crate::v2026_04::models::orders::types::OrderNotEligibleForRetry,
    ),
    /// Manual retry limit exceeded.
    #[error("{message}: {0}", message = "Manual retry limit exceeded.")]
    ManualRetryLimitExceeded(
        #[source] crate::v2026_04::models::orders::types::ManualRetryLimitExceeded,
    ),
}
pub(super) type CustomerPortalOrdersConfirmRetryPaymentErrors = (
    crate::v2026_04::error::RequestFailure,
    crate::v2026_04::models::common::types::ResourceNotFound,
    crate::v2026_04::models::orders::types::PaymentAlreadyInProgress,
    crate::v2026_04::models::orders::types::OrderNotEligibleForRetry,
    crate::v2026_04::models::orders::types::ManualRetryLimitExceeded,
);
impl<T> From<eros::ErrorUnion<CustomerPortalOrdersConfirmRetryPaymentErrors>>
    for CustomerPortalOrdersConfirmRetryPaymentError<T>
{
    fn from(error: eros::ErrorUnion<CustomerPortalOrdersConfirmRetryPaymentErrors>) -> Self {
        match error.into_enum() {
            eros::E5::A(error) => match error {
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
            eros::E5::B(error) => Self::ResourceNotFound(error),
            eros::E5::C(error) => Self::PaymentAlreadyInProgress(error),
            eros::E5::D(error) => Self::OrderNotEligibleForRetry(error),
            eros::E5::E(error) => Self::ManualRetryLimitExceeded(error),
        }
    }
}
impl<T> CustomerPortalOrdersConfirmRetryPaymentError<T> {
    pub(super) fn from_failure(error: crate::v2026_04::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomerPortalOrdersConfirmRetryPaymentErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Confirm a retry payment using a Stripe confirmation token.
///
/// **Scopes**: `customer_portal:write`
pub struct CustomerPortalOrdersConfirmRetryPayment<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2026_04::models::common::ids::OrderId,
    pub(super) payload: crate::v2026_04::models::orders::types::CustomerOrderConfirmPayment,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomerPortalOrdersConfirmRetryPayment<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/customer-portal/orders/{}/confirm-payment", self.id);
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::POST, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomerPortalOrdersConfirmRetryPayment<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_04::models::orders::types::CustomerOrderPaymentConfirmation,
        CustomerPortalOrdersConfirmRetryPaymentError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CustomerPortalOrdersConfirmRetryPaymentError::from_failure(
                RequestFailure::BuildRequest(error),
            )
        })?;
        let response = self
            .client
            .send(request)
            .await
            .map_err(CustomerPortalOrdersConfirmRetryPaymentError::Transport)?;
        let result: eros::Result<_, CustomerPortalOrdersConfirmRetryPaymentErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2026_04::models::orders::types::CustomerOrderPaymentConfirmation,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::common::types::ResourceNotFound,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::orders::types::PaymentAlreadyInProgress,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::orders::types::OrderNotEligibleForRetry,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::TOO_MANY_REQUESTS => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::orders::types::ManualRetryLimitExceeded,
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
        crate::v2026_04::models::orders::types::CustomerOrderPaymentConfirmation,
        CustomerPortalOrdersConfirmRetryPaymentError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomerPortalOrdersConfirmRetryPaymentError::from_failure(RequestFailure::Timeout(
                error,
            ))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomerPortalOrdersConfirmRetryPayment<'a, ClientService>
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
        crate::v2026_04::models::orders::types::CustomerOrderPaymentConfirmation,
        CustomerPortalOrdersConfirmRetryPaymentError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
