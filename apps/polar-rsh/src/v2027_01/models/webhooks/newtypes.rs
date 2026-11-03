/// The URL where the webhook events will be sent.
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct WebhookEndpointUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WebhookEndpointUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl WebhookEndpointUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, WebhookEndpointUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for WebhookEndpointUrl {
    type Error = WebhookEndpointUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(WebhookEndpointUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(WebhookEndpointUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<WebhookEndpointUrl> for url::Url {
    fn from(value: WebhookEndpointUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for WebhookEndpointUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for WebhookEndpointUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
