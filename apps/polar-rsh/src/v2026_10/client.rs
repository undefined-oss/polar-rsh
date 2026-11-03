use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use derive_more::{Deref, Eq, PartialEq};
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use url::Url;

use crate::v2026_10::error::BuildRequestError;
/// API environments declared by the OpenAPI document. Production is the
/// default.
#[derive(Debug, PartialEq, Eq, Default)]
pub enum PolarEnvironment {
    #[default]
    Production,
    Sandbox,
}
impl PolarEnvironment {
    fn base_url(&self) -> Url {
        let base_url = match self {
            PolarEnvironment::Production => "https://api.polar.sh",
            PolarEnvironment::Sandbox => "https://sandbox-api.polar.sh",
        };
        Url::parse(base_url).expect("Polar environment URLs are valid")
    }
}
/// API version used by this generated client.
pub const POLAR_VERSION: &str = "2026-10";
/// Configure authentication, endpoint, and timeout before supplying a
/// transport.
///
/// Defaults to production with a five-second operation timeout. An explicit
/// base URL takes precedence over environment selection, regardless of call
/// order.
pub struct PolarBuilder {
    access_token: Option<AccessToken>,
    environment: PolarEnvironment,
    timeout: Duration,
    base_url: Url,
    base_url_overridden: bool,
}
impl PolarBuilder {
    /// Start with production and a five-second timeout. Pass `None` for no
    /// default authentication.
    pub fn new(access_token: impl Into<Option<AccessToken>>) -> Self {
        let default_environment = PolarEnvironment::default();
        Self {
            access_token: access_token.into(),
            base_url: default_environment.base_url(),
            timeout: Duration::from_secs(5),
            environment: default_environment,
            base_url_overridden: false,
        }
    }

    /// Set the default operation timeout, including request construction,
    /// transport readiness, sending, response body collection, and decoding.
    ///
    /// Individual operations can override this through
    /// [`RequestOptions::timeout`].
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Select an environment's endpoint unless an explicit base URL was
    /// supplied.
    pub fn environment(mut self, environment: PolarEnvironment) -> Self {
        self.environment = environment;
        if self.base_url_overridden {
            return self;
        }
        self.base_url = self.environment.base_url();
        self
    }

    /// Override the environment's endpoint, including subsequent environment
    /// changes.
    ///
    /// Relative operation paths are joined to this URL with its path treated as
    /// a directory. URL joining errors are returned when building a request.
    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self.base_url_overridden = true;
        self
    }

    /// Build a client that owns the caller-supplied transport.
    ///
    /// Construction places no service bounds on `S`. Sending requires a
    /// cloneable Tower service accepting `http::Request<RequestBody>`;
    /// response body bounds apply at the operation level. Passing `()`
    /// supports request construction only.
    ///
    /// # Errors
    /// Returns an error if the access token cannot form a valid HTTP header
    /// value.
    pub fn build_with<S>(
        self,
        transport: S,
    ) -> Result<PolarClient<S>, http::header::InvalidHeaderValue> {
        let authorization_header =
            self.access_token.as_ref().map(AccessToken::to_header).transpose()?;
        Ok(PolarClient {
            transport,
            config: ClientConfig {
                authorization: authorization_header,
                base_url: self.base_url,
                timeout: self.timeout,
            },
        })
    }
}
struct ClientConfig {
    authorization: Option<http::HeaderValue>,
    base_url: Url,
    timeout: Duration,
}
/// Client for [`POLAR_VERSION`], using a caller-supplied Tower transport.
///
/// The transport is cloned for each send. The client does not create a
/// connection pool or impose unconditional `Send` or `Sync` bounds. Use
/// [`Self::builder`] to configure the client before supplying a transport.
pub struct PolarClient<S = ()> {
    transport: S,
    config: ClientConfig,
}
impl PolarClient<()> {
    /// Create a builder with production and a five-second operation timeout.
    /// Pass `None` to omit the default bearer credential.
    pub fn builder(access_token: impl Into<Option<AccessToken>>) -> PolarBuilder {
        PolarBuilder::new(access_token)
    }
}
/// Body type accepted by the client's transport.
pub type RequestBody = http_body_util::Full<bytes::Bytes>;
impl<ClientService, Body> PolarClient<ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
{
    /// Wait for readiness and send through a cloned transport, preserving its
    /// concrete error type. Operation code applies timeouts and decodes
    /// responses.
    pub(crate) async fn send(
        &self,
        request: http::Request<RequestBody>,
    ) -> Result<http::Response<Body>, ClientService::Error> {
        let transport = self.transport.clone();
        transport.oneshot(request).await
    }
}
impl<ClientService> PolarClient<ClientService> {
    /// Build a request with the generated API version header and optional
    /// authentication.
    ///
    /// A present body is sent unchanged with JSON content type. A missing body
    /// produces empty bytes without content type. Per-request authentication
    /// overrides leave the client's defaults unchanged.
    ///
    /// # Errors
    /// Returns URL joining or HTTP construction errors, including invalid
    /// authorization header values supplied by a request override.
    pub(crate) fn build_request(
        &self,
        method: http::Method,
        path: &str,
        body: Option<bytes::Bytes>,
        options: &RequestOptions,
    ) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let request_url = crate::v2026_10::path::join(&self.config.base_url, path)?;
        let authorization_value = if let Some(access_token) = &options.access_token {
            Some(access_token.to_header().map_err(http::Error::from)?)
        } else {
            self.config.authorization.clone()
        };
        let mut request = http::Request::builder()
            .uri(request_url.as_str())
            .method(method)
            .header("Accept", "application/json")
            .header("Polar-Version", POLAR_VERSION);
        if let Some(authorization_value) = authorization_value {
            request = request.header(http::header::AUTHORIZATION, authorization_value);
        }
        if body.is_some() {
            request = request.header(http::header::CONTENT_TYPE, "application/json");
        }
        request.body(RequestBody::new(body.unwrap_or_default())).map_err(BuildRequestError::from)
    }

    /// Access benefit_grants operations borrowing this client.
    pub fn benefit_grants(
        &self,
    ) -> crate::v2026_10::models::benefit_grants::BenefitGrants<'_, ClientService> {
        crate::v2026_10::models::benefit_grants::BenefitGrants::new(self)
    }

    /// Access benefits operations borrowing this client.
    pub fn benefits(&self) -> crate::v2026_10::models::benefits::Benefits<'_, ClientService> {
        crate::v2026_10::models::benefits::Benefits::new(self)
    }

    /// Access checkout_links operations borrowing this client.
    pub fn checkout_links(
        &self,
    ) -> crate::v2026_10::models::checkout_links::CheckoutLinks<'_, ClientService> {
        crate::v2026_10::models::checkout_links::CheckoutLinks::new(self)
    }

    /// Access checkouts operations borrowing this client.
    pub fn checkouts(&self) -> crate::v2026_10::models::checkouts::Checkouts<'_, ClientService> {
        crate::v2026_10::models::checkouts::Checkouts::new(self)
    }

    /// Access custom_fields operations borrowing this client.
    pub fn custom_fields(
        &self,
    ) -> crate::v2026_10::models::custom_fields::CustomFields<'_, ClientService> {
        crate::v2026_10::models::custom_fields::CustomFields::new(self)
    }

    /// Access customer_meters operations borrowing this client.
    pub fn customer_meters(
        &self,
    ) -> crate::v2026_10::models::customer_meters::CustomerMeters<'_, ClientService> {
        crate::v2026_10::models::customer_meters::CustomerMeters::new(self)
    }

    /// Access customer_portal operations borrowing this client.
    pub fn customer_portal(
        &self,
    ) -> crate::v2026_10::models::customer_portal::CustomerPortal<'_, ClientService> {
        crate::v2026_10::models::customer_portal::CustomerPortal::new(self)
    }

    /// Access customer_seats operations borrowing this client.
    pub fn customer_seats(
        &self,
    ) -> crate::v2026_10::models::customer_seats::CustomerSeats<'_, ClientService> {
        crate::v2026_10::models::customer_seats::CustomerSeats::new(self)
    }

    /// Access customer_sessions operations borrowing this client.
    pub fn customer_sessions(
        &self,
    ) -> crate::v2026_10::models::customer_sessions::CustomerSessions<'_, ClientService> {
        crate::v2026_10::models::customer_sessions::CustomerSessions::new(self)
    }

    /// Access customers operations borrowing this client.
    pub fn customers(&self) -> crate::v2026_10::models::customers::Customers<'_, ClientService> {
        crate::v2026_10::models::customers::Customers::new(self)
    }

    /// Access discounts operations borrowing this client.
    pub fn discounts(&self) -> crate::v2026_10::models::discounts::Discounts<'_, ClientService> {
        crate::v2026_10::models::discounts::Discounts::new(self)
    }

    /// Access disputes operations borrowing this client.
    pub fn disputes(&self) -> crate::v2026_10::models::disputes::Disputes<'_, ClientService> {
        crate::v2026_10::models::disputes::Disputes::new(self)
    }

    /// Access event_types operations borrowing this client.
    pub fn event_types(
        &self,
    ) -> crate::v2026_10::models::event_types::EventTypes<'_, ClientService> {
        crate::v2026_10::models::event_types::EventTypes::new(self)
    }

    /// Access events operations borrowing this client.
    pub fn events(&self) -> crate::v2026_10::models::events::Events<'_, ClientService> {
        crate::v2026_10::models::events::Events::new(self)
    }

    /// Access files operations borrowing this client.
    pub fn files(&self) -> crate::v2026_10::models::files::Files<'_, ClientService> {
        crate::v2026_10::models::files::Files::new(self)
    }

    /// Access license_keys operations borrowing this client.
    pub fn license_keys(
        &self,
    ) -> crate::v2026_10::models::license_keys::LicenseKeys<'_, ClientService> {
        crate::v2026_10::models::license_keys::LicenseKeys::new(self)
    }

    /// Access meters operations borrowing this client.
    pub fn meters(&self) -> crate::v2026_10::models::meters::Meters<'_, ClientService> {
        crate::v2026_10::models::meters::Meters::new(self)
    }

    /// Access metrics operations borrowing this client.
    pub fn metrics(&self) -> crate::v2026_10::models::metrics::MetricsResource<'_, ClientService> {
        crate::v2026_10::models::metrics::MetricsResource::new(self)
    }

    /// Access oauth2 operations borrowing this client.
    pub fn oauth2(&self) -> crate::v2026_10::models::oauth2::Oauth2<'_, ClientService> {
        crate::v2026_10::models::oauth2::Oauth2::new(self)
    }

    /// Access orders operations borrowing this client.
    pub fn orders(&self) -> crate::v2026_10::models::orders::Orders<'_, ClientService> {
        crate::v2026_10::models::orders::Orders::new(self)
    }

    /// Access organizations operations borrowing this client.
    pub fn organizations(
        &self,
    ) -> crate::v2026_10::models::organizations::Organizations<'_, ClientService> {
        crate::v2026_10::models::organizations::Organizations::new(self)
    }

    /// Access payments operations borrowing this client.
    pub fn payments(&self) -> crate::v2026_10::models::payments::Payments<'_, ClientService> {
        crate::v2026_10::models::payments::Payments::new(self)
    }

    /// Access products operations borrowing this client.
    pub fn products(&self) -> crate::v2026_10::models::products::Products<'_, ClientService> {
        crate::v2026_10::models::products::Products::new(self)
    }

    /// Access refunds operations borrowing this client.
    pub fn refunds(&self) -> crate::v2026_10::models::refunds::Refunds<'_, ClientService> {
        crate::v2026_10::models::refunds::Refunds::new(self)
    }

    /// Access subscriptions operations borrowing this client.
    pub fn subscriptions(
        &self,
    ) -> crate::v2026_10::models::subscriptions::Subscriptions<'_, ClientService> {
        crate::v2026_10::models::subscriptions::Subscriptions::new(self)
    }

    /// Access webhooks operations borrowing this client.
    pub fn webhooks(&self) -> crate::v2026_10::models::webhooks::Webhooks<'_, ClientService> {
        crate::v2026_10::models::webhooks::Webhooks::new(self)
    }

    pub(crate) fn default_timeout(&self) -> Duration {
        self.config.timeout
    }
}
/// Bearer credential created from a raw token without the `Bearer ` prefix.
///
/// [`Debug`] and [`Display`] redact the credential; [`Self::to_header`] uses
/// its actual value. Derived serialization and dereferencing expose the stored
/// credential, including the prefix, and do not apply formatting redaction.
#[derive(Clone, Deref, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccessToken(String);
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("access token can't be empty")]
pub struct AccessTokenValidationError;
impl AccessToken {
    /// Construct a bearer credential from a raw token.
    ///
    /// # Panics
    /// Panics if `token` is empty. Use [`Self::try_new`] for fallible
    /// construction.
    pub fn new_or_panic(token: &str) -> Self {
        Self::try_new(token).unwrap()
    }

    /// Prefix a nonempty raw token with `Bearer `.
    ///
    /// HTTP header validation is deferred to [`Self::to_header`].
    ///
    /// # Errors
    /// Returns [`AccessTokenValidationError`] if `token` is empty.
    pub fn try_new(token: &str) -> Result<Self, AccessTokenValidationError> {
        if token.is_empty() {
            return Err(AccessTokenValidationError);
        }
        Ok(Self(format!("Bearer {token}")))
    }

    /// Convert the credential into an authorization header marked as sensitive.
    ///
    /// # Errors
    /// Returns an error if the credential contains invalid HTTP header
    /// characters.
    pub fn to_header(&self) -> Result<http::HeaderValue, http::header::InvalidHeaderValue> {
        let mut header = http::HeaderValue::from_str(&self.0)?;
        header.set_sensitive(true);
        Ok(header)
    }
}
impl Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Bearer [REDACTED]")
    }
}
impl Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Bearer [REDACTED]")
    }
}
impl TryFrom<String> for AccessToken {
    type Error = AccessTokenValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AccessToken::try_new(value.as_ref())
    }
}
impl TryFrom<&str> for AccessToken {
    type Error = AccessTokenValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AccessToken::try_new(value)
    }
}
/// Overrides for a single operation. Absent values inherit client defaults.
#[derive(Default, Clone)]
pub struct RequestOptions {
    /// Bearer credential for this operation. `None` inherits the client
    /// setting.
    pub access_token: Option<AccessToken>,
    /// Timeout for this operation.
    pub timeout: Option<Duration>,
}
