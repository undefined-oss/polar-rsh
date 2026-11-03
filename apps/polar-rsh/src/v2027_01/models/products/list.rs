use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone, Default)]
pub struct ProductsListParams {
    /// Filter products granting specific benefit.
    pub benefit_id: Vec<crate::v2027_01::models::common::ids::BenefitId>,
    /// Filter by product external ID.
    pub external_id: Vec<String>,
    /// Filter by product ID.
    pub id: Vec<crate::v2027_01::models::common::ids::ProductId>,
    /// Filter on archived products.
    pub is_archived: Option<bool>,
    /// Filter on recurring products. If `true`, only subscriptions tiers are
    /// returned. If `false`, only one-time purchase products are returned.
    pub is_recurring: Option<bool>,
    /// Filter by metadata key-value pairs. It uses the `deepObject` style, e.g.
    /// `?metadata[key]=value`.
    pub metadata: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion,
    >,
    /// Filter by organization ID.
    pub organization_id: Vec<crate::v2027_01::models::common::ids::OrganizationId>,
    /// Filter by product name.
    pub query: Option<String>,
    /// Filter by visibility.
    pub visibility: Vec<crate::v2027_01::models::common::enums::ProductVisibility>,
    /// Page number, defaults to 1.
    pub page: Option<std::num::NonZeroU64>,
    /// Size of a page, defaults to 10. Maximum is 100.
    pub limit: Option<crate::v2027_01::models::pagination::PageLimit>,
    /// Sorting criterion. Several criteria can be used simultaneously and will
    /// be applied in order. Add a minus sign `-` before the criteria name to
    /// sort by descending order.
    pub sorting: Vec<(
        crate::v2027_01::models::products::enums::ProductSortProperty,
        crate::v2027_01::query::Sorting,
    )>,
}
#[allow(deprecated)]
impl crate::v2027_01::query::EncodeQuery for ProductsListParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        for value in &self.benefit_id {
            params.push(("benefit_id", value.to_string()));
        }
        for value in &self.external_id {
            params.push(("external_id", value.to_string()));
        }
        for value in &self.id {
            params.push(("id", value.to_string()));
        }
        if let Some(value) = &self.is_archived {
            params.push(("is_archived", value.to_string()));
        }
        if let Some(value) = &self.is_recurring {
            params.push(("is_recurring", value.to_string()));
        }
        for value in &self.organization_id {
            params.push(("organization_id", value.to_string()));
        }
        if let Some(value) = &self.query {
            params.push(("query", value.to_string()));
        }
        for value in &self.visibility {
            params.push(("visibility", value.to_string()));
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
pub enum ProductsListError<TransportError> {
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
pub(super) type ProductsListErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<ProductsListErrors>> for ProductsListError<T> {
    fn from(error: eros::ErrorUnion<ProductsListErrors>) -> Self {
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
impl<T> ProductsListError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<ProductsListErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// List products.
///
/// **Scopes**: `products:read` `products:write`
pub struct ProductsList<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: Option<ProductsListParams>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> ProductsList<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = self
            .params
            .as_ref()
            .map_or_else(String::new, crate::v2027_01::query::EncodeQuery::encode_query);
        let mut path = "v1/products/".to_owned();
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: ProductsListParams) -> Self {
        self.params = Some(params);
        self
    }
}
impl<'a, ClientService, Body> ProductsList<'a, ClientService>
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
            crate::v2027_01::models::common::types::Product,
        >,
        ProductsListError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            ProductsListError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self.client.send(request).await.map_err(ProductsListError::Transport)?;
        let result: eros::Result<_, ProductsListErrors> = async {
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
                        crate::v2027_01::models::common::types::Product,
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
            crate::v2027_01::models::common::types::Product,
        >,
        ProductsListError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| ProductsListError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for ProductsList<'a, ClientService>
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
            crate::v2027_01::models::common::types::Product,
        >,
        ProductsListError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
impl<'a, ClientService, Body> ProductsList<'a, ClientService>
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
                crate::v2027_01::models::common::types::Product,
            >,
            ProductsListError<ClientService::Error>,
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
            crate::v2027_01::models::common::types::Product,
            ProductsListError<ClientService::Error>,
        >,
    > + 'a {
        use futures_util::TryStreamExt;
        self.pages()
            .map_ok(|page| futures_util::stream::iter(page.items.into_iter().map(Ok)))
            .try_flatten()
    }
}
