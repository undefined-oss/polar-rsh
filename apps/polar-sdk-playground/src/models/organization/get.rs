use http_body_util::BodyExt;

use super::types::Organization;
use crate::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
    models::common::{
        OrganizationId,
        errors::{HttpValidationError, ResourceNotFound},
    },
};

/// Errors returned by the get organization operation.
#[derive(Debug, thiserror::Error)]
pub enum GetOrganizationError<TransportError> {
    #[error("HTTP transport failed: {0}")]
    Transport(#[source] TransportError),

    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),

    #[error("resource not found: {0}")]
    NotFound(#[source] ResourceNotFound),

    #[error("validation failed: {0}")]
    Validation(#[source] HttpValidationError),
}

pub(super) type GetOrganizationErrors = (RequestFailure, ResourceNotFound, HttpValidationError);

impl<T> From<eros::ErrorUnion<GetOrganizationErrors>> for GetOrganizationError<T> {
    fn from(error: eros::ErrorUnion<GetOrganizationErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
                RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
                RequestFailure::ReadBody(error) => Self::ReadBody(error),
                RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
                RequestFailure::UnexpectedStatus { status, body } => {
                    Self::UnexpectedStatus { status, body }
                }
                RequestFailure::Timeout(error) => Self::Timeout(error),
            },
            eros::E3::B(error) => Self::NotFound(error),
            eros::E3::C(error) => Self::Validation(error),
        }
    }
}
impl<T> GetOrganizationError<T> {
    pub(super) fn from_failure(error: RequestFailure) -> Self {
        let union: eros::ErrorUnion<GetOrganizationErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}

pub struct GetOrganization<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: OrganizationId,
    pub(super) options: RequestOptions,
}

impl<'a, ClientService> GetOrganization<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/organizations/{}", self.id);
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, new_options: RequestOptions) -> Self {
        self.options = new_options;
        self
    }
}

impl<'a, ClientService, Body> GetOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<Organization, GetOrganizationError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            GetOrganizationError::from_failure(RequestFailure::BuildRequest(error))
        })?;

        let response = self.client.send(request).await.map_err(GetOrganizationError::Transport)?;
        let result: eros::Result<_, GetOrganizationErrors> = async {
            let response_code = response.status();

            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();

            match response_code {
                http::StatusCode::OK => serde_json::from_slice(&body_slice)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::ResourceNotFound,
                    >(&body_slice)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;

                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::HttpValidationError,
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

    pub async fn send(self) -> Result<Organization, GetOrganizationError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());

        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| GetOrganizationError::from_failure(RequestFailure::Timeout(error)))?
    }
}

impl<'a, ClientService, Body> std::future::IntoFuture for GetOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<Organization, GetOrganizationError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

// Second operation
