use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomerMetersCreditsError<TransportError> {
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
    /// The external_id is already used by a different event, or the external
    /// customer ID matches customers in several accessible organizations.
    #[error(
        "{message}: {0}",
        message = "The external_id is already used by a different event, or the external customer \
                   ID matches customers in several accessible organizations."
    )]
    CustomerMeterCreditExternalIdConflict(
        #[source]
        crate::v2027_01::models::customer_meters::types::CustomerMeterCreditExternalIdConflict,
    ),
    /// The external_id is already used by a different event, or the external
    /// customer ID matches customers in several accessible organizations.
    #[error(
        "{message}: {0}",
        message = "The external_id is already used by a different event, or the external customer \
                   ID matches customers in several accessible organizations."
    )]
    AmbiguousExternalCustomerId(
        #[source] crate::v2027_01::models::common::types::AmbiguousExternalCustomerId,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type CustomerMetersCreditsErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::NotPermitted,
    crate::v2027_01::models::customer_meters::types::CustomerMeterCreditExternalIdConflict,
    crate::v2027_01::models::common::types::AmbiguousExternalCustomerId,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomerMetersCreditsErrors>> for CustomerMetersCreditsError<T> {
    fn from(error: eros::ErrorUnion<CustomerMetersCreditsErrors>) -> Self {
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
            eros::E5::C(error) => Self::CustomerMeterCreditExternalIdConflict(error),
            eros::E5::D(error) => Self::AmbiguousExternalCustomerId(error),
            eros::E5::E(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomerMetersCreditsError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomerMetersCreditsErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Manually grant credits on a customer meter.
///
/// Returns the `meter.credited` event. The customer meter balance is updated
/// before the response is returned, unless the meter is being updated
/// concurrently, in which case it catches up shortly after.
///
/// **Scopes**: `customers:write`
pub struct CustomerMetersCredits<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) payload:
        crate::v2027_01::models::customer_meters::unions::CustomerMetersCreditsRequestUnion,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomerMetersCredits<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = "v1/customer-meters/credits".to_owned();
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::POST, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomerMetersCredits<'a, ClientService>
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
        CustomerMetersCreditsError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CustomerMetersCreditsError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(CustomerMetersCreditsError::Transport)?;
        let result: eros::Result<_, CustomerMetersCreditsErrors> = async {
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
                http::StatusCode::CONFLICT => {
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    enum ResponseError {
                        Payload0(
                            crate::v2027_01::models::customer_meters::types::CustomerMeterCreditExternalIdConflict,
                        ),
                        Payload1(
                            crate::v2027_01::models::common::types::AmbiguousExternalCustomerId,
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
        CustomerMetersCreditsError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomerMetersCreditsError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for CustomerMetersCredits<'a, ClientService>
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
        CustomerMetersCreditsError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
