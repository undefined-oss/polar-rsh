use http_body_util::BodyExt;

use crate::v2027_01::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone)]
pub struct MetersQuantitiesParams {
    /// If set, will first compute the quantities per customer before
    /// aggregating them using the given function. If not set, the quantities
    /// will be aggregated across all events.
    pub customer_aggregation_function:
        Option<crate::v2027_01::models::meters::enums::AggregationFunction>,
    /// Filter by customer ID.
    pub customer_id: Vec<crate::v2027_01::models::common::ids::CustomerId>,
    /// End timestamp.
    pub end_timestamp: jiff::Timestamp,
    /// Filter by external customer ID.
    pub external_customer_id: Vec<String>,
    /// Interval between two timestamps.
    pub interval: crate::v2027_01::models::common::enums::TimeInterval,
    /// Filter by metadata key-value pairs. It uses the `deepObject` style, e.g.
    /// `?metadata[key]=value`.
    pub metadata: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion,
    >,
    /// Start timestamp.
    pub start_timestamp: jiff::Timestamp,
    /// Timezone to use for the timestamps. Default is UTC.
    pub timezone: Option<crate::v2027_01::models::meters::enums::MetersQuantitiesTimezone>,
}
#[derive(Debug, Clone)]
pub struct MetersQuantitiesParamsRequired {
    /// End timestamp.
    pub end_timestamp: jiff::Timestamp,
    /// Interval between two timestamps.
    pub interval: crate::v2027_01::models::common::enums::TimeInterval,
    /// Start timestamp.
    pub start_timestamp: jiff::Timestamp,
}
impl MetersQuantitiesParams {
    pub fn new(required: MetersQuantitiesParamsRequired) -> Self {
        Self {
            customer_aggregation_function: Default::default(),
            customer_id: Default::default(),
            end_timestamp: required.end_timestamp,
            external_customer_id: Default::default(),
            interval: required.interval,
            metadata: Default::default(),
            start_timestamp: required.start_timestamp,
            timezone: Default::default(),
        }
    }
}
#[allow(deprecated)]
impl crate::v2027_01::query::EncodeQuery for MetersQuantitiesParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        if let Some(value) = &self.customer_aggregation_function {
            params.push(("customer_aggregation_function", value.to_string()));
        }
        for value in &self.customer_id {
            params.push(("customer_id", value.to_string()));
        }
        params.push(("end_timestamp", self.end_timestamp.to_string()));
        for value in &self.external_customer_id {
            params.push(("external_customer_id", value.to_string()));
        }
        params.push(("interval", self.interval.to_string()));
        params.push(("start_timestamp", self.start_timestamp.to_string()));
        if let Some(value) = &self.timezone {
            params.push(("timezone", value.to_string()));
        }
        params
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
pub enum MetersQuantitiesError<TransportError> {
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
    /// Meter not found.
    #[error("{message}: {0}", message = "Meter not found.")]
    ResourceNotFound(#[source] crate::v2027_01::models::common::types::ResourceNotFound),
    /// Validation Error
    #[error("{message}: {0}", message = "Validation Error")]
    HttpValidationError(#[source] crate::v2027_01::models::common::types::HttpValidationError),
}
pub(super) type MetersQuantitiesErrors = (
    crate::v2027_01::error::RequestFailure,
    crate::v2027_01::models::common::types::ResourceNotFound,
    crate::v2027_01::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<MetersQuantitiesErrors>> for MetersQuantitiesError<T> {
    fn from(error: eros::ErrorUnion<MetersQuantitiesErrors>) -> Self {
        match error.into_enum() {
            eros::E3::A(error) => match error {
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
            eros::E3::B(error) => Self::ResourceNotFound(error),
            eros::E3::C(error) => Self::HttpValidationError(error),
        }
    }
}
impl<T> MetersQuantitiesError<T> {
    pub(super) fn from_failure(error: crate::v2027_01::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<MetersQuantitiesErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// Get quantities of a meter over a time period.
///
/// **Scopes**: `meters:read` `meters:write`
pub struct MetersQuantities<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) id: crate::v2027_01::models::common::ids::MeterId,
    pub(super) params: MetersQuantitiesParams,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> MetersQuantities<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = crate::v2027_01::query::EncodeQuery::encode_query(&self.params);
        let mut path = format!("v1/meters/{}/quantities", self.id);
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: MetersQuantitiesParams) -> Self {
        self.params = params;
        self
    }
}
impl<'a, ClientService, Body> MetersQuantities<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2027_01::models::meters::types::MeterQuantities,
        MetersQuantitiesError<ClientService::Error>,
    > {
        let request = self.build_request().map_err(|error| {
            MetersQuantitiesError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response = self.client.send(request).await.map_err(MetersQuantitiesError::Transport)?;
        let result: eros::Result<_, MetersQuantitiesErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2027_01::models::meters::types::MeterQuantities,
                >(&body_slice)
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::v2027_01::models::common::types::ResourceNotFound,
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
        crate::v2027_01::models::meters::types::MeterQuantities,
        MetersQuantitiesError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| MetersQuantitiesError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for MetersQuantities<'a, ClientService>
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
        crate::v2027_01::models::meters::types::MeterQuantities,
        MetersQuantitiesError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
