use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, thiserror::Error)]
pub enum CustomersMembersDeleteExternalError<TransportError> {
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
    /// Member not found.
    #[error("{message}: {0}", message = "Member not found.")]
    ResourceNotFound(#[source] crate::v2026_10::models::common::types::ResourceNotFound),
    /// The external customer ID matches customers in several accessible
    /// organizations.
    #[error(
        "{message}: {0}",
        message = "The external customer ID matches customers in several accessible organizations."
    )]
    AmbiguousExternalCustomerId(
        #[source] crate::v2026_10::models::common::types::AmbiguousExternalCustomerId,
    ),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type CustomersMembersDeleteExternalErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::ResourceNotFound,
    crate::v2026_10::models::common::types::AmbiguousExternalCustomerId,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomersMembersDeleteExternalErrors>>
    for CustomersMembersDeleteExternalError<T>
{
    fn from(error: eros::ErrorUnion<CustomersMembersDeleteExternalErrors>) -> Self {
        match error.into_enum() {
            eros::E4::A(error) => match error {
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
            eros::E4::B(error) => Self::ResourceNotFound(error),
            eros::E4::C(error) => Self::AmbiguousExternalCustomerId(error),
            eros::E4::D(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomersMembersDeleteExternalError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomersMembersDeleteExternalErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Delete a member by external ID for a customer identified by its external ID.
///
/// **Scopes**: `members:write`
pub struct CustomersMembersDeleteExternal<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) external_id: String,
    pub(super) member_external_id: String,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomersMembersDeleteExternal<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let encoded_external_id = crate::v2026_10::path::encode_segment(&self.external_id);
        let encoded_member_external_id =
            crate::v2026_10::path::encode_segment(&self.member_external_id);
        let path = format!(
            "v1/customers/external/{}/members/{}",
            encoded_external_id, encoded_member_external_id
        );
        self.client.build_request(http::Method::DELETE, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}
impl<'a, ClientService, Body> CustomersMembersDeleteExternal<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<(), CustomersMembersDeleteExternalError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            CustomersMembersDeleteExternalError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self
            .client
            .send(request)
            .await
            .map_err(CustomersMembersDeleteExternalError::Transport)?;
        let result: eros::Result<_, CustomersMembersDeleteExternalErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::NO_CONTENT => Ok(()),
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::ResourceNotFound,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::AmbiguousExternalCustomerId,
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
    ) -> Result<(), CustomersMembersDeleteExternalError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomersMembersDeleteExternalError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomersMembersDeleteExternal<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<(), CustomersMembersDeleteExternalError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
