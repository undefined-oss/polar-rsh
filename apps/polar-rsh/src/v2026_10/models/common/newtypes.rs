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
pub struct AggregateFieldValue(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AggregateFieldValueValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 255 characters")]
    TooLong,
}
impl AggregateFieldValue {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, AggregateFieldValueValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for AggregateFieldValue {
    type Error = AggregateFieldValueValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(AggregateFieldValueValidationError::TooShort);
        }
        if length > 255u64 {
            return Err(AggregateFieldValueValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<AggregateFieldValue> for String {
    fn from(value: AggregateFieldValue) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for AggregateFieldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for AggregateFieldValue {
    type Error = AggregateFieldValueValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for AggregateFieldValue {
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
pub struct BenefitLicenseKeyExpirationPropertiesTtl(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitLicenseKeyExpirationPropertiesTtlValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl BenefitLicenseKeyExpirationPropertiesTtl {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, BenefitLicenseKeyExpirationPropertiesTtlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for BenefitLicenseKeyExpirationPropertiesTtl {
    type Error = BenefitLicenseKeyExpirationPropertiesTtlValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(BenefitLicenseKeyExpirationPropertiesTtlValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<BenefitLicenseKeyExpirationPropertiesTtl> for u64 {
    fn from(value: BenefitLicenseKeyExpirationPropertiesTtl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitLicenseKeyExpirationPropertiesTtl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Template for the channel name. Supports placeholders: {customer_name},
/// {customer_email_local}, and {metadata.<key>} for any value stored in
/// customer user metadata.
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
pub struct BenefitSlackSharedChannelPropertiesChannelNameTemplate(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelPropertiesChannelNameTemplateValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 80 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelPropertiesChannelNameTemplate {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelPropertiesChannelNameTemplateValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelPropertiesChannelNameTemplate {
    type Error = BenefitSlackSharedChannelPropertiesChannelNameTemplateValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(
                BenefitSlackSharedChannelPropertiesChannelNameTemplateValidationError::TooShort,
            );
        }
        if length > 80u64 {
            return Err(
                BenefitSlackSharedChannelPropertiesChannelNameTemplateValidationError::TooLong,
            );
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelPropertiesChannelNameTemplate> for String {
    fn from(value: BenefitSlackSharedChannelPropertiesChannelNameTemplate) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelPropertiesChannelNameTemplate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelPropertiesChannelNameTemplate {
    type Error = BenefitSlackSharedChannelPropertiesChannelNameTemplateValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelPropertiesChannelNameTemplate {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Optional message posted to the channel right after creation.
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
pub struct BenefitSlackSharedChannelPropertiesWelcomeMessage(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitSlackSharedChannelPropertiesWelcomeMessageValidationError {
    #[error("must contain at most 4000 characters")]
    TooLong,
}
impl BenefitSlackSharedChannelPropertiesWelcomeMessage {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitSlackSharedChannelPropertiesWelcomeMessageValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitSlackSharedChannelPropertiesWelcomeMessage {
    type Error = BenefitSlackSharedChannelPropertiesWelcomeMessageValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 4000u64 {
            return Err(BenefitSlackSharedChannelPropertiesWelcomeMessageValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<BenefitSlackSharedChannelPropertiesWelcomeMessage> for String {
    fn from(value: BenefitSlackSharedChannelPropertiesWelcomeMessage) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitSlackSharedChannelPropertiesWelcomeMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitSlackSharedChannelPropertiesWelcomeMessage {
    type Error = BenefitSlackSharedChannelPropertiesWelcomeMessageValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitSlackSharedChannelPropertiesWelcomeMessage {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Amount in cents, before discounts and taxes. Only useful for custom prices,
/// it'll be ignored for fixed and free prices.
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
pub struct CheckoutPriceCreateAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutPriceCreateAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateAmount {
    type Error = CheckoutPriceCreateAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutPriceCreateAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateAmount> for u64 {
    fn from(value: CheckoutPriceCreateAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateAmount {
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
pub struct CheckoutPriceCreateCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutPriceCreateCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutPriceCreateCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutPriceCreateCustomerMetadataKey {
    type Error = CheckoutPriceCreateCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutPriceCreateCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutPriceCreateCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateCustomerMetadataKey> for String {
    fn from(value: CheckoutPriceCreateCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutPriceCreateCustomerMetadataKey {
    type Error = CheckoutPriceCreateCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutPriceCreateCustomerMetadataKey {
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
pub struct CheckoutPriceCreateCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutPriceCreateCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutPriceCreateCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutPriceCreateCustomerMetadataValueString {
    type Error = CheckoutPriceCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutPriceCreateCustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutPriceCreateCustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateCustomerMetadataValueString> for String {
    fn from(value: CheckoutPriceCreateCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutPriceCreateCustomerMetadataValueString {
    type Error = CheckoutPriceCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutPriceCreateCustomerMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the customer.
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
pub struct CheckoutPriceCreateCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutPriceCreateCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutPriceCreateCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutPriceCreateCustomerName {
    type Error = CheckoutPriceCreateCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutPriceCreateCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateCustomerName> for String {
    fn from(value: CheckoutPriceCreateCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutPriceCreateCustomerName {
    type Error = CheckoutPriceCreateCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutPriceCreateCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Maximum number of seats (works with seat-based pricing only)
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
pub struct CheckoutPriceCreateMaxSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateMaxSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutPriceCreateMaxSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateMaxSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateMaxSeats {
    type Error = CheckoutPriceCreateMaxSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateMaxSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutPriceCreateMaxSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateMaxSeats> for u64 {
    fn from(value: CheckoutPriceCreateMaxSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateMaxSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Maximum number of units (works with unit-based pricing only)
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
pub struct CheckoutPriceCreateMaxUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateMaxUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutPriceCreateMaxUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateMaxUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateMaxUnits {
    type Error = CheckoutPriceCreateMaxUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateMaxUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutPriceCreateMaxUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateMaxUnits> for u64 {
    fn from(value: CheckoutPriceCreateMaxUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateMaxUnits {
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
pub struct CheckoutPriceCreateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutPriceCreateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutPriceCreateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutPriceCreateMetadataKey {
    type Error = CheckoutPriceCreateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutPriceCreateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutPriceCreateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateMetadataKey> for String {
    fn from(value: CheckoutPriceCreateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutPriceCreateMetadataKey {
    type Error = CheckoutPriceCreateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutPriceCreateMetadataKey {
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
pub struct CheckoutPriceCreateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutPriceCreateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutPriceCreateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutPriceCreateMetadataValueString {
    type Error = CheckoutPriceCreateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutPriceCreateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutPriceCreateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateMetadataValueString> for String {
    fn from(value: CheckoutPriceCreateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutPriceCreateMetadataValueString {
    type Error = CheckoutPriceCreateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutPriceCreateMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Minimum number of seats (works with seat-based pricing only)
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
pub struct CheckoutPriceCreateMinSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateMinSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutPriceCreateMinSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateMinSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateMinSeats {
    type Error = CheckoutPriceCreateMinSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateMinSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutPriceCreateMinSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateMinSeats> for u64 {
    fn from(value: CheckoutPriceCreateMinSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateMinSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Minimum number of units (works with unit-based pricing only)
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
pub struct CheckoutPriceCreateMinUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateMinUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutPriceCreateMinUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateMinUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateMinUnits {
    type Error = CheckoutPriceCreateMinUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateMinUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutPriceCreateMinUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateMinUnits> for u64 {
    fn from(value: CheckoutPriceCreateMinUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateMinUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// When set, a back button will be shown in the checkout to return to this URL.
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
pub struct CheckoutPriceCreateReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutPriceCreateReturnUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutPriceCreateReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutPriceCreateReturnUrl {
    type Error = CheckoutPriceCreateReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutPriceCreateReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutPriceCreateReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateReturnUrl> for url::Url {
    fn from(value: CheckoutPriceCreateReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutPriceCreateReturnUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Predefined number of seats (works with seat-based pricing only)
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
pub struct CheckoutPriceCreateSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutPriceCreateSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateSeats {
    type Error = CheckoutPriceCreateSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutPriceCreateSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateSeats> for u64 {
    fn from(value: CheckoutPriceCreateSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// URL where the customer will be redirected after a successful payment.You can
/// add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout
/// session id.
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
pub struct CheckoutPriceCreateSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutPriceCreateSuccessUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutPriceCreateSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutPriceCreateSuccessUrl {
    type Error = CheckoutPriceCreateSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutPriceCreateSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutPriceCreateSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateSuccessUrl> for url::Url {
    fn from(value: CheckoutPriceCreateSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutPriceCreateSuccessUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
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
pub struct CheckoutPriceCreateTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutPriceCreateTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutPriceCreateTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateTrialIntervalCount {
    type Error = CheckoutPriceCreateTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(CheckoutPriceCreateTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateTrialIntervalCount> for u64 {
    fn from(value: CheckoutPriceCreateTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateTrialIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Predefined number of units (works with unit-based pricing only)
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
pub struct CheckoutPriceCreateUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutPriceCreateUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutPriceCreateUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutPriceCreateUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutPriceCreateUnits {
    type Error = CheckoutPriceCreateUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutPriceCreateUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutPriceCreateUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutPriceCreateUnits> for u64 {
    fn from(value: CheckoutPriceCreateUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutPriceCreateUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Amount in cents, before discounts and taxes. Only useful for custom prices,
/// it'll be ignored for fixed and free prices.
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
pub struct CheckoutProductAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutProductAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductAmount {
    type Error = CheckoutProductAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutProductAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductAmount> for u64 {
    fn from(value: CheckoutProductAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductAmount {
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
pub struct CheckoutProductCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutProductCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductCustomerMetadataKey {
    type Error = CheckoutProductCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutProductCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductCustomerMetadataKey> for String {
    fn from(value: CheckoutProductCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductCustomerMetadataKey {
    type Error = CheckoutProductCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductCustomerMetadataKey {
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
pub struct CheckoutProductCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutProductCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductCustomerMetadataValueString {
    type Error = CheckoutProductCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductCustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutProductCustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductCustomerMetadataValueString> for String {
    fn from(value: CheckoutProductCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductCustomerMetadataValueString {
    type Error = CheckoutProductCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductCustomerMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the customer.
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
pub struct CheckoutProductCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutProductCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductCustomerName {
    type Error = CheckoutProductCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutProductCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductCustomerName> for String {
    fn from(value: CheckoutProductCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductCustomerName {
    type Error = CheckoutProductCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Maximum number of seats (works with seat-based pricing only)
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
pub struct CheckoutProductMaxSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductMaxSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductMaxSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductMaxSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductMaxSeats {
    type Error = CheckoutProductMaxSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductMaxSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductMaxSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductMaxSeats> for u64 {
    fn from(value: CheckoutProductMaxSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductMaxSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Maximum number of units (works with unit-based pricing only)
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
pub struct CheckoutProductMaxUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductMaxUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductMaxUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductMaxUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductMaxUnits {
    type Error = CheckoutProductMaxUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductMaxUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductMaxUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductMaxUnits> for u64 {
    fn from(value: CheckoutProductMaxUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductMaxUnits {
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
pub struct CheckoutProductMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutProductMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductMetadataKey {
    type Error = CheckoutProductMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutProductMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductMetadataKey> for String {
    fn from(value: CheckoutProductMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductMetadataKey {
    type Error = CheckoutProductMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductMetadataKey {
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
pub struct CheckoutProductMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutProductMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductMetadataValueString {
    type Error = CheckoutProductMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutProductMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductMetadataValueString> for String {
    fn from(value: CheckoutProductMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductMetadataValueString {
    type Error = CheckoutProductMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Minimum number of seats (works with seat-based pricing only)
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
pub struct CheckoutProductMinSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductMinSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductMinSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductMinSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductMinSeats {
    type Error = CheckoutProductMinSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductMinSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductMinSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductMinSeats> for u64 {
    fn from(value: CheckoutProductMinSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductMinSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Minimum number of units (works with unit-based pricing only)
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
pub struct CheckoutProductMinUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductMinUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductMinUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductMinUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductMinUnits {
    type Error = CheckoutProductMinUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductMinUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductMinUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductMinUnits> for u64 {
    fn from(value: CheckoutProductMinUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductMinUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// When set, a back button will be shown in the checkout to return to this URL.
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
pub struct CheckoutProductReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutProductReturnUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutProductReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutProductReturnUrl {
    type Error = CheckoutProductReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutProductReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductReturnUrl> for url::Url {
    fn from(value: CheckoutProductReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutProductReturnUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Predefined number of seats (works with seat-based pricing only)
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
pub struct CheckoutProductSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductSeats {
    type Error = CheckoutProductSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductSeats> for u64 {
    fn from(value: CheckoutProductSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// URL where the customer will be redirected after a successful payment.You can
/// add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout
/// session id.
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
pub struct CheckoutProductSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutProductSuccessUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutProductSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutProductSuccessUrl {
    type Error = CheckoutProductSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutProductSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductSuccessUrl> for url::Url {
    fn from(value: CheckoutProductSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutProductSuccessUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
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
pub struct CheckoutProductTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutProductTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductTrialIntervalCount {
    type Error = CheckoutProductTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(CheckoutProductTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductTrialIntervalCount> for u64 {
    fn from(value: CheckoutProductTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductTrialIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Predefined number of units (works with unit-based pricing only)
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
pub struct CheckoutProductUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductUnits {
    type Error = CheckoutProductUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductUnits> for u64 {
    fn from(value: CheckoutProductUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductUnits {
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
pub struct CustomFieldCheckboxPropertiesFormHelpText(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCheckboxPropertiesFormHelpTextValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCheckboxPropertiesFormHelpText {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCheckboxPropertiesFormHelpTextValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCheckboxPropertiesFormHelpText {
    type Error = CustomFieldCheckboxPropertiesFormHelpTextValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCheckboxPropertiesFormHelpTextValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCheckboxPropertiesFormHelpText> for String {
    fn from(value: CustomFieldCheckboxPropertiesFormHelpText) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCheckboxPropertiesFormHelpText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCheckboxPropertiesFormHelpText {
    type Error = CustomFieldCheckboxPropertiesFormHelpTextValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCheckboxPropertiesFormHelpText {
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
pub struct CustomFieldCheckboxPropertiesFormLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCheckboxPropertiesFormLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCheckboxPropertiesFormLabel {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCheckboxPropertiesFormLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCheckboxPropertiesFormLabel {
    type Error = CustomFieldCheckboxPropertiesFormLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCheckboxPropertiesFormLabelValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCheckboxPropertiesFormLabel> for String {
    fn from(value: CustomFieldCheckboxPropertiesFormLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCheckboxPropertiesFormLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCheckboxPropertiesFormLabel {
    type Error = CustomFieldCheckboxPropertiesFormLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCheckboxPropertiesFormLabel {
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
pub struct CustomFieldCheckboxPropertiesFormPlaceholder(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCheckboxPropertiesFormPlaceholderValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCheckboxPropertiesFormPlaceholder {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCheckboxPropertiesFormPlaceholderValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCheckboxPropertiesFormPlaceholder {
    type Error = CustomFieldCheckboxPropertiesFormPlaceholderValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCheckboxPropertiesFormPlaceholderValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCheckboxPropertiesFormPlaceholder> for String {
    fn from(value: CustomFieldCheckboxPropertiesFormPlaceholder) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCheckboxPropertiesFormPlaceholder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCheckboxPropertiesFormPlaceholder {
    type Error = CustomFieldCheckboxPropertiesFormPlaceholderValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCheckboxPropertiesFormPlaceholder {
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
pub struct CustomFieldDatePropertiesFormHelpText(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldDatePropertiesFormHelpTextValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldDatePropertiesFormHelpText {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldDatePropertiesFormHelpTextValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldDatePropertiesFormHelpText {
    type Error = CustomFieldDatePropertiesFormHelpTextValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldDatePropertiesFormHelpTextValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldDatePropertiesFormHelpText> for String {
    fn from(value: CustomFieldDatePropertiesFormHelpText) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldDatePropertiesFormHelpText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldDatePropertiesFormHelpText {
    type Error = CustomFieldDatePropertiesFormHelpTextValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldDatePropertiesFormHelpText {
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
pub struct CustomFieldDatePropertiesFormLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldDatePropertiesFormLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldDatePropertiesFormLabel {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldDatePropertiesFormLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldDatePropertiesFormLabel {
    type Error = CustomFieldDatePropertiesFormLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldDatePropertiesFormLabelValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldDatePropertiesFormLabel> for String {
    fn from(value: CustomFieldDatePropertiesFormLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldDatePropertiesFormLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldDatePropertiesFormLabel {
    type Error = CustomFieldDatePropertiesFormLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldDatePropertiesFormLabel {
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
pub struct CustomFieldDatePropertiesFormPlaceholder(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldDatePropertiesFormPlaceholderValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldDatePropertiesFormPlaceholder {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldDatePropertiesFormPlaceholderValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldDatePropertiesFormPlaceholder {
    type Error = CustomFieldDatePropertiesFormPlaceholderValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldDatePropertiesFormPlaceholderValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldDatePropertiesFormPlaceholder> for String {
    fn from(value: CustomFieldDatePropertiesFormPlaceholder) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldDatePropertiesFormPlaceholder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldDatePropertiesFormPlaceholder {
    type Error = CustomFieldDatePropertiesFormPlaceholderValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldDatePropertiesFormPlaceholder {
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
pub struct CustomFieldDatePropertiesGe(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldDatePropertiesGeValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomFieldDatePropertiesGe {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, CustomFieldDatePropertiesGeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for CustomFieldDatePropertiesGe {
    type Error = CustomFieldDatePropertiesGeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(CustomFieldDatePropertiesGeValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomFieldDatePropertiesGeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldDatePropertiesGe> for i64 {
    fn from(value: CustomFieldDatePropertiesGe) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldDatePropertiesGe {
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
pub struct CustomFieldDatePropertiesLe(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldDatePropertiesLeValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomFieldDatePropertiesLe {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, CustomFieldDatePropertiesLeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for CustomFieldDatePropertiesLe {
    type Error = CustomFieldDatePropertiesLeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(CustomFieldDatePropertiesLeValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomFieldDatePropertiesLeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldDatePropertiesLe> for i64 {
    fn from(value: CustomFieldDatePropertiesLe) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldDatePropertiesLe {
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
pub struct CustomFieldNumberPropertiesFormHelpText(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldNumberPropertiesFormHelpTextValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldNumberPropertiesFormHelpText {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldNumberPropertiesFormHelpTextValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldNumberPropertiesFormHelpText {
    type Error = CustomFieldNumberPropertiesFormHelpTextValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldNumberPropertiesFormHelpTextValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldNumberPropertiesFormHelpText> for String {
    fn from(value: CustomFieldNumberPropertiesFormHelpText) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldNumberPropertiesFormHelpText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldNumberPropertiesFormHelpText {
    type Error = CustomFieldNumberPropertiesFormHelpTextValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldNumberPropertiesFormHelpText {
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
pub struct CustomFieldNumberPropertiesFormLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldNumberPropertiesFormLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldNumberPropertiesFormLabel {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldNumberPropertiesFormLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldNumberPropertiesFormLabel {
    type Error = CustomFieldNumberPropertiesFormLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldNumberPropertiesFormLabelValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldNumberPropertiesFormLabel> for String {
    fn from(value: CustomFieldNumberPropertiesFormLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldNumberPropertiesFormLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldNumberPropertiesFormLabel {
    type Error = CustomFieldNumberPropertiesFormLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldNumberPropertiesFormLabel {
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
pub struct CustomFieldNumberPropertiesFormPlaceholder(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldNumberPropertiesFormPlaceholderValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldNumberPropertiesFormPlaceholder {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldNumberPropertiesFormPlaceholderValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldNumberPropertiesFormPlaceholder {
    type Error = CustomFieldNumberPropertiesFormPlaceholderValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldNumberPropertiesFormPlaceholderValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldNumberPropertiesFormPlaceholder> for String {
    fn from(value: CustomFieldNumberPropertiesFormPlaceholder) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldNumberPropertiesFormPlaceholder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldNumberPropertiesFormPlaceholder {
    type Error = CustomFieldNumberPropertiesFormPlaceholderValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldNumberPropertiesFormPlaceholder {
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
pub struct CustomFieldNumberPropertiesGe(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldNumberPropertiesGeValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomFieldNumberPropertiesGe {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, CustomFieldNumberPropertiesGeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for CustomFieldNumberPropertiesGe {
    type Error = CustomFieldNumberPropertiesGeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(CustomFieldNumberPropertiesGeValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomFieldNumberPropertiesGeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldNumberPropertiesGe> for i64 {
    fn from(value: CustomFieldNumberPropertiesGe) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldNumberPropertiesGe {
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
pub struct CustomFieldNumberPropertiesLe(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldNumberPropertiesLeValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomFieldNumberPropertiesLe {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, CustomFieldNumberPropertiesLeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for CustomFieldNumberPropertiesLe {
    type Error = CustomFieldNumberPropertiesLeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(CustomFieldNumberPropertiesLeValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomFieldNumberPropertiesLeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldNumberPropertiesLe> for i64 {
    fn from(value: CustomFieldNumberPropertiesLe) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldNumberPropertiesLe {
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
pub struct CustomFieldSelectOptionLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldSelectOptionLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldSelectOptionLabel {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldSelectOptionLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldSelectOptionLabel {
    type Error = CustomFieldSelectOptionLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldSelectOptionLabelValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldSelectOptionLabel> for String {
    fn from(value: CustomFieldSelectOptionLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldSelectOptionLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldSelectOptionLabel {
    type Error = CustomFieldSelectOptionLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldSelectOptionLabel {
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
pub struct CustomFieldSelectOptionValue(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldSelectOptionValueValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldSelectOptionValue {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldSelectOptionValueValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldSelectOptionValue {
    type Error = CustomFieldSelectOptionValueValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldSelectOptionValueValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldSelectOptionValue> for String {
    fn from(value: CustomFieldSelectOptionValue) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldSelectOptionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldSelectOptionValue {
    type Error = CustomFieldSelectOptionValueValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldSelectOptionValue {
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
pub struct CustomFieldSelectPropertiesFormHelpText(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldSelectPropertiesFormHelpTextValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldSelectPropertiesFormHelpText {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldSelectPropertiesFormHelpTextValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldSelectPropertiesFormHelpText {
    type Error = CustomFieldSelectPropertiesFormHelpTextValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldSelectPropertiesFormHelpTextValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldSelectPropertiesFormHelpText> for String {
    fn from(value: CustomFieldSelectPropertiesFormHelpText) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldSelectPropertiesFormHelpText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldSelectPropertiesFormHelpText {
    type Error = CustomFieldSelectPropertiesFormHelpTextValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldSelectPropertiesFormHelpText {
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
pub struct CustomFieldSelectPropertiesFormLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldSelectPropertiesFormLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldSelectPropertiesFormLabel {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldSelectPropertiesFormLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldSelectPropertiesFormLabel {
    type Error = CustomFieldSelectPropertiesFormLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldSelectPropertiesFormLabelValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldSelectPropertiesFormLabel> for String {
    fn from(value: CustomFieldSelectPropertiesFormLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldSelectPropertiesFormLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldSelectPropertiesFormLabel {
    type Error = CustomFieldSelectPropertiesFormLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldSelectPropertiesFormLabel {
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
pub struct CustomFieldSelectPropertiesFormPlaceholder(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldSelectPropertiesFormPlaceholderValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldSelectPropertiesFormPlaceholder {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldSelectPropertiesFormPlaceholderValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldSelectPropertiesFormPlaceholder {
    type Error = CustomFieldSelectPropertiesFormPlaceholderValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldSelectPropertiesFormPlaceholderValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldSelectPropertiesFormPlaceholder> for String {
    fn from(value: CustomFieldSelectPropertiesFormPlaceholder) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldSelectPropertiesFormPlaceholder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldSelectPropertiesFormPlaceholder {
    type Error = CustomFieldSelectPropertiesFormPlaceholderValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldSelectPropertiesFormPlaceholder {
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
pub struct CustomFieldTextPropertiesFormHelpText(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldTextPropertiesFormHelpTextValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldTextPropertiesFormHelpText {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldTextPropertiesFormHelpTextValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldTextPropertiesFormHelpText {
    type Error = CustomFieldTextPropertiesFormHelpTextValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldTextPropertiesFormHelpTextValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldTextPropertiesFormHelpText> for String {
    fn from(value: CustomFieldTextPropertiesFormHelpText) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldTextPropertiesFormHelpText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldTextPropertiesFormHelpText {
    type Error = CustomFieldTextPropertiesFormHelpTextValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldTextPropertiesFormHelpText {
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
pub struct CustomFieldTextPropertiesFormLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldTextPropertiesFormLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldTextPropertiesFormLabel {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldTextPropertiesFormLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldTextPropertiesFormLabel {
    type Error = CustomFieldTextPropertiesFormLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldTextPropertiesFormLabelValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldTextPropertiesFormLabel> for String {
    fn from(value: CustomFieldTextPropertiesFormLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldTextPropertiesFormLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldTextPropertiesFormLabel {
    type Error = CustomFieldTextPropertiesFormLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldTextPropertiesFormLabel {
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
pub struct CustomFieldTextPropertiesFormPlaceholder(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldTextPropertiesFormPlaceholderValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldTextPropertiesFormPlaceholder {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldTextPropertiesFormPlaceholderValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldTextPropertiesFormPlaceholder {
    type Error = CustomFieldTextPropertiesFormPlaceholderValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldTextPropertiesFormPlaceholderValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldTextPropertiesFormPlaceholder> for String {
    fn from(value: CustomFieldTextPropertiesFormPlaceholder) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldTextPropertiesFormPlaceholder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldTextPropertiesFormPlaceholder {
    type Error = CustomFieldTextPropertiesFormPlaceholderValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldTextPropertiesFormPlaceholder {
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
pub struct CustomFieldTextPropertiesMaxLength(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldTextPropertiesMaxLengthValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomFieldTextPropertiesMaxLength {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CustomFieldTextPropertiesMaxLengthValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CustomFieldTextPropertiesMaxLength {
    type Error = CustomFieldTextPropertiesMaxLengthValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CustomFieldTextPropertiesMaxLengthValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomFieldTextPropertiesMaxLengthValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldTextPropertiesMaxLength> for u64 {
    fn from(value: CustomFieldTextPropertiesMaxLength) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldTextPropertiesMaxLength {
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
    Copy,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct CustomFieldTextPropertiesMinLength(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldTextPropertiesMinLengthValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomFieldTextPropertiesMinLength {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CustomFieldTextPropertiesMinLengthValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CustomFieldTextPropertiesMinLength {
    type Error = CustomFieldTextPropertiesMinLengthValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CustomFieldTextPropertiesMinLengthValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomFieldTextPropertiesMinLengthValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldTextPropertiesMinLength> for u64 {
    fn from(value: CustomFieldTextPropertiesMinLength) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldTextPropertiesMinLength {
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
    Copy,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct FilterClauseValueInteger(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FilterClauseValueIntegerValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl FilterClauseValueInteger {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, FilterClauseValueIntegerValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for FilterClauseValueInteger {
    type Error = FilterClauseValueIntegerValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(FilterClauseValueIntegerValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(FilterClauseValueIntegerValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<FilterClauseValueInteger> for i64 {
    fn from(value: FilterClauseValueInteger) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for FilterClauseValueInteger {
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
pub struct FilterClauseValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FilterClauseValueStringValidationError {
    #[error("must contain at most 1000 characters")]
    TooLong,
}
impl FilterClauseValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, FilterClauseValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for FilterClauseValueString {
    type Error = FilterClauseValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 1000u64 {
            return Err(FilterClauseValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<FilterClauseValueString> for String {
    fn from(value: FilterClauseValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for FilterClauseValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for FilterClauseValueString {
    type Error = FilterClauseValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for FilterClauseValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The price in cents.
/// Minimum amounts per currency:
/// - USD: 0.5
/// - AED: 2
/// - ALL: 50
/// - AMD: 200
/// - AOA: 500
/// - ARS: 750
/// - AUD: 0.7
/// - AWG: 1
/// - AZN: 1
/// - BAM: 1
/// - BBD: 2
/// - BDT: 70
/// - BIF: 2,000
/// - BMD: 1
/// - BND: 1
/// - BOB: 5
/// - BRL: 2.5
/// - BSD: 1
/// - BWP: 10
/// - BZD: 2
/// - CAD: 0.7
/// - CDF: 2,000
/// - CHF: 0.5
/// - CLP: 500
/// - CNY: 5
/// - COP: 2,000
/// - CRC: 300
/// - CVE: 50
/// - CZK: 15
/// - DJF: 100
/// - DKK: 3.2
/// - DOP: 40
/// - DZD: 70
/// - EGP: 30
/// - ETB: 80
/// - EUR: 0.5
/// - FJD: 2
/// - FKP: 1
/// - GBP: 0.4
/// - GEL: 2
/// - GNF: 5,000
/// - GIP: 1
/// - GMD: 40
/// - GTQ: 5
/// - GYD: 200
/// - HKD: 4
/// - HNL: 20
/// - HTG: 70
/// - HUF: 175
/// - IDR: 9,000
/// - ILS: 1.5
/// - INR: 60
/// - ISK: 70
/// - JMD: 80
/// - JPY: 80
/// - KES: 70
/// - KGS: 50
/// - KHR: 3,000
/// - KMF: 500
/// - KRW: 800
/// - KYD: 1
/// - KZT: 300
/// - LAK: 20,000
/// - LKR: 200
/// - LRD: 100
/// - LSL: 10
/// - MAD: 5
/// - MDL: 10
/// - MGA: 3,000
/// - MKD: 50
/// - MNT: 2,000
/// - MOP: 5
/// - MUR: 50
/// - MVR: 8
/// - MXN: 9
/// - MWK: 1,000
/// - MYR: 2
/// - MZN: 50
/// - NAD: 10
/// - NGN: 700
/// - NIO: 20
/// - NOK: 5
/// - NPR: 80
/// - NZD: 0.9
/// - PAB: 1
/// - PEN: 2
/// - PGK: 3
/// - PHP: 35
/// - PKR: 200
/// - PLN: 2
/// - PYG: 4,000
/// - QAR: 2
/// - RON: 2.5
/// - RSD: 60
/// - RWF: 1,000
/// - SAR: 2
/// - SBD: 4
/// - SCR: 8
/// - SEK: 5
/// - SGD: 0.7
/// - SHP: 1
/// - SOS: 500
/// - SRD: 20
/// - SZL: 10
/// - THB: 20
/// - TJS: 5
/// - TOP: 2
/// - TRY: 30
/// - TTD: 4
/// - TWD: 20
/// - TZS: 2,000
/// - UAH: 30
/// - UGX: 2,000
/// - UYU: 20
/// - UZS: 7,000
/// - VND: 20,000
/// - VUV: 100
/// - WST: 2
/// - XAF: 500
/// - XCD: 2
/// - XCG: 1
/// - XOF: 500
/// - XPF: 100
/// - YER: 200
/// - ZAR: 9
/// - ZMW: 10
/// - Other currencies: 50 minor units
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
pub struct ProductPriceCustomMaximumAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceCustomMaximumAmountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl ProductPriceCustomMaximumAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceCustomMaximumAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceCustomMaximumAmount {
    type Error = ProductPriceCustomMaximumAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductPriceCustomMaximumAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceCustomMaximumAmount> for u64 {
    fn from(value: ProductPriceCustomMaximumAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceCustomMaximumAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The minimum amount the customer can pay. If set to 0, the price is 'free or
/// pay what you want' and $0 is accepted. If set to a value below the minimum
/// price amount for the currency, it will be rejected. Defaults to the minimum
/// price amount for the currency. Minimum per currency:
/// - USD: 0.5
/// - AED: 2
/// - ALL: 50
/// - AMD: 200
/// - AOA: 500
/// - ARS: 750
/// - AUD: 0.7
/// - AWG: 1
/// - AZN: 1
/// - BAM: 1
/// - BBD: 2
/// - BDT: 70
/// - BIF: 2,000
/// - BMD: 1
/// - BND: 1
/// - BOB: 5
/// - BRL: 2.5
/// - BSD: 1
/// - BWP: 10
/// - BZD: 2
/// - CAD: 0.7
/// - CDF: 2,000
/// - CHF: 0.5
/// - CLP: 500
/// - CNY: 5
/// - COP: 2,000
/// - CRC: 300
/// - CVE: 50
/// - CZK: 15
/// - DJF: 100
/// - DKK: 3.2
/// - DOP: 40
/// - DZD: 70
/// - EGP: 30
/// - ETB: 80
/// - EUR: 0.5
/// - FJD: 2
/// - FKP: 1
/// - GBP: 0.4
/// - GEL: 2
/// - GNF: 5,000
/// - GIP: 1
/// - GMD: 40
/// - GTQ: 5
/// - GYD: 200
/// - HKD: 4
/// - HNL: 20
/// - HTG: 70
/// - HUF: 175
/// - IDR: 9,000
/// - ILS: 1.5
/// - INR: 60
/// - ISK: 70
/// - JMD: 80
/// - JPY: 80
/// - KES: 70
/// - KGS: 50
/// - KHR: 3,000
/// - KMF: 500
/// - KRW: 800
/// - KYD: 1
/// - KZT: 300
/// - LAK: 20,000
/// - LKR: 200
/// - LRD: 100
/// - LSL: 10
/// - MAD: 5
/// - MDL: 10
/// - MGA: 3,000
/// - MKD: 50
/// - MNT: 2,000
/// - MOP: 5
/// - MUR: 50
/// - MVR: 8
/// - MXN: 9
/// - MWK: 1,000
/// - MYR: 2
/// - MZN: 50
/// - NAD: 10
/// - NGN: 700
/// - NIO: 20
/// - NOK: 5
/// - NPR: 80
/// - NZD: 0.9
/// - PAB: 1
/// - PEN: 2
/// - PGK: 3
/// - PHP: 35
/// - PKR: 200
/// - PLN: 2
/// - PYG: 4,000
/// - QAR: 2
/// - RON: 2.5
/// - RSD: 60
/// - RWF: 1,000
/// - SAR: 2
/// - SBD: 4
/// - SCR: 8
/// - SEK: 5
/// - SGD: 0.7
/// - SHP: 1
/// - SOS: 500
/// - SRD: 20
/// - SZL: 10
/// - THB: 20
/// - TJS: 5
/// - TOP: 2
/// - TRY: 30
/// - TTD: 4
/// - TWD: 20
/// - TZS: 2,000
/// - UAH: 30
/// - UGX: 2,000
/// - UYU: 20
/// - UZS: 7,000
/// - VND: 20,000
/// - VUV: 100
/// - WST: 2
/// - XAF: 500
/// - XCD: 2
/// - XCG: 1
/// - XOF: 500
/// - XPF: 100
/// - YER: 200
/// - ZAR: 9
/// - ZMW: 10
/// - Other currencies: 50 minor units
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
pub struct ProductPriceCustomMinimumAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceCustomMinimumAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl ProductPriceCustomMinimumAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceCustomMinimumAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceCustomMinimumAmount {
    type Error = ProductPriceCustomMinimumAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(ProductPriceCustomMinimumAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceCustomMinimumAmount> for u64 {
    fn from(value: ProductPriceCustomMinimumAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceCustomMinimumAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The price in cents.
/// Minimum amounts per currency:
/// - USD: 0.5
/// - AED: 2
/// - ALL: 50
/// - AMD: 200
/// - AOA: 500
/// - ARS: 750
/// - AUD: 0.7
/// - AWG: 1
/// - AZN: 1
/// - BAM: 1
/// - BBD: 2
/// - BDT: 70
/// - BIF: 2,000
/// - BMD: 1
/// - BND: 1
/// - BOB: 5
/// - BRL: 2.5
/// - BSD: 1
/// - BWP: 10
/// - BZD: 2
/// - CAD: 0.7
/// - CDF: 2,000
/// - CHF: 0.5
/// - CLP: 500
/// - CNY: 5
/// - COP: 2,000
/// - CRC: 300
/// - CVE: 50
/// - CZK: 15
/// - DJF: 100
/// - DKK: 3.2
/// - DOP: 40
/// - DZD: 70
/// - EGP: 30
/// - ETB: 80
/// - EUR: 0.5
/// - FJD: 2
/// - FKP: 1
/// - GBP: 0.4
/// - GEL: 2
/// - GNF: 5,000
/// - GIP: 1
/// - GMD: 40
/// - GTQ: 5
/// - GYD: 200
/// - HKD: 4
/// - HNL: 20
/// - HTG: 70
/// - HUF: 175
/// - IDR: 9,000
/// - ILS: 1.5
/// - INR: 60
/// - ISK: 70
/// - JMD: 80
/// - JPY: 80
/// - KES: 70
/// - KGS: 50
/// - KHR: 3,000
/// - KMF: 500
/// - KRW: 800
/// - KYD: 1
/// - KZT: 300
/// - LAK: 20,000
/// - LKR: 200
/// - LRD: 100
/// - LSL: 10
/// - MAD: 5
/// - MDL: 10
/// - MGA: 3,000
/// - MKD: 50
/// - MNT: 2,000
/// - MOP: 5
/// - MUR: 50
/// - MVR: 8
/// - MXN: 9
/// - MWK: 1,000
/// - MYR: 2
/// - MZN: 50
/// - NAD: 10
/// - NGN: 700
/// - NIO: 20
/// - NOK: 5
/// - NPR: 80
/// - NZD: 0.9
/// - PAB: 1
/// - PEN: 2
/// - PGK: 3
/// - PHP: 35
/// - PKR: 200
/// - PLN: 2
/// - PYG: 4,000
/// - QAR: 2
/// - RON: 2.5
/// - RSD: 60
/// - RWF: 1,000
/// - SAR: 2
/// - SBD: 4
/// - SCR: 8
/// - SEK: 5
/// - SGD: 0.7
/// - SHP: 1
/// - SOS: 500
/// - SRD: 20
/// - SZL: 10
/// - THB: 20
/// - TJS: 5
/// - TOP: 2
/// - TRY: 30
/// - TTD: 4
/// - TWD: 20
/// - TZS: 2,000
/// - UAH: 30
/// - UGX: 2,000
/// - UYU: 20
/// - UZS: 7,000
/// - VND: 20,000
/// - VUV: 100
/// - WST: 2
/// - XAF: 500
/// - XCD: 2
/// - XCG: 1
/// - XOF: 500
/// - XPF: 100
/// - YER: 200
/// - ZAR: 9
/// - ZMW: 10
/// - Other currencies: 50 minor units
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
pub struct ProductPriceCustomPresetAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceCustomPresetAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl ProductPriceCustomPresetAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceCustomPresetAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceCustomPresetAmount {
    type Error = ProductPriceCustomPresetAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(ProductPriceCustomPresetAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceCustomPresetAmount> for u64 {
    fn from(value: ProductPriceCustomPresetAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceCustomPresetAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The price in cents. Set to `0` for a free price.
/// Minimum amounts per currency:
/// - USD: 0.5
/// - AED: 2
/// - ALL: 50
/// - AMD: 200
/// - AOA: 500
/// - ARS: 750
/// - AUD: 0.7
/// - AWG: 1
/// - AZN: 1
/// - BAM: 1
/// - BBD: 2
/// - BDT: 70
/// - BIF: 2,000
/// - BMD: 1
/// - BND: 1
/// - BOB: 5
/// - BRL: 2.5
/// - BSD: 1
/// - BWP: 10
/// - BZD: 2
/// - CAD: 0.7
/// - CDF: 2,000
/// - CHF: 0.5
/// - CLP: 500
/// - CNY: 5
/// - COP: 2,000
/// - CRC: 300
/// - CVE: 50
/// - CZK: 15
/// - DJF: 100
/// - DKK: 3.2
/// - DOP: 40
/// - DZD: 70
/// - EGP: 30
/// - ETB: 80
/// - EUR: 0.5
/// - FJD: 2
/// - FKP: 1
/// - GBP: 0.4
/// - GEL: 2
/// - GNF: 5,000
/// - GIP: 1
/// - GMD: 40
/// - GTQ: 5
/// - GYD: 200
/// - HKD: 4
/// - HNL: 20
/// - HTG: 70
/// - HUF: 175
/// - IDR: 9,000
/// - ILS: 1.5
/// - INR: 60
/// - ISK: 70
/// - JMD: 80
/// - JPY: 80
/// - KES: 70
/// - KGS: 50
/// - KHR: 3,000
/// - KMF: 500
/// - KRW: 800
/// - KYD: 1
/// - KZT: 300
/// - LAK: 20,000
/// - LKR: 200
/// - LRD: 100
/// - LSL: 10
/// - MAD: 5
/// - MDL: 10
/// - MGA: 3,000
/// - MKD: 50
/// - MNT: 2,000
/// - MOP: 5
/// - MUR: 50
/// - MVR: 8
/// - MXN: 9
/// - MWK: 1,000
/// - MYR: 2
/// - MZN: 50
/// - NAD: 10
/// - NGN: 700
/// - NIO: 20
/// - NOK: 5
/// - NPR: 80
/// - NZD: 0.9
/// - PAB: 1
/// - PEN: 2
/// - PGK: 3
/// - PHP: 35
/// - PKR: 200
/// - PLN: 2
/// - PYG: 4,000
/// - QAR: 2
/// - RON: 2.5
/// - RSD: 60
/// - RWF: 1,000
/// - SAR: 2
/// - SBD: 4
/// - SCR: 8
/// - SEK: 5
/// - SGD: 0.7
/// - SHP: 1
/// - SOS: 500
/// - SRD: 20
/// - SZL: 10
/// - THB: 20
/// - TJS: 5
/// - TOP: 2
/// - TRY: 30
/// - TTD: 4
/// - TWD: 20
/// - TZS: 2,000
/// - UAH: 30
/// - UGX: 2,000
/// - UYU: 20
/// - UZS: 7,000
/// - VND: 20,000
/// - VUV: 100
/// - WST: 2
/// - XAF: 500
/// - XCD: 2
/// - XCG: 1
/// - XOF: 500
/// - XPF: 100
/// - YER: 200
/// - ZAR: 9
/// - ZMW: 10
/// - Other currencies: 50 minor units
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
pub struct ProductPriceFixedPriceAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceFixedPriceAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl ProductPriceFixedPriceAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceFixedPriceAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceFixedPriceAmount {
    type Error = ProductPriceFixedPriceAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(ProductPriceFixedPriceAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceFixedPriceAmount> for u64 {
    fn from(value: ProductPriceFixedPriceAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceFixedPriceAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum amount in cents that can be charged, regardless of the
/// number of units consumed.
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
pub struct ProductPriceMeteredTiersCapAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceMeteredTiersCapAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl ProductPriceMeteredTiersCapAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceMeteredTiersCapAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceMeteredTiersCapAmount {
    type Error = ProductPriceMeteredTiersCapAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(ProductPriceMeteredTiersCapAmountValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(ProductPriceMeteredTiersCapAmountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceMeteredTiersCapAmount> for u64 {
    fn from(value: ProductPriceMeteredTiersCapAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceMeteredTiersCapAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum amount in cents that can be charged, regardless of the
/// number of units consumed.
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
pub struct ProductPriceMeteredUnitCapAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceMeteredUnitCapAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl ProductPriceMeteredUnitCapAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceMeteredUnitCapAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceMeteredUnitCapAmount {
    type Error = ProductPriceMeteredUnitCapAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(ProductPriceMeteredUnitCapAmountValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(ProductPriceMeteredUnitCapAmountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceMeteredUnitCapAmount> for u64 {
    fn from(value: ProductPriceMeteredUnitCapAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceMeteredUnitCapAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The price per unit in cents. Supports up to 12 decimal places.
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
pub struct ProductPriceMeteredUnitUnitAmountNumber(f64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceMeteredUnitUnitAmountNumberValidationError {
    #[error("must be finite")]
    NotFinite,
    #[error("must be greater than 0")]
    BelowMinimum,
}
impl ProductPriceMeteredUnitUnitAmountNumber {
    /// Validate a value.
    pub fn try_new(
        value: f64,
    ) -> Result<Self, ProductPriceMeteredUnitUnitAmountNumberValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> f64 {
        self.0
    }
}
impl TryFrom<f64> for ProductPriceMeteredUnitUnitAmountNumber {
    type Error = ProductPriceMeteredUnitUnitAmountNumberValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(ProductPriceMeteredUnitUnitAmountNumberValidationError::NotFinite);
        }
        if value <= 0f64 {
            return Err(ProductPriceMeteredUnitUnitAmountNumberValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceMeteredUnitUnitAmountNumber> for f64 {
    fn from(value: ProductPriceMeteredUnitUnitAmountNumber) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceMeteredUnitUnitAmountNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <f64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Maximum number of seats (inclusive). None for unlimited.
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
pub struct ProductPriceSeatTierMaxSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceSeatTierMaxSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl ProductPriceSeatTierMaxSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceSeatTierMaxSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceSeatTierMaxSeats {
    type Error = ProductPriceSeatTierMaxSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductPriceSeatTierMaxSeatsValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceSeatTierMaxSeats> for u64 {
    fn from(value: ProductPriceSeatTierMaxSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceSeatTierMaxSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Minimum number of seats (inclusive)
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
pub struct ProductPriceSeatTierMinSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceSeatTierMinSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl ProductPriceSeatTierMinSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceSeatTierMinSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceSeatTierMinSeats {
    type Error = ProductPriceSeatTierMinSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductPriceSeatTierMinSeatsValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceSeatTierMinSeats> for u64 {
    fn from(value: ProductPriceSeatTierMinSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceSeatTierMinSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Price per seat in cents for this tier
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
pub struct ProductPriceSeatTierPricePerSeat(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceSeatTierPricePerSeatValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 9223372036854776000")]
    AboveMaximum,
}
impl ProductPriceSeatTierPricePerSeat {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceSeatTierPricePerSeatValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceSeatTierPricePerSeat {
    type Error = ProductPriceSeatTierPricePerSeatValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(ProductPriceSeatTierPricePerSeatValidationError::BelowMinimum);
        }
        if i128::from(value) > 9223372036854776000i128 {
            return Err(ProductPriceSeatTierPricePerSeatValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceSeatTierPricePerSeat> for u64 {
    fn from(value: ProductPriceSeatTierPricePerSeat) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceSeatTierPricePerSeat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The minimum purchasable quantity (inclusive). Defaults to 1 when not set.
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
pub struct ProductPriceUnitBasedMinimumUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceUnitBasedMinimumUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl ProductPriceUnitBasedMinimumUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceUnitBasedMinimumUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceUnitBasedMinimumUnits {
    type Error = ProductPriceUnitBasedMinimumUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductPriceUnitBasedMinimumUnitsValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceUnitBasedMinimumUnits> for u64 {
    fn from(value: ProductPriceUnitBasedMinimumUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceUnitBasedMinimumUnits {
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
    Copy,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct TierBound(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TierBoundValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl TierBound {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, TierBoundValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for TierBound {
    type Error = TierBoundValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(TierBoundValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<TierBound> for u64 {
    fn from(value: TierBound) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for TierBound {
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
    Copy,
    Eq,
    Ord,
    Hash,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct TierInputBound(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TierInputBoundValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl TierInputBound {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, TierInputBoundValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for TierInputBound {
    type Error = TierInputBoundValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(TierInputBoundValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<TierInputBound> for u64 {
    fn from(value: TierInputBound) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for TierInputBound {
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
    Copy,
    serde::Serialize,
    derive_more::Display,
    derive_more::Deref,
)]
#[serde(transparent)]
pub struct TierInputUnitAmountNumber(f64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TierInputUnitAmountNumberValidationError {
    #[error("must be finite")]
    NotFinite,
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 9223372036854776000")]
    AboveMaximum,
}
impl TierInputUnitAmountNumber {
    /// Validate a value.
    pub fn try_new(value: f64) -> Result<Self, TierInputUnitAmountNumberValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> f64 {
        self.0
    }
}
impl TryFrom<f64> for TierInputUnitAmountNumber {
    type Error = TierInputUnitAmountNumberValidationError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(TierInputUnitAmountNumberValidationError::NotFinite);
        }
        if value < 0f64 {
            return Err(TierInputUnitAmountNumberValidationError::BelowMinimum);
        }
        if value > 9223372036854776000f64 {
            return Err(TierInputUnitAmountNumberValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<TierInputUnitAmountNumber> for f64 {
    fn from(value: TierInputUnitAmountNumber) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for TierInputUnitAmountNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <f64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
