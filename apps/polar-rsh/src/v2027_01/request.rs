use std::{
    fmt::Debug,
    future::{Future, IntoFuture},
    time::Duration,
};

use futures_util::future::BoxFuture;

use super::{
    Polar,
    generated::client::{ApiOpError, HttpClient},
};

/// Options apply only to the request on which they are set.
#[derive(Clone, Default)]
pub struct RequestOptions {
    pub access_token: Option<String>,
    pub timeout: Option<Duration>,
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError<E: Debug> {
    #[error(transparent)]
    Api(#[from] ApiOpError<E>),
    #[error("Polar request exceeded its {0:?} deadline")]
    Timeout(Duration),
}

type Execute<T, E> =
    Box<dyn FnOnce(HttpClient) -> BoxFuture<'static, Result<T, ApiOpError<E>>> + Send>;

/// A lazy, owned request. No HTTP call is made until `.await` or
/// `.send().await`.
#[must_use = "requests do nothing until awaited"]
pub struct Request<T, E: Debug> {
    client: HttpClient,
    timeout: Duration,
    execute: Execute<T, E>,
}

impl<T: Send + 'static, E: Debug + Send + 'static> Request<T, E> {
    pub(crate) fn new<F, Fut>(polar: &Polar, execute: F) -> Self
    where
        F: FnOnce(HttpClient) -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, ApiOpError<E>>> + Send + 'static,
    {
        Self {
            client: polar.client.clone(),
            timeout: polar.timeout,
            execute: Box::new(move |client| Box::pin(execute(client))),
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn access_token(mut self, token: impl Into<String>) -> Self {
        self.client = self.client.with_api_key(token);
        self
    }

    pub fn options(mut self, options: RequestOptions) -> Self {
        if let Some(timeout) = options.timeout {
            self = self.timeout(timeout);
        }
        if let Some(token) = options.access_token {
            self = self.access_token(token);
        }
        self
    }

    // Keep the generated typed API error directly accessible to callers.
    #[allow(clippy::result_large_err)]
    pub async fn send(self) -> Result<T, RequestError<E>> {
        tokio::time::timeout(self.timeout, (self.execute)(self.client))
            .await
            .map_err(|_| RequestError::Timeout(self.timeout))?
            .map_err(RequestError::Api)
    }
}

impl<T: Send + 'static, E: Debug + Send + 'static> IntoFuture for Request<T, E> {
    type IntoFuture = BoxFuture<'static, Self::Output>;
    type Output = Result<T, RequestError<E>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
