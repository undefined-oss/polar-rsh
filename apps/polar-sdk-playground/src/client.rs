use std::time::Duration;

use tower::ServiceExt;

use crate::{error::BuildRequestError, models::organization::Organizations};

#[derive(Debug, PartialEq, Eq, Default)]
pub enum PolarEnvironment {
    #[default]
    Production,
    Sandbox,
}

impl PolarEnvironment {
    fn base_url(&self) -> &'static str {
        match self {
            PolarEnvironment::Production => "https://api.polar.sh",
            PolarEnvironment::Sandbox => "https://sandbox-api.polar.sh",
        }
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
    access_token: String, // NewType ??
    environment: PolarEnvironment,
    timeout: Duration,
    base_url: String,
    base_url_overridden: bool,
}

impl PolarBuilder {
    pub fn new(access_token: impl Into<String>) -> Self {
        let default_environment = PolarEnvironment::Production;
        Self {
            access_token: access_token.into(),
            base_url: default_environment.base_url().into(),
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

        self.base_url = self.environment.base_url().into();
        self
    }

    // String should be URL type and as a newtype
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self.base_url_overridden = true;
        self
    }

    // Custom error;
    pub fn build_with<S>(
        self,
        transport: S,
    ) -> Result<PolarClient<S>, http::header::InvalidHeaderValue> {
        let bearer_token = format!("Bearer {}", self.access_token);
        let mut authorization_header = http::HeaderValue::from_str(&bearer_token)?;
        authorization_header.set_sensitive(true);

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
    authorization: http::HeaderValue,
    base_url: String,
    timeout: Duration,
}

pub struct PolarClient<S = ()> {
    // Agnostict http client that implements http and tower crate.
    transport: S,
    config: ClientConfig,
}

impl PolarClient<()> {
    pub fn builder(access_token: impl Into<String>) -> PolarBuilder {
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
        // Deferred trimming
        let request_url = format!("{}/{}", self.config.base_url, path);

        let authorization_value = if let Some(access_token) = &options.access_token {
            let bearer_token = format!("Bearer {}", access_token);
            let mut authorization_header =
                http::HeaderValue::from_str(&bearer_token).map_err(http::Error::from)?;
            authorization_header.set_sensitive(true);
            authorization_header
        } else {
            self.config.authorization.clone()
        };

        let mut request = http::Request::builder()
            .uri(request_url)
            .method(method)
            .header("Authorization", authorization_value)
            .header("Accept", "application/json")
            .header("Polar-Version", PolarVersion::V2026_10.version());

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

#[derive(Default, Clone)]
pub struct RequestOptions {
    pub access_token: Option<String>,
    pub timeout: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http::{Method, header};
    use http_body_util::BodyExt;

    use super::{PolarClient, PolarEnvironment, RequestOptions};
    use crate::error::BuildRequestError;

    fn client() -> PolarClient<()> {
        PolarClient::builder("example-token")
            .environment(PolarEnvironment::Sandbox)
            .build_with(())
            .unwrap()
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
                    access_token: Some("override-token".into()),
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
                access_token: Some("invalid\ntoken".into()),
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
