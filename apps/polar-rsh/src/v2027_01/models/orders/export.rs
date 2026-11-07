use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone, Default)]
pub struct OrdersExportParams {
    /// Columns to include in the CSV, in order. Defaults to email, created_at,
    /// product, net_amount, currency, status and invoice_number.
    pub columns: Vec<crate::v2027_01::models::orders::enums::OrderExportColumn>,
    /// Only include orders created after this date. Must include a UTC offset.
    pub created_after: Option<jiff::Timestamp>,
    /// Only include orders created before this date. Must include a UTC offset.
    pub created_before: Option<jiff::Timestamp>,
    /// Filter by product external ID.
    pub external_product_id: Vec<String>,
    /// Filter by organization ID.
    pub organization_id: Vec<crate::v2027_01::models::common::ids::OrganizationId>,
    /// Filter by product ID.
    pub product_id: Vec<crate::v2027_01::models::common::ids::ProductId>,
    /// Filter by order status.
    pub status: Vec<crate::v2027_01::models::common::enums::OrderStatus>,
    /// Time zone used to render dates in the CSV.
    pub timezone: Option<String>,
}
#[allow(deprecated)]
impl crate::v2027_01::query::EncodeQuery for OrdersExportParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        for value in &self.columns {
            params.push(("columns", value.to_string()));
        }
        if let Some(value) = &self.created_after {
            params.push(("created_after", value.to_string()));
        }
        if let Some(value) = &self.created_before {
            params.push(("created_before", value.to_string()));
        }
        for value in &self.external_product_id {
            params.push(("external_product_id", value.to_string()));
        }
        for value in &self.organization_id {
            params.push(("organization_id", value.to_string()));
        }
        for value in &self.product_id {
            params.push(("product_id", value.to_string()));
        }
        for value in &self.status {
            params.push(("status", value.to_string()));
        }
        if let Some(value) = &self.timezone {
            params.push(("timezone", value.to_string()));
        }
        params
    }
}
#[derive(Debug, thiserror::Error)]
pub enum OrdersExportError<TransportError> {
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
pub(super) type OrdersExportErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<OrdersExportErrors>> for OrdersExportError<T> {
    fn from(error: eros::ErrorUnion<OrdersExportErrors>) -> Self {
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
impl<T> OrdersExportError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<OrdersExportErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Export orders as a CSV file.
///
/// **Scopes**: `orders:read`
pub struct OrdersExport<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: Option<OrdersExportParams>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> OrdersExport<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = self
            .params
            .as_ref()
            .map_or_else(String::new, crate::v2027_01::query::EncodeQuery::encode_query);
        let mut path = "v1/orders/export".to_owned();
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        let mut request =
            self.client.build_request(http::Method::GET, &path, None, &self.options)?;
        request
            .headers_mut()
            .insert(http::header::ACCEPT, http::HeaderValue::from_static("text/csv"));
        Ok(request)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: OrdersExportParams) -> Self {
        self.params = Some(params);
        self
    }
}
impl<'a, ClientService, Body> OrdersExport<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<bytes::Bytes, OrdersExportError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            OrdersExportError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self.client.send(request).await.map_err(OrdersExportError::Transport)?;
        let result: eros::Result<_, OrdersExportErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => Ok(body_slice),
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

    pub async fn send(self) -> Result<bytes::Bytes, OrdersExportError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| OrdersExportError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for OrdersExport<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<bytes::Bytes, OrdersExportError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
