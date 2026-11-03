use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum OrdersGenerateInvoiceError<TransportError> {
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
    /// Order not found.
    #[error("{message}: {0}", message = "Order not found.")]
    ResourceNotFound(#[source] crate::v2027_01::models::common::types::ResourceNotFound),
    /// Order is not eligible for invoice generation (invalid status).
    #[error(
        "{message}: {0}",
        message = "Order is not eligible for invoice generation (invalid status)."
    )]
    OrderNotEligibleForInvoice(
        #[source] crate::v2027_01::models::orders::types::OrderNotEligibleForInvoice,
    ),
    /// Order is missing billing name or address.
    #[error("{message}: {0}", message = "Order is missing billing name or address.")]
    MissingInvoiceBillingDetails(
        #[source] crate::v2027_01::models::orders::types::MissingInvoiceBillingDetails,
    ),
}
pub(super) type OrdersGenerateInvoiceErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::ResourceNotFound,
    crate::v2027_01::models::orders::types::OrderNotEligibleForInvoice,
    crate::v2027_01::models::orders::types::MissingInvoiceBillingDetails,
);
impl<T> From<eros::ErrorUnion<OrdersGenerateInvoiceErrors>> for OrdersGenerateInvoiceError<T> {
    fn from(error: eros::ErrorUnion<OrdersGenerateInvoiceErrors>) -> Self {
        match error.into_enum() {
            eros::E4::A(error) => match error {
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
            eros::E4::B(error) => Self::ResourceNotFound(error),
            eros::E4::C(error) => Self::OrderNotEligibleForInvoice(error),
            eros::E4::D(error) => Self::MissingInvoiceBillingDetails(error),
        }
    }
}
impl<T> OrdersGenerateInvoiceError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<OrdersGenerateInvoiceErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Trigger generation of an order's invoice.
///
/// **Scopes**: `orders:read`
pub struct OrdersGenerateInvoice<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2027_01::models::common::ids::OrderId,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> OrdersGenerateInvoice<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/orders/{}/invoice", self.id);
        self.client.build_request(http::Method::POST, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> OrdersGenerateInvoice<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<serde_json::Value, OrdersGenerateInvoiceError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            OrdersGenerateInvoiceError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(OrdersGenerateInvoiceError::Transport)?;
        let result: eros::Result<_, OrdersGenerateInvoiceErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::ACCEPTED => {
                    serde_json::from_slice::<serde_json::Value>(&body_slice).map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })
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
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::orders::types::OrderNotEligibleForInvoice,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::orders::types::MissingInvoiceBillingDetails,
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
    ) -> Result<serde_json::Value, OrdersGenerateInvoiceError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            OrdersGenerateInvoiceError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for OrdersGenerateInvoice<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<serde_json::Value, OrdersGenerateInvoiceError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
