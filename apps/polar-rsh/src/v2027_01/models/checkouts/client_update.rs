use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CheckoutsClientUpdateError<TransportError> {
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
    /// Checkout session not found.
    #[error("{message}: {0}", message = "Checkout session not found.")]
    ResourceNotFound(#[source] crate::v2027_01::models::common::types::ResourceNotFound),
    /// The checkout is expired, the customer already has an active
    /// subscription, or the organization is not ready to accept payments.
    #[error(
        "{message}: {0}",
        message = "The checkout is expired, the customer already has an active subscription, or \
                   the organization is not ready to accept payments."
    )]
    AlreadyActiveSubscriptionError(
        #[source] crate::v2027_01::models::checkouts::types::AlreadyActiveSubscriptionError,
    ),
    /// The checkout is expired, the customer already has an active
    /// subscription, or the organization is not ready to accept payments.
    #[error(
        "{message}: {0}",
        message = "The checkout is expired, the customer already has an active subscription, or \
                   the organization is not ready to accept payments."
    )]
    NotOpenCheckout(#[source] crate::v2027_01::models::checkouts::types::NotOpenCheckout),
    /// The checkout is expired, the customer already has an active
    /// subscription, or the organization is not ready to accept payments.
    #[error(
        "{message}: {0}",
        message = "The checkout is expired, the customer already has an active subscription, or \
                   the organization is not ready to accept payments."
    )]
    NotPermitted(#[source] crate::v2027_01::models::common::types::NotPermitted),
    /// The checkout is expired, the customer already has an active
    /// subscription, or the organization is not ready to accept payments.
    #[error(
        "{message}: {0}",
        message = "The checkout is expired, the customer already has an active subscription, or \
                   the organization is not ready to accept payments."
    )]
    PaymentNotReady(#[source] crate::v2027_01::models::common::types::PaymentNotReady),
    /// The checkout is expired, the customer already has an active
    /// subscription, or the organization is not ready to accept payments.
    #[error(
        "{message}: {0}",
        message = "The checkout is expired, the customer already has an active subscription, or \
                   the organization is not ready to accept payments."
    )]
    TrialAlreadyRedeemed(#[source] crate::v2027_01::models::common::types::TrialAlreadyRedeemed),
    /// The checkout is expired, the customer already has an active
    /// subscription, or the organization is not ready to accept payments.
    #[error(
        "{message}: {0}",
        message = "The checkout is expired, the customer already has an active subscription, or \
                   the organization is not ready to accept payments."
    )]
    DiscountRedemptionLimitReached(
        #[source] crate::v2027_01::models::checkouts::types::DiscountRedemptionLimitReached,
    ),
    /// The checkout session is being processed.
    #[error("{message}: {0}", message = "The checkout session is being processed.")]
    CheckoutLocked(#[source] crate::v2027_01::models::checkouts::types::CheckoutLocked),
    /// The checkout session is expired.
    #[error("{message}: {0}", message = "The checkout session is expired.")]
    ExpiredCheckoutError(#[source] crate::v2027_01::models::checkouts::types::ExpiredCheckoutError),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type CheckoutsClientUpdateErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::ResourceNotFound,
    crate::v2027_01::models::checkouts::types::AlreadyActiveSubscriptionError,
    crate::v2027_01::models::checkouts::types::NotOpenCheckout,
    crate::v2027_01::models::common::types::NotPermitted,
    crate::v2027_01::models::common::types::PaymentNotReady,
    crate::v2027_01::models::common::types::TrialAlreadyRedeemed,
    crate::v2027_01::models::checkouts::types::DiscountRedemptionLimitReached,
    crate::v2027_01::models::checkouts::types::CheckoutLocked,
    crate::v2027_01::models::checkouts::types::ExpiredCheckoutError,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CheckoutsClientUpdateErrors>> for CheckoutsClientUpdateError<T> {
    fn from(error: eros::ErrorUnion<CheckoutsClientUpdateErrors>) -> Self {
        match error.into_enum() {
            eros::E11::A(error) => match error {
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
            eros::E11::B(error) => Self::ResourceNotFound(error),
            eros::E11::C(error) => Self::AlreadyActiveSubscriptionError(error),
            eros::E11::D(error) => Self::NotOpenCheckout(error),
            eros::E11::E(error) => Self::NotPermitted(error),
            eros::E11::F(error) => Self::PaymentNotReady(error),
            eros::E11::G(error) => Self::TrialAlreadyRedeemed(error),
            eros::E11::H(error) => Self::DiscountRedemptionLimitReached(error),
            eros::E11::I(error) => Self::CheckoutLocked(error),
            eros::E11::J(error) => Self::ExpiredCheckoutError(error),
            eros::E11::K(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CheckoutsClientUpdateError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CheckoutsClientUpdateErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Update a checkout session by client secret.
pub struct CheckoutsClientUpdate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) client_secret: String,
    pub(super) payload: crate::v2027_01::models::checkouts::types::CheckoutUpdatePublic,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CheckoutsClientUpdate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let encoded_client_secret = crate::v2027_01::path::encode_segment(&self.client_secret);
        let path = format!("v1/checkouts/client/{}", encoded_client_secret);
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::PATCH, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CheckoutsClientUpdate<'a, ClientService>
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
        CheckoutsClientUpdateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CheckoutsClientUpdateError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(CheckoutsClientUpdateError::Transport)?;
        let result: eros::Result<_, CheckoutsClientUpdateErrors> = async {
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
                http::StatusCode::OK => {
                    serde_json::from_slice::<
                        crate::v2027_01::models::checkouts::types::CheckoutPublic,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))
                }
                http::StatusCode::FORBIDDEN => {
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    enum ResponseError {
                        Payload0(
                            crate::v2027_01::models::checkouts::types::AlreadyActiveSubscriptionError,
                        ),
                        Payload1(
                            crate::v2027_01::models::checkouts::types::NotOpenCheckout,
                        ),
                        Payload2(crate::v2027_01::models::common::types::NotPermitted),
                        Payload3(
                            crate::v2027_01::models::common::types::PaymentNotReady,
                        ),
                        Payload4(
                            crate::v2027_01::models::common::types::TrialAlreadyRedeemed,
                        ),
                        Payload5(
                            crate::v2027_01::models::checkouts::types::DiscountRedemptionLimitReached,
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
                            ResponseError::Payload2(error) => {
                                eros::ErrorUnion::new(error)
                            }
                            ResponseError::Payload3(error) => {
                                eros::ErrorUnion::new(error)
                            }
                            ResponseError::Payload4(error) => {
                                eros::ErrorUnion::new(error)
                            }
                            ResponseError::Payload5(error) => {
                                eros::ErrorUnion::new(error)
                            }
                        },
                    )
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
                        crate::v2027_01::models::checkouts::types::CheckoutLocked,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::GONE => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::checkouts::types::ExpiredCheckoutError,
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
        crate::v2027_01::models::checkouts::types::CheckoutPublic,
        CheckoutsClientUpdateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CheckoutsClientUpdateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for CheckoutsClientUpdate<'a, ClientService>
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
        CheckoutsClientUpdateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
