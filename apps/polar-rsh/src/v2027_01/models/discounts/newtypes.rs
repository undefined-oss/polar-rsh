/// Fixed amount to discount from the invoice total.
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
pub struct DiscountAmountsValue(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountAmountsValueValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 999999999999")]
    AboveMaximum,
}
impl DiscountAmountsValue {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountAmountsValueValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountAmountsValue {
    type Error = DiscountAmountsValueValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(DiscountAmountsValueValidationError::BelowMinimum);
        }
        if i128::from(value) > 999999999999i128 {
            return Err(DiscountAmountsValueValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountAmountsValue> for u64 {
    fn from(value: DiscountAmountsValue) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountAmountsValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Discount percentage in basis points.
///
/// A basis point is 1/100th of a percent.
/// For example, to create a 25.5% discount, set this to 2550.
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
pub struct DiscountBasisPoints(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountBasisPointsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl DiscountBasisPoints {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountBasisPointsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountBasisPoints {
    type Error = DiscountBasisPointsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountBasisPointsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(DiscountBasisPointsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountBasisPoints> for u64 {
    fn from(value: DiscountBasisPoints) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountBasisPoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of months the discount should be applied.
///
/// For this to work on yearly pricing, you should multiply this by 12.
/// For example, to apply the discount for 2 years, set this to 24.
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
pub struct DiscountDurationInMonths(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountDurationInMonthsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 999")]
    AboveMaximum,
}
impl DiscountDurationInMonths {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountDurationInMonthsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountDurationInMonths {
    type Error = DiscountDurationInMonthsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountDurationInMonthsValidationError::BelowMinimum);
        }
        if i128::from(value) > 999i128 {
            return Err(DiscountDurationInMonthsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountDurationInMonths> for u64 {
    fn from(value: DiscountDurationInMonths) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountDurationInMonths {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Fixed amount to discount from the invoice total.
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
pub struct DiscountFixedCreateAmountsValue(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateAmountsValueValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
    #[error("must be at most 999999999999")]
    AboveMaximum,
}
impl DiscountFixedCreateAmountsValue {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountFixedCreateAmountsValueValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountFixedCreateAmountsValue {
    type Error = DiscountFixedCreateAmountsValueValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(DiscountFixedCreateAmountsValueValidationError::BelowMinimum);
        }
        if i128::from(value) > 999999999999i128 {
            return Err(DiscountFixedCreateAmountsValueValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateAmountsValue> for u64 {
    fn from(value: DiscountFixedCreateAmountsValue) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateAmountsValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of months the discount should be applied.
///
/// Required when `duration` is `repeating`. Must be omitted otherwise.
///
/// For this to work on yearly pricing, you should multiply this by 12.
/// For example, to apply the discount for 2 years, set this to 24.
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
pub struct DiscountFixedCreateDurationInMonths(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateDurationInMonthsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 999")]
    AboveMaximum,
}
impl DiscountFixedCreateDurationInMonths {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountFixedCreateDurationInMonthsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountFixedCreateDurationInMonths {
    type Error = DiscountFixedCreateDurationInMonthsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountFixedCreateDurationInMonthsValidationError::BelowMinimum);
        }
        if i128::from(value) > 999i128 {
            return Err(DiscountFixedCreateDurationInMonthsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateDurationInMonths> for u64 {
    fn from(value: DiscountFixedCreateDurationInMonths) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateDurationInMonths {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum number of times the discount can be redeemed.
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
pub struct DiscountFixedCreateMaxRedemptions(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateMaxRedemptionsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl DiscountFixedCreateMaxRedemptions {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountFixedCreateMaxRedemptionsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountFixedCreateMaxRedemptions {
    type Error = DiscountFixedCreateMaxRedemptionsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountFixedCreateMaxRedemptionsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(DiscountFixedCreateMaxRedemptionsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateMaxRedemptions> for u64 {
    fn from(value: DiscountFixedCreateMaxRedemptions) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateMaxRedemptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum number of times the discount can be redeemed by a single
/// customer.
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
pub struct DiscountFixedCreateMaxRedemptionsPerCustomer(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateMaxRedemptionsPerCustomerValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl DiscountFixedCreateMaxRedemptionsPerCustomer {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, DiscountFixedCreateMaxRedemptionsPerCustomerValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountFixedCreateMaxRedemptionsPerCustomer {
    type Error = DiscountFixedCreateMaxRedemptionsPerCustomerValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountFixedCreateMaxRedemptionsPerCustomerValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(DiscountFixedCreateMaxRedemptionsPerCustomerValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateMaxRedemptionsPerCustomer> for u64 {
    fn from(value: DiscountFixedCreateMaxRedemptionsPerCustomer) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateMaxRedemptionsPerCustomer {
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
pub struct DiscountFixedCreateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl DiscountFixedCreateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountFixedCreateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountFixedCreateMetadataKey {
    type Error = DiscountFixedCreateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountFixedCreateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(DiscountFixedCreateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateMetadataKey> for String {
    fn from(value: DiscountFixedCreateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountFixedCreateMetadataKey {
    type Error = DiscountFixedCreateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountFixedCreateMetadataKey {
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
pub struct DiscountFixedCreateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl DiscountFixedCreateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountFixedCreateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountFixedCreateMetadataValueString {
    type Error = DiscountFixedCreateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountFixedCreateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(DiscountFixedCreateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateMetadataValueString> for String {
    fn from(value: DiscountFixedCreateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountFixedCreateMetadataValueString {
    type Error = DiscountFixedCreateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountFixedCreateMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the discount. Will be displayed to the customer when the discount is
/// applied.
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
pub struct DiscountFixedCreateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountFixedCreateNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl DiscountFixedCreateName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountFixedCreateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountFixedCreateName {
    type Error = DiscountFixedCreateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountFixedCreateNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<DiscountFixedCreateName> for String {
    fn from(value: DiscountFixedCreateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountFixedCreateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountFixedCreateName {
    type Error = DiscountFixedCreateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountFixedCreateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Optional maximum number of times the discount can be redeemed.
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
pub struct DiscountMaxRedemptions(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountMaxRedemptionsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl DiscountMaxRedemptions {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountMaxRedemptionsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountMaxRedemptions {
    type Error = DiscountMaxRedemptionsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountMaxRedemptionsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(DiscountMaxRedemptionsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountMaxRedemptions> for u64 {
    fn from(value: DiscountMaxRedemptions) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountMaxRedemptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum number of times the discount can be redeemed by a single
/// customer.
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
pub struct DiscountMaxRedemptionsPerCustomer(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountMaxRedemptionsPerCustomerValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl DiscountMaxRedemptionsPerCustomer {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountMaxRedemptionsPerCustomerValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountMaxRedemptionsPerCustomer {
    type Error = DiscountMaxRedemptionsPerCustomerValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountMaxRedemptionsPerCustomerValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(DiscountMaxRedemptionsPerCustomerValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountMaxRedemptionsPerCustomer> for u64 {
    fn from(value: DiscountMaxRedemptionsPerCustomer) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountMaxRedemptionsPerCustomer {
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
pub struct DiscountMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl DiscountMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, DiscountMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountMetadataKey {
    type Error = DiscountMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(DiscountMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<DiscountMetadataKey> for String {
    fn from(value: DiscountMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountMetadataKey {
    type Error = DiscountMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountMetadataKey {
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
pub struct DiscountMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl DiscountMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountMetadataValueString {
    type Error = DiscountMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(DiscountMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<DiscountMetadataValueString> for String {
    fn from(value: DiscountMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountMetadataValueString {
    type Error = DiscountMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the discount. Will be displayed to the customer when the discount is
/// applied.
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
pub struct DiscountName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl DiscountName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, DiscountNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountName {
    type Error = DiscountNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<DiscountName> for String {
    fn from(value: DiscountName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountName {
    type Error = DiscountNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Discount percentage in basis points.
///
/// A basis point is 1/100th of a percent.
/// For example, to create a 25.5% discount, set this to 2550.
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
pub struct DiscountPercentageCreateBasisPoints(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateBasisPointsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl DiscountPercentageCreateBasisPoints {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, DiscountPercentageCreateBasisPointsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountPercentageCreateBasisPoints {
    type Error = DiscountPercentageCreateBasisPointsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountPercentageCreateBasisPointsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(DiscountPercentageCreateBasisPointsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateBasisPoints> for u64 {
    fn from(value: DiscountPercentageCreateBasisPoints) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateBasisPoints {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Number of months the discount should be applied.
///
/// Required when `duration` is `repeating`. Must be omitted otherwise.
///
/// For this to work on yearly pricing, you should multiply this by 12.
/// For example, to apply the discount for 2 years, set this to 24.
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
pub struct DiscountPercentageCreateDurationInMonths(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateDurationInMonthsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 999")]
    AboveMaximum,
}
impl DiscountPercentageCreateDurationInMonths {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, DiscountPercentageCreateDurationInMonthsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountPercentageCreateDurationInMonths {
    type Error = DiscountPercentageCreateDurationInMonthsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountPercentageCreateDurationInMonthsValidationError::BelowMinimum);
        }
        if i128::from(value) > 999i128 {
            return Err(DiscountPercentageCreateDurationInMonthsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateDurationInMonths> for u64 {
    fn from(value: DiscountPercentageCreateDurationInMonths) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateDurationInMonths {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum number of times the discount can be redeemed.
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
pub struct DiscountPercentageCreateMaxRedemptions(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateMaxRedemptionsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl DiscountPercentageCreateMaxRedemptions {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, DiscountPercentageCreateMaxRedemptionsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountPercentageCreateMaxRedemptions {
    type Error = DiscountPercentageCreateMaxRedemptionsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(DiscountPercentageCreateMaxRedemptionsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(DiscountPercentageCreateMaxRedemptionsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateMaxRedemptions> for u64 {
    fn from(value: DiscountPercentageCreateMaxRedemptions) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateMaxRedemptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Optional maximum number of times the discount can be redeemed by a single
/// customer.
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
pub struct DiscountPercentageCreateMaxRedemptionsPerCustomer(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateMaxRedemptionsPerCustomerValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl DiscountPercentageCreateMaxRedemptionsPerCustomer {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, DiscountPercentageCreateMaxRedemptionsPerCustomerValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for DiscountPercentageCreateMaxRedemptionsPerCustomer {
    type Error = DiscountPercentageCreateMaxRedemptionsPerCustomerValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(
                DiscountPercentageCreateMaxRedemptionsPerCustomerValidationError::BelowMinimum,
            );
        }
        if i128::from(value) > 2147483647i128 {
            return Err(
                DiscountPercentageCreateMaxRedemptionsPerCustomerValidationError::AboveMaximum,
            );
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateMaxRedemptionsPerCustomer> for u64 {
    fn from(value: DiscountPercentageCreateMaxRedemptionsPerCustomer) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateMaxRedemptionsPerCustomer {
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
pub struct DiscountPercentageCreateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl DiscountPercentageCreateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountPercentageCreateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountPercentageCreateMetadataKey {
    type Error = DiscountPercentageCreateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountPercentageCreateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(DiscountPercentageCreateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateMetadataKey> for String {
    fn from(value: DiscountPercentageCreateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountPercentageCreateMetadataKey {
    type Error = DiscountPercentageCreateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountPercentageCreateMetadataKey {
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
pub struct DiscountPercentageCreateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl DiscountPercentageCreateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountPercentageCreateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountPercentageCreateMetadataValueString {
    type Error = DiscountPercentageCreateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountPercentageCreateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(DiscountPercentageCreateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateMetadataValueString> for String {
    fn from(value: DiscountPercentageCreateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountPercentageCreateMetadataValueString {
    type Error = DiscountPercentageCreateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountPercentageCreateMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the discount. Will be displayed to the customer when the discount is
/// applied.
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
pub struct DiscountPercentageCreateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DiscountPercentageCreateNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl DiscountPercentageCreateName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, DiscountPercentageCreateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for DiscountPercentageCreateName {
    type Error = DiscountPercentageCreateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(DiscountPercentageCreateNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<DiscountPercentageCreateName> for String {
    fn from(value: DiscountPercentageCreateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for DiscountPercentageCreateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for DiscountPercentageCreateName {
    type Error = DiscountPercentageCreateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for DiscountPercentageCreateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
