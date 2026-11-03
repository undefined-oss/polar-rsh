use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone, Default)]
pub struct RefundsListParams {
    /// Filter by customer ID.
    pub customer_id: Vec<crate::v2026_10::models::common::ids::CustomerId>,
    /// Filter by customer external ID.
    pub external_customer_id: Vec<String>,
    /// Filter by refund ID.
    pub id: Vec<crate::v2026_10::models::common::ids::RefundId>,
    /// Filter by order ID.
    pub order_id: Vec<crate::v2026_10::models::common::ids::OrderId>,
    /// Filter by organization ID.
    pub organization_id: Vec<crate::v2026_10::models::common::ids::OrganizationId>,
    /// Filter by subscription ID.
    pub subscription_id: Vec<crate::v2026_10::models::common::ids::SubscriptionId>,
    /// Filter by `succeeded`.
    pub succeeded: Option<bool>,
    /// Page number, defaults to 1.
    pub page: Option<std::num::NonZeroU64>,
    /// Size of a page, defaults to 10. Maximum is 100.
    pub limit: Option<crate::v2026_10::models::pagination::PageLimit>,
    /// Sorting criterion. Several criteria can be used simultaneously and will
    /// be applied in order. Add a minus sign `-` before the criteria name to
    /// sort by descending order.
    pub sorting: Vec<(
        crate::v2026_10::models::refunds::enums::RefundSortProperty,
        crate::v2026_10::query::Sorting,
    )>,
}
#[allow(deprecated)]
impl crate::v2026_10::query::EncodeQuery for RefundsListParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        for value in &self.customer_id {
            params.push(("customer_id", value.to_string()));
        }
        for value in &self.external_customer_id {
            params.push(("external_customer_id", value.to_string()));
        }
        for value in &self.id {
            params.push(("id", value.to_string()));
        }
        for value in &self.order_id {
            params.push(("order_id", value.to_string()));
        }
        for value in &self.organization_id {
            params.push(("organization_id", value.to_string()));
        }
        for value in &self.subscription_id {
            params.push(("subscription_id", value.to_string()));
        }
        if let Some(value) = &self.succeeded {
            params.push(("succeeded", value.to_string()));
        }
        if let Some(value) = &self.page {
            params.push(("page", value.to_string()));
        }
        if let Some(value) = &self.limit {
            params.push(("limit", value.to_string()));
        }
        params
    }

    fn sorting_values(&self) -> Vec<String> {
        self.sorting.iter().map(|(field, direction)| direction.encode(field.as_str())).collect()
    }
}
#[derive(Debug, thiserror::Error)]
pub enum RefundsListError<TransportError> {
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
pub(super) type RefundsListErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<RefundsListErrors>> for RefundsListError<T> {
    fn from(error: eros::ErrorUnion<RefundsListErrors>) -> Self {
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
impl<T> RefundsListError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<RefundsListErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// List refunds.
///
/// **Scopes**: `refunds:read` `refunds:write`
pub struct RefundsList<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: Option<RefundsListParams>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> RefundsList<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = self
            .params
            .as_ref()
            .map_or_else(String::new, crate::v2026_10::query::EncodeQuery::encode_query);
        let mut path = "v1/refunds/".to_owned();
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: RefundsListParams) -> Self {
        self.params = Some(params);
        self
    }
}
impl<'a, ClientService, Body> RefundsList<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::pagination::ListResource<
            crate::v2026_10::models::refunds::types::Refund,
        >,
        RefundsListError<ClientService::Error>,
    > {
        let request = self
            .build_request()
            .map_err(|error| RefundsListError::from_failure(RequestFailure::BuildRequest(error)))?;
        let response = self.client.send(request).await.map_err(RefundsListError::Transport)?;
        let result: eros::Result<_, RefundsListErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2026_10::models::pagination::ListResource<
                        crate::v2026_10::models::refunds::types::Refund,
                    >,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
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
    ) -> Result<
        crate::v2026_10::models::pagination::ListResource<
            crate::v2026_10::models::refunds::types::Refund,
        >,
        RefundsListError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| RefundsListError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for RefundsList<'a, ClientService>
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
        crate::v2026_10::models::pagination::ListResource<
            crate::v2026_10::models::refunds::types::Refund,
        >,
        RefundsListError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
impl<'a, ClientService, Body> RefundsList<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    #[allow(clippy::clone_on_copy)]
    pub fn pages(
        self,
    ) -> impl futures_util::Stream<
        Item = Result<
            crate::v2026_10::models::pagination::ListResource<
                crate::v2026_10::models::refunds::types::Refund,
            >,
            RefundsListError<ClientService::Error>,
        >,
    > + 'a {
        futures_util::stream::try_unfold(Some(self), |state| async move {
            let Some(mut request) = state else {
                return Ok(None);
            };
            let mut params = request.params.take().unwrap_or_default();
            let current = params.page.unwrap_or(std::num::NonZeroU64::MIN);
            params.page = Some(current);
            let client = request.client;
            let options = request.options.clone();
            let response = request.params(params.clone()).send().await?;
            let next = response.pagination.next_page(current).map(|page| {
                params.page = Some(page);
                Self { client, options, params: Some(params) }
            });
            Ok(Some((response, next)))
        })
    }

    pub fn items(
        self,
    ) -> impl futures_util::Stream<
        Item = Result<
            crate::v2026_10::models::refunds::types::Refund,
            RefundsListError<ClientService::Error>,
        >,
    > + 'a {
        use futures_util::TryStreamExt;
        self.pages()
            .map_ok(|page| futures_util::stream::iter(page.items.into_iter().map(Ok)))
            .try_flatten()
    }
}
