use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone)]
pub struct MetricsExportParams {
    /// Filter by billing type. `recurring` will filter data corresponding to
    /// subscriptions creations or renewals. `one_time` will filter data
    /// corresponding to one-time purchases.
    pub billing_type: Vec<crate::v2026_10::models::common::enums::ProductBillingType>,
    /// Filter by customer ID.
    pub customer_id: Vec<crate::v2026_10::models::common::ids::CustomerId>,
    /// End date.
    pub end_date: jiff::civil::Date,
    /// Interval between two timestamps.
    pub interval: crate::v2026_10::models::common::enums::TimeInterval,
    /// List of metric slugs to include in the export. If not provided, all
    /// metrics are exported.
    pub metrics: Vec<String>,
    /// Filter by organization ID.
    pub organization_id: Vec<crate::v2026_10::models::common::ids::OrganizationId>,
    /// Filter by product ID.
    pub product_id: Vec<crate::v2026_10::models::common::ids::ProductId>,
    /// Start date.
    pub start_date: jiff::civil::Date,
    /// Timezone to use for the timestamps. Default is UTC.
    pub timezone: Option<crate::v2026_10::models::metrics::enums::MetricsExportTimezone>,
}
#[derive(Debug, Clone)]
pub struct MetricsExportParamsRequired {
    /// End date.
    pub end_date: jiff::civil::Date,
    /// Interval between two timestamps.
    pub interval: crate::v2026_10::models::common::enums::TimeInterval,
    /// Start date.
    pub start_date: jiff::civil::Date,
}
impl MetricsExportParams {
    pub fn new(required: MetricsExportParamsRequired) -> Self {
        Self {
            billing_type: Default::default(),
            customer_id: Default::default(),
            end_date: required.end_date,
            interval: required.interval,
            metrics: Default::default(),
            organization_id: Default::default(),
            product_id: Default::default(),
            start_date: required.start_date,
            timezone: Default::default(),
        }
    }
}
#[allow(deprecated)]
impl crate::v2026_10::query::EncodeQuery for MetricsExportParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        for value in &self.billing_type {
            params.push(("billing_type", value.to_string()));
        }
        for value in &self.customer_id {
            params.push(("customer_id", value.to_string()));
        }
        params.push(("end_date", self.end_date.to_string()));
        params.push(("interval", self.interval.to_string()));
        for value in &self.metrics {
            params.push(("metrics", value.to_string()));
        }
        for value in &self.organization_id {
            params.push(("organization_id", value.to_string()));
        }
        for value in &self.product_id {
            params.push(("product_id", value.to_string()));
        }
        params.push(("start_date", self.start_date.to_string()));
        if let Some(value) = &self.timezone {
            params.push(("timezone", value.to_string()));
        }
        params
    }
}
#[derive(Debug, thiserror::Error)]
pub enum MetricsExportError<TransportError> {
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
pub(super) type MetricsExportErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<MetricsExportErrors>> for MetricsExportError<T> {
    fn from(error: eros::ErrorUnion<MetricsExportErrors>) -> Self {
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
impl<T> MetricsExportError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<MetricsExportErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Export metrics as a CSV file.
///
/// **Scopes**: `metrics:read`
pub struct MetricsExport<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: MetricsExportParams,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> MetricsExport<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = crate::v2026_10::query::EncodeQuery::encode_query(&self.params);
        let mut path = "v1/metrics/export".to_owned();
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

    pub fn params(mut self, params: MetricsExportParams) -> Self {
        self.params = params;
        self
    }
}
impl<'a, ClientService, Body> MetricsExport<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<bytes::Bytes, MetricsExportError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            MetricsExportError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self.client.send(request).await.map_err(MetricsExportError::Transport)?;
        let result: eros::Result<_, MetricsExportErrors> = async {
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

    pub async fn send(self) -> Result<bytes::Bytes, MetricsExportError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| MetricsExportError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for MetricsExport<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<bytes::Bytes, MetricsExportError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
