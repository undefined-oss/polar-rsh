/// Your unique identifier for this credit. Retrying with the same `external_id`
/// doesn't grant the credit twice.
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
pub struct CustomerMeterCreditsExternalIdCreateExternalId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMeterCreditsExternalIdCreateExternalIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomerMeterCreditsExternalIdCreateExternalId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerMeterCreditsExternalIdCreateExternalIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerMeterCreditsExternalIdCreateExternalId {
    type Error = CustomerMeterCreditsExternalIdCreateExternalIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerMeterCreditsExternalIdCreateExternalIdValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomerMeterCreditsExternalIdCreateExternalId> for String {
    fn from(value: CustomerMeterCreditsExternalIdCreateExternalId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMeterCreditsExternalIdCreateExternalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerMeterCreditsExternalIdCreateExternalId {
    type Error = CustomerMeterCreditsExternalIdCreateExternalIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerMeterCreditsExternalIdCreateExternalId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The external ID of the meter, as set by config as code.
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
pub struct CustomerMeterCreditsExternalIdCreateExternalMeterId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMeterCreditsExternalIdCreateExternalMeterIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomerMeterCreditsExternalIdCreateExternalMeterId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerMeterCreditsExternalIdCreateExternalMeterIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerMeterCreditsExternalIdCreateExternalMeterId {
    type Error = CustomerMeterCreditsExternalIdCreateExternalMeterIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(
                CustomerMeterCreditsExternalIdCreateExternalMeterIdValidationError::TooShort,
            );
        }
        Ok(Self(value))
    }
}
impl From<CustomerMeterCreditsExternalIdCreateExternalMeterId> for String {
    fn from(value: CustomerMeterCreditsExternalIdCreateExternalMeterId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMeterCreditsExternalIdCreateExternalMeterId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerMeterCreditsExternalIdCreateExternalMeterId {
    type Error = CustomerMeterCreditsExternalIdCreateExternalMeterIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerMeterCreditsExternalIdCreateExternalMeterId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The number of units to credit.
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
pub struct CustomerMeterCreditsExternalIdCreateUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMeterCreditsExternalIdCreateUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000000000")]
    AboveMaximum,
}
impl CustomerMeterCreditsExternalIdCreateUnits {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CustomerMeterCreditsExternalIdCreateUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CustomerMeterCreditsExternalIdCreateUnits {
    type Error = CustomerMeterCreditsExternalIdCreateUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CustomerMeterCreditsExternalIdCreateUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000000000i128 {
            return Err(CustomerMeterCreditsExternalIdCreateUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomerMeterCreditsExternalIdCreateUnits> for u64 {
    fn from(value: CustomerMeterCreditsExternalIdCreateUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMeterCreditsExternalIdCreateUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Your unique identifier for this credit. Retrying with the same `external_id`
/// doesn't grant the credit twice.
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
pub struct CustomerMeterCreditsIdCreateExternalId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMeterCreditsIdCreateExternalIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomerMeterCreditsIdCreateExternalId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerMeterCreditsIdCreateExternalIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerMeterCreditsIdCreateExternalId {
    type Error = CustomerMeterCreditsIdCreateExternalIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerMeterCreditsIdCreateExternalIdValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomerMeterCreditsIdCreateExternalId> for String {
    fn from(value: CustomerMeterCreditsIdCreateExternalId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMeterCreditsIdCreateExternalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerMeterCreditsIdCreateExternalId {
    type Error = CustomerMeterCreditsIdCreateExternalIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerMeterCreditsIdCreateExternalId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The number of units to credit.
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
pub struct CustomerMeterCreditsIdCreateUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMeterCreditsIdCreateUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000000000")]
    AboveMaximum,
}
impl CustomerMeterCreditsIdCreateUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CustomerMeterCreditsIdCreateUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CustomerMeterCreditsIdCreateUnits {
    type Error = CustomerMeterCreditsIdCreateUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CustomerMeterCreditsIdCreateUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000000000i128 {
            return Err(CustomerMeterCreditsIdCreateUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CustomerMeterCreditsIdCreateUnits> for u64 {
    fn from(value: CustomerMeterCreditsIdCreateUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMeterCreditsIdCreateUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
