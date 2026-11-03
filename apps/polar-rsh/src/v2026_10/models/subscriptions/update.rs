use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum SubscriptionsUpdateError<TransportError> {
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
    /// The charge failed, or requires customer authentication that can't be
    /// completed off-session.
    #[error(
        "{message}: {0}",
        message = "The charge failed, or requires customer authentication that can't be completed \
                   off-session."
    )]
    PaymentFailed(#[source] crate::v2026_10::models::common::types::PaymentFailed),
    /// The charge failed, or requires customer authentication that can't be
    /// completed off-session.
    #[error(
        "{message}: {0}",
        message = "The charge failed, or requires customer authentication that can't be completed \
                   off-session."
    )]
    PaymentActionRequired(#[source] crate::v2026_10::models::common::types::PaymentActionRequired),
    /// Subscription is already canceled or will be at the end of the period, is
    /// not active, or the organization is not ready to renew subscriptions.
    #[error(
        "{message}: {0}",
        message = "Subscription is already canceled or will be at the end of the period, is not \
                   active, or the organization is not ready to renew subscriptions."
    )]
    AlreadyCanceledSubscription(
        #[source] crate::v2026_10::models::subscriptions::types::AlreadyCanceledSubscription,
    ),
    /// Subscription is already canceled or will be at the end of the period, is
    /// not active, or the organization is not ready to renew subscriptions.
    #[error(
        "{message}: {0}",
        message = "Subscription is already canceled or will be at the end of the period, is not \
                   active, or the organization is not ready to renew subscriptions."
    )]
    InactiveSubscription(
        #[source] crate::v2026_10::models::subscriptions::types::InactiveSubscription,
    ),
    /// Subscription is already canceled or will be at the end of the period, is
    /// not active, or the organization is not ready to renew subscriptions.
    #[error(
        "{message}: {0}",
        message = "Subscription is already canceled or will be at the end of the period, is not \
                   active, or the organization is not ready to renew subscriptions."
    )]
    PaymentNotReady(#[source] crate::v2026_10::models::common::types::PaymentNotReady),
    /// Subscription not found.
    #[error("{message}: {0}", message = "Subscription not found.")]
    ResourceNotFound(#[source] crate::v2026_10::models::common::types::ResourceNotFound),
    /// Subscription is pending an update, or is not scheduled to be canceled.
    #[error(
        "{message}: {0}",
        message = "Subscription is pending an update, or is not scheduled to be canceled."
    )]
    SubscriptionLocked(#[source] crate::v2026_10::models::subscriptions::types::SubscriptionLocked),
    /// Subscription is pending an update, or is not scheduled to be canceled.
    #[error(
        "{message}: {0}",
        message = "Subscription is pending an update, or is not scheduled to be canceled."
    )]
    SubscriptionNotScheduledToCancel(
        #[source] crate::v2026_10::models::subscriptions::types::SubscriptionNotScheduledToCancel,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type SubscriptionsUpdateErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::PaymentFailed,
    crate::v2026_10::models::common::types::PaymentActionRequired,
    crate::v2026_10::models::subscriptions::types::AlreadyCanceledSubscription,
    crate::v2026_10::models::subscriptions::types::InactiveSubscription,
    crate::v2026_10::models::common::types::PaymentNotReady,
    crate::v2026_10::models::common::types::ResourceNotFound,
    crate::v2026_10::models::subscriptions::types::SubscriptionLocked,
    crate::v2026_10::models::subscriptions::types::SubscriptionNotScheduledToCancel,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<SubscriptionsUpdateErrors>> for SubscriptionsUpdateError<T> {
    fn from(error: eros::ErrorUnion<SubscriptionsUpdateErrors>) -> Self {
        match error.into_enum() {
            eros::E10::A(error) => match error {
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
            eros::E10::B(error) => Self::PaymentFailed(error),
            eros::E10::C(error) => Self::PaymentActionRequired(error),
            eros::E10::D(error) => Self::AlreadyCanceledSubscription(error),
            eros::E10::E(error) => Self::InactiveSubscription(error),
            eros::E10::F(error) => Self::PaymentNotReady(error),
            eros::E10::G(error) => Self::ResourceNotFound(error),
            eros::E10::H(error) => Self::SubscriptionLocked(error),
            eros::E10::I(error) => Self::SubscriptionNotScheduledToCancel(error),
            eros::E10::J(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> SubscriptionsUpdateError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<SubscriptionsUpdateErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Update a subscription.
///
/// **Scopes**: `subscriptions:write`
pub struct SubscriptionsUpdate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2026_10::models::common::ids::SubscriptionId,
    pub(super) payload: crate::v2026_10::models::subscriptions::unions::SubscriptionUpdate,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> SubscriptionsUpdate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/subscriptions/{}", self.id);
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::PATCH, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> SubscriptionsUpdate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::subscriptions::types::Subscription,
        SubscriptionsUpdateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            SubscriptionsUpdateError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(SubscriptionsUpdateError::Transport)?;
        let result: eros::Result<_, SubscriptionsUpdateErrors> = async {
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
                        crate::v2026_10::models::subscriptions::types::Subscription,
                    >(&body_slice)
                        .map_err(|error| eros::ErrorUnion::new(
                            RequestFailure::DecodeResponse(error),
                        ))
                }
                http::StatusCode::PAYMENT_REQUIRED => {
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    enum ResponseError {
                        Payload0(crate::v2026_10::models::common::types::PaymentFailed),
                        Payload1(
                            crate::v2026_10::models::common::types::PaymentActionRequired,
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
                http::StatusCode::FORBIDDEN => {
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    enum ResponseError {
                        Payload0(
                            crate::v2026_10::models::subscriptions::types::AlreadyCanceledSubscription,
                        ),
                        Payload1(
                            crate::v2026_10::models::subscriptions::types::InactiveSubscription,
                        ),
                        Payload2(
                            crate::v2026_10::models::common::types::PaymentNotReady,
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
                        },
                    )
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::ResourceNotFound,
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
                            crate::v2026_10::models::subscriptions::types::SubscriptionLocked,
                        ),
                        Payload1(
                            crate::v2026_10::models::subscriptions::types::SubscriptionNotScheduledToCancel,
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
                        crate::v2026_10::models::common::types::HttpValidationError,
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
        crate::v2026_10::models::subscriptions::types::Subscription,
        SubscriptionsUpdateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            SubscriptionsUpdateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for SubscriptionsUpdate<'a, ClientService>
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
        crate::v2026_10::models::subscriptions::types::Subscription,
        SubscriptionsUpdateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
