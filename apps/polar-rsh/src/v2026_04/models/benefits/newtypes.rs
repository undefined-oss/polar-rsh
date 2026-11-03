/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitCustomDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitCustomDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitCustomDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitCustomDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitCustomDescription {
    type Error = BenefitCustomDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitCustomDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitCustomDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitCustomDescription> for String {
    fn from(value: BenefitCustomDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitCustomDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitCustomDescription {
    type Error = BenefitCustomDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitCustomDescription {
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
pub struct BenefitCustomMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitCustomMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitCustomMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitCustomMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitCustomMetadataKey {
    type Error = BenefitCustomMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitCustomMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitCustomMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitCustomMetadataKey> for String {
    fn from(value: BenefitCustomMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitCustomMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitCustomMetadataKey {
    type Error = BenefitCustomMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitCustomMetadataKey {
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
pub struct BenefitCustomMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitCustomMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitCustomMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitCustomMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitCustomMetadataValueString {
    type Error = BenefitCustomMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitCustomMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitCustomMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitCustomMetadataValueString> for String {
    fn from(value: BenefitCustomMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitCustomMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitCustomMetadataValueString {
    type Error = BenefitCustomMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitCustomMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitDiscordDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitDiscordDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitDiscordDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitDiscordDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitDiscordDescription {
    type Error = BenefitDiscordDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitDiscordDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitDiscordDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitDiscordDescription> for String {
    fn from(value: BenefitDiscordDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitDiscordDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitDiscordDescription {
    type Error = BenefitDiscordDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitDiscordDescription {
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
pub struct BenefitDiscordMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitDiscordMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitDiscordMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitDiscordMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitDiscordMetadataKey {
    type Error = BenefitDiscordMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitDiscordMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitDiscordMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitDiscordMetadataKey> for String {
    fn from(value: BenefitDiscordMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitDiscordMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitDiscordMetadataKey {
    type Error = BenefitDiscordMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitDiscordMetadataKey {
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
pub struct BenefitDiscordMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitDiscordMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitDiscordMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitDiscordMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitDiscordMetadataValueString {
    type Error = BenefitDiscordMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitDiscordMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitDiscordMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitDiscordMetadataValueString> for String {
    fn from(value: BenefitDiscordMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitDiscordMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitDiscordMetadataValueString {
    type Error = BenefitDiscordMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitDiscordMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitDownloadablesDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitDownloadablesDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitDownloadablesDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitDownloadablesDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitDownloadablesDescription {
    type Error = BenefitDownloadablesDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitDownloadablesDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitDownloadablesDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitDownloadablesDescription> for String {
    fn from(value: BenefitDownloadablesDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitDownloadablesDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitDownloadablesDescription {
    type Error = BenefitDownloadablesDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitDownloadablesDescription {
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
pub struct BenefitDownloadablesMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitDownloadablesMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitDownloadablesMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitDownloadablesMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitDownloadablesMetadataKey {
    type Error = BenefitDownloadablesMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitDownloadablesMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitDownloadablesMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitDownloadablesMetadataKey> for String {
    fn from(value: BenefitDownloadablesMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitDownloadablesMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitDownloadablesMetadataKey {
    type Error = BenefitDownloadablesMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitDownloadablesMetadataKey {
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
pub struct BenefitDownloadablesMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitDownloadablesMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitDownloadablesMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitDownloadablesMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitDownloadablesMetadataValueString {
    type Error = BenefitDownloadablesMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitDownloadablesMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitDownloadablesMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitDownloadablesMetadataValueString> for String {
    fn from(value: BenefitDownloadablesMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitDownloadablesMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitDownloadablesMetadataValueString {
    type Error = BenefitDownloadablesMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitDownloadablesMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitFeatureFlagDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitFeatureFlagDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitFeatureFlagDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitFeatureFlagDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitFeatureFlagDescription {
    type Error = BenefitFeatureFlagDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitFeatureFlagDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitFeatureFlagDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitFeatureFlagDescription> for String {
    fn from(value: BenefitFeatureFlagDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitFeatureFlagDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitFeatureFlagDescription {
    type Error = BenefitFeatureFlagDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitFeatureFlagDescription {
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
pub struct BenefitFeatureFlagMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitFeatureFlagMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitFeatureFlagMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitFeatureFlagMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitFeatureFlagMetadataKey {
    type Error = BenefitFeatureFlagMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitFeatureFlagMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitFeatureFlagMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitFeatureFlagMetadataKey> for String {
    fn from(value: BenefitFeatureFlagMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitFeatureFlagMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitFeatureFlagMetadataKey {
    type Error = BenefitFeatureFlagMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitFeatureFlagMetadataKey {
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
pub struct BenefitFeatureFlagMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitFeatureFlagMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitFeatureFlagMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitFeatureFlagMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitFeatureFlagMetadataValueString {
    type Error = BenefitFeatureFlagMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitFeatureFlagMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitFeatureFlagMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitFeatureFlagMetadataValueString> for String {
    fn from(value: BenefitFeatureFlagMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitFeatureFlagMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitFeatureFlagMetadataValueString {
    type Error = BenefitFeatureFlagMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitFeatureFlagMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitGitHubRepositoryDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitGitHubRepositoryDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitGitHubRepositoryDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitGitHubRepositoryDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitGitHubRepositoryDescription {
    type Error = BenefitGitHubRepositoryDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitGitHubRepositoryDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitGitHubRepositoryDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitGitHubRepositoryDescription> for String {
    fn from(value: BenefitGitHubRepositoryDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitGitHubRepositoryDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitGitHubRepositoryDescription {
    type Error = BenefitGitHubRepositoryDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitGitHubRepositoryDescription {
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
pub struct BenefitGitHubRepositoryMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitGitHubRepositoryMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitGitHubRepositoryMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitGitHubRepositoryMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitGitHubRepositoryMetadataKey {
    type Error = BenefitGitHubRepositoryMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitGitHubRepositoryMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitGitHubRepositoryMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitGitHubRepositoryMetadataKey> for String {
    fn from(value: BenefitGitHubRepositoryMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitGitHubRepositoryMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitGitHubRepositoryMetadataKey {
    type Error = BenefitGitHubRepositoryMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitGitHubRepositoryMetadataKey {
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
pub struct BenefitGitHubRepositoryMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitGitHubRepositoryMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitGitHubRepositoryMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitGitHubRepositoryMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitGitHubRepositoryMetadataValueString {
    type Error = BenefitGitHubRepositoryMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitGitHubRepositoryMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitGitHubRepositoryMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitGitHubRepositoryMetadataValueString> for String {
    fn from(value: BenefitGitHubRepositoryMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitGitHubRepositoryMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitGitHubRepositoryMetadataValueString {
    type Error = BenefitGitHubRepositoryMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitGitHubRepositoryMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
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
pub struct BenefitLicenseKeyActivationCreatePropertiesLimit(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitLicenseKeyActivationCreatePropertiesLimitValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl BenefitLicenseKeyActivationCreatePropertiesLimit {
    /// Validate a value.
    pub fn try_new(
        value: i64,
    ) -> Result<Self, BenefitLicenseKeyActivationCreatePropertiesLimitValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for BenefitLicenseKeyActivationCreatePropertiesLimit {
    type Error = BenefitLicenseKeyActivationCreatePropertiesLimitValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(
                BenefitLicenseKeyActivationCreatePropertiesLimitValidationError::BelowMinimum,
            );
        }
        if i128::from(value) > 2147483647i128 {
            return Err(
                BenefitLicenseKeyActivationCreatePropertiesLimitValidationError::AboveMaximum,
            );
        }
        Ok(Self(value))
    }
}
impl From<BenefitLicenseKeyActivationCreatePropertiesLimit> for i64 {
    fn from(value: BenefitLicenseKeyActivationCreatePropertiesLimit) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitLicenseKeyActivationCreatePropertiesLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
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
pub struct BenefitLicenseKeysCreatePropertiesLimitUsage(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitLicenseKeysCreatePropertiesLimitUsageValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl BenefitLicenseKeysCreatePropertiesLimitUsage {
    /// Validate a value.
    pub fn try_new(
        value: i64,
    ) -> Result<Self, BenefitLicenseKeysCreatePropertiesLimitUsageValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for BenefitLicenseKeysCreatePropertiesLimitUsage {
    type Error = BenefitLicenseKeysCreatePropertiesLimitUsageValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(BenefitLicenseKeysCreatePropertiesLimitUsageValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(BenefitLicenseKeysCreatePropertiesLimitUsageValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<BenefitLicenseKeysCreatePropertiesLimitUsage> for i64 {
    fn from(value: BenefitLicenseKeysCreatePropertiesLimitUsage) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitLicenseKeysCreatePropertiesLimitUsage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitLicenseKeysDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitLicenseKeysDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitLicenseKeysDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitLicenseKeysDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitLicenseKeysDescription {
    type Error = BenefitLicenseKeysDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitLicenseKeysDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitLicenseKeysDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitLicenseKeysDescription> for String {
    fn from(value: BenefitLicenseKeysDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitLicenseKeysDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitLicenseKeysDescription {
    type Error = BenefitLicenseKeysDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitLicenseKeysDescription {
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
pub struct BenefitLicenseKeysMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitLicenseKeysMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitLicenseKeysMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitLicenseKeysMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitLicenseKeysMetadataKey {
    type Error = BenefitLicenseKeysMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitLicenseKeysMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitLicenseKeysMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitLicenseKeysMetadataKey> for String {
    fn from(value: BenefitLicenseKeysMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitLicenseKeysMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitLicenseKeysMetadataKey {
    type Error = BenefitLicenseKeysMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitLicenseKeysMetadataKey {
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
pub struct BenefitLicenseKeysMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitLicenseKeysMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitLicenseKeysMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitLicenseKeysMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitLicenseKeysMetadataValueString {
    type Error = BenefitLicenseKeysMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitLicenseKeysMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitLicenseKeysMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitLicenseKeysMetadataValueString> for String {
    fn from(value: BenefitLicenseKeysMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitLicenseKeysMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitLicenseKeysMetadataValueString {
    type Error = BenefitLicenseKeysMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitLicenseKeysMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
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
pub struct BenefitMeterCreditCreatePropertiesUnits(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitMeterCreditCreatePropertiesUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl BenefitMeterCreditCreatePropertiesUnits {
    /// Validate a value.
    pub fn try_new(
        value: i64,
    ) -> Result<Self, BenefitMeterCreditCreatePropertiesUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for BenefitMeterCreditCreatePropertiesUnits {
    type Error = BenefitMeterCreditCreatePropertiesUnitsValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(BenefitMeterCreditCreatePropertiesUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(BenefitMeterCreditCreatePropertiesUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<BenefitMeterCreditCreatePropertiesUnits> for i64 {
    fn from(value: BenefitMeterCreditCreatePropertiesUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitMeterCreditCreatePropertiesUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitMeterCreditDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitMeterCreditDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitMeterCreditDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitMeterCreditDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitMeterCreditDescription {
    type Error = BenefitMeterCreditDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitMeterCreditDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitMeterCreditDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitMeterCreditDescription> for String {
    fn from(value: BenefitMeterCreditDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitMeterCreditDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitMeterCreditDescription {
    type Error = BenefitMeterCreditDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitMeterCreditDescription {
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
pub struct BenefitMeterCreditMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitMeterCreditMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitMeterCreditMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitMeterCreditMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitMeterCreditMetadataKey {
    type Error = BenefitMeterCreditMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitMeterCreditMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitMeterCreditMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitMeterCreditMetadataKey> for String {
    fn from(value: BenefitMeterCreditMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitMeterCreditMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitMeterCreditMetadataKey {
    type Error = BenefitMeterCreditMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitMeterCreditMetadataKey {
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
pub struct BenefitMeterCreditMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitMeterCreditMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitMeterCreditMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitMeterCreditMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitMeterCreditMetadataValueString {
    type Error = BenefitMeterCreditMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitMeterCreditMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitMeterCreditMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitMeterCreditMetadataValueString> for String {
    fn from(value: BenefitMeterCreditMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitMeterCreditMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitMeterCreditMetadataValueString {
    type Error = BenefitMeterCreditMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitMeterCreditMetadataValueString {
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
pub struct BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelCreatePropertiesChannelNameTemplateValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 80 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelCreatePropertiesChannelNameTemplateValidationError>
    {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate {
    type Error = BenefitSlackSharedChannelCreatePropertiesChannelNameTemplateValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(
                BenefitSlackSharedChannelCreatePropertiesChannelNameTemplateValidationError::TooShort,
            );
        }
        if length > 80u64 {
            return Err(
                BenefitSlackSharedChannelCreatePropertiesChannelNameTemplateValidationError::TooLong,
            );
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate> for String {
    fn from(value: BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate {
    type Error = BenefitSlackSharedChannelCreatePropertiesChannelNameTemplateValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate {
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
pub struct BenefitSlackSharedChannelCreatePropertiesWelcomeMessage(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelCreatePropertiesWelcomeMessageValidationError {
    #[error("must contain at most 4000 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelCreatePropertiesWelcomeMessage {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelCreatePropertiesWelcomeMessageValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelCreatePropertiesWelcomeMessage {
    type Error = BenefitSlackSharedChannelCreatePropertiesWelcomeMessageValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 4000u64 {
            return Err(
                BenefitSlackSharedChannelCreatePropertiesWelcomeMessageValidationError::TooLong,
            );
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelCreatePropertiesWelcomeMessage> for String {
    fn from(value: BenefitSlackSharedChannelCreatePropertiesWelcomeMessage) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelCreatePropertiesWelcomeMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelCreatePropertiesWelcomeMessage {
    type Error = BenefitSlackSharedChannelCreatePropertiesWelcomeMessageValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelCreatePropertiesWelcomeMessage {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The description of the benefit. Will be displayed on products having this
/// benefit.
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
pub struct BenefitSlackSharedChannelDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelDescriptionValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 42 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelDescription {
    type Error = BenefitSlackSharedChannelDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(BenefitSlackSharedChannelDescriptionValidationError::TooShort);
        }
        if length > 42u64 {
            return Err(BenefitSlackSharedChannelDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelDescription> for String {
    fn from(value: BenefitSlackSharedChannelDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelDescription {
    type Error = BenefitSlackSharedChannelDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelDescription {
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
pub struct BenefitSlackSharedChannelMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelMetadataKey {
    type Error = BenefitSlackSharedChannelMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitSlackSharedChannelMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(BenefitSlackSharedChannelMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelMetadataKey> for String {
    fn from(value: BenefitSlackSharedChannelMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelMetadataKey {
    type Error = BenefitSlackSharedChannelMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelMetadataKey {
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
pub struct BenefitSlackSharedChannelMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelMetadataValueString {
    type Error = BenefitSlackSharedChannelMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitSlackSharedChannelMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(BenefitSlackSharedChannelMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelMetadataValueString> for String {
    fn from(value: BenefitSlackSharedChannelMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelMetadataValueString {
    type Error = BenefitSlackSharedChannelMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
