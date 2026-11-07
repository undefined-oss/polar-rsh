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
pub struct OrganizationAvatarUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationAvatarUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl OrganizationAvatarUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, OrganizationAvatarUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OrganizationAvatarUrl {
    type Error = OrganizationAvatarUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OrganizationAvatarUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(OrganizationAvatarUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrganizationAvatarUrl> for url::Url {
    fn from(value: OrganizationAvatarUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationAvatarUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OrganizationAvatarUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Concede disputes below this amount, in USD cents, without asking the
/// organization. A dispute charged in another currency converts at the rate its
/// payment settled at. `null` turns it off. The disputed amount and the
/// processor's dispute fee are still deducted.
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Copy,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct OrganizationDisputeSettingsAutoAcceptBelowAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationDisputeSettingsAutoAcceptBelowAmountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl OrganizationDisputeSettingsAutoAcceptBelowAmount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, OrganizationDisputeSettingsAutoAcceptBelowAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrganizationDisputeSettingsAutoAcceptBelowAmount {
    type Error = OrganizationDisputeSettingsAutoAcceptBelowAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(
                OrganizationDisputeSettingsAutoAcceptBelowAmountValidationError::BelowMinimum,
            );
        }
        if i128::from(value) > 10000i128 {
            return Err(
                OrganizationDisputeSettingsAutoAcceptBelowAmountValidationError::AboveMaximum,
            );
        }
        Ok(Self(value))
    }
}
impl From<OrganizationDisputeSettingsAutoAcceptBelowAmount> for u64 {
    fn from(value: OrganizationDisputeSettingsAutoAcceptBelowAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationDisputeSettingsAutoAcceptBelowAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
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
pub struct OrganizationName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationNameValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
}
impl OrganizationName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, OrganizationNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrganizationName {
    type Error = OrganizationNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(OrganizationNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<OrganizationName> for String {
    fn from(value: OrganizationName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrganizationName {
    type Error = OrganizationNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrganizationName {
    fn as_ref(&self) -> &str {
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
pub struct OrganizationSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationSlugValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 64 characters")]
    TooLong,
}
impl OrganizationSlug {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, OrganizationSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrganizationSlug {
    type Error = OrganizationSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(OrganizationSlugValidationError::TooShort);
        }
        if length > 64u64 {
            return Err(OrganizationSlugValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrganizationSlug> for String {
    fn from(value: OrganizationSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrganizationSlug {
    type Error = OrganizationSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrganizationSlug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The URL to the organization profile
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
pub struct OrganizationSocialLinkUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationSocialLinkUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl OrganizationSocialLinkUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, OrganizationSocialLinkUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OrganizationSocialLinkUrl {
    type Error = OrganizationSocialLinkUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OrganizationSocialLinkUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(OrganizationSocialLinkUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrganizationSocialLinkUrl> for url::Url {
    fn from(value: OrganizationSocialLinkUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationSocialLinkUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OrganizationSocialLinkUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Official website of the organization.
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
pub struct OrganizationWebsite(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationWebsiteValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl OrganizationWebsite {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, OrganizationWebsiteValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for OrganizationWebsite {
    type Error = OrganizationWebsiteValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(OrganizationWebsiteValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(OrganizationWebsiteValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrganizationWebsite> for url::Url {
    fn from(value: OrganizationWebsite) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationWebsite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for OrganizationWebsite {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
