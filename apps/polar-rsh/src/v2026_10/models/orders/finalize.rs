use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum OrdersFinalizeError<TransportError> {
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
    /// The charge failed, or requires customer authentication (e.g. a 3DS
    /// challenge) that can't be completed off-session.
    #[error(
        "{message}: {0}",
        message = "The charge failed, or requires customer authentication (e.g. a 3DS challenge) \
                   that can't be completed off-session."
    )]
    PaymentFailed(#[source] crate::v2026_10::models::common::types::PaymentFailed),
    /// The charge failed, or requires customer authentication (e.g. a 3DS
    /// challenge) that can't be completed off-session.
    #[error(
        "{message}: {0}",
        message = "The charge failed, or requires customer authentication (e.g. a 3DS challenge) \
                   that can't be completed off-session."
    )]
    PaymentActionRequired(#[source] crate::v2026_10::models::common::types::PaymentActionRequired),
    /// Off-session charges are not enabled for this organization, or its
    /// account can't currently accept payments.
    #[error(
        "{message}: {0}",
        message = "Off-session charges are not enabled for this organization, or its account \
                   can't currently accept payments."
    )]
    OffSessionChargesNotEnabled(
        #[source] crate::v2026_10::models::orders::types::OffSessionChargesNotEnabled,
    ),
    /// Off-session charges are not enabled for this organization, or its
    /// account can't currently accept payments.
    #[error(
        "{message}: {0}",
        message = "Off-session charges are not enabled for this organization, or its account \
                   can't currently accept payments."
    )]
    OrganizationNotReadyForPayments(
        #[source] crate::v2026_10::models::orders::types::OrganizationNotReadyForPayments,
    ),
    /// Order not found.
    #[error("{message}: {0}", message = "Order not found.")]
    ResourceNotFound(#[source] crate::v2026_10::models::common::types::ResourceNotFound),
    /// The order is not in `draft` status.
    #[error("{message}: {0}", message = "The order is not in `draft` status.")]
    OrderNotDraft(#[source] crate::v2026_10::models::orders::types::OrderNotDraft),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type OrdersFinalizeErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::PaymentFailed,
    crate::v2026_10::models::common::types::PaymentActionRequired,
    crate::v2026_10::models::orders::types::OffSessionChargesNotEnabled,
    crate::v2026_10::models::orders::types::OrganizationNotReadyForPayments,
    crate::v2026_10::models::common::types::ResourceNotFound,
    crate::v2026_10::models::orders::types::OrderNotDraft,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<OrdersFinalizeErrors>> for OrdersFinalizeError<T> {
    fn from(error: eros::ErrorUnion<OrdersFinalizeErrors>) -> Self {
        match error.into_enum() {
            eros::E8::A(error) => match error {
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
            eros::E8::B(error) => Self::PaymentFailed(error),
            eros::E8::C(error) => Self::PaymentActionRequired(error),
            eros::E8::D(error) => Self::OffSessionChargesNotEnabled(error),
            eros::E8::E(error) => Self::OrganizationNotReadyForPayments(error),
            eros::E8::F(error) => Self::ResourceNotFound(error),
            eros::E8::G(error) => Self::OrderNotDraft(error),
            eros::E8::H(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> OrdersFinalizeError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<OrdersFinalizeErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Finalize a draft order and synchronously attempt an off-session charge.
///
/// On success, the order transitions to `paid` and benefit grants fire
/// before the response returns. On failure (decline, missing payment method,
/// SCA challenge), the order stays in `draft` and a 4xx error is returned.
///
/// The request fails with 412 if the order is not in `draft` status.
///
/// **Scopes**: `orders:write`
pub struct OrdersFinalize<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2026_10::models::common::ids::OrderId,
    pub(super) payload: Option<crate::v2026_10::models::orders::types::OrderFinalize>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> OrdersFinalize<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/orders/{}/finalize", self.id);
        let body = self.payload.as_ref().map(serde_json::to_vec).transpose()?;
        self.client.build_request(http::Method::POST, &path, body.map(Into::into), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    /// Set the optional request body.
    pub fn payload(
        mut self,
        payload: crate::v2026_10::models::orders::types::OrderFinalize,
    ) -> Self {
        self.payload = Some(payload);
        self
    }
}
impl<'a, ClientService, Body> OrdersFinalize<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::orders::types::Order,
        OrdersFinalizeError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            OrdersFinalizeError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self.client.send(request).await.map_err(OrdersFinalizeError::Transport)?;
        let result: eros::Result<_, OrdersFinalizeErrors> =
            async {
                let response_code = response.status();
                let body_slice = response
                    .into_body()
                    .collect()
                    .await
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                    .to_bytes();
                match response_code {
                    http::StatusCode::OK => serde_json::from_slice::<
                        crate::v2026_10::models::orders::types::Order,
                    >(&body_slice)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                    http::StatusCode::PAYMENT_REQUIRED => {
                        #[derive(serde::Deserialize)]
                        #[serde(untagged)]
                        enum ResponseError {
                            Payload0(crate::v2026_10::models::common::types::PaymentFailed),
                            Payload1(crate::v2026_10::models::common::types::PaymentActionRequired),
                        }
                        let error = serde_json::from_slice::<ResponseError>(&body_slice).map_err(
                            |error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error)),
                        )?;
                        Err(match error {
                            ResponseError::Payload0(error) => eros::ErrorUnion::new(error),
                            ResponseError::Payload1(error) => eros::ErrorUnion::new(error),
                        })
                    }
                    http::StatusCode::FORBIDDEN => {
                        #[derive(serde::Deserialize)]
                        #[serde(untagged)]
                        enum ResponseError {
                            Payload0(
                            crate::v2026_10::models::orders::types::OffSessionChargesNotEnabled,
                        ),
                        Payload1(
                            crate::v2026_10::models::orders::types::OrganizationNotReadyForPayments,
                        ),
                    }
                        let error = serde_json::from_slice::<ResponseError>(&body_slice).map_err(
                            |error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error)),
                        )?;
                        Err(match error {
                            ResponseError::Payload0(error) => eros::ErrorUnion::new(error),
                            ResponseError::Payload1(error) => eros::ErrorUnion::new(error),
                        })
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
                    http::StatusCode::PRECONDITION_FAILED => {
                        let error = serde_json::from_slice::<
                            crate::v2026_10::models::orders::types::OrderNotDraft,
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
        crate::v2026_10::models::orders::types::Order,
        OrdersFinalizeError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| OrdersFinalizeError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for OrdersFinalize<'a, ClientService>
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
        crate::v2026_10::models::orders::types::Order,
        OrdersFinalizeError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
