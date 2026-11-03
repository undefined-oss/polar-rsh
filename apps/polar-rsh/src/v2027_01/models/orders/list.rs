use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone, Default)]
pub struct OrdersListParams {
    /// Filter by checkout ID.
    pub checkout_id: Vec<crate::v2027_01::models::common::ids::CheckoutId>,
    /// Only include orders created after this date
    pub created_after: Option<jiff::Timestamp>,
    /// Only include orders created before this date
    pub created_before: Option<jiff::Timestamp>,
    /// Filter by customer ID.
    pub customer_id: Vec<crate::v2027_01::models::common::ids::CustomerId>,
    /// Filter by discount ID.
    pub discount_id: Vec<crate::v2027_01::models::common::ids::DiscountId>,
    /// Filter by customer external ID.
    pub external_customer_id: Vec<String>,
    /// Filter by metadata key-value pairs. It uses the `deepObject` style, e.g.
    /// `?metadata[key]=value`.
    pub metadata: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion,
    >,
    /// Filter by organization ID.
    pub organization_id: Vec<crate::v2027_01::models::common::ids::OrganizationId>,
    /// Filter by product billing type. `recurring` will filter data
    /// corresponding to subscriptions creations or renewals. `one_time` will
    /// filter data corresponding to one-time purchases.
    pub product_billing_type: Vec<crate::v2027_01::models::common::enums::ProductBillingType>,
    /// Filter by product ID.
    pub product_id: Vec<crate::v2027_01::models::common::ids::ProductId>,
    /// Filter by order status.
    pub status: Vec<crate::v2027_01::models::common::enums::OrderStatus>,
    /// Filter by subscription ID.
    pub subscription_id: Vec<crate::v2027_01::models::common::ids::SubscriptionId>,
    /// Page number, defaults to 1.
    pub page: Option<std::num::NonZeroU64>,
    /// Size of a page, defaults to 10. Maximum is 100.
    pub limit: Option<crate::v2027_01::models::pagination::PageLimit>,
    /// Sorting criterion. Several criteria can be used simultaneously and will
    /// be applied in order. Add a minus sign `-` before the criteria name to
    /// sort by descending order.
    pub sorting: Vec<(
        crate::v2027_01::models::orders::enums::OrderSortProperty,
        crate::v2027_01::query::Sorting,
    )>,
}
#[allow(deprecated)]
impl crate::v2027_01::query::EncodeQuery for OrdersListParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        for value in &self.checkout_id {
            params.push(("checkout_id", value.to_string()));
        }
        if let Some(value) = &self.created_after {
            params.push(("created_after", value.to_string()));
        }
        if let Some(value) = &self.created_before {
            params.push(("created_before", value.to_string()));
        }
        for value in &self.customer_id {
            params.push(("customer_id", value.to_string()));
        }
        for value in &self.discount_id {
            params.push(("discount_id", value.to_string()));
        }
        for value in &self.external_customer_id {
            params.push(("external_customer_id", value.to_string()));
        }
        for value in &self.organization_id {
            params.push(("organization_id", value.to_string()));
        }
        for value in &self.product_billing_type {
            params.push(("product_billing_type", value.to_string()));
        }
        for value in &self.product_id {
            params.push(("product_id", value.to_string()));
        }
        for value in &self.status {
            params.push(("status", value.to_string()));
        }
        for value in &self.subscription_id {
            params.push(("subscription_id", value.to_string()));
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

    fn metadata_params(&self) -> Vec<(String, Vec<String>)> {
        self.metadata
            .iter()
            .map(|(key, value)| {
                let values = match value {
                    crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::String(
                        values,
                    ) => vec![values.to_string()],
                    crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::Integer(
                        values,
                    ) => vec![values.to_string()],
                    crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::Boolean(
                        values,
                    ) => vec![values.to_string()],
                    crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::StringArray(
                        values,
                    ) => values.iter().map(ToString::to_string).collect(),
                    crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::IntegerArray(
                        values,
                    ) => values.iter().map(ToString::to_string).collect(),
                    crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::BooleanArray(
                        values,
                    ) => values.iter().map(ToString::to_string).collect(),
                };
                (key.clone(), values)
            })
            .collect()
    }
}
#[derive(Debug, thiserror::Error)]
pub enum OrdersListError<TransportError> {
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
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type OrdersListErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<OrdersListErrors>> for OrdersListError<T> {
    fn from(error: eros::ErrorUnion<OrdersListErrors>) -> Self {
        match error.into_enum() {
            eros::E2::A(error) => match error {
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
            eros::E2::B(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> OrdersListError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<OrdersListErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// List orders.
///
/// **Scopes**: `orders:read`
pub struct OrdersList<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: Option<OrdersListParams>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> OrdersList<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = self
            .params
            .as_ref()
            .map_or_else(String::new, crate::v2027_01::query::EncodeQuery::encode_query);
        let mut path = "v1/orders/".to_owned();
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: OrdersListParams) -> Self {
        self.params = Some(params);
        self
    }
}
impl<'a, ClientService, Body> OrdersList<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::pagination::ListResource<
            crate::v2027_01::models::orders::types::Order,
        >,
        OrdersListError<ClientService::Error>,
    > {
        let request = self
            .build_request()
            .map_err(|error| OrdersListError::from_failure(RequestFailure::BuildRequest(error)))?;
        let response = self.client.send(request).await.map_err(OrdersListError::Transport)?;
        let result: eros::Result<_, OrdersListErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2027_01::models::pagination::ListResource<
                        crate::v2027_01::models::orders::types::Order,
                    >,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
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
        crate::v2027_01::models::pagination::ListResource<
            crate::v2027_01::models::orders::types::Order,
        >,
        OrdersListError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| OrdersListError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for OrdersList<'a, ClientService>
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
        crate::v2027_01::models::pagination::ListResource<
            crate::v2027_01::models::orders::types::Order,
        >,
        OrdersListError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
impl<'a, ClientService, Body> OrdersList<'a, ClientService>
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
            crate::v2027_01::models::pagination::ListResource<
                crate::v2027_01::models::orders::types::Order,
            >,
            OrdersListError<ClientService::Error>,
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
            crate::v2027_01::models::orders::types::Order,
            OrdersListError<ClientService::Error>,
        >,
    > + 'a {
        use futures_util::TryStreamExt;
        self.pages()
            .map_ok(|page| futures_util::stream::iter(page.items.into_iter().map(Ok)))
            .try_flatten()
    }
}
