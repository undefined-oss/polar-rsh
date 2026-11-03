#[derive(Debug, thiserror::Error)]
pub(crate) enum RequestFailure {
    #[error("failed to build request: {0}")]
    BuildRequest(#[source] BuildRequestError),

    #[error("failed to read response body: {0}")]
    ReadBody(#[source] tower::BoxError),

    #[error("failed to decode response: {0}")]
    DecodeResponse(#[source] serde_json::Error),

    #[error("unexpected HTTP status: {status}")]
    UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },

    #[error("request timed out")]
    Timeout(#[source] tokio::time::error::Elapsed),
}

#[derive(Debug, thiserror::Error)]
pub enum BuildRequestError {
    #[error("failed to construct request URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("failed to construct HTTP request: {0}")]
    Http(#[from] http::Error),

    #[error("failed to serialize request body: {0}")]
    Serialize(#[from] serde_json::Error),
}
