use http_body_util::BodyExt;

use crate::v2026_10::{
    client::{PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
};
#[derive(Debug, Clone, Default)]
pub struct EventsListParams {
    /// Filter by customer ID.
    pub customer_id: Vec<crate::v2026_10::models::common::ids::CustomerId>,
    /// Fetch descendants up to this depth. When set: 0=root events only,
    /// 1=roots+children, etc. Max 5. When not set, returns all events.
    pub depth: Option<i64>,
    /// Filter events before this timestamp.
    pub end_timestamp: Option<jiff::Timestamp>,
    /// Filter by external customer ID.
    pub external_customer_id: Vec<String>,
    /// Filter events following filter clauses. JSON string following the same
    /// schema a meter filter clause.
    pub filter: Option<String>,
    /// Size of a page, defaults to 10. Maximum is 100.
    pub limit: Option<crate::v2026_10::models::pagination::PageLimit>,
    /// Filter by metadata key-value pairs. It uses the `deepObject` style, e.g.
    /// `?metadata[key]=value`.
    pub metadata: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion,
    >,
    /// Filter by a meter filter clause.
    pub meter_id: Option<crate::v2026_10::models::common::ids::MeterId>,
    /// Filter by event name.
    pub name: Vec<String>,
    /// Filter by organization ID.
    pub organization_id: Vec<crate::v2026_10::models::common::ids::OrganizationId>,
    /// Page number, defaults to 1.
    pub page: Option<std::num::NonZeroU64>,
    /// When combined with depth, use this event as the anchor instead of root
    /// events.
    pub parent_id: Option<crate::v2026_10::models::common::ids::EventsListId>,
    /// Query to filter events.
    pub query: Option<String>,
    /// Filter by event source.
    pub source: Vec<crate::v2026_10::models::common::enums::EventSource>,
    /// Filter events after this timestamp.
    pub start_timestamp: Option<jiff::Timestamp>,
    /// Sorting criterion. Several criteria can be used simultaneously and will
    /// be applied in order. Add a minus sign `-` before the criteria name to
    /// sort by descending order.
    pub sorting: Vec<(
        crate::v2026_10::models::events::enums::EventSortProperty,
        crate::v2026_10::query::Sorting,
    )>,
}
#[allow(deprecated)]
impl crate::v2026_10::query::EncodeQuery for EventsListParams {
    #[allow(clippy::vec_init_then_push)]
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();
        for value in &self.customer_id {
            params.push(("customer_id", value.to_string()));
        }
        if let Some(value) = &self.depth {
            params.push(("depth", value.to_string()));
        }
        if let Some(value) = &self.end_timestamp {
            params.push(("end_timestamp", value.to_string()));
        }
        for value in &self.external_customer_id {
            params.push(("external_customer_id", value.to_string()));
        }
        if let Some(value) = &self.filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = &self.limit {
            params.push(("limit", value.to_string()));
        }
        if let Some(value) = &self.meter_id {
            params.push(("meter_id", value.to_string()));
        }
        for value in &self.name {
            params.push(("name", value.to_string()));
        }
        for value in &self.organization_id {
            params.push(("organization_id", value.to_string()));
        }
        if let Some(value) = &self.page {
            params.push(("page", value.to_string()));
        }
        if let Some(value) = &self.parent_id {
            params.push(("parent_id", value.to_string()));
        }
        if let Some(value) = &self.query {
            params.push(("query", value.to_string()));
        }
        for value in &self.source {
            params.push(("source", value.to_string()));
        }
        if let Some(value) = &self.start_timestamp {
            params.push(("start_timestamp", value.to_string()));
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
                    crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::String(
                        values,
                    ) => vec![values.to_string()],
                    crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::Integer(
                        values,
                    ) => vec![values.to_string()],
                    crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::Boolean(
                        values,
                    ) => vec![values.to_string()],
                    crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::StringArray(
                        values,
                    ) => values.iter().map(ToString::to_string).collect(),
                    crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::IntegerArray(
                        values,
                    ) => values.iter().map(ToString::to_string).collect(),
                    crate::v2026_10::models::common::unions::MetadataQueryAnyOf0AdditionalPropertiesUnion::BooleanArray(
                        values,
                    ) => values.iter().map(ToString::to_string).collect(),
                };
                (key.clone(), values)
            })
            .collect()
    }
}
#[derive(Debug, thiserror::Error)]
pub enum EventsListError<TransportError> {
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
pub(super) type EventsListErrors = (
    crate::v2026_10::error::RequestFailure,
    crate::v2026_10::models::common::types::HttpValidationError,
);
impl<T> From<eros::ErrorUnion<EventsListErrors>> for EventsListError<T> {
    fn from(error: eros::ErrorUnion<EventsListErrors>) -> Self {
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
impl<T> EventsListError<T> {
    pub(super) fn from_failure(error: crate::v2026_10::error::RequestFailure) -> Self {
        let union: eros::ErrorUnion<EventsListErrors> = eros::ErrorUnion::new(error);
        union.into()
    }
}
/// List events.
///
/// **Scopes**: `events:read` `events:write`
pub struct EventsList<'a, ClientService> {
    pub(super) client: &'a PolarClient<ClientService>,
    pub(super) params: Option<EventsListParams>,
    pub(super) options: RequestOptions,
}
impl<'a, ClientService> EventsList<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params = self
            .params
            .as_ref()
            .map_or_else(String::new, crate::v2026_10::query::EncodeQuery::encode_query);
        let mut path = "v1/events/".to_owned();
        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }
        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }

    pub fn params(mut self, params: EventsListParams) -> Self {
        self.params = Some(params);
        self
    }
}
impl<'a, ClientService, Body> EventsList<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<
        crate::v2026_10::models::events::unions::EventsListResponse200Union,
        EventsListError<ClientService::Error>,
    > {
        let request = self
            .build_request()
            .map_err(|error| EventsListError::from_failure(RequestFailure::BuildRequest(error)))?;
        let response = self.client.send(request).await.map_err(EventsListError::Transport)?;
        let result: eros::Result<_, EventsListErrors> = async {
            let response_code = response.status();
            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();
            match response_code {
                http::StatusCode::OK => serde_json::from_slice::<
                    crate::v2026_10::models::events::unions::EventsListResponse200Union,
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
        crate::v2026_10::models::events::unions::EventsListResponse200Union,
        EventsListError<ClientService::Error>,
    > {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| EventsListError::from_failure(RequestFailure::Timeout(error)))?
    }
}
impl<'a, ClientService, Body> std::future::IntoFuture for EventsList<'a, ClientService>
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
        crate::v2026_10::models::events::unions::EventsListResponse200Union,
        EventsListError<ClientService::Error>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
