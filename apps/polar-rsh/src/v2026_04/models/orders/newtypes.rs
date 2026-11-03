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
