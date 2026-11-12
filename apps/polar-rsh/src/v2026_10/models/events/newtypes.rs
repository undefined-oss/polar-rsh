/// The name of the event.
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
pub struct EventCreateCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventCreateCustomerNameValidationError {
    #[error("must contain at most 128 characters")]
    TooLong,
}
impl EventCreateCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventCreateCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventCreateCustomerName {
    type Error = EventCreateCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 128u64 {
            return Err(EventCreateCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventCreateCustomerName> for String {
    fn from(value: EventCreateCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventCreateCustomerName {
    type Error = EventCreateCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventCreateCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the event.
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
pub struct EventCreateExternalCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventCreateExternalCustomerNameValidationError {
    #[error("must contain at most 128 characters")]
    TooLong,
}
impl EventCreateExternalCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventCreateExternalCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventCreateExternalCustomerName {
    type Error = EventCreateExternalCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 128u64 {
            return Err(EventCreateExternalCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventCreateExternalCustomerName> for String {
    fn from(value: EventCreateExternalCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateExternalCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventCreateExternalCustomerName {
    type Error = EventCreateExternalCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventCreateExternalCustomerName {
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
pub struct EventMetadataInputValueInteger(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventMetadataInputValueIntegerValidationError {
    #[error("must be at least -9007199254740990")]
    BelowMinimum,
    #[error("must be at most 9007199254740990")]
    AboveMaximum,
}
impl EventMetadataInputValueInteger {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, EventMetadataInputValueIntegerValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for EventMetadataInputValueInteger {
    type Error = EventMetadataInputValueIntegerValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -9007199254740990i128 {
            return Err(EventMetadataInputValueIntegerValidationError::BelowMinimum);
        }
        if i128::from(value) > 9007199254740990i128 {
            return Err(EventMetadataInputValueIntegerValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<EventMetadataInputValueInteger> for i64 {
    fn from(value: EventMetadataInputValueInteger) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventMetadataInputValueInteger {
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
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct EventMetadataInputValueNumber(f64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventMetadataInputValueNumberValidationError {
    #[error("must be finite")]
    NotFinite,
    #[error("must be at least -9007199254740990")]
    BelowMinimum,
    #[error("must be at most 9007199254740990")]
    AboveMaximum,
}
impl EventMetadataInputValueNumber {
    /// Validate a value.
    pub fn try_new(value: f64) -> Result<Self, EventMetadataInputValueNumberValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> f64 {
        self.0
    }
}
impl TryFrom<f64> for EventMetadataInputValueNumber {
    type Error = EventMetadataInputValueNumberValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(EventMetadataInputValueNumberValidationError::NotFinite);
        }
        if value < -9007199254740990f64 {
            return Err(EventMetadataInputValueNumberValidationError::BelowMinimum);
        }
        if value > 9007199254740990f64 {
            return Err(EventMetadataInputValueNumberValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<EventMetadataInputValueNumber> for f64 {
    fn from(value: EventMetadataInputValueNumber) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventMetadataInputValueNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <f64 as serde::Deserialize>::deserialize(deserializer)?;
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
pub struct EventMetadataInputValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventMetadataInputValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl EventMetadataInputValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventMetadataInputValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventMetadataInputValueString {
    type Error = EventMetadataInputValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(EventMetadataInputValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(EventMetadataInputValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventMetadataInputValueString> for String {
    fn from(value: EventMetadataInputValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventMetadataInputValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventMetadataInputValueString {
    type Error = EventMetadataInputValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventMetadataInputValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The number of LLM cached tokens that were used for the event.
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
pub struct LlmMetadataCachedInputTokens(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LlmMetadataCachedInputTokensValidationError {
    #[error("must be at least -9007199254740990")]
    BelowMinimum,
    #[error("must be at most 9007199254740990")]
    AboveMaximum,
}
impl LlmMetadataCachedInputTokens {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LlmMetadataCachedInputTokensValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LlmMetadataCachedInputTokens {
    type Error = LlmMetadataCachedInputTokensValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -9007199254740990i128 {
            return Err(LlmMetadataCachedInputTokensValidationError::BelowMinimum);
        }
        if i128::from(value) > 9007199254740990i128 {
            return Err(LlmMetadataCachedInputTokensValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LlmMetadataCachedInputTokens> for i64 {
    fn from(value: LlmMetadataCachedInputTokens) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LlmMetadataCachedInputTokens {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The number of LLM input tokens used for the event.
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
pub struct LlmMetadataInputTokens(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LlmMetadataInputTokensValidationError {
    #[error("must be at least -9007199254740990")]
    BelowMinimum,
    #[error("must be at most 9007199254740990")]
    AboveMaximum,
}
impl LlmMetadataInputTokens {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LlmMetadataInputTokensValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LlmMetadataInputTokens {
    type Error = LlmMetadataInputTokensValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -9007199254740990i128 {
            return Err(LlmMetadataInputTokensValidationError::BelowMinimum);
        }
        if i128::from(value) > 9007199254740990i128 {
            return Err(LlmMetadataInputTokensValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LlmMetadataInputTokens> for i64 {
    fn from(value: LlmMetadataInputTokens) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LlmMetadataInputTokens {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The number of LLM output tokens used for the event.
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
pub struct LlmMetadataOutputTokens(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LlmMetadataOutputTokensValidationError {
    #[error("must be at least -9007199254740990")]
    BelowMinimum,
    #[error("must be at most 9007199254740990")]
    AboveMaximum,
}
impl LlmMetadataOutputTokens {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LlmMetadataOutputTokensValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LlmMetadataOutputTokens {
    type Error = LlmMetadataOutputTokensValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -9007199254740990i128 {
            return Err(LlmMetadataOutputTokensValidationError::BelowMinimum);
        }
        if i128::from(value) > 9007199254740990i128 {
            return Err(LlmMetadataOutputTokensValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LlmMetadataOutputTokens> for i64 {
    fn from(value: LlmMetadataOutputTokens) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LlmMetadataOutputTokens {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The total number of LLM tokens used for the event.
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
pub struct LlmMetadataTotalTokens(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LlmMetadataTotalTokensValidationError {
    #[error("must be at least -9007199254740990")]
    BelowMinimum,
    #[error("must be at most 9007199254740990")]
    AboveMaximum,
}
impl LlmMetadataTotalTokens {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, LlmMetadataTotalTokensValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for LlmMetadataTotalTokens {
    type Error = LlmMetadataTotalTokensValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -9007199254740990i128 {
            return Err(LlmMetadataTotalTokensValidationError::BelowMinimum);
        }
        if i128::from(value) > 9007199254740990i128 {
            return Err(LlmMetadataTotalTokensValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<LlmMetadataTotalTokens> for i64 {
    fn from(value: LlmMetadataTotalTokens) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for LlmMetadataTotalTokens {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
