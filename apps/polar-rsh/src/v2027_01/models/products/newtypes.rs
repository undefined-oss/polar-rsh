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
pub struct ProductCreateOneTimeMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateOneTimeMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl ProductCreateOneTimeMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductCreateOneTimeMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductCreateOneTimeMetadataKey {
    type Error = ProductCreateOneTimeMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(ProductCreateOneTimeMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(ProductCreateOneTimeMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateOneTimeMetadataKey> for String {
    fn from(value: ProductCreateOneTimeMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateOneTimeMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductCreateOneTimeMetadataKey {
    type Error = ProductCreateOneTimeMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductCreateOneTimeMetadataKey {
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
pub struct ProductCreateOneTimeMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateOneTimeMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl ProductCreateOneTimeMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductCreateOneTimeMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductCreateOneTimeMetadataValueString {
    type Error = ProductCreateOneTimeMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(ProductCreateOneTimeMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(ProductCreateOneTimeMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateOneTimeMetadataValueString> for String {
    fn from(value: ProductCreateOneTimeMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateOneTimeMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductCreateOneTimeMetadataValueString {
    type Error = ProductCreateOneTimeMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductCreateOneTimeMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the product.
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
pub struct ProductCreateOneTimeName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateOneTimeNameValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 64 characters")]
    TooLong,
}
impl ProductCreateOneTimeName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductCreateOneTimeNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductCreateOneTimeName {
    type Error = ProductCreateOneTimeNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(ProductCreateOneTimeNameValidationError::TooShort);
        }
        if length > 64u64 {
            return Err(ProductCreateOneTimeNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateOneTimeName> for String {
    fn from(value: ProductCreateOneTimeName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateOneTimeName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductCreateOneTimeName {
    type Error = ProductCreateOneTimeNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductCreateOneTimeName {
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
pub struct ProductCreateRecurringMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateRecurringMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl ProductCreateRecurringMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductCreateRecurringMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductCreateRecurringMetadataKey {
    type Error = ProductCreateRecurringMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(ProductCreateRecurringMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(ProductCreateRecurringMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateRecurringMetadataKey> for String {
    fn from(value: ProductCreateRecurringMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateRecurringMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductCreateRecurringMetadataKey {
    type Error = ProductCreateRecurringMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductCreateRecurringMetadataKey {
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
pub struct ProductCreateRecurringMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateRecurringMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl ProductCreateRecurringMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductCreateRecurringMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductCreateRecurringMetadataValueString {
    type Error = ProductCreateRecurringMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(ProductCreateRecurringMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(ProductCreateRecurringMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateRecurringMetadataValueString> for String {
    fn from(value: ProductCreateRecurringMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateRecurringMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductCreateRecurringMetadataValueString {
    type Error = ProductCreateRecurringMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductCreateRecurringMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Number of meter interval units. Defaults to 1 when `meter_interval` is set.
/// Ignored when `meter_interval` is `None`.
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
pub struct ProductCreateRecurringMeterIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateRecurringMeterIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 999")]
    AboveMaximum,
}
impl ProductCreateRecurringMeterIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, ProductCreateRecurringMeterIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductCreateRecurringMeterIntervalCount {
    type Error = ProductCreateRecurringMeterIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductCreateRecurringMeterIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 999i128 {
            return Err(ProductCreateRecurringMeterIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateRecurringMeterIntervalCount> for u64 {
    fn from(value: ProductCreateRecurringMeterIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateRecurringMeterIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The name of the product.
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
pub struct ProductCreateRecurringName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateRecurringNameValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 64 characters")]
    TooLong,
}
impl ProductCreateRecurringName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductCreateRecurringNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductCreateRecurringName {
    type Error = ProductCreateRecurringNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(ProductCreateRecurringNameValidationError::TooShort);
        }
        if length > 64u64 {
            return Err(ProductCreateRecurringNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateRecurringName> for String {
    fn from(value: ProductCreateRecurringName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateRecurringName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductCreateRecurringName {
    type Error = ProductCreateRecurringNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductCreateRecurringName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Number of interval units of the subscription. If this is set to 1 the charge
/// will happen every interval (e.g. every month), if set to 2 it will be every
/// other month, and so on.
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
pub struct ProductCreateRecurringRecurringIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateRecurringRecurringIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 999")]
    AboveMaximum,
}
impl ProductCreateRecurringRecurringIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, ProductCreateRecurringRecurringIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductCreateRecurringRecurringIntervalCount {
    type Error = ProductCreateRecurringRecurringIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductCreateRecurringRecurringIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 999i128 {
            return Err(ProductCreateRecurringRecurringIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateRecurringRecurringIntervalCount> for u64 {
    fn from(value: ProductCreateRecurringRecurringIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateRecurringRecurringIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The number of interval units for the trial period.
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
pub struct ProductCreateRecurringTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductCreateRecurringTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl ProductCreateRecurringTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, ProductCreateRecurringTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductCreateRecurringTrialIntervalCount {
    type Error = ProductCreateRecurringTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductCreateRecurringTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(ProductCreateRecurringTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductCreateRecurringTrialIntervalCount> for u64 {
    fn from(value: ProductCreateRecurringTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductCreateRecurringTrialIntervalCount {
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
pub struct ProductMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl ProductMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, ProductMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductMetadataKey {
    type Error = ProductMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(ProductMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(ProductMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductMetadataKey> for String {
    fn from(value: ProductMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductMetadataKey {
    type Error = ProductMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductMetadataKey {
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
pub struct ProductMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl ProductMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, ProductMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductMetadataValueString {
    type Error = ProductMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(ProductMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(ProductMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductMetadataValueString> for String {
    fn from(value: ProductMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductMetadataValueString {
    type Error = ProductMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the product.
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
pub struct ProductName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductNameValidationError {
    #[error("must contain at least 3 characters")]
    TooShort,
    #[error("must contain at most 64 characters")]
    TooLong,
}
impl ProductName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, ProductNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for ProductName {
    type Error = ProductNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 3u64 {
            return Err(ProductNameValidationError::TooShort);
        }
        if length > 64u64 {
            return Err(ProductNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<ProductName> for String {
    fn from(value: ProductName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for ProductName {
    type Error = ProductNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for ProductName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Number of interval units of the subscription. If this is set to 1 the charge
/// will happen every interval (e.g. every month), if set to 2 it will be every
/// other month, and so on. Once set, it can't be changed.**
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
pub struct ProductRecurringIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductRecurringIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 999")]
    AboveMaximum,
}
impl ProductRecurringIntervalCount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductRecurringIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductRecurringIntervalCount {
    type Error = ProductRecurringIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductRecurringIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 999i128 {
            return Err(ProductRecurringIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductRecurringIntervalCount> for u64 {
    fn from(value: ProductRecurringIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductRecurringIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The number of interval units for the trial period.
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
pub struct ProductTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl ProductTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductTrialIntervalCount {
    type Error = ProductTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(ProductTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductTrialIntervalCount> for u64 {
    fn from(value: ProductTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductTrialIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
