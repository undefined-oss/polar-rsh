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
pub struct CustomFieldCreateCheckboxMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateCheckboxMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldCreateCheckboxMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateCheckboxMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateCheckboxMetadataKey {
    type Error = CustomFieldCreateCheckboxMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateCheckboxMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldCreateCheckboxMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateCheckboxMetadataKey> for String {
    fn from(value: CustomFieldCreateCheckboxMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateCheckboxMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateCheckboxMetadataKey {
    type Error = CustomFieldCreateCheckboxMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateCheckboxMetadataKey {
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
pub struct CustomFieldCreateCheckboxMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateCheckboxMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldCreateCheckboxMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateCheckboxMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateCheckboxMetadataValueString {
    type Error = CustomFieldCreateCheckboxMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateCheckboxMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldCreateCheckboxMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateCheckboxMetadataValueString> for String {
    fn from(value: CustomFieldCreateCheckboxMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateCheckboxMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateCheckboxMetadataValueString {
    type Error = CustomFieldCreateCheckboxMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateCheckboxMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldCreateCheckboxName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateCheckboxNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateCheckboxName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateCheckboxNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateCheckboxName {
    type Error = CustomFieldCreateCheckboxNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateCheckboxNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateCheckboxName> for String {
    fn from(value: CustomFieldCreateCheckboxName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateCheckboxName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateCheckboxName {
    type Error = CustomFieldCreateCheckboxNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateCheckboxName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldCreateCheckboxSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateCheckboxSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateCheckboxSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateCheckboxSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateCheckboxSlug {
    type Error = CustomFieldCreateCheckboxSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateCheckboxSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateCheckboxSlug> for String {
    fn from(value: CustomFieldCreateCheckboxSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateCheckboxSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateCheckboxSlug {
    type Error = CustomFieldCreateCheckboxSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateCheckboxSlug {
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
pub struct CustomFieldCreateDateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateDateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldCreateDateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateDateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateDateMetadataKey {
    type Error = CustomFieldCreateDateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateDateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldCreateDateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateDateMetadataKey> for String {
    fn from(value: CustomFieldCreateDateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateDateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateDateMetadataKey {
    type Error = CustomFieldCreateDateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateDateMetadataKey {
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
pub struct CustomFieldCreateDateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateDateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldCreateDateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateDateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateDateMetadataValueString {
    type Error = CustomFieldCreateDateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateDateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldCreateDateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateDateMetadataValueString> for String {
    fn from(value: CustomFieldCreateDateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateDateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateDateMetadataValueString {
    type Error = CustomFieldCreateDateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateDateMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldCreateDateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateDateNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateDateName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateDateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateDateName {
    type Error = CustomFieldCreateDateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateDateNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateDateName> for String {
    fn from(value: CustomFieldCreateDateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateDateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateDateName {
    type Error = CustomFieldCreateDateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateDateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldCreateDateSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateDateSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateDateSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateDateSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateDateSlug {
    type Error = CustomFieldCreateDateSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateDateSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateDateSlug> for String {
    fn from(value: CustomFieldCreateDateSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateDateSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateDateSlug {
    type Error = CustomFieldCreateDateSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateDateSlug {
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
pub struct CustomFieldCreateNumberMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateNumberMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldCreateNumberMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateNumberMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateNumberMetadataKey {
    type Error = CustomFieldCreateNumberMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateNumberMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldCreateNumberMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateNumberMetadataKey> for String {
    fn from(value: CustomFieldCreateNumberMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateNumberMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateNumberMetadataKey {
    type Error = CustomFieldCreateNumberMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateNumberMetadataKey {
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
pub struct CustomFieldCreateNumberMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateNumberMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldCreateNumberMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateNumberMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateNumberMetadataValueString {
    type Error = CustomFieldCreateNumberMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateNumberMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldCreateNumberMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateNumberMetadataValueString> for String {
    fn from(value: CustomFieldCreateNumberMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateNumberMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateNumberMetadataValueString {
    type Error = CustomFieldCreateNumberMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateNumberMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldCreateNumberName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateNumberNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateNumberName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateNumberNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateNumberName {
    type Error = CustomFieldCreateNumberNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateNumberNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateNumberName> for String {
    fn from(value: CustomFieldCreateNumberName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateNumberName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateNumberName {
    type Error = CustomFieldCreateNumberNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateNumberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldCreateNumberSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateNumberSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateNumberSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateNumberSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateNumberSlug {
    type Error = CustomFieldCreateNumberSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateNumberSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateNumberSlug> for String {
    fn from(value: CustomFieldCreateNumberSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateNumberSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateNumberSlug {
    type Error = CustomFieldCreateNumberSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateNumberSlug {
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
pub struct CustomFieldCreateSelectMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateSelectMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldCreateSelectMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateSelectMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateSelectMetadataKey {
    type Error = CustomFieldCreateSelectMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateSelectMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldCreateSelectMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateSelectMetadataKey> for String {
    fn from(value: CustomFieldCreateSelectMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateSelectMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateSelectMetadataKey {
    type Error = CustomFieldCreateSelectMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateSelectMetadataKey {
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
pub struct CustomFieldCreateSelectMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateSelectMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldCreateSelectMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateSelectMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateSelectMetadataValueString {
    type Error = CustomFieldCreateSelectMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateSelectMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldCreateSelectMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateSelectMetadataValueString> for String {
    fn from(value: CustomFieldCreateSelectMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateSelectMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateSelectMetadataValueString {
    type Error = CustomFieldCreateSelectMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateSelectMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldCreateSelectName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateSelectNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateSelectName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateSelectNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateSelectName {
    type Error = CustomFieldCreateSelectNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateSelectNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateSelectName> for String {
    fn from(value: CustomFieldCreateSelectName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateSelectName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateSelectName {
    type Error = CustomFieldCreateSelectNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateSelectName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldCreateSelectSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateSelectSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateSelectSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateSelectSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateSelectSlug {
    type Error = CustomFieldCreateSelectSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateSelectSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateSelectSlug> for String {
    fn from(value: CustomFieldCreateSelectSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateSelectSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateSelectSlug {
    type Error = CustomFieldCreateSelectSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateSelectSlug {
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
pub struct CustomFieldCreateTextMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateTextMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldCreateTextMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateTextMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateTextMetadataKey {
    type Error = CustomFieldCreateTextMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateTextMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldCreateTextMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateTextMetadataKey> for String {
    fn from(value: CustomFieldCreateTextMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateTextMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateTextMetadataKey {
    type Error = CustomFieldCreateTextMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateTextMetadataKey {
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
pub struct CustomFieldCreateTextMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateTextMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldCreateTextMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateTextMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateTextMetadataValueString {
    type Error = CustomFieldCreateTextMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateTextMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldCreateTextMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateTextMetadataValueString> for String {
    fn from(value: CustomFieldCreateTextMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateTextMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateTextMetadataValueString {
    type Error = CustomFieldCreateTextMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateTextMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldCreateTextName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateTextNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateTextName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateTextNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateTextName {
    type Error = CustomFieldCreateTextNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateTextNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateTextName> for String {
    fn from(value: CustomFieldCreateTextName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateTextName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateTextName {
    type Error = CustomFieldCreateTextNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateTextName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldCreateTextSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldCreateTextSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldCreateTextSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldCreateTextSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldCreateTextSlug {
    type Error = CustomFieldCreateTextSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldCreateTextSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldCreateTextSlug> for String {
    fn from(value: CustomFieldCreateTextSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldCreateTextSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldCreateTextSlug {
    type Error = CustomFieldCreateTextSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldCreateTextSlug {
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
pub struct CustomFieldUpdateCheckboxMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateCheckboxMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldUpdateCheckboxMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateCheckboxMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateCheckboxMetadataKey {
    type Error = CustomFieldUpdateCheckboxMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateCheckboxMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldUpdateCheckboxMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateCheckboxMetadataKey> for String {
    fn from(value: CustomFieldUpdateCheckboxMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateCheckboxMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateCheckboxMetadataKey {
    type Error = CustomFieldUpdateCheckboxMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateCheckboxMetadataKey {
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
pub struct CustomFieldUpdateCheckboxMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateCheckboxMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldUpdateCheckboxMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateCheckboxMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateCheckboxMetadataValueString {
    type Error = CustomFieldUpdateCheckboxMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateCheckboxMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldUpdateCheckboxMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateCheckboxMetadataValueString> for String {
    fn from(value: CustomFieldUpdateCheckboxMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateCheckboxMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateCheckboxMetadataValueString {
    type Error = CustomFieldUpdateCheckboxMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateCheckboxMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldUpdateCheckboxName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateCheckboxNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateCheckboxName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateCheckboxNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateCheckboxName {
    type Error = CustomFieldUpdateCheckboxNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateCheckboxNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateCheckboxName> for String {
    fn from(value: CustomFieldUpdateCheckboxName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateCheckboxName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateCheckboxName {
    type Error = CustomFieldUpdateCheckboxNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateCheckboxName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldUpdateCheckboxSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateCheckboxSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateCheckboxSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateCheckboxSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateCheckboxSlug {
    type Error = CustomFieldUpdateCheckboxSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateCheckboxSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateCheckboxSlug> for String {
    fn from(value: CustomFieldUpdateCheckboxSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateCheckboxSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateCheckboxSlug {
    type Error = CustomFieldUpdateCheckboxSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateCheckboxSlug {
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
pub struct CustomFieldUpdateDateMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateDateMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldUpdateDateMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateDateMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateDateMetadataKey {
    type Error = CustomFieldUpdateDateMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateDateMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldUpdateDateMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateDateMetadataKey> for String {
    fn from(value: CustomFieldUpdateDateMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateDateMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateDateMetadataKey {
    type Error = CustomFieldUpdateDateMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateDateMetadataKey {
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
pub struct CustomFieldUpdateDateMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateDateMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldUpdateDateMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateDateMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateDateMetadataValueString {
    type Error = CustomFieldUpdateDateMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateDateMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldUpdateDateMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateDateMetadataValueString> for String {
    fn from(value: CustomFieldUpdateDateMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateDateMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateDateMetadataValueString {
    type Error = CustomFieldUpdateDateMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateDateMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldUpdateDateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateDateNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateDateName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateDateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateDateName {
    type Error = CustomFieldUpdateDateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateDateNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateDateName> for String {
    fn from(value: CustomFieldUpdateDateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateDateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateDateName {
    type Error = CustomFieldUpdateDateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateDateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldUpdateDateSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateDateSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateDateSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateDateSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateDateSlug {
    type Error = CustomFieldUpdateDateSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateDateSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateDateSlug> for String {
    fn from(value: CustomFieldUpdateDateSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateDateSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateDateSlug {
    type Error = CustomFieldUpdateDateSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateDateSlug {
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
pub struct CustomFieldUpdateNumberMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateNumberMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldUpdateNumberMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateNumberMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateNumberMetadataKey {
    type Error = CustomFieldUpdateNumberMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateNumberMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldUpdateNumberMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateNumberMetadataKey> for String {
    fn from(value: CustomFieldUpdateNumberMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateNumberMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateNumberMetadataKey {
    type Error = CustomFieldUpdateNumberMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateNumberMetadataKey {
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
pub struct CustomFieldUpdateNumberMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateNumberMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldUpdateNumberMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateNumberMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateNumberMetadataValueString {
    type Error = CustomFieldUpdateNumberMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateNumberMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldUpdateNumberMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateNumberMetadataValueString> for String {
    fn from(value: CustomFieldUpdateNumberMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateNumberMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateNumberMetadataValueString {
    type Error = CustomFieldUpdateNumberMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateNumberMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldUpdateNumberName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateNumberNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateNumberName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateNumberNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateNumberName {
    type Error = CustomFieldUpdateNumberNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateNumberNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateNumberName> for String {
    fn from(value: CustomFieldUpdateNumberName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateNumberName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateNumberName {
    type Error = CustomFieldUpdateNumberNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateNumberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldUpdateNumberSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateNumberSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateNumberSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateNumberSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateNumberSlug {
    type Error = CustomFieldUpdateNumberSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateNumberSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateNumberSlug> for String {
    fn from(value: CustomFieldUpdateNumberSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateNumberSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateNumberSlug {
    type Error = CustomFieldUpdateNumberSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateNumberSlug {
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
pub struct CustomFieldUpdateSelectMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateSelectMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldUpdateSelectMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateSelectMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateSelectMetadataKey {
    type Error = CustomFieldUpdateSelectMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateSelectMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldUpdateSelectMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateSelectMetadataKey> for String {
    fn from(value: CustomFieldUpdateSelectMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateSelectMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateSelectMetadataKey {
    type Error = CustomFieldUpdateSelectMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateSelectMetadataKey {
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
pub struct CustomFieldUpdateSelectMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateSelectMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldUpdateSelectMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateSelectMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateSelectMetadataValueString {
    type Error = CustomFieldUpdateSelectMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateSelectMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldUpdateSelectMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateSelectMetadataValueString> for String {
    fn from(value: CustomFieldUpdateSelectMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateSelectMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateSelectMetadataValueString {
    type Error = CustomFieldUpdateSelectMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateSelectMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldUpdateSelectName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateSelectNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateSelectName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateSelectNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateSelectName {
    type Error = CustomFieldUpdateSelectNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateSelectNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateSelectName> for String {
    fn from(value: CustomFieldUpdateSelectName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateSelectName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateSelectName {
    type Error = CustomFieldUpdateSelectNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateSelectName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldUpdateSelectSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateSelectSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateSelectSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateSelectSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateSelectSlug {
    type Error = CustomFieldUpdateSelectSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateSelectSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateSelectSlug> for String {
    fn from(value: CustomFieldUpdateSelectSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateSelectSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateSelectSlug {
    type Error = CustomFieldUpdateSelectSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateSelectSlug {
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
pub struct CustomFieldUpdateTextMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateTextMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CustomFieldUpdateTextMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateTextMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateTextMetadataKey {
    type Error = CustomFieldUpdateTextMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateTextMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CustomFieldUpdateTextMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateTextMetadataKey> for String {
    fn from(value: CustomFieldUpdateTextMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateTextMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateTextMetadataKey {
    type Error = CustomFieldUpdateTextMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateTextMetadataKey {
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
pub struct CustomFieldUpdateTextMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateTextMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CustomFieldUpdateTextMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateTextMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateTextMetadataValueString {
    type Error = CustomFieldUpdateTextMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateTextMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CustomFieldUpdateTextMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateTextMetadataValueString> for String {
    fn from(value: CustomFieldUpdateTextMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateTextMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateTextMetadataValueString {
    type Error = CustomFieldUpdateTextMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateTextMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Name of the custom field.
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
pub struct CustomFieldUpdateTextName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateTextNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateTextName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateTextNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateTextName {
    type Error = CustomFieldUpdateTextNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateTextNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateTextName> for String {
    fn from(value: CustomFieldUpdateTextName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateTextName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateTextName {
    type Error = CustomFieldUpdateTextNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateTextName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Identifier of the custom field. It'll be used as key when storing the value.
/// Must be unique across the organization.It can only contain ASCII letters,
/// numbers and hyphens.
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
pub struct CustomFieldUpdateTextSlug(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomFieldUpdateTextSlugValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl CustomFieldUpdateTextSlug {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomFieldUpdateTextSlugValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomFieldUpdateTextSlug {
    type Error = CustomFieldUpdateTextSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CustomFieldUpdateTextSlugValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<CustomFieldUpdateTextSlug> for String {
    fn from(value: CustomFieldUpdateTextSlug) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomFieldUpdateTextSlug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomFieldUpdateTextSlug {
    type Error = CustomFieldUpdateTextSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomFieldUpdateTextSlug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
