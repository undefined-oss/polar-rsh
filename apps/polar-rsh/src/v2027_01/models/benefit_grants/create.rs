use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum BenefitGrantsCreateError<TransportError> {
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
    /// Conflict
    #[error("{message}: {0}", message = "Conflict")]
    BenefitGrantAlreadyExists(
        #[source] crate::v2027_01::models::benefit_grants::types::BenefitGrantAlreadyExists,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type BenefitGrantsCreateErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::benefit_grants::types::BenefitGrantAlreadyExists,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<BenefitGrantsCreateErrors>> for BenefitGrantsCreateError<T> {
    fn from(error: eros::ErrorUnion<BenefitGrantsCreateErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
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
            eros::E3::B(error) => Self::BenefitGrantAlreadyExists(error),
            eros::E3::C(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> BenefitGrantsCreateError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<BenefitGrantsCreateErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Manually grant a benefit to a customer or one of their members.
///
/// Use either Polar IDs for every reference or external IDs for every
/// reference. Fulfillment is asynchronous: the returned grant may not yet be
/// granted. Meter-credit benefits cannot be granted manually.
///
/// **Scopes**: `benefits:write`
pub struct BenefitGrantsCreate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) payload:
        crate::v2027_01::models::benefit_grants::unions::BenefitGrantsCreateRequestUnion,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> BenefitGrantsCreate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = "v1/benefit-grants/".to_owned();
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::POST, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> BenefitGrantsCreate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::common::types::BenefitGrant,
        BenefitGrantsCreateError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            BenefitGrantsCreateError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(BenefitGrantsCreateError::Transport)?;
        let result: eros::Result<_, BenefitGrantsCreateErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::CREATED => serde_json::from_slice::<
                    crate::v2027_01::models::common::types::BenefitGrant,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::benefit_grants::types::BenefitGrantAlreadyExists,
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
        crate::v2027_01::models::common::types::BenefitGrant,
        BenefitGrantsCreateError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            BenefitGrantsCreateError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for BenefitGrantsCreate<'a, ClientService>
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
        crate::v2027_01::models::common::types::BenefitGrant,
        BenefitGrantsCreateError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
