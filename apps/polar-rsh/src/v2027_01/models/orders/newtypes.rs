/// A custom amount to charge, in the smallest currency unit. Overrides the
/// product's price; defaults to the product's configured price (0 for free
/// products). A positive amount must be at least the currency's minimum. Can't
/// be combined with `units`.
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
pub struct OrderAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl OrderAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderAmount {
    type Error = OrderAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(OrderAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<OrderAmount> for u64 {
    fn from(value: OrderAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// A custom amount to charge, in the smallest currency unit. Overrides the
/// product's price; defaults to the product's configured price (0 for free
/// products). A positive amount must be at least the currency's minimum. Can't
/// be combined with `units`.
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
pub struct OrderCreateExternalAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl OrderCreateExternalAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderCreateExternalAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderCreateExternalAmount {
    type Error = OrderCreateExternalAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(OrderCreateExternalAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalAmount> for u64 {
    fn from(value: OrderCreateExternalAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// A custom amount to charge, in the smallest currency unit. Overrides the
/// product's price; defaults to the product's configured price (0 for free
/// products). A positive amount must be at least the currency's minimum. Can't
/// be combined with `units`.
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
pub struct OrderCreateExternalCustomerAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalCustomerAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl OrderCreateExternalCustomerAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderCreateExternalCustomerAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderCreateExternalCustomerAmount {
    type Error = OrderCreateExternalCustomerAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(OrderCreateExternalCustomerAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalCustomerAmount> for u64 {
    fn from(value: OrderCreateExternalCustomerAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalCustomerAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// A custom description for the order's line item, shown on the invoice and
/// receipt (e.g. `5,000 tokens`). Defaults to the product name.
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
pub struct OrderCreateExternalCustomerDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalCustomerDescriptionValidationError {
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderCreateExternalCustomerDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalCustomerDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalCustomerDescription {
    type Error = OrderCreateExternalCustomerDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 500u64 {
            return Err(OrderCreateExternalCustomerDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalCustomerDescription> for String {
    fn from(value: OrderCreateExternalCustomerDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalCustomerDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalCustomerDescription {
    type Error = OrderCreateExternalCustomerDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalCustomerDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The ID of the customer in your system the order is for. Must belong to the
/// order's organization.
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
pub struct OrderCreateExternalCustomerExternalCustomerId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalCustomerExternalCustomerIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl OrderCreateExternalCustomerExternalCustomerId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalCustomerExternalCustomerIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalCustomerExternalCustomerId {
    type Error = OrderCreateExternalCustomerExternalCustomerIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalCustomerExternalCustomerIdValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalCustomerExternalCustomerId> for String {
    fn from(value: OrderCreateExternalCustomerExternalCustomerId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalCustomerExternalCustomerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalCustomerExternalCustomerId {
    type Error = OrderCreateExternalCustomerExternalCustomerIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalCustomerExternalCustomerId {
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
pub struct OrderCreateExternalCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl OrderCreateExternalCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalCustomerMetadataKey {
    type Error = OrderCreateExternalCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(OrderCreateExternalCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalCustomerMetadataKey> for String {
    fn from(value: OrderCreateExternalCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalCustomerMetadataKey {
    type Error = OrderCreateExternalCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalCustomerMetadataKey {
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
pub struct OrderCreateExternalCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderCreateExternalCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalCustomerMetadataValueString {
    type Error = OrderCreateExternalCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalCustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(OrderCreateExternalCustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalCustomerMetadataValueString> for String {
    fn from(value: OrderCreateExternalCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalCustomerMetadataValueString {
    type Error = OrderCreateExternalCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalCustomerMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The number of units to charge for. Required when the product has unit-based
/// pricing, and rejected otherwise. The amount comes from the price's tiers.
/// Can't be combined with `amount`.
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
pub struct OrderCreateExternalCustomerUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalCustomerUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl OrderCreateExternalCustomerUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderCreateExternalCustomerUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderCreateExternalCustomerUnits {
    type Error = OrderCreateExternalCustomerUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(OrderCreateExternalCustomerUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(OrderCreateExternalCustomerUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalCustomerUnits> for u64 {
    fn from(value: OrderCreateExternalCustomerUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalCustomerUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// A custom description for the order's line item, shown on the invoice and
/// receipt (e.g. `5,000 tokens`). Defaults to the product name.
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
pub struct OrderCreateExternalDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalDescriptionValidationError {
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderCreateExternalDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalDescription {
    type Error = OrderCreateExternalDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 500u64 {
            return Err(OrderCreateExternalDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalDescription> for String {
    fn from(value: OrderCreateExternalDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalDescription {
    type Error = OrderCreateExternalDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The ID of the customer in your system the order is for. Must belong to the
/// order's organization.
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
pub struct OrderCreateExternalExternalCustomerId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalExternalCustomerIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl OrderCreateExternalExternalCustomerId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalExternalCustomerIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalExternalCustomerId {
    type Error = OrderCreateExternalExternalCustomerIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalExternalCustomerIdValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalExternalCustomerId> for String {
    fn from(value: OrderCreateExternalExternalCustomerId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalExternalCustomerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalExternalCustomerId {
    type Error = OrderCreateExternalExternalCustomerIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalExternalCustomerId {
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
pub struct OrderCreateExternalMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl OrderCreateExternalMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalMetadataKey {
    type Error = OrderCreateExternalMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(OrderCreateExternalMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalMetadataKey> for String {
    fn from(value: OrderCreateExternalMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalMetadataKey {
    type Error = OrderCreateExternalMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalMetadataKey {
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
pub struct OrderCreateExternalMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderCreateExternalMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalMetadataValueString {
    type Error = OrderCreateExternalMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(OrderCreateExternalMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalMetadataValueString> for String {
    fn from(value: OrderCreateExternalMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalMetadataValueString {
    type Error = OrderCreateExternalMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// A custom amount to charge, in the smallest currency unit. Overrides the
/// product's price; defaults to the product's configured price (0 for free
/// products). A positive amount must be at least the currency's minimum. Can't
/// be combined with `units`.
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
pub struct OrderCreateExternalProductAmount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalProductAmountValidationError {
    #[error("must be at least 0")]
    BelowMinimum,
}
impl OrderCreateExternalProductAmount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderCreateExternalProductAmountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderCreateExternalProductAmount {
    type Error = OrderCreateExternalProductAmountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 0i128 {
            return Err(OrderCreateExternalProductAmountValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalProductAmount> for u64 {
    fn from(value: OrderCreateExternalProductAmount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalProductAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// A custom description for the order's line item, shown on the invoice and
/// receipt (e.g. `5,000 tokens`). Defaults to the product name.
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
pub struct OrderCreateExternalProductDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalProductDescriptionValidationError {
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderCreateExternalProductDescription {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalProductDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalProductDescription {
    type Error = OrderCreateExternalProductDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 500u64 {
            return Err(OrderCreateExternalProductDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalProductDescription> for String {
    fn from(value: OrderCreateExternalProductDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalProductDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalProductDescription {
    type Error = OrderCreateExternalProductDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalProductDescription {
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
pub struct OrderCreateExternalProductMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalProductMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl OrderCreateExternalProductMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalProductMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalProductMetadataKey {
    type Error = OrderCreateExternalProductMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalProductMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(OrderCreateExternalProductMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalProductMetadataKey> for String {
    fn from(value: OrderCreateExternalProductMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalProductMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalProductMetadataKey {
    type Error = OrderCreateExternalProductMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalProductMetadataKey {
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
pub struct OrderCreateExternalProductMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalProductMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderCreateExternalProductMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderCreateExternalProductMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderCreateExternalProductMetadataValueString {
    type Error = OrderCreateExternalProductMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderCreateExternalProductMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(OrderCreateExternalProductMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalProductMetadataValueString> for String {
    fn from(value: OrderCreateExternalProductMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalProductMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderCreateExternalProductMetadataValueString {
    type Error = OrderCreateExternalProductMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderCreateExternalProductMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The number of units to charge for. Required when the product has unit-based
/// pricing, and rejected otherwise. The amount comes from the price's tiers.
/// Can't be combined with `amount`.
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
pub struct OrderCreateExternalProductUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalProductUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl OrderCreateExternalProductUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderCreateExternalProductUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderCreateExternalProductUnits {
    type Error = OrderCreateExternalProductUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(OrderCreateExternalProductUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(OrderCreateExternalProductUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalProductUnits> for u64 {
    fn from(value: OrderCreateExternalProductUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalProductUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// The number of units to charge for. Required when the product has unit-based
/// pricing, and rejected otherwise. The amount comes from the price's tiers.
/// Can't be combined with `amount`.
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
pub struct OrderCreateExternalUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderCreateExternalUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl OrderCreateExternalUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderCreateExternalUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderCreateExternalUnits {
    type Error = OrderCreateExternalUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(OrderCreateExternalUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(OrderCreateExternalUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<OrderCreateExternalUnits> for u64 {
    fn from(value: OrderCreateExternalUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderCreateExternalUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// A custom description for the order's line item, shown on the invoice and
/// receipt (e.g. `5,000 tokens`). Defaults to the product name.
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
pub struct OrderDescription(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderDescriptionValidationError {
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderDescription {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, OrderDescriptionValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderDescription {
    type Error = OrderDescriptionValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 500u64 {
            return Err(OrderDescriptionValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderDescription> for String {
    fn from(value: OrderDescription) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderDescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderDescription {
    type Error = OrderDescriptionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderDescription {
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
pub struct OrderMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl OrderMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, OrderMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderMetadataKey {
    type Error = OrderMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(OrderMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderMetadataKey> for String {
    fn from(value: OrderMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderMetadataKey {
    type Error = OrderMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderMetadataKey {
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
pub struct OrderMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl OrderMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, OrderMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for OrderMetadataValueString {
    type Error = OrderMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(OrderMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(OrderMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<OrderMetadataValueString> for String {
    fn from(value: OrderMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for OrderMetadataValueString {
    type Error = OrderMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for OrderMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The number of units to charge for. Required when the product has unit-based
/// pricing, and rejected otherwise. The amount comes from the price's tiers.
/// Can't be combined with `amount`.
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
pub struct OrderUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrderUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl OrderUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, OrderUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for OrderUnits {
    type Error = OrderUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(OrderUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(OrderUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<OrderUnits> for u64 {
    fn from(value: OrderUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrderUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
