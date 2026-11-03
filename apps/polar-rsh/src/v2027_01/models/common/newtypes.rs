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
/// The minimum purchasable seat quantity (inclusive). Defaults to 1 when not
/// set.
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
pub struct ProductPriceSeatBasedMinimumUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductPriceSeatBasedMinimumUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
}
impl ProductPriceSeatBasedMinimumUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, ProductPriceSeatBasedMinimumUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for ProductPriceSeatBasedMinimumUnits {
    type Error = ProductPriceSeatBasedMinimumUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(ProductPriceSeatBasedMinimumUnitsValidationError::BelowMinimum);
        }
        Ok(Self(value))
    }
}
impl From<ProductPriceSeatBasedMinimumUnits> for u64 {
    fn from(value: ProductPriceSeatBasedMinimumUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductPriceSeatBasedMinimumUnits {
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
