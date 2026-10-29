use futures_util::TryStreamExt;

use crate::{
    client::{AccessToken, PolarClient, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestFailure},
    models::{common::OrganizationId, pagination::ListResource},
    query::EncodeQuery,
};
mod enums;
mod errors;
mod types;

#[cfg(test)]
mod page_stream_tests;

#[cfg(test)]
mod item_stream_tests;

#[cfg(test)]
mod create_tests;

#[cfg(test)]
mod complete_create_tests;

#[cfg(test)]
mod update_tests;

#[cfg(test)]
mod error_tests;

pub use enums::{
    OrganizationSocialPlatforms, OrganizationSortField,
    OrganizationSubscriptionSettingsProrationBehavior, OrganizationSwitchingFrom,
};
pub use errors::{
    CannotCreateOrganizationError, CannotCreateOrganizationErrorKind, CreateOrganizationError,
    DisputeAutoAcceptNotEnabled, DisputeAutoAcceptNotEnabledKind, GetOrganizationError,
    ListOrganizationsError, SsoEnforcementRequiresConnection, SsoEnforcementRequiresConnectionKind,
    UpdateOrganizationError,
};
use http_body_util::BodyExt;
pub use types::{
    CustomerPortalCustomerSettings, CustomerPortalSubscriptionSettings,
    CustomerPortalUsageSettings, ListOrganizationsParams, Organization, OrganizationCreate,
    OrganizationCreateRequired, OrganizationCustomerEmailSettings,
    OrganizationCustomerPortalSettings, OrganizationDetails, OrganizationDisputeSettingsUpdate,
    OrganizationFeatureSettingsUpdate, OrganizationLegalEntity, OrganizationSocialLink,
    OrganizationSubscriptionSettings, OrganizationUpdate,
};

pub struct Organizations<'a, ClientService> {
    client: &'a PolarClient<ClientService>,
}

impl<'a, ClientService> Organizations<'a, ClientService> {
    pub(crate) fn new(client: &'a PolarClient<ClientService>) -> Self {
        Self { client }
    }

    pub fn get(&self, organization_id: OrganizationId) -> GetOrganization<'a, ClientService> {
        GetOrganization {
            client: self.client,
            id: organization_id,
            options: RequestOptions::default(),
        }
    }

    pub fn list(&self) -> ListOrganizations<'a, ClientService> {
        ListOrganizations { client: self.client, options: RequestOptions::default(), params: None }
    }

    pub fn create(&self, body: OrganizationCreate) -> CreateOrganization<'a, ClientService> {
        CreateOrganization {
            client: self.client,
            options: RequestOptions::default(),
            payload: body,
        }
    }

    pub fn update(
        &self,
        organization_id: OrganizationId,
        payload: OrganizationUpdate,
    ) -> UpdateOrganization<'a, ClientService> {
        UpdateOrganization {
            client: self.client,
            id: organization_id,
            payload,
            options: RequestOptions::default(),
        }
    }
}

pub struct GetOrganization<'a, ClientService> {
    client: &'a PolarClient<ClientService>,
    id: OrganizationId,
    options: RequestOptions,
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
        let result: eros::Result<_, errors::GetOrganizationErrors> = async {
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

pub struct ListOrganizations<'a, ClientService> {
    client: &'a PolarClient<ClientService>,
    options: RequestOptions,
    params: Option<ListOrganizationsParams>,
}

impl<'a, ClientService> ListOrganizations<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let query_params =
            self.params.as_ref().map_or_default(ListOrganizationsParams::encode_query);
        let mut path = format!("v1/organizations/");

        if !query_params.is_empty() {
            path.push_str(format!("?{query_params}").as_ref());
        }

        self.client.build_request(http::Method::GET, &path, None, &self.options)
    }

    pub fn options(mut self, new_options: RequestOptions) -> Self {
        self.options = new_options;
        self
    }

    pub fn params(mut self, params: ListOrganizationsParams) -> Self {
        self.params = Some(params);
        self
    }
}

impl<'a, ClientService, Body> ListOrganizations<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(
        self,
    ) -> Result<ListResource<Organization>, ListOrganizationsError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            ListOrganizationsError::from_failure(RequestFailure::BuildRequest(error))
        })?;

        let response =
            self.client.send(request).await.map_err(ListOrganizationsError::Transport)?;
        let result: eros::Result<_, errors::ListOrganizationsErrors> = async {
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

    pub async fn send(
        self,
    ) -> Result<ListResource<Organization>, ListOrganizationsError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());

        tokio::time::timeout(duration, self.execute())
            .await
            .map_err(|error| ListOrganizationsError::from_failure(RequestFailure::Timeout(error)))?
    }
}

impl<'a, ClientService, Body> std::future::IntoFuture for ListOrganizations<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<ListResource<Organization>, ListOrganizationsError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

impl<'a, ClientService, Body> ListOrganizations<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    pub fn pages(
        self,
    ) -> impl futures_util::Stream<
        Item = Result<ListResource<Organization>, ListOrganizationsError<ClientService::Error>>,
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

                client.organizations().list().params(params).options(options)
            });

            Ok(Some((response, next)))
        })
    }

    pub fn items(
        self,
    ) -> impl futures_util::Stream<
        Item = Result<Organization, ListOrganizationsError<ClientService::Error>>,
    > + 'a {
        self.pages()
            .map_ok(|page| futures_util::stream::iter(page.items.into_iter().map(Ok)))
            .try_flatten()
    }
}

pub struct CreateOrganization<'a, ClientService> {
    client: &'a PolarClient<ClientService>,
    payload: OrganizationCreate,
    options: RequestOptions,
}

impl<'a, ClientService> CreateOrganization<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/organizations/");

        let payload_bytes = serde_json::to_vec(&self.payload)?;

        self.client.build_request(
            http::Method::POST,
            &path,
            Some(payload_bytes.into()),
            &self.options,
        )
    }

    pub fn options(mut self, new_options: RequestOptions) -> Self {
        self.options = new_options;
        self
    }
}

impl<'a, ClientService, Body> CreateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<Organization, CreateOrganizationError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            CreateOrganizationError::from_failure(RequestFailure::BuildRequest(error))
        })?;

        let response =
            self.client.send(request).await.map_err(CreateOrganizationError::Transport)?;
        let result: eros::Result<_, errors::CreateOrganizationErrors> = async {
            let response_code = response.status();

            let body_slice = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();

            match response_code {
                http::StatusCode::CREATED => serde_json::from_slice(&body_slice)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error =
                        serde_json::from_slice::<CannotCreateOrganizationError>(&body_slice)
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

    pub async fn send(self) -> Result<Organization, CreateOrganizationError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());

        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            CreateOrganizationError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}

impl<'a, ClientService, Body> std::future::IntoFuture for CreateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<Organization, CreateOrganizationError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

pub struct UpdateOrganization<'a, ClientService> {
    client: &'a PolarClient<ClientService>,
    id: OrganizationId,
    payload: OrganizationUpdate,
    options: RequestOptions,
}

impl<'a, ClientService> UpdateOrganization<'a, ClientService> {
    pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let path = format!("v1/organizations/{}", self.id);
        let body = serde_json::to_vec(&self.payload)?;
        self.client.build_request(http::Method::PATCH, &path, Some(body.into()), &self.options)
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        self.options = options;
        self
    }
}

impl<'a, ClientService, Body> UpdateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
    Body: http_body::Body<Data = bytes::Bytes>,
    Body::Error: Into<tower::BoxError>,
{
    async fn execute(self) -> Result<Organization, UpdateOrganizationError<ClientService::Error>> {
        let request = self.build_request().map_err(|error| {
            UpdateOrganizationError::from_failure(RequestFailure::BuildRequest(error))
        })?;
        let response =
            self.client.send(request).await.map_err(UpdateOrganizationError::Transport)?;
        let result: eros::Result<_, errors::UpdateOrganizationErrors> = async {
            let status = response.status();
            let body = response
                .into_body()
                .collect()
                .await
                .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?
                .to_bytes();

            match status {
                http::StatusCode::OK => serde_json::from_slice(&body)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))),
                http::StatusCode::FORBIDDEN => {
                    let error =
                        serde_json::from_slice::<errors::OrganizationUpdateForbidden>(&body)
                            .map_err(|error| {
                                eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                            })?;
                    Err(match error {
                        errors::OrganizationUpdateForbidden::NotPermitted(error) => {
                            eros::ErrorUnion::new(error)
                        }
                        errors::OrganizationUpdateForbidden::DisputeAutoAcceptNotEnabled(error) => {
                            eros::ErrorUnion::new(error)
                        }
                    })
                }
                http::StatusCode::NOT_FOUND => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::ResourceNotFound,
                    >(&body)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::CONFLICT => {
                    let error = serde_json::from_slice::<SsoEnforcementRequiresConnection>(&body)
                        .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                http::StatusCode::UNPROCESSABLE_ENTITY => {
                    let error = serde_json::from_slice::<
                        crate::models::common::errors::HttpValidationError,
                    >(&body)
                    .map_err(|error| {
                        eros::ErrorUnion::new(RequestFailure::DecodeResponse(error))
                    })?;
                    Err(eros::ErrorUnion::new(error))
                }
                _ => Err(eros::ErrorUnion::new(RequestFailure::UnexpectedStatus { status, body })),
            }
        }
        .await;
        result.map_err(Into::into)
    }

    pub async fn send(self) -> Result<Organization, UpdateOrganizationError<ClientService::Error>> {
        let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
        tokio::time::timeout(duration, self.execute()).await.map_err(|error| {
            UpdateOrganizationError::from_failure(RequestFailure::Timeout(error))
        })?
    }
}

impl<'a, ClientService, Body> std::future::IntoFuture for UpdateOrganization<'a, ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
    ClientService::Future: 'a,
    ClientService::Error: 'a,
    Body: http_body::Body<Data = bytes::Bytes> + 'a,
    Body::Error: Into<tower::BoxError>,
{
    type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
    type Output = Result<Organization, UpdateOrganizationError<ClientService::Error>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[cfg(test)]
mod list_execution_tests {
    use std::{convert::Infallible, num::NonZeroU64, time::Duration};

    use super::{ListOrganizationsParams, OrganizationSortField};
    use crate::{
        client::{AccessToken, PolarClient, PolarEnvironment, RequestBody, RequestOptions},
        models::{
            common::{OrganizationSlug, PageLimit},
            organization::ListOrganizationsError,
        },
        query::Sorting,
    };

    const EMPTY_PAGE: &[u8] = br#"{"items":[],"pagination":{"total_count":0,"max_page":0}}"#;

    fn mock_transport(
        status: http::StatusCode,
        body: &'static [u8],
        expected_query: Option<&'static str>,
    ) -> impl tower::Service<
        http::Request<RequestBody>,
        Response = http::Response<RequestBody>,
        Error = Infallible,
    > + Clone {
        tower::service_fn(move |request: http::Request<RequestBody>| async move {
            assert_eq!(request.method(), http::Method::GET);
            assert_eq!(request.uri().scheme_str(), Some("https"));
            assert_eq!(request.uri().host(), Some("sandbox-api.polar.sh"));
            assert_eq!(request.uri().path(), "/v1/organizations/");
            assert_eq!(request.uri().query(), expected_query);
            assert_eq!(request.headers()["Authorization"], "Bearer example-token");
            assert!(request.headers()["Authorization"].is_sensitive());

            let response = http::Response::builder()
                .status(status)
                .body(RequestBody::new(bytes::Bytes::from_static(body)))
                .unwrap();
            Ok::<_, Infallible>(response)
        })
    }

    #[tokio::test]
    async fn send_decodes_an_empty_page() {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(http::StatusCode::OK, EMPTY_PAGE, None))
                .unwrap();

        let page = client.organizations().list().send().await.unwrap();
        assert!(page.items.is_empty());
        assert_eq!(page.pagination.total_count, 0);
        assert_eq!(page.pagination.max_page, 0);
    }

    #[tokio::test]
    async fn direct_await_decodes_an_empty_page_and_forwards_query_params() {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    http::StatusCode::OK,
                    EMPTY_PAGE,
                    Some("slug=acme+%26+sons&page=2&limit=20&sorting=-name&sorting=created_at"),
                ))
                .unwrap();
        let params = ListOrganizationsParams {
            slug: Some(OrganizationSlug::try_from("acme & sons").unwrap()),
            page: NonZeroU64::new(2),
            limit: Some(PageLimit::try_from(20).expect("valid test limit")),
            sorting: vec![
                (OrganizationSortField::Name, Sorting::Desc),
                (OrganizationSortField::CreatedAt, Sorting::Asc),
            ],
        };

        let page = client.organizations().list().params(params).await.unwrap();
        assert!(page.items.is_empty());
        assert_eq!(page.pagination.total_count, 0);
        assert_eq!(page.pagination.max_page, 0);
    }

    #[tokio::test]
    async fn decodes_validation_errors() {
        let body =
            br#"{"detail":[{"loc":["query","limit"],"msg":"Invalid limit","type":"value_error"}]}"#;
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(http::StatusCode::UNPROCESSABLE_ENTITY, body, None))
                .unwrap();

        let result = client.organizations().list().await;
        match result {
            Err(ListOrganizationsError::Validation(error)) => {
                let details = error.detail.expect("expected validation details");
                assert_eq!(details.len(), 1);
                assert_eq!(details[0].msg, "Invalid limit");
                assert_eq!(details[0].kind, "value_error");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn rejects_invalid_success_and_validation_bodies() {
        for (status, body) in [
            (http::StatusCode::OK, b"not-json".as_slice()),
            (http::StatusCode::OK, b"{}".as_slice()),
            (http::StatusCode::UNPROCESSABLE_ENTITY, b"not-json".as_slice()),
        ] {
            let client = PolarClient::builder(
                AccessToken::try_from("example-token").expect("valid test token"),
            )
            .environment(PolarEnvironment::Sandbox)
            .build_with(mock_transport(status, body, None))
            .unwrap();

            let result = client.organizations().list().await;
            assert!(
                matches!(&result, Err(ListOrganizationsError::DecodeResponse(_))),
                "expected DecodeResponse for {status} and {body:?}, got {result:?}"
            );
        }
    }

    #[tokio::test]
    async fn preserves_unexpected_status_and_body_including_404() {
        for (expected_status, expected_body) in [
            (
                http::StatusCode::NOT_FOUND,
                br#"{"error":"ResourceNotFound","detail":"Missing route"}"#.as_slice(),
            ),
            (http::StatusCode::INTERNAL_SERVER_ERROR, b"server failure".as_slice()),
        ] {
            let client = PolarClient::builder(
                AccessToken::try_from("example-token").expect("valid test token"),
            )
            .environment(PolarEnvironment::Sandbox)
            .build_with(mock_transport(expected_status, expected_body, None))
            .unwrap();

            let result = client.organizations().list().await;
            assert!(
                matches!(
                    &result,
                    Err(ListOrganizationsError::UnexpectedStatus { status, body })
                        if *status == expected_status && body.as_ref() == expected_body
                ),
                "expected UnexpectedStatus preserving {expected_status} and its body, got \
                 {result:?}"
            );
        }
    }

    #[tokio::test]
    async fn send_inherits_the_client_timeout() {
        let transport = tower::service_fn(|_: http::Request<RequestBody>| {
            std::future::pending::<Result<http::Response<RequestBody>, Infallible>>()
        });
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .timeout(Duration::from_millis(10))
                .build_with(transport)
                .unwrap();

        let result =
            tokio::time::timeout(Duration::from_secs(1), client.organizations().list().send())
                .await
                .expect("the inherited timeout must expire before the test deadline");
        assert!(matches!(&result, Err(ListOrganizationsError::Timeout(_))), "got {result:?}");
    }

    #[tokio::test]
    async fn direct_await_honors_timeout_override_without_changing_client_default() {
        let transport = tower::service_fn(|_: http::Request<RequestBody>| {
            std::future::pending::<Result<http::Response<RequestBody>, Infallible>>()
        });
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .timeout(Duration::from_secs(60))
                .build_with(transport)
                .unwrap();

        let result = tokio::time::timeout(Duration::from_secs(1), async {
            client
                .organizations()
                .list()
                .options(RequestOptions {
                    timeout: Some(Duration::from_millis(10)),
                    ..Default::default()
                })
                .await
        })
        .await
        .expect("the override must expire before the test deadline");
        assert!(matches!(&result, Err(ListOrganizationsError::Timeout(_))), "got {result:?}");
        assert_eq!(client.default_timeout(), Duration::from_secs(60));
    }
}

#[cfg(test)]
mod list_request_tests {
    use std::num::NonZeroU64;

    use http_body_util::BodyExt;

    use super::{ListOrganizationsParams, OrganizationSortField};
    use crate::{
        client::{AccessToken, PolarClient, PolarEnvironment, RequestOptions},
        models::common::{OrganizationSlug, PageLimit},
        query::Sorting,
    };

    fn client() -> PolarClient<()> {
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(())
            .unwrap()
    }

    #[tokio::test]
    async fn builds_list_without_params_with_headers_and_empty_body() {
        let client = client();
        let request = client.organizations().list().build_request().unwrap();

        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/");
        assert_eq!(request.uri().query(), None);
        assert_eq!(request.headers()["Authorization"], "Bearer example-token");
        assert!(request.headers()["Authorization"].is_sensitive());
        assert_eq!(request.headers()["Accept"], "application/json");
        assert_eq!(request.headers()["Polar-Version"], "2026-10");
        assert!(request.into_body().collect().await.unwrap().to_bytes().is_empty());
    }

    #[test]
    fn default_params_do_not_append_a_question_mark() {
        let client = client();
        let request = client
            .organizations()
            .list()
            .params(ListOrganizationsParams::default())
            .build_request()
            .unwrap();

        assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/");
        assert_eq!(request.uri().query(), None);
    }

    #[test]
    fn builds_encoded_query_preserving_sorting_order() {
        let client = client();
        let params = ListOrganizationsParams {
            slug: Some(OrganizationSlug::try_from("acme & sons+").expect("valid test slug")),
            page: NonZeroU64::new(2),
            limit: Some(PageLimit::try_from(20).expect("valid test limit")),
            sorting: vec![
                (OrganizationSortField::Name, Sorting::Desc),
                (OrganizationSortField::CreatedAt, Sorting::Asc),
            ],
        };
        let request = client.organizations().list().params(params).build_request().unwrap();

        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(
            request.uri().to_string(),
            "https://sandbox-api.polar.sh/v1/organizations/?slug=acme+%26+sons%2B&page=2&limit=20&sorting=-name&sorting=created_at"
        );
    }

    #[test]
    fn token_override_is_sensitive_and_preserves_client_default() {
        let client = client();
        let request = client
            .organizations()
            .list()
            .options(RequestOptions {
                access_token: Some(
                    AccessToken::try_from("override-token").expect("nonempty test token"),
                ),
                ..Default::default()
            })
            .build_request()
            .unwrap();

        assert_eq!(request.headers()["Authorization"], "Bearer override-token");
        assert!(request.headers()["Authorization"].is_sensitive());

        let ordinary = client.organizations().list().build_request().unwrap();
        assert_eq!(ordinary.headers()["Authorization"], "Bearer example-token");
        assert!(ordinary.headers()["Authorization"].is_sensitive());
    }

    #[test]
    fn rejects_invalid_token_override() {
        let client = client();
        let result = client
            .organizations()
            .list()
            .options(RequestOptions {
                access_token: Some(
                    AccessToken::try_from("invalid\ntoken").expect("nonempty test token"),
                ),
                ..Default::default()
            })
            .build_request();

        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn params_replaces_the_previous_parameter_struct() {
        let client = client();
        let first = ListOrganizationsParams {
            slug: Some(OrganizationSlug::try_from("old-slug").expect("valid test slug")),
            page: NonZeroU64::new(2),
            limit: Some(PageLimit::try_from(20).expect("valid test limit")),
            sorting: vec![(OrganizationSortField::Name, Sorting::Desc)],
        };
        let second = ListOrganizationsParams { page: NonZeroU64::new(3), ..Default::default() };
        let request =
            client.organizations().list().params(first).params(second).build_request().unwrap();

        assert_eq!(
            request.uri().to_string(),
            "https://sandbox-api.polar.sh/v1/organizations/?page=3"
        );
    }
}
