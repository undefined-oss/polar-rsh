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
pub struct CustomerIndividualMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerIndividualMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomerIndividualMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerIndividualMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerIndividualMetadataKey {
    type Error = CustomerIndividualMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerIndividualMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomerIndividualMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerIndividualMetadataKey> for String {
    fn from(value: CustomerIndividualMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerIndividualMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerIndividualMetadataKey {
    type Error = CustomerIndividualMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerIndividualMetadataKey {
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
pub struct CustomerIndividualMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerIndividualMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomerIndividualMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerIndividualMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerIndividualMetadataValueString {
    type Error = CustomerIndividualMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerIndividualMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomerIndividualMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerIndividualMetadataValueString> for String {
    fn from(value: CustomerIndividualMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerIndividualMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerIndividualMetadataValueString {
    type Error = CustomerIndividualMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerIndividualMetadataValueString {
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
pub struct CustomerIndividualName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerIndividualNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CustomerIndividualName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerIndividualNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerIndividualName {
    type Error = CustomerIndividualNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CustomerIndividualNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerIndividualName> for String {
    fn from(value: CustomerIndividualName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerIndividualName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerIndividualName {
    type Error = CustomerIndividualNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerIndividualName {
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
pub struct CustomerMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomerMetadataKey {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, CustomerMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerMetadataKey {
    type Error = CustomerMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomerMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerMetadataKey> for String {
    fn from(value: CustomerMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerMetadataKey {
    type Error = CustomerMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerMetadataKey {
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
pub struct CustomerMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomerMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerMetadataValueString {
    type Error = CustomerMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomerMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerMetadataValueString> for String {
    fn from(value: CustomerMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerMetadataValueString {
    type Error = CustomerMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerMetadataValueString {
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
pub struct CustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CustomerName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, CustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerName {
    type Error = CustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerName> for String {
    fn from(value: CustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerName {
    type Error = CustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerName {
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
pub struct CustomerTeamMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerTeamMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomerTeamMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerTeamMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerTeamMetadataKey {
    type Error = CustomerTeamMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerTeamMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomerTeamMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerTeamMetadataKey> for String {
    fn from(value: CustomerTeamMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTeamMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerTeamMetadataKey {
    type Error = CustomerTeamMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerTeamMetadataKey {
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
pub struct CustomerTeamMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerTeamMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomerTeamMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerTeamMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerTeamMetadataValueString {
    type Error = CustomerTeamMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerTeamMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomerTeamMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerTeamMetadataValueString> for String {
    fn from(value: CustomerTeamMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTeamMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerTeamMetadataValueString {
    type Error = CustomerTeamMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerTeamMetadataValueString {
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
pub struct CustomerTeamName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerTeamNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CustomerTeamName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, CustomerTeamNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerTeamName {
    type Error = CustomerTeamNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CustomerTeamNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerTeamName> for String {
    fn from(value: CustomerTeamName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerTeamName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerTeamName {
    type Error = CustomerTeamNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerTeamName {
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
pub struct CustomerUpdateExternalIdMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerUpdateExternalIdMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomerUpdateExternalIdMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerUpdateExternalIdMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerUpdateExternalIdMetadataKey {
    type Error = CustomerUpdateExternalIdMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerUpdateExternalIdMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomerUpdateExternalIdMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerUpdateExternalIdMetadataKey> for String {
    fn from(value: CustomerUpdateExternalIdMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerUpdateExternalIdMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerUpdateExternalIdMetadataKey {
    type Error = CustomerUpdateExternalIdMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerUpdateExternalIdMetadataKey {
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
pub struct CustomerUpdateExternalIdMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerUpdateExternalIdMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomerUpdateExternalIdMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerUpdateExternalIdMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerUpdateExternalIdMetadataValueString {
    type Error = CustomerUpdateExternalIdMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomerUpdateExternalIdMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomerUpdateExternalIdMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerUpdateExternalIdMetadataValueString> for String {
    fn from(value: CustomerUpdateExternalIdMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerUpdateExternalIdMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerUpdateExternalIdMetadataValueString {
    type Error = CustomerUpdateExternalIdMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerUpdateExternalIdMetadataValueString {
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
pub struct CustomerUpdateExternalIdName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerUpdateExternalIdNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CustomerUpdateExternalIdName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerUpdateExternalIdNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerUpdateExternalIdName {
    type Error = CustomerUpdateExternalIdNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CustomerUpdateExternalIdNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerUpdateExternalIdName> for String {
    fn from(value: CustomerUpdateExternalIdName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerUpdateExternalIdName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerUpdateExternalIdName {
    type Error = CustomerUpdateExternalIdNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerUpdateExternalIdName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the member.
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
pub struct MemberCreateFromCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MemberCreateFromCustomerNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl MemberCreateFromCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, MemberCreateFromCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MemberCreateFromCustomerName {
    type Error = MemberCreateFromCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(MemberCreateFromCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<MemberCreateFromCustomerName> for String {
    fn from(value: MemberCreateFromCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MemberCreateFromCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MemberCreateFromCustomerName {
    type Error = MemberCreateFromCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MemberCreateFromCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the member.
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
pub struct MemberName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MemberNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl MemberName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, MemberNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MemberName {
    type Error = MemberNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(MemberNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<MemberName> for String {
    fn from(value: MemberName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MemberName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MemberName {
    type Error = MemberNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MemberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the member.
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
pub struct MemberOwnerCreateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MemberOwnerCreateNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl MemberOwnerCreateName {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, MemberOwnerCreateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MemberOwnerCreateName {
    type Error = MemberOwnerCreateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(MemberOwnerCreateNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<MemberOwnerCreateName> for String {
    fn from(value: MemberOwnerCreateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MemberOwnerCreateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MemberOwnerCreateName {
    type Error = MemberOwnerCreateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MemberOwnerCreateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
