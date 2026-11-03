/// The name of the event.
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
pub struct EventCreateCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventCreateCustomerNameValidationError {
    #[error("must contain at most 128 characters")]
    TooLong,
}
impl EventCreateCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventCreateCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventCreateCustomerName {
    type Error = EventCreateCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 128u64 {
            return Err(EventCreateCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventCreateCustomerName> for String {
    fn from(value: EventCreateCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventCreateCustomerName {
    type Error = EventCreateCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventCreateCustomerName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The name of the event.
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
pub struct EventCreateExternalCustomerName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventCreateExternalCustomerNameValidationError {
    #[error("must contain at most 128 characters")]
    TooLong,
}
impl EventCreateExternalCustomerName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventCreateExternalCustomerNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventCreateExternalCustomerName {
    type Error = EventCreateExternalCustomerNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 128u64 {
            return Err(EventCreateExternalCustomerNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventCreateExternalCustomerName> for String {
    fn from(value: EventCreateExternalCustomerName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateExternalCustomerName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventCreateExternalCustomerName {
    type Error = EventCreateExternalCustomerNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventCreateExternalCustomerName {
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
pub struct EventMetadataInputValueString(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventMetadataInputValueStringValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 500 characters")]
    TooLong,
}
impl EventMetadataInputValueString {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventMetadataInputValueStringValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventMetadataInputValueString {
    type Error = EventMetadataInputValueStringValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(EventMetadataInputValueStringValidationError::TooShort);
        }
        if length > 500u64 {
            return Err(EventMetadataInputValueStringValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventMetadataInputValueString> for String {
    fn from(value: EventMetadataInputValueString) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventMetadataInputValueString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventMetadataInputValueString {
    type Error = EventMetadataInputValueStringValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventMetadataInputValueString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
