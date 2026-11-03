use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use derive_more::{Deref, Eq, PartialEq};
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use url::Url;

use crate::{error::BuildRequestError, models::organization::Organizations};

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

#[derive(Debug)]
pub enum PolarVersion {
    V2026_04,
    V2026_10,
    V2027_01,
}

impl PolarVersion {
    fn version(&self) -> &'static str {
        match self {
            PolarVersion::V2026_04 => "2026-04",
            PolarVersion::V2026_10 => "2026-10",
            PolarVersion::V2027_01 => "2027-01",
        }
    }
}

pub struct PolarBuilder {
    access_token: Option<AccessToken>,
    environment: PolarEnvironment,
    timeout: Duration,
    base_url: Url,
    base_url_overridden: bool,
}

impl PolarBuilder {
    /// Pass `None` for no default authentication.
    pub fn new(access_token: impl Into<Option<AccessToken>>) -> Self {
        let default_environment = PolarEnvironment::Production;
        Self {
            access_token: access_token.into(),
            base_url: default_environment.base_url(),
            timeout: Duration::from_secs(5),
            environment: default_environment,
            base_url_overridden: false,
        }
    }

    // TODO: We need a type to enforce always secs
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn environment(mut self, environment: PolarEnvironment) -> Self {
        self.environment = environment;

        if self.base_url_overridden {
            return self;
        }

        self.base_url = self.environment.base_url();
        self
    }

    pub fn base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self.base_url_overridden = true;
        self
    }

    // Custom error;
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

pub struct PolarClient<S = ()> {
    // Agnostict http client that implements http and tower crate.
    transport: S,
    config: ClientConfig,
}

impl PolarClient<()> {
    /// Pass `None` to omit the default bearer credential.
    pub fn builder(access_token: impl Into<Option<AccessToken>>) -> PolarBuilder {
        PolarBuilder::new(access_token)
    }
}

pub type RequestBody = http_body_util::Full<bytes::Bytes>;

impl<ClientService, Body> PolarClient<ClientService>
where
    ClientService:
        tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
{
    pub(crate) async fn send(
        &self,
        request: http::Request<RequestBody>,
    ) -> Result<http::Response<Body>, ClientService::Error> {
        let transport = self.transport.clone();
        transport.oneshot(request).await
    }
}

impl<ClientService> PolarClient<ClientService> {
    pub(crate) fn build_request(
        &self,
        method: http::Method,
        path: &str,
        body: Option<bytes::Bytes>,
        options: &RequestOptions,
    ) -> Result<http::Request<RequestBody>, BuildRequestError> {
        let mut base_url = self.config.base_url.clone();
        // Treat a custom base path as a directory when resolving endpoint
        // paths.
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        let request_url = base_url.join(path)?;

        let authorization_value = if let Some(access_token) = &options.access_token {
            Some(access_token.to_header().map_err(http::Error::from)?)
        } else {
            self.config.authorization.clone()
        };

        let mut request = http::Request::builder()
            .uri(request_url.as_str())
            .method(method)
            .header("Accept", "application/json")
            .header("Polar-Version", PolarVersion::V2026_10.version());

        if let Some(authorization_value) = authorization_value {
            request = request.header(http::header::AUTHORIZATION, authorization_value);
        }

        if body.is_some() {
            request = request.header(http::header::CONTENT_TYPE, "application/json");
        }

        request.body(RequestBody::new(body.unwrap_or_default())).map_err(BuildRequestError::from)
    }

    pub fn organizations(&self) -> Organizations<'_, ClientService> {
        Organizations::new(self)
    }

    pub(crate) fn default_timeout(&self) -> Duration {
        self.config.timeout
    }
}

// Custom Debug and Display to hide the value
#[derive(Clone, Deref, PartialEq, Eq, Deserialize, Serialize)]
pub struct AccessToken(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("access token can't be empty")]
pub struct AccessTokenValidationError;

impl AccessToken {
    pub fn new_or_panic(token: &str) -> Self {
        Self::try_new(token).unwrap()
    }

    pub fn try_new(token: &str) -> Result<Self, AccessTokenValidationError> {
        if token.is_empty() {
            return Err(AccessTokenValidationError);
        }

        Ok(Self(format!("Bearer {token}")))
    }

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

#[derive(Default, Clone)]
pub struct RequestOptions {
    /// Bearer credential for this operation. `None` inherits the client
    /// setting.
    pub access_token: Option<AccessToken>,
    pub timeout: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http::{Method, header};
    use http_body_util::BodyExt;
    use url::Url;

    use super::{AccessToken, PolarClient, PolarEnvironment, RequestOptions};
    use crate::error::BuildRequestError;

    #[test]
    fn access_token_formatting_is_redacted_but_header_preserves_credentials() {
        let token = AccessToken::try_from("example-secret").unwrap();
        assert_eq!(format!("{token}"), "Bearer [REDACTED]");
        assert_eq!(format!("{token:?}"), "Bearer [REDACTED]");
        assert_eq!(format!("{token:#?}"), "Bearer [REDACTED]");

        let header = token.to_header().unwrap();
        assert_eq!(header, "Bearer example-secret");
        assert!(header.is_sensitive());
        assert!(!format!("{header:?}").contains("example-secret"));
    }

    #[test]
    fn builder_rejects_invalid_authorization_from_an_access_token() {
        let token = AccessToken::try_from("invalid\ntoken").unwrap();
        assert!(PolarClient::builder(token).build_with(()).is_err());
    }

    fn client() -> PolarClient<()> {
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(())
            .unwrap()
    }

    #[test]
    fn custom_base_url_preserves_path_prefix_and_encoded_query() {
        for base in ["https://example.com/sdk%20proxy", "https://example.com/sdk%20proxy/"] {
            let client = PolarClient::builder(
                AccessToken::try_from("example-token").expect("valid test token"),
            )
            .base_url(Url::parse(base).unwrap())
            .environment(PolarEnvironment::Sandbox)
            .build_with(())
            .unwrap();
            let request = client
                .build_request(
                    Method::GET,
                    "v1/organizations/?slug=acme+%26+sons&sorting=-name&sorting=created_at",
                    None,
                    &RequestOptions::default(),
                )
                .unwrap();
            assert_eq!(request.uri().to_string(), "https://example.com/sdk%20proxy/v1/organizations/?slug=acme+%26+sons&sorting=-name&sorting=created_at");
        }
    }

    #[test]
    fn non_base_url_returns_a_url_build_error() {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .base_url(Url::parse("mailto:support@example.com").unwrap())
                .build_with(())
                .unwrap();
        let result = client.build_request(
            Method::GET,
            "v1/organizations/",
            None,
            &RequestOptions::default(),
        );
        assert!(matches!(result, Err(BuildRequestError::Url(_))));
    }

    #[tokio::test]
    async fn bodyless_get_omits_content_type_and_has_an_empty_body() {
        let request = client()
            .build_request(Method::GET, "v1/organizations/", None, &RequestOptions::default())
            .unwrap();

        assert_eq!(request.method(), Method::GET);
        assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/");
        assert_eq!(request.headers()[header::ACCEPT], "application/json");
        assert_eq!(request.headers()["Polar-Version"], "2026-10");
        assert!(!request.headers().contains_key(header::CONTENT_TYPE));
        assert!(request.into_body().collect().await.unwrap().to_bytes().is_empty());
    }

    #[tokio::test]
    async fn json_post_preserves_payload_bytes_and_sets_content_type() {
        let payload = Bytes::from_static(br#"{ "name": "Acme & Sons", "slug": "acme" }"#);
        let request = client()
            .build_request(
                Method::POST,
                "v1/organizations/",
                Some(payload.clone()),
                &RequestOptions::default(),
            )
            .unwrap();

        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/");
        assert_eq!(request.headers()[header::CONTENT_TYPE], "application/json");
        assert_eq!(request.headers()[header::ACCEPT], "application/json");
        assert_eq!(request.headers()["Polar-Version"], "2026-10");
        let body = request.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, payload, "the builder must not re-encode serialized JSON");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
            serde_json::json!({"name": "Acme & Sons", "slug": "acme"})
        );
    }

    #[test]
    fn json_request_token_override_is_sensitive_and_preserves_the_default() {
        let client = client();
        let request = client
            .build_request(
                Method::POST,
                "v1/organizations/",
                Some(Bytes::from_static(b"{}")),
                &RequestOptions {
                    access_token: AccessToken::try_from("override-token").ok(),
                    ..Default::default()
                },
            )
            .unwrap();

        assert_eq!(request.headers()[header::AUTHORIZATION], "Bearer override-token");
        assert!(request.headers()[header::AUTHORIZATION].is_sensitive());
        assert_eq!(request.headers()[header::CONTENT_TYPE], "application/json");

        let ordinary = client
            .build_request(Method::GET, "v1/organizations/", None, &RequestOptions::default())
            .unwrap();
        assert_eq!(ordinary.headers()[header::AUTHORIZATION], "Bearer example-token");
        assert!(ordinary.headers()[header::AUTHORIZATION].is_sensitive());
    }

    #[test]
    fn invalid_authorization_override_returns_an_http_build_error() {
        let result = client().build_request(
            Method::POST,
            "v1/organizations/",
            Some(Bytes::from_static(b"{}")),
            &RequestOptions {
                access_token: AccessToken::try_from("invalid\ntoken").ok(),
                ..Default::default()
            },
        );

        assert!(matches!(result, Err(BuildRequestError::Http(_))));
    }

    #[tokio::test]
    async fn explicit_empty_bytes_still_mark_a_body_as_present() {
        let request = client()
            .build_request(
                Method::POST,
                "v1/organizations/",
                Some(Bytes::new()),
                &RequestOptions::default(),
            )
            .unwrap();

        assert_eq!(request.headers()[header::CONTENT_TYPE], "application/json");
        assert!(request.into_body().collect().await.unwrap().to_bytes().is_empty());
    }
}
