use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomerMetersReverseCreditError<TransportError> {
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
    /// You don't have permission to manage customers.
    #[error("{message}: {0}", message = "You don't have permission to manage customers.")]
    NotPermitted(#[source] crate::v2027_01::models::common::types::NotPermitted),
    /// Manual credit not found.
    #[error("{message}: {0}", message = "Manual credit not found.")]
    ResourceNotFound(#[source] crate::v2027_01::models::common::types::ResourceNotFound),
    /// The credit can't be reversed, or the customer meter is being updated:
    /// retry shortly.
    #[error(
        "{message}: {0}",
        message = "The credit can't be reversed, or the customer meter is being updated: retry \
                   shortly."
    )]
    CustomerMeterCreditNotReversible(
        #[source] crate::v2027_01::models::customer_meters::types::CustomerMeterCreditNotReversible,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type CustomerMetersReverseCreditErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::NotPermitted,
    crate::v2027_01::models::common::types::ResourceNotFound,
    crate::v2027_01::models::customer_meters::types::CustomerMeterCreditNotReversible,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomerMetersReverseCreditErrors>>
    for CustomerMetersReverseCreditError<T>
{
    fn from(error: eros::ErrorUnion<CustomerMetersReverseCreditErrors>) -> Self {
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
            eros::E5::D(error) => Self::CustomerMeterCreditNotReversible(error),
            eros::E5::E(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomerMetersReverseCreditError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomerMetersReverseCreditErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Reverse a manual credit, removing the units the customer hasn't used yet.
///
/// Works once per credit, within 7 days and in the current billing period.
/// Returns a `meter.credited` event with negative units and
/// `parent_id` set to the reversed credit.
///
/// **Scopes**: `customers:write`
pub struct CustomerMetersReverseCredit<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2027_01::models::common::ids::CreditId,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomerMetersReverseCredit<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/customer-meters/credits/{}/reverse", self.id);
        self.client.build_request(http::Method::POST, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomerMetersReverseCredit<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::common::types::MeterCreditEvent,
        CustomerMetersReverseCreditError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CustomerMetersReverseCreditError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(CustomerMetersReverseCreditError::Transport)?;
        let result: eros::Result<_, CustomerMetersReverseCreditErrors> = async {
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
                http::StatusCode::CREATED => {
                    serde_json::from_slice::<
                        crate::v2027_01::models::common::types::MeterCreditEvent,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))
                }
                http::StatusCode::FORBIDDEN => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::NotPermitted,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::ResourceNotFound,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::customer_meters::types::CustomerMeterCreditNotReversible,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::HttpValidationError,
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
        crate::v2027_01::models::common::types::MeterCreditEvent,
        CustomerMetersReverseCreditError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomerMetersReverseCreditError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomerMetersReverseCredit<'a, ClientService>
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
        crate::v2027_01::models::common::types::MeterCreditEvent,
        CustomerMetersReverseCreditError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
