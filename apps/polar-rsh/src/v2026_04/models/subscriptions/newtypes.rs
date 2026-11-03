/// Update the number of seats for this subscription.
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
pub struct CustomerSubscriptionUpdateSeatsSeats(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerSubscriptionUpdateSeatsSeatsValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomerSubscriptionUpdateSeatsSeats {
    /// Validate a value.
    pub fn try_new(
        value: i64,
    ) -> Result<Self, CustomerSubscriptionUpdateSeatsSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for CustomerSubscriptionUpdateSeatsSeats {
    type Error = CustomerSubscriptionUpdateSeatsSeatsValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(CustomerSubscriptionUpdateSeatsSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomerSubscriptionUpdateSeatsSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomerSubscriptionUpdateSeatsSeats> for i64 {
    fn from(value: CustomerSubscriptionUpdateSeatsSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerSubscriptionUpdateSeatsSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Update the number of units for this subscription.
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
pub struct CustomerSubscriptionUpdateUnitsUnits(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerSubscriptionUpdateUnitsUnitsValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CustomerSubscriptionUpdateUnitsUnits {
    /// Validate a value.
    pub fn try_new(
        value: i64,
    ) -> Result<Self, CustomerSubscriptionUpdateUnitsUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for CustomerSubscriptionUpdateUnitsUnits {
    type Error = CustomerSubscriptionUpdateUnitsUnitsValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(CustomerSubscriptionUpdateUnitsUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CustomerSubscriptionUpdateUnitsUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomerSubscriptionUpdateUnitsUnits> for i64 {
    fn from(value: CustomerSubscriptionUpdateUnitsUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerSubscriptionUpdateUnitsUnits {
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
pub struct SubscriptionCreateCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionCreateCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl SubscriptionCreateCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, SubscriptionCreateCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SubscriptionCreateCustomerMetadataKey {
    type Error = SubscriptionCreateCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(SubscriptionCreateCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(SubscriptionCreateCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionCreateCustomerMetadataKey> for String {
    fn from(value: SubscriptionCreateCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCreateCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for SubscriptionCreateCustomerMetadataKey {
    type Error = SubscriptionCreateCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for SubscriptionCreateCustomerMetadataKey {
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
pub struct SubscriptionCreateCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionCreateCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl SubscriptionCreateCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, SubscriptionCreateCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SubscriptionCreateCustomerMetadataValueString {
    type Error = SubscriptionCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(SubscriptionCreateCustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(SubscriptionCreateCustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionCreateCustomerMetadataValueString> for String {
    fn from(value: SubscriptionCreateCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCreateCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for SubscriptionCreateCustomerMetadataValueString {
    type Error = SubscriptionCreateCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for SubscriptionCreateCustomerMetadataValueString {
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
pub struct SubscriptionCreateExternalCustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionCreateExternalCustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl SubscriptionCreateExternalCustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, SubscriptionCreateExternalCustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SubscriptionCreateExternalCustomerMetadataKey {
    type Error = SubscriptionCreateExternalCustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(SubscriptionCreateExternalCustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(SubscriptionCreateExternalCustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionCreateExternalCustomerMetadataKey> for String {
    fn from(value: SubscriptionCreateExternalCustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCreateExternalCustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for SubscriptionCreateExternalCustomerMetadataKey {
    type Error = SubscriptionCreateExternalCustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for SubscriptionCreateExternalCustomerMetadataKey {
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
pub struct SubscriptionCreateExternalCustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionCreateExternalCustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl SubscriptionCreateExternalCustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, SubscriptionCreateExternalCustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SubscriptionCreateExternalCustomerMetadataValueString {
    type Error = SubscriptionCreateExternalCustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(
                SubscriptionCreateExternalCustomerMetadataValueStringValidationError::TooShort,
            );
        }
        if length > 500u64 {
            return Err(
                SubscriptionCreateExternalCustomerMetadataValueStringValidationError::TooLong,
            );
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionCreateExternalCustomerMetadataValueString> for String {
    fn from(value: SubscriptionCreateExternalCustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionCreateExternalCustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for SubscriptionCreateExternalCustomerMetadataValueString {
    type Error = SubscriptionCreateExternalCustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for SubscriptionCreateExternalCustomerMetadataValueString {
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
pub struct SubscriptionUpdateBaseMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionUpdateBaseMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl SubscriptionUpdateBaseMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, SubscriptionUpdateBaseMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SubscriptionUpdateBaseMetadataKey {
    type Error = SubscriptionUpdateBaseMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(SubscriptionUpdateBaseMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(SubscriptionUpdateBaseMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionUpdateBaseMetadataKey> for String {
    fn from(value: SubscriptionUpdateBaseMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionUpdateBaseMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for SubscriptionUpdateBaseMetadataKey {
    type Error = SubscriptionUpdateBaseMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for SubscriptionUpdateBaseMetadataKey {
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
pub struct SubscriptionUpdateBaseMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionUpdateBaseMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl SubscriptionUpdateBaseMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, SubscriptionUpdateBaseMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for SubscriptionUpdateBaseMetadataValueString {
    type Error = SubscriptionUpdateBaseMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(SubscriptionUpdateBaseMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(SubscriptionUpdateBaseMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionUpdateBaseMetadataValueString> for String {
    fn from(value: SubscriptionUpdateBaseMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionUpdateBaseMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for SubscriptionUpdateBaseMetadataValueString {
    type Error = SubscriptionUpdateBaseMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for SubscriptionUpdateBaseMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Update the number of seats for this subscription.
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
pub struct SubscriptionUpdateSeatsSeats(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionUpdateSeatsSeatsValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl SubscriptionUpdateSeatsSeats {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, SubscriptionUpdateSeatsSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for SubscriptionUpdateSeatsSeats {
    type Error = SubscriptionUpdateSeatsSeatsValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(SubscriptionUpdateSeatsSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(SubscriptionUpdateSeatsSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionUpdateSeatsSeats> for i64 {
    fn from(value: SubscriptionUpdateSeatsSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionUpdateSeatsSeats {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Update the number of units for this subscription.
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
pub struct SubscriptionUpdateUnitsUnits(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SubscriptionUpdateUnitsUnitsValidationError {
    #[error("must be at least -2147483648")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl SubscriptionUpdateUnitsUnits {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, SubscriptionUpdateUnitsUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for SubscriptionUpdateUnitsUnits {
    type Error = SubscriptionUpdateUnitsUnitsValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) < -2147483648i128 {
            return Err(SubscriptionUpdateUnitsUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(SubscriptionUpdateUnitsUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<SubscriptionUpdateUnitsUnits> for i64 {
    fn from(value: SubscriptionUpdateUnitsUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionUpdateUnitsUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
