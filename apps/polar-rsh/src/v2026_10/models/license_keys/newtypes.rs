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
pub struct LicenseKeyActivateConditionsKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyActivateConditionsKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl LicenseKeyActivateConditionsKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, LicenseKeyActivateConditionsKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for LicenseKeyActivateConditionsKey {
    type Error = LicenseKeyActivateConditionsKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(LicenseKeyActivateConditionsKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(LicenseKeyActivateConditionsKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyActivateConditionsKey> for String {
    fn from(value: LicenseKeyActivateConditionsKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyActivateConditionsKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for LicenseKeyActivateConditionsKey {
    type Error = LicenseKeyActivateConditionsKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for LicenseKeyActivateConditionsKey {
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
pub struct LicenseKeyActivateConditionsValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyActivateConditionsValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl LicenseKeyActivateConditionsValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, LicenseKeyActivateConditionsValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for LicenseKeyActivateConditionsValueString {
    type Error = LicenseKeyActivateConditionsValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(LicenseKeyActivateConditionsValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(LicenseKeyActivateConditionsValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyActivateConditionsValueString> for String {
    fn from(value: LicenseKeyActivateConditionsValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyActivateConditionsValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for LicenseKeyActivateConditionsValueString {
    type Error = LicenseKeyActivateConditionsValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for LicenseKeyActivateConditionsValueString {
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
pub struct LicenseKeyActivateMetaKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyActivateMetaKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl LicenseKeyActivateMetaKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, LicenseKeyActivateMetaKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for LicenseKeyActivateMetaKey {
    type Error = LicenseKeyActivateMetaKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(LicenseKeyActivateMetaKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(LicenseKeyActivateMetaKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyActivateMetaKey> for String {
    fn from(value: LicenseKeyActivateMetaKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyActivateMetaKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for LicenseKeyActivateMetaKey {
    type Error = LicenseKeyActivateMetaKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for LicenseKeyActivateMetaKey {
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
pub struct LicenseKeyActivateMetaValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyActivateMetaValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl LicenseKeyActivateMetaValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, LicenseKeyActivateMetaValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for LicenseKeyActivateMetaValueString {
    type Error = LicenseKeyActivateMetaValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(LicenseKeyActivateMetaValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(LicenseKeyActivateMetaValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyActivateMetaValueString> for String {
    fn from(value: LicenseKeyActivateMetaValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyActivateMetaValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for LicenseKeyActivateMetaValueString {
    type Error = LicenseKeyActivateMetaValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for LicenseKeyActivateMetaValueString {
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
pub struct LicenseKeyUpdateLimitActivations(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyUpdateLimitActivationsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl LicenseKeyUpdateLimitActivations {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LicenseKeyUpdateLimitActivationsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LicenseKeyUpdateLimitActivations {
    type Error = LicenseKeyUpdateLimitActivationsValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(LicenseKeyUpdateLimitActivationsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(LicenseKeyUpdateLimitActivationsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyUpdateLimitActivations> for i64 {
    fn from(value: LicenseKeyUpdateLimitActivations) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyUpdateLimitActivations {
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
pub struct LicenseKeyUpdateLimitUsage(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyUpdateLimitUsageValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl LicenseKeyUpdateLimitUsage {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LicenseKeyUpdateLimitUsageValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LicenseKeyUpdateLimitUsage {
    type Error = LicenseKeyUpdateLimitUsageValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(LicenseKeyUpdateLimitUsageValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(LicenseKeyUpdateLimitUsageValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyUpdateLimitUsage> for i64 {
    fn from(value: LicenseKeyUpdateLimitUsage) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyUpdateLimitUsage {
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
pub struct LicenseKeyUpdateUsage(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyUpdateUsageValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl LicenseKeyUpdateUsage {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LicenseKeyUpdateUsageValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LicenseKeyUpdateUsage {
    type Error = LicenseKeyUpdateUsageValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(LicenseKeyUpdateUsageValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(LicenseKeyUpdateUsageValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyUpdateUsage> for i64 {
    fn from(value: LicenseKeyUpdateUsage) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyUpdateUsage {
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
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct LicenseKeyValidateConditionsKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyValidateConditionsKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl LicenseKeyValidateConditionsKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, LicenseKeyValidateConditionsKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for LicenseKeyValidateConditionsKey {
    type Error = LicenseKeyValidateConditionsKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(LicenseKeyValidateConditionsKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(LicenseKeyValidateConditionsKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyValidateConditionsKey> for String {
    fn from(value: LicenseKeyValidateConditionsKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyValidateConditionsKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for LicenseKeyValidateConditionsKey {
    type Error = LicenseKeyValidateConditionsKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for LicenseKeyValidateConditionsKey {
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
pub struct LicenseKeyValidateConditionsValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyValidateConditionsValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl LicenseKeyValidateConditionsValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, LicenseKeyValidateConditionsValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for LicenseKeyValidateConditionsValueString {
    type Error = LicenseKeyValidateConditionsValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(LicenseKeyValidateConditionsValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(LicenseKeyValidateConditionsValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyValidateConditionsValueString> for String {
    fn from(value: LicenseKeyValidateConditionsValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyValidateConditionsValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for LicenseKeyValidateConditionsValueString {
    type Error = LicenseKeyValidateConditionsValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for LicenseKeyValidateConditionsValueString {
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
pub struct LicenseKeyValidateIncrementUsage(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LicenseKeyValidateIncrementUsageValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl LicenseKeyValidateIncrementUsage {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, LicenseKeyValidateIncrementUsageValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for LicenseKeyValidateIncrementUsage {
    type Error = LicenseKeyValidateIncrementUsageValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(LicenseKeyValidateIncrementUsageValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<LicenseKeyValidateIncrementUsage> for u64 {
    fn from(value: LicenseKeyValidateIncrementUsage) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LicenseKeyValidateIncrementUsage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
