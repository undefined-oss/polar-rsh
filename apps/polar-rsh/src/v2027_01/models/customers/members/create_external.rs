use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomersMembersCreateExternalError<TransportError> {
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
    /// Not permitted to add members.
    #[error("{message}: {0}", message = "Not permitted to add members.")]
    NotPermitted(#[source] crate::v2027_01::models::common::types::NotPermitted),
    /// Customer not found.
    #[error("{message}: {0}", message = "Customer not found.")]
    ResourceNotFound(#[source] crate::v2027_01::models::common::types::ResourceNotFound),
    /// The external customer ID matches customers in several accessible
    /// organizations.
    #[error(
        "{message}: {0}",
        message = "The external customer ID matches customers in several accessible organizations."
    )]
    AmbiguousExternalCustomerId(
        #[source] crate::v2027_01::models::common::types::AmbiguousExternalCustomerId,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type CustomersMembersCreateExternalErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::NotPermitted,
    crate::v2027_01::models::common::types::ResourceNotFound,
    crate::v2027_01::models::common::types::AmbiguousExternalCustomerId,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomersMembersCreateExternalErrors>>
    for CustomersMembersCreateExternalError<T>
{
    fn from(error: eros::ErrorUnion<CustomersMembersCreateExternalErrors>) -> Self {
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
            eros::E5::D(error) => Self::AmbiguousExternalCustomerId(error),
            eros::E5::E(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomersMembersCreateExternalError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomersMembersCreateExternalErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Create a new member for a customer identified by its external ID.
///
/// **Scopes**: `members:write`
pub struct CustomersMembersCreateExternal<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) external_id: String,
    pub(super) payload: crate::v2027_01::models::customers::types::MemberCreateFromCustomer,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomersMembersCreateExternal<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let encoded_external_id = crate::v2027_01::path::encode_segment(&self.external_id);
        let path = format!("v1/customers/external/{}/members", encoded_external_id);
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::POST, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomersMembersCreateExternal<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::common::types::Member,
        CustomersMembersCreateExternalError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            CustomersMembersCreateExternalError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self
            .client
            .send(request)
            .await
            .map_err(CustomersMembersCreateExternalError::Transport)?;
        let result: eros::Result<_, CustomersMembersCreateExternalErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::CREATED => serde_json::from_slice::<
                    crate::v2027_01::models::common::types::Member,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::NotPermitted,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
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
                        crate::v2027_01::models::common::types::AmbiguousExternalCustomerId,
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
        crate::v2027_01::models::common::types::Member,
        CustomersMembersCreateExternalError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomersMembersCreateExternalError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomersMembersCreateExternal<'a, ClientService>
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
        crate::v2027_01::models::common::types::Member,
        CustomersMembersCreateExternalError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
