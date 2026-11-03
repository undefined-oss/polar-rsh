use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone)]
pub struct CustomerPortalCustomersCheckEmailUpdateParams {
    pub token: String,
}
#[derive(Debug, Clone)]
pub struct CustomerPortalCustomersCheckEmailUpdateParamsRequired {
    pub token: String,
}
impl CustomerPortalCustomersCheckEmailUpdateParams {
    pub fn new(required: CustomerPortalCustomersCheckEmailUpdateParamsRequired) -> Self {
        Self { token: required.token }
    }
}
#[allow(deprecated)]
impl crate::v2026_10::query::EncodeQuery for CustomerPortalCustomersCheckEmailUpdateParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        params.push(("token", self.token.to_string()));
        params
    }
}
#[derive(Debug, thiserror::Error)]
pub enum CustomerPortalCustomersCheckEmailUpdateError<TransportError> {
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
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_10::models::common::types::HttpValidationError),
}
pub(super) type CustomerPortalCustomersCheckEmailUpdateErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomerPortalCustomersCheckEmailUpdateErrors>>
    for CustomerPortalCustomersCheckEmailUpdateError<T>
{
    fn from(error: eros::ErrorUnion<CustomerPortalCustomersCheckEmailUpdateErrors>) -> Self {
        match error.into_enum() {
            eros::E2::A(error) => match error {
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
            eros::E2::B(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomerPortalCustomersCheckEmailUpdateError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomerPortalCustomersCheckEmailUpdateErrors> =
            eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Check if an email change verification token is still valid.
pub struct CustomerPortalCustomersCheckEmailUpdate<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: CustomerPortalCustomersCheckEmailUpdateParams,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomerPortalCustomersCheckEmailUpdate<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = crate::v2026_10::query::EncodeQuery::encode_query(&self.params);
        let mut path = "v1/customer-portal/customers/me/email-update/check".to_owned();
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: CustomerPortalCustomersCheckEmailUpdateParams) -> Self {
        self.params = params;
        self
    }
}
impl<'a, ClientService, Body> CustomerPortalCustomersCheckEmailUpdate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<(), CustomerPortalCustomersCheckEmailUpdateError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            CustomerPortalCustomersCheckEmailUpdateError::from_failure(
                RequestFailure::BuildRequest(error),
            )
        })?;
        let response = self
            .client
            .send(request)
            .await
            .map_err(CustomerPortalCustomersCheckEmailUpdateError::Transport)?;
        let result: eros::Result<_, CustomerPortalCustomersCheckEmailUpdateErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::NO_CONTENT => Ok(()),
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2026_10::models::common::types::HttpValidationError,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNAUTHORIZED => {
                    Err(eros::ErrorUnion::new(RequestFailure::HttpStatus {
                        status: response_code,
                        body: body_slice,
                    }))
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
    ) -> Result<(), CustomerPortalCustomersCheckEmailUpdateError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomerPortalCustomersCheckEmailUpdateError::from_failure(RequestFailure::Timeout(
                error,
            ))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture
    for CustomerPortalCustomersCheckEmailUpdate<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<(), CustomerPortalCustomersCheckEmailUpdateError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
