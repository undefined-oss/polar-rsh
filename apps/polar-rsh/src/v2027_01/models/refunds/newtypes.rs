/// Amount to refund in cents. Minimum is 1.
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
pub struct RefundAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefundAmountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl RefundAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, RefundAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for RefundAmount {
    type Error = RefundAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(RefundAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<RefundAmount> for u64 {
    fn from(value: RefundAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for RefundAmount {
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
pub struct RefundMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefundMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl RefundMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, RefundMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for RefundMetadataKey {
    type Error = RefundMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(RefundMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(RefundMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<RefundMetadataKey> for String {
    fn from(value: RefundMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for RefundMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for RefundMetadataKey {
    type Error = RefundMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for RefundMetadataKey {
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
pub struct RefundMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefundMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl RefundMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, RefundMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for RefundMetadataValueString {
    type Error = RefundMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(RefundMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(RefundMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<RefundMetadataValueString> for String {
    fn from(value: RefundMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for RefundMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for RefundMetadataValueString {
    type Error = RefundMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for RefundMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
