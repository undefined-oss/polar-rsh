use http_body_util::BodyExt;

use crate::v2026_04::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone, Default)]
pub struct CustomersDeleteExternalParams {
    /// If true, also anonymize the customer's personal data for GDPR
    /// compliance.
    pub anonymize: Option<bool>,
}
#[allow(deprecated)]
impl crate::v2026_04::query::EncodeQuery for CustomersDeleteExternalParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        if let Some(value) = &self.anonymize {
            params.push(("anonymize", value.to_string()));
        }
        params
    }
}
#[derive(Debug, thiserror::Error)]
pub enum CustomersDeleteExternalError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),
    #[error("failed to build request: {0}")]
    BuildRequest(#[source] crate::v2026_04::error::BuildRequestError),
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
    /// Customer not found.
    #[error("{message}: {0}", message = "Customer not found.")]
    ResourceNotFound(#[source] crate::v2026_04::models::common::types::ResourceNotFound),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2026_04::models::common::types::HttpValidationError),
}
pub(super) type CustomersDeleteExternalErrors = (
    crate::v2026_04::error::RequestFailure,
    crate::v2026_04::models::common::types::ResourceNotFound,
    crate::v2026_04::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<CustomersDeleteExternalErrors>> for CustomersDeleteExternalError<T> {
    fn from(error: eros::ErrorUnion<CustomersDeleteExternalErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
                crate::v2026_04::error::RequestFailure::BuildRequest(error) => {
                    Self::BuildRequest(error)
                }
                crate::v2026_04::error::RequestFailure::ReadBody(error) => Self::ReadBody(error),
                crate::v2026_04::error::RequestFailure::DecodeResponse(error) => {
                    Self::DecodeResponse(error)
                }
                crate::v2026_04::error::RequestFailure::HttpStatus { status, body } => {
                    Self::HttpStatus { status, body }
                }
                crate::v2026_04::error::RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                crate::v2026_04::error::RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E3::B(error) => Self::ResourceNotFound(error),
            eros::E3::C(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> CustomersDeleteExternalError<T> {
    pub(super) fn from_failure(error: crate::v2026_04::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<CustomersDeleteExternalErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Delete a customer by external ID.
///
/// Immediately cancels any active subscriptions and revokes any active
/// benefits.
///
/// Set `anonymize=true` to also anonymize PII for GDPR compliance.
///
/// **Scopes**: `customers:write`
pub struct CustomersDeleteExternal<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) external_id: String,
    pub(super) params: Option<CustomersDeleteExternalParams>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> CustomersDeleteExternal<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let encoded_external_id = crate::v2026_04::path::encode_segment(&self.external_id);
        let query_params = self
            .params
            .as_ref()
            .map_or_else(String::new, crate::v2026_04::query::EncodeQuery::encode_query);
        let mut path = format!("v1/customers/external/{}", encoded_external_id);
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::DELETE, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: CustomersDeleteExternalParams) -> Self {
        self.params = Some(params);
        self
    }
}
impl<'a, ClientService, Body> CustomersDeleteExternal<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<(), CustomersDeleteExternalError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            CustomersDeleteExternalError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(CustomersDeleteExternalError::Transport)?;
        let result: eros::Result<_, CustomersDeleteExternalErrors> = async {
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
                        crate::v2026_04::models::common::types::ResourceNotFound,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::v2026_04::models::common::types::HttpValidationError,
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

    pub async fn send(self) -> Result<(), CustomersDeleteExternalError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CustomersDeleteExternalError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for CustomersDeleteExternal<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<(), CustomersDeleteExternalError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
