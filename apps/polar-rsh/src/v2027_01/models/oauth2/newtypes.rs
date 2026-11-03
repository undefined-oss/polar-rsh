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
pub struct AuthorizationCodeTokenRequestRedirectUri(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AuthorizationCodeTokenRequestRedirectUriValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl AuthorizationCodeTokenRequestRedirectUri {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, AuthorizationCodeTokenRequestRedirectUriValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for AuthorizationCodeTokenRequestRedirectUri {
    type Error = AuthorizationCodeTokenRequestRedirectUriValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(AuthorizationCodeTokenRequestRedirectUriValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(AuthorizationCodeTokenRequestRedirectUriValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<AuthorizationCodeTokenRequestRedirectUri> for url::Url {
    fn from(value: AuthorizationCodeTokenRequestRedirectUri) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationCodeTokenRequestRedirectUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for AuthorizationCodeTokenRequestRedirectUri {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
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
pub struct OAuth2ClientConfigurationLogoUri(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OAuth2ClientConfigurationLogoUriValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl OAuth2ClientConfigurationLogoUri {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, OAuth2ClientConfigurationLogoUriValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OAuth2ClientConfigurationLogoUri {
    type Error = OAuth2ClientConfigurationLogoUriValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OAuth2ClientConfigurationLogoUriValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(OAuth2ClientConfigurationLogoUriValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OAuth2ClientConfigurationLogoUri> for url::Url {
    fn from(value: OAuth2ClientConfigurationLogoUri) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2ClientConfigurationLogoUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OAuth2ClientConfigurationLogoUri {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
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
pub struct OAuth2ClientConfigurationPolicyUri(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OAuth2ClientConfigurationPolicyUriValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl OAuth2ClientConfigurationPolicyUri {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, OAuth2ClientConfigurationPolicyUriValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OAuth2ClientConfigurationPolicyUri {
    type Error = OAuth2ClientConfigurationPolicyUriValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OAuth2ClientConfigurationPolicyUriValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(OAuth2ClientConfigurationPolicyUriValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OAuth2ClientConfigurationPolicyUri> for url::Url {
    fn from(value: OAuth2ClientConfigurationPolicyUri) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2ClientConfigurationPolicyUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OAuth2ClientConfigurationPolicyUri {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
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
pub struct OAuth2ClientConfigurationRedirectUrisItem(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OAuth2ClientConfigurationRedirectUrisItemValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl OAuth2ClientConfigurationRedirectUrisItem {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, OAuth2ClientConfigurationRedirectUrisItemValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OAuth2ClientConfigurationRedirectUrisItem {
    type Error = OAuth2ClientConfigurationRedirectUrisItemValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OAuth2ClientConfigurationRedirectUrisItemValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<OAuth2ClientConfigurationRedirectUrisItem> for url::Url {
    fn from(value: OAuth2ClientConfigurationRedirectUrisItem) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2ClientConfigurationRedirectUrisItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OAuth2ClientConfigurationRedirectUrisItem {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
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
pub struct OAuth2ClientConfigurationTosUri(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OAuth2ClientConfigurationTosUriValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl OAuth2ClientConfigurationTosUri {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, OAuth2ClientConfigurationTosUriValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OAuth2ClientConfigurationTosUri {
    type Error = OAuth2ClientConfigurationTosUriValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OAuth2ClientConfigurationTosUriValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(OAuth2ClientConfigurationTosUriValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OAuth2ClientConfigurationTosUri> for url::Url {
    fn from(value: OAuth2ClientConfigurationTosUri) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OAuth2ClientConfigurationTosUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OAuth2ClientConfigurationTosUri {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
