//! Typed Polar API client for API version 2027-01.
//!
//! Resource methods return an awaitable [`Request`]. Configure a single request
//! with [`Request::timeout`] or [`Request::access_token`] before awaiting it.
//! This client requires a Tokio runtime with time enabled.

pub mod generated;
mod request;
pub mod resources;
pub mod webhooks;

use std::time::Duration;

use generated::client::HttpClient;
pub use generated::types as models;
pub use presence_rs::Presence;
pub use request::{Request, RequestError, RequestOptions};

pub const API_VERSION: &str = "2027-01";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Environment {
    #[default]
    Production,
    Sandbox,
}

impl Environment {
    pub const fn base_url(self) -> &'static str {
        match self {
            Self::Production => "https://api.polar.sh",
            Self::Sandbox => "https://sandbox-api.polar.sh",
        }
    }
}

/// A reusable client. Clones share the underlying HTTP connection pool.
#[derive(Clone)]
pub struct Polar {
    pub(crate) client: HttpClient,
    pub(crate) timeout: Duration,
}

impl Polar {
    pub fn new(access_token: impl Into<String>) -> Result<Self, reqwest::Error> {
        Self::builder(access_token).build()
    }

    pub fn builder(access_token: impl Into<String>) -> PolarBuilder {
        PolarBuilder {
            access_token: access_token.into(),
            environment: Environment::Production,
            base_url: None,
            timeout: Duration::from_secs(5),
            organization_id: None,
            transport: None,
            tracing: true,
        }
    }

    /// Access the underlying generated client. Calls through this escape hatch
    /// do not apply the facade's request timeout or per-request options.
    pub fn raw(&self) -> &HttpClient {
        &self.client
    }
}

pub struct PolarBuilder {
    access_token: String,
    environment: Environment,
    base_url: Option<url::Url>,
    timeout: Duration,
    organization_id: Option<uuid::Uuid>,
    transport: Option<reqwest::Client>,
    tracing: bool,
}

impl PolarBuilder {
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Overrides the environment URL, for proxies and local mock servers.
    pub fn base_url(mut self, base_url: url::Url) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Sets the default deadline for a request, including reading its response.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn organization_id(mut self, organization_id: uuid::Uuid) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    /// Custom TLS, proxy, or transport configuration. A transport-level timeout
    /// still applies independently of the SDK deadline; leave it unset to allow
    /// individual requests to extend the SDK deadline.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.transport = Some(client);
        self
    }

    pub fn tracing(mut self, enabled: bool) -> Self {
        self.tracing = enabled;
        self
    }

    pub fn build(self) -> Result<Polar, reqwest::Error> {
        let transport = match self.transport {
            Some(client) => client,
            None => reqwest::Client::builder().build()?,
        };
        let base_url = self
            .base_url
            .as_ref()
            .map(url::Url::as_str)
            .unwrap_or_else(|| self.environment.base_url());
        let mut client = HttpClient::with_client_and_config(transport, self.tracing)
            .with_base_url(base_url.trim_end_matches('/'))
            .with_api_key(self.access_token)
            .with_header("Polar-Version", API_VERSION)
            .with_header("User-Agent", concat!("polar-rsh/", env!("CARGO_PKG_VERSION")));
        if let Some(organization_id) = self.organization_id {
            client = client.with_header("Polar-Organization", organization_id.to_string());
        }
        Ok(Polar { client, timeout: self.timeout })
    }
}
