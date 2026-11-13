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
pub struct CheckoutLinkCreateProductsMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutLinkCreateProductsMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutLinkCreateProductsMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutLinkCreateProductsMetadataKey {
    type Error = CheckoutLinkCreateProductsMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkCreateProductsMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutLinkCreateProductsMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsMetadataKey> for String {
    fn from(value: CheckoutLinkCreateProductsMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutLinkCreateProductsMetadataKey {
    type Error = CheckoutLinkCreateProductsMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutLinkCreateProductsMetadataKey {
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
pub struct CheckoutLinkCreateProductsMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutLinkCreateProductsMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutLinkCreateProductsMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutLinkCreateProductsMetadataValueString {
    type Error = CheckoutLinkCreateProductsMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkCreateProductsMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutLinkCreateProductsMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsMetadataValueString> for String {
    fn from(value: CheckoutLinkCreateProductsMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutLinkCreateProductsMetadataValueString {
    type Error = CheckoutLinkCreateProductsMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutLinkCreateProductsMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
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
pub struct CheckoutLinkCreateProductsReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutLinkCreateProductsReturnUrl {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, CheckoutLinkCreateProductsReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutLinkCreateProductsReturnUrl {
    type Error = CheckoutLinkCreateProductsReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkCreateProductsReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutLinkCreateProductsReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsReturnUrl> for url::Url {
    fn from(value: CheckoutLinkCreateProductsReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutLinkCreateProductsReturnUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Preconfigured number of seats for seat-based pricing. When set, checkout
/// sessions created from this link are locked to this number of seats and the
/// customer won't be able to change it. All products on the link must use
/// seat-based pricing and allow this number of seats. If the products no longer
/// accommodate this value when the link is opened, it'll be ignored.
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
pub struct CheckoutLinkCreateProductsSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutLinkCreateProductsSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutLinkCreateProductsSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutLinkCreateProductsSeats {
    type Error = CheckoutLinkCreateProductsSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutLinkCreateProductsSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutLinkCreateProductsSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsSeats> for u64 {
    fn from(value: CheckoutLinkCreateProductsSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsSeats {
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
pub struct CheckoutLinkCreateProductsSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutLinkCreateProductsSuccessUrl {
    /// Validate a value.
    pub fn try_new(
        value: url::Url,
    ) -> Result<Self, CheckoutLinkCreateProductsSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutLinkCreateProductsSuccessUrl {
    type Error = CheckoutLinkCreateProductsSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkCreateProductsSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutLinkCreateProductsSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsSuccessUrl> for url::Url {
    fn from(value: CheckoutLinkCreateProductsSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutLinkCreateProductsSuccessUrl {
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
pub struct CheckoutLinkCreateProductsTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutLinkCreateProductsTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(
        value: u64,
    ) -> Result<Self, CheckoutLinkCreateProductsTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutLinkCreateProductsTrialIntervalCount {
    type Error = CheckoutLinkCreateProductsTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutLinkCreateProductsTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(CheckoutLinkCreateProductsTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsTrialIntervalCount> for u64 {
    fn from(value: CheckoutLinkCreateProductsTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsTrialIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Preconfigured number of units for unit-based pricing. When set, checkout
/// sessions created from this link are locked to this number of units and the
/// customer won't be able to change it. All products on the link must use
/// unit-based pricing and allow this number of units. If the products no longer
/// accommodate this value when the link is opened, it'll be ignored.
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
pub struct CheckoutLinkCreateProductsUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkCreateProductsUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutLinkCreateProductsUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutLinkCreateProductsUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutLinkCreateProductsUnits {
    type Error = CheckoutLinkCreateProductsUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutLinkCreateProductsUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutLinkCreateProductsUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkCreateProductsUnits> for u64 {
    fn from(value: CheckoutLinkCreateProductsUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkCreateProductsUnits {
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
pub struct CheckoutLinkMetadataKey(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkMetadataKeyValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 40 characters")]
    TooLong,
}
impl CheckoutLinkMetadataKey {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutLinkMetadataKeyValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutLinkMetadataKey {
    type Error = CheckoutLinkMetadataKeyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkMetadataKeyValidationError::TooShort);
        }
        if length > 40u64 {
            return Err(CheckoutLinkMetadataKeyValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkMetadataKey> for String {
    fn from(value: CheckoutLinkMetadataKey) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkMetadataKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutLinkMetadataKey {
    type Error = CheckoutLinkMetadataKeyValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutLinkMetadataKey {
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
pub struct CheckoutLinkMetadataValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkMetadataValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl CheckoutLinkMetadataValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CheckoutLinkMetadataValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CheckoutLinkMetadataValueString {
    type Error = CheckoutLinkMetadataValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkMetadataValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(CheckoutLinkMetadataValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkMetadataValueString> for String {
    fn from(value: CheckoutLinkMetadataValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkMetadataValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CheckoutLinkMetadataValueString {
    type Error = CheckoutLinkMetadataValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CheckoutLinkMetadataValueString {
    fn as_ref(&self) -> &str {
        &self.0
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
pub struct CheckoutLinkReturnUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkReturnUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutLinkReturnUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutLinkReturnUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutLinkReturnUrl {
    type Error = CheckoutLinkReturnUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkReturnUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutLinkReturnUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkReturnUrl> for url::Url {
    fn from(value: CheckoutLinkReturnUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkReturnUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutLinkReturnUrl {
    fn as_ref(&self) -> &url::Url {
        &self.0
    }
}
/// Preconfigured number of seats for seat-based pricing. When set, checkout
/// sessions created from this link are locked to this number of seats and the
/// customer won't be able to change it. All products on the link must use
/// seat-based pricing and allow this number of seats. If the products no longer
/// accommodate this value when the link is opened, it'll be ignored.
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
pub struct CheckoutLinkSeats(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkSeatsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 10000")]
    AboveMaximum,
}
impl CheckoutLinkSeats {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutLinkSeatsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutLinkSeats {
    type Error = CheckoutLinkSeatsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutLinkSeatsValidationError::BelowMinimum);
        }
        if i128::from(value) > 10000i128 {
            return Err(CheckoutLinkSeatsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkSeats> for u64 {
    fn from(value: CheckoutLinkSeats) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkSeats {
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
pub struct CheckoutLinkSuccessUrl(url::Url);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkSuccessUrlValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 2083 characters")]
    TooLong,
}
impl CheckoutLinkSuccessUrl {
    /// Validate a value.
    pub fn try_new(value: url::Url) -> Result<Self, CheckoutLinkSuccessUrlValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> url::Url {
        self.0
    }
}
impl TryFrom<url::Url> for CheckoutLinkSuccessUrl {
    type Error = CheckoutLinkSuccessUrlValidationError;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        let length = value.as_str().chars().count() as u64;
        if length < 1u64 {
            return Err(CheckoutLinkSuccessUrlValidationError::TooShort);
        }
        if length > 2083u64 {
            return Err(CheckoutLinkSuccessUrlValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkSuccessUrl> for url::Url {
    fn from(value: CheckoutLinkSuccessUrl) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkSuccessUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <url::Url as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl AsRef<url::Url> for CheckoutLinkSuccessUrl {
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
pub struct CheckoutLinkTrialIntervalCount(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkTrialIntervalCountValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 1000")]
    AboveMaximum,
}
impl CheckoutLinkTrialIntervalCount {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutLinkTrialIntervalCountValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutLinkTrialIntervalCount {
    type Error = CheckoutLinkTrialIntervalCountValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutLinkTrialIntervalCountValidationError::BelowMinimum);
        }
        if i128::from(value) > 1000i128 {
            return Err(CheckoutLinkTrialIntervalCountValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkTrialIntervalCount> for u64 {
    fn from(value: CheckoutLinkTrialIntervalCount) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkTrialIntervalCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Preconfigured number of units for unit-based pricing. When set, checkout
/// sessions created from this link are locked to this number of units and the
/// customer won't be able to change it. All products on the link must use
/// unit-based pricing and allow this number of units. If the products no longer
/// accommodate this value when the link is opened, it'll be ignored.
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
pub struct CheckoutLinkUnits(u64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CheckoutLinkUnitsValidationError {
    #[error("must be at least 1")]
    BelowMinimum,
    #[error("must be at most 2147483647")]
    AboveMaximum,
}
impl CheckoutLinkUnits {
    /// Validate a value.
    pub fn try_new(value: u64) -> Result<Self, CheckoutLinkUnitsValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
impl TryFrom<u64> for CheckoutLinkUnits {
    type Error = CheckoutLinkUnitsValidationError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if i128::from(value) < 1i128 {
            return Err(CheckoutLinkUnitsValidationError::BelowMinimum);
        }
        if i128::from(value) > 2147483647i128 {
            return Err(CheckoutLinkUnitsValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<CheckoutLinkUnits> for u64 {
    fn from(value: CheckoutLinkUnits) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <u64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
