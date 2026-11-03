/// The multiplier to convert from the base unit to display scale, e.g. 1000 to
/// display per 1000 units. Defaults to 1 when not provided.
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
pub struct MeterCustomMultiplier(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MeterCustomMultiplierValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl MeterCustomMultiplier {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, MeterCustomMultiplierValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for MeterCustomMultiplier {
    type Error = MeterCustomMultiplierValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(MeterCustomMultiplierValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<MeterCustomMultiplier> for u64 {
    fn from(value: MeterCustomMultiplier) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MeterCustomMultiplier {
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
pub struct MeterMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MeterMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl MeterMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, MeterMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MeterMetadataKey {
    type Error = MeterMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(MeterMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(MeterMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<MeterMetadataKey> for String {
    fn from(value: MeterMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MeterMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MeterMetadataKey {
    type Error = MeterMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MeterMetadataKey {
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
pub struct MeterMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MeterMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl MeterMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, MeterMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MeterMetadataValueString {
    type Error = MeterMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(MeterMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(MeterMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<MeterMetadataValueString> for String {
    fn from(value: MeterMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MeterMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MeterMetadataValueString {
    type Error = MeterMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MeterMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the meter. Will be shown on customer's invoices and usage.
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
pub struct MeterName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MeterNameValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
}
impl MeterName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, MeterNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MeterName {
    type Error = MeterNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(MeterNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<MeterName> for String {
    fn from(value: MeterName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MeterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MeterName {
    type Error = MeterNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MeterName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
