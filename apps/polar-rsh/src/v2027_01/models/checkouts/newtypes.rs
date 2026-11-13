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
pub struct CheckoutAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutAmount {
    type Error = CheckoutAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutAmount> for u64 {
    fn from(value: CheckoutAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutAmount {
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
pub struct CheckoutConfirmSharedPaymentTokenAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmSharedPaymentTokenAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutConfirmSharedPaymentTokenAmount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutConfirmSharedPaymentTokenAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutConfirmSharedPaymentTokenAmount {
    type Error = CheckoutConfirmSharedPaymentTokenAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutConfirmSharedPaymentTokenAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmSharedPaymentTokenAmount> for u64 {
    fn from(value: CheckoutConfirmSharedPaymentTokenAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmSharedPaymentTokenAmount {
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
pub struct CheckoutConfirmSharedPaymentTokenCustomerBillingName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmSharedPaymentTokenCustomerBillingNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutConfirmSharedPaymentTokenCustomerBillingName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutConfirmSharedPaymentTokenCustomerBillingNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutConfirmSharedPaymentTokenCustomerBillingName {
    type Error = CheckoutConfirmSharedPaymentTokenCustomerBillingNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(
                CheckoutConfirmSharedPaymentTokenCustomerBillingNameValidationError::TooLong,
            );
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmSharedPaymentTokenCustomerBillingName> for String {
    fn from(value: CheckoutConfirmSharedPaymentTokenCustomerBillingName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmSharedPaymentTokenCustomerBillingName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutConfirmSharedPaymentTokenCustomerBillingName {
    type Error = CheckoutConfirmSharedPaymentTokenCustomerBillingNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutConfirmSharedPaymentTokenCustomerBillingName {
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
pub struct CheckoutConfirmSharedPaymentTokenCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmSharedPaymentTokenCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutConfirmSharedPaymentTokenCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutConfirmSharedPaymentTokenCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutConfirmSharedPaymentTokenCustomerName {
    type Error = CheckoutConfirmSharedPaymentTokenCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutConfirmSharedPaymentTokenCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmSharedPaymentTokenCustomerName> for String {
    fn from(value: CheckoutConfirmSharedPaymentTokenCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmSharedPaymentTokenCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutConfirmSharedPaymentTokenCustomerName {
    type Error = CheckoutConfirmSharedPaymentTokenCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutConfirmSharedPaymentTokenCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Number of seats for seat-based pricing.
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
pub struct CheckoutConfirmSharedPaymentTokenSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmSharedPaymentTokenSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutConfirmSharedPaymentTokenSeats {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutConfirmSharedPaymentTokenSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutConfirmSharedPaymentTokenSeats {
    type Error = CheckoutConfirmSharedPaymentTokenSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutConfirmSharedPaymentTokenSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutConfirmSharedPaymentTokenSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmSharedPaymentTokenSeats> for u64 {
    fn from(value: CheckoutConfirmSharedPaymentTokenSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmSharedPaymentTokenSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of units for unit-based pricing.
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
pub struct CheckoutConfirmSharedPaymentTokenUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmSharedPaymentTokenUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutConfirmSharedPaymentTokenUnits {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutConfirmSharedPaymentTokenUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutConfirmSharedPaymentTokenUnits {
    type Error = CheckoutConfirmSharedPaymentTokenUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutConfirmSharedPaymentTokenUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutConfirmSharedPaymentTokenUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmSharedPaymentTokenUnits> for u64 {
    fn from(value: CheckoutConfirmSharedPaymentTokenUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmSharedPaymentTokenUnits {
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
pub struct CheckoutConfirmStripeAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmStripeAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutConfirmStripeAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutConfirmStripeAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutConfirmStripeAmount {
    type Error = CheckoutConfirmStripeAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutConfirmStripeAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmStripeAmount> for u64 {
    fn from(value: CheckoutConfirmStripeAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmStripeAmount {
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
pub struct CheckoutConfirmStripeCustomerBillingName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmStripeCustomerBillingNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutConfirmStripeCustomerBillingName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutConfirmStripeCustomerBillingNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutConfirmStripeCustomerBillingName {
    type Error = CheckoutConfirmStripeCustomerBillingNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutConfirmStripeCustomerBillingNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmStripeCustomerBillingName> for String {
    fn from(value: CheckoutConfirmStripeCustomerBillingName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmStripeCustomerBillingName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutConfirmStripeCustomerBillingName {
    type Error = CheckoutConfirmStripeCustomerBillingNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutConfirmStripeCustomerBillingName {
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
pub struct CheckoutConfirmStripeCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmStripeCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutConfirmStripeCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutConfirmStripeCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutConfirmStripeCustomerName {
    type Error = CheckoutConfirmStripeCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutConfirmStripeCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmStripeCustomerName> for String {
    fn from(value: CheckoutConfirmStripeCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmStripeCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutConfirmStripeCustomerName {
    type Error = CheckoutConfirmStripeCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutConfirmStripeCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Number of seats for seat-based pricing.
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
pub struct CheckoutConfirmStripeSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmStripeSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutConfirmStripeSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutConfirmStripeSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutConfirmStripeSeats {
    type Error = CheckoutConfirmStripeSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutConfirmStripeSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutConfirmStripeSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmStripeSeats> for u64 {
    fn from(value: CheckoutConfirmStripeSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmStripeSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of units for unit-based pricing.
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
pub struct CheckoutConfirmStripeUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutConfirmStripeUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutConfirmStripeUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutConfirmStripeUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutConfirmStripeUnits {
    type Error = CheckoutConfirmStripeUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutConfirmStripeUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutConfirmStripeUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutConfirmStripeUnits> for u64 {
    fn from(value: CheckoutConfirmStripeUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutConfirmStripeUnits {
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
pub struct CheckoutCustomerBillingName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutCustomerBillingNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutCustomerBillingName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutCustomerBillingNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutCustomerBillingName {
    type Error = CheckoutCustomerBillingNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutCustomerBillingNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutCustomerBillingName> for String {
    fn from(value: CheckoutCustomerBillingName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCustomerBillingName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutCustomerBillingName {
    type Error = CheckoutCustomerBillingNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutCustomerBillingName {
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
pub struct CheckoutCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutCustomerMetadataKey {
    type Error = CheckoutCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutCustomerMetadataKey> for String {
    fn from(value: CheckoutCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutCustomerMetadataKey {
    type Error = CheckoutCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutCustomerMetadataKey {
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
pub struct CheckoutCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutCustomerMetadataValueString {
    type Error = CheckoutCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutCustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutCustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutCustomerMetadataValueString> for String {
    fn from(value: CheckoutCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutCustomerMetadataValueString {
    type Error = CheckoutCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutCustomerMetadataValueString {
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
pub struct CheckoutCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutCustomerName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, CheckoutCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutCustomerName {
    type Error = CheckoutCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutCustomerName> for String {
    fn from(value: CheckoutCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutCustomerName {
    type Error = CheckoutCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutCustomerName {
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
pub struct CheckoutMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, CheckoutMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutMetadataKey {
    type Error = CheckoutMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutMetadataKey> for String {
    fn from(value: CheckoutMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutMetadataKey {
    type Error = CheckoutMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutMetadataKey {
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
pub struct CheckoutMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutMetadataValueString {
    type Error = CheckoutMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutMetadataValueString> for String {
    fn from(value: CheckoutMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutMetadataValueString {
    type Error = CheckoutMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutMetadataValueString {
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
pub struct CheckoutProductsCreateAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutProductsCreateAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateAmount {
    type Error = CheckoutProductsCreateAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutProductsCreateAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateAmount> for u64 {
    fn from(value: CheckoutProductsCreateAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateAmount {
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
pub struct CheckoutProductsCreateCustomerBillingName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateCustomerBillingNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutProductsCreateCustomerBillingName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsCreateCustomerBillingNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsCreateCustomerBillingName {
    type Error = CheckoutProductsCreateCustomerBillingNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutProductsCreateCustomerBillingNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateCustomerBillingName> for String {
    fn from(value: CheckoutProductsCreateCustomerBillingName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateCustomerBillingName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsCreateCustomerBillingName {
    type Error = CheckoutProductsCreateCustomerBillingNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsCreateCustomerBillingName {
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
pub struct CheckoutProductsCreateCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutProductsCreateCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsCreateCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsCreateCustomerMetadataKey {
    type Error = CheckoutProductsCreateCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsCreateCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutProductsCreateCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateCustomerMetadataKey> for String {
    fn from(value: CheckoutProductsCreateCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsCreateCustomerMetadataKey {
    type Error = CheckoutProductsCreateCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsCreateCustomerMetadataKey {
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
pub struct CheckoutProductsCreateCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutProductsCreateCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsCreateCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsCreateCustomerMetadataValueString {
    type Error = CheckoutProductsCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsCreateCustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutProductsCreateCustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateCustomerMetadataValueString> for String {
    fn from(value: CheckoutProductsCreateCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsCreateCustomerMetadataValueString {
    type Error = CheckoutProductsCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsCreateCustomerMetadataValueString {
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
pub struct CheckoutProductsCreateCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutProductsCreateCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsCreateCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsCreateCustomerName {
    type Error = CheckoutProductsCreateCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutProductsCreateCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateCustomerName> for String {
    fn from(value: CheckoutProductsCreateCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsCreateCustomerName {
    type Error = CheckoutProductsCreateCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsCreateCustomerName {
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
pub struct CheckoutProductsCreateMaxSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateMaxSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductsCreateMaxSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateMaxSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateMaxSeats {
    type Error = CheckoutProductsCreateMaxSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateMaxSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductsCreateMaxSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateMaxSeats> for u64 {
    fn from(value: CheckoutProductsCreateMaxSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateMaxSeats {
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
pub struct CheckoutProductsCreateMaxUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateMaxUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductsCreateMaxUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateMaxUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateMaxUnits {
    type Error = CheckoutProductsCreateMaxUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateMaxUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductsCreateMaxUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateMaxUnits> for u64 {
    fn from(value: CheckoutProductsCreateMaxUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateMaxUnits {
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
pub struct CheckoutProductsCreateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutProductsCreateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsCreateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsCreateMetadataKey {
    type Error = CheckoutProductsCreateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsCreateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutProductsCreateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateMetadataKey> for String {
    fn from(value: CheckoutProductsCreateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsCreateMetadataKey {
    type Error = CheckoutProductsCreateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsCreateMetadataKey {
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
pub struct CheckoutProductsCreateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutProductsCreateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsCreateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsCreateMetadataValueString {
    type Error = CheckoutProductsCreateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsCreateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutProductsCreateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateMetadataValueString> for String {
    fn from(value: CheckoutProductsCreateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsCreateMetadataValueString {
    type Error = CheckoutProductsCreateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsCreateMetadataValueString {
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
pub struct CheckoutProductsCreateMinSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateMinSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductsCreateMinSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateMinSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateMinSeats {
    type Error = CheckoutProductsCreateMinSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateMinSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductsCreateMinSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateMinSeats> for u64 {
    fn from(value: CheckoutProductsCreateMinSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateMinSeats {
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
pub struct CheckoutProductsCreateMinUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateMinUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductsCreateMinUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateMinUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateMinUnits {
    type Error = CheckoutProductsCreateMinUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateMinUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductsCreateMinUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateMinUnits> for u64 {
    fn from(value: CheckoutProductsCreateMinUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateMinUnits {
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
pub struct CheckoutProductsCreateReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutProductsCreateReturnUrl {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, CheckoutProductsCreateReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutProductsCreateReturnUrl {
    type Error = CheckoutProductsCreateReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsCreateReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutProductsCreateReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateReturnUrl> for url::Url {
    fn from(value: CheckoutProductsCreateReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutProductsCreateReturnUrl {
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
pub struct CheckoutProductsCreateSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductsCreateSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateSeats {
    type Error = CheckoutProductsCreateSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductsCreateSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateSeats> for u64 {
    fn from(value: CheckoutProductsCreateSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateSeats {
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
pub struct CheckoutProductsCreateSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutProductsCreateSuccessUrl {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, CheckoutProductsCreateSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutProductsCreateSuccessUrl {
    type Error = CheckoutProductsCreateSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsCreateSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutProductsCreateSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateSuccessUrl> for url::Url {
    fn from(value: CheckoutProductsCreateSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutProductsCreateSuccessUrl {
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
pub struct CheckoutProductsCreateTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutProductsCreateTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsCreateTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateTrialIntervalCount {
    type Error = CheckoutProductsCreateTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(CheckoutProductsCreateTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateTrialIntervalCount> for u64 {
    fn from(value: CheckoutProductsCreateTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateTrialIntervalCount {
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
pub struct CheckoutProductsCreateUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsCreateUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductsCreateUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsCreateUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsCreateUnits {
    type Error = CheckoutProductsCreateUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsCreateUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductsCreateUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsCreateUnits> for u64 {
    fn from(value: CheckoutProductsCreateUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsCreateUnits {
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
pub struct CheckoutProductsExternalCreateAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutProductsExternalCreateAmount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsExternalCreateAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateAmount {
    type Error = CheckoutProductsExternalCreateAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutProductsExternalCreateAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateAmount> for u64 {
    fn from(value: CheckoutProductsExternalCreateAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateAmount {
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
pub struct CheckoutProductsExternalCreateCustomerBillingName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateCustomerBillingNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateCustomerBillingName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsExternalCreateCustomerBillingNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsExternalCreateCustomerBillingName {
    type Error = CheckoutProductsExternalCreateCustomerBillingNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutProductsExternalCreateCustomerBillingNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateCustomerBillingName> for String {
    fn from(value: CheckoutProductsExternalCreateCustomerBillingName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateCustomerBillingName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsExternalCreateCustomerBillingName {
    type Error = CheckoutProductsExternalCreateCustomerBillingNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsExternalCreateCustomerBillingName {
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
pub struct CheckoutProductsExternalCreateCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsExternalCreateCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsExternalCreateCustomerMetadataKey {
    type Error = CheckoutProductsExternalCreateCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsExternalCreateCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutProductsExternalCreateCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateCustomerMetadataKey> for String {
    fn from(value: CheckoutProductsExternalCreateCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsExternalCreateCustomerMetadataKey {
    type Error = CheckoutProductsExternalCreateCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsExternalCreateCustomerMetadataKey {
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
pub struct CheckoutProductsExternalCreateCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsExternalCreateCustomerMetadataValueStringValidationError>
    {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsExternalCreateCustomerMetadataValueString {
    type Error = CheckoutProductsExternalCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(
                CheckoutProductsExternalCreateCustomerMetadataValueStringValidationError::TooShort,
            );
        }
        if length > 500u64 {
            return Err(
                CheckoutProductsExternalCreateCustomerMetadataValueStringValidationError::TooLong,
            );
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateCustomerMetadataValueString> for String {
    fn from(value: CheckoutProductsExternalCreateCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsExternalCreateCustomerMetadataValueString {
    type Error = CheckoutProductsExternalCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsExternalCreateCustomerMetadataValueString {
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
pub struct CheckoutProductsExternalCreateCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsExternalCreateCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsExternalCreateCustomerName {
    type Error = CheckoutProductsExternalCreateCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutProductsExternalCreateCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateCustomerName> for String {
    fn from(value: CheckoutProductsExternalCreateCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsExternalCreateCustomerName {
    type Error = CheckoutProductsExternalCreateCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsExternalCreateCustomerName {
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
pub struct CheckoutProductsExternalCreateMaxSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateMaxSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateMaxSeats {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsExternalCreateMaxSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateMaxSeats {
    type Error = CheckoutProductsExternalCreateMaxSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsExternalCreateMaxSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductsExternalCreateMaxSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateMaxSeats> for u64 {
    fn from(value: CheckoutProductsExternalCreateMaxSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateMaxSeats {
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
pub struct CheckoutProductsExternalCreateMaxUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateMaxUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateMaxUnits {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsExternalCreateMaxUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateMaxUnits {
    type Error = CheckoutProductsExternalCreateMaxUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsExternalCreateMaxUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductsExternalCreateMaxUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateMaxUnits> for u64 {
    fn from(value: CheckoutProductsExternalCreateMaxUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateMaxUnits {
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
pub struct CheckoutProductsExternalCreateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsExternalCreateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsExternalCreateMetadataKey {
    type Error = CheckoutProductsExternalCreateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsExternalCreateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutProductsExternalCreateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateMetadataKey> for String {
    fn from(value: CheckoutProductsExternalCreateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsExternalCreateMetadataKey {
    type Error = CheckoutProductsExternalCreateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsExternalCreateMetadataKey {
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
pub struct CheckoutProductsExternalCreateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutProductsExternalCreateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutProductsExternalCreateMetadataValueString {
    type Error = CheckoutProductsExternalCreateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsExternalCreateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutProductsExternalCreateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateMetadataValueString> for String {
    fn from(value: CheckoutProductsExternalCreateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutProductsExternalCreateMetadataValueString {
    type Error = CheckoutProductsExternalCreateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutProductsExternalCreateMetadataValueString {
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
pub struct CheckoutProductsExternalCreateMinSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateMinSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateMinSeats {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsExternalCreateMinSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateMinSeats {
    type Error = CheckoutProductsExternalCreateMinSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsExternalCreateMinSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductsExternalCreateMinSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateMinSeats> for u64 {
    fn from(value: CheckoutProductsExternalCreateMinSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateMinSeats {
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
pub struct CheckoutProductsExternalCreateMinUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateMinUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateMinUnits {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsExternalCreateMinUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateMinUnits {
    type Error = CheckoutProductsExternalCreateMinUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsExternalCreateMinUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductsExternalCreateMinUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateMinUnits> for u64 {
    fn from(value: CheckoutProductsExternalCreateMinUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateMinUnits {
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
pub struct CheckoutProductsExternalCreateReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateReturnUrl {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, CheckoutProductsExternalCreateReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutProductsExternalCreateReturnUrl {
    type Error = CheckoutProductsExternalCreateReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsExternalCreateReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutProductsExternalCreateReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateReturnUrl> for url::Url {
    fn from(value: CheckoutProductsExternalCreateReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutProductsExternalCreateReturnUrl {
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
pub struct CheckoutProductsExternalCreateSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsExternalCreateSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateSeats {
    type Error = CheckoutProductsExternalCreateSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsExternalCreateSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutProductsExternalCreateSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateSeats> for u64 {
    fn from(value: CheckoutProductsExternalCreateSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateSeats {
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
pub struct CheckoutProductsExternalCreateSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutProductsExternalCreateSuccessUrl {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, CheckoutProductsExternalCreateSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutProductsExternalCreateSuccessUrl {
    type Error = CheckoutProductsExternalCreateSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutProductsExternalCreateSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutProductsExternalCreateSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateSuccessUrl> for url::Url {
    fn from(value: CheckoutProductsExternalCreateSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutProductsExternalCreateSuccessUrl {
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
pub struct CheckoutProductsExternalCreateTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutProductsExternalCreateTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateTrialIntervalCount {
    type Error = CheckoutProductsExternalCreateTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(
                CheckoutProductsExternalCreateTrialIntervalCountValidationError::BelowMinimum,
            );
        }
        if i128::from(value) > 1000i128 {
            return Err(
                CheckoutProductsExternalCreateTrialIntervalCountValidationError::AboveMaximum,
            );
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateTrialIntervalCount> for u64 {
    fn from(value: CheckoutProductsExternalCreateTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateTrialIntervalCount {
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
pub struct CheckoutProductsExternalCreateUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutProductsExternalCreateUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutProductsExternalCreateUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutProductsExternalCreateUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutProductsExternalCreateUnits {
    type Error = CheckoutProductsExternalCreateUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutProductsExternalCreateUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutProductsExternalCreateUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutProductsExternalCreateUnits> for u64 {
    fn from(value: CheckoutProductsExternalCreateUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutProductsExternalCreateUnits {
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
pub struct CheckoutReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutReturnUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutReturnUrl {
    type Error = CheckoutReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutReturnUrl> for url::Url {
    fn from(value: CheckoutReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutReturnUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Number of seats for seat-based pricing.
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
pub struct CheckoutSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutSeats {
    type Error = CheckoutSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutSeats> for u64 {
    fn from(value: CheckoutSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSeats {
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
pub struct CheckoutSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutSuccessUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutSuccessUrl {
    type Error = CheckoutSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutSuccessUrl> for url::Url {
    fn from(value: CheckoutSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutSuccessUrl {
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
pub struct CheckoutTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutTrialIntervalCount {
    type Error = CheckoutTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(CheckoutTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutTrialIntervalCount> for u64 {
    fn from(value: CheckoutTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutTrialIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of units for unit-based pricing.
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
pub struct CheckoutUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutUnits {
    type Error = CheckoutUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutUnits> for u64 {
    fn from(value: CheckoutUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUnits {
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
pub struct CheckoutUpdatePublicAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutUpdatePublicAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl CheckoutUpdatePublicAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutUpdatePublicAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutUpdatePublicAmount {
    type Error = CheckoutUpdatePublicAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(CheckoutUpdatePublicAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutUpdatePublicAmount> for u64 {
    fn from(value: CheckoutUpdatePublicAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUpdatePublicAmount {
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
pub struct CheckoutUpdatePublicCustomerBillingName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutUpdatePublicCustomerBillingNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutUpdatePublicCustomerBillingName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutUpdatePublicCustomerBillingNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutUpdatePublicCustomerBillingName {
    type Error = CheckoutUpdatePublicCustomerBillingNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutUpdatePublicCustomerBillingNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutUpdatePublicCustomerBillingName> for String {
    fn from(value: CheckoutUpdatePublicCustomerBillingName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUpdatePublicCustomerBillingName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutUpdatePublicCustomerBillingName {
    type Error = CheckoutUpdatePublicCustomerBillingNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutUpdatePublicCustomerBillingName {
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
pub struct CheckoutUpdatePublicCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutUpdatePublicCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CheckoutUpdatePublicCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutUpdatePublicCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutUpdatePublicCustomerName {
    type Error = CheckoutUpdatePublicCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CheckoutUpdatePublicCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutUpdatePublicCustomerName> for String {
    fn from(value: CheckoutUpdatePublicCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUpdatePublicCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutUpdatePublicCustomerName {
    type Error = CheckoutUpdatePublicCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutUpdatePublicCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Number of seats for seat-based pricing.
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
pub struct CheckoutUpdatePublicSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutUpdatePublicSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutUpdatePublicSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutUpdatePublicSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutUpdatePublicSeats {
    type Error = CheckoutUpdatePublicSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutUpdatePublicSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutUpdatePublicSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutUpdatePublicSeats> for u64 {
    fn from(value: CheckoutUpdatePublicSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUpdatePublicSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of units for unit-based pricing.
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
pub struct CheckoutUpdatePublicUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutUpdatePublicUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutUpdatePublicUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutUpdatePublicUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutUpdatePublicUnits {
    type Error = CheckoutUpdatePublicUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutUpdatePublicUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutUpdatePublicUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutUpdatePublicUnits> for u64 {
    fn from(value: CheckoutUpdatePublicUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutUpdatePublicUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
