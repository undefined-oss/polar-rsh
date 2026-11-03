/// The label for the event type.
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
pub struct EventTypeLabel(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventTypeLabelValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 128 characters")]
    TooLong,
}
impl EventTypeLabel {
    /// Validate a value.
    pub fn try_new(value: impl Into<String>) -> Result<Self, EventTypeLabelValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventTypeLabel {
    type Error = EventTypeLabelValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(EventTypeLabelValidationError::TooShort);
        }
        if length > 128u64 {
            return Err(EventTypeLabelValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventTypeLabel> for String {
    fn from(value: EventTypeLabel) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventTypeLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventTypeLabel {
    type Error = EventTypeLabelValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventTypeLabel {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Property path to extract dynamic label from event metadata (e.g., 'subject'
/// or 'metadata.subject').
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
pub struct EventTypeLabelPropertySelector(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EventTypeLabelPropertySelectorValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl EventTypeLabelPropertySelector {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, EventTypeLabelPropertySelectorValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for EventTypeLabelPropertySelector {
    type Error = EventTypeLabelPropertySelectorValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(EventTypeLabelPropertySelectorValidationError::TooShort);
        }
        if length > 256u64 {
            return Err(EventTypeLabelPropertySelectorValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<EventTypeLabelPropertySelector> for String {
    fn from(value: EventTypeLabelPropertySelector) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for EventTypeLabelPropertySelector {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for EventTypeLabelPropertySelector {
    type Error = EventTypeLabelPropertySelectorValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for EventTypeLabelPropertySelector {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
