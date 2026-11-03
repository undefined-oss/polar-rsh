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
pub struct CustomerPortalMemberName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomerPortalMemberNameValidationError {
    #[error("must contain at most 256 characters")]
    TooLong,
}
impl CustomerPortalMemberName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, CustomerPortalMemberNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for CustomerPortalMemberName {
    type Error = CustomerPortalMemberNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length > 256u64 {
            return Err(CustomerPortalMemberNameValidationError::TooLong);
        }
        Ok(Self(value))
    }
}
impl From<CustomerPortalMemberName> for String {
    fn from(value: CustomerPortalMemberName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for CustomerPortalMemberName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for CustomerPortalMemberName {
    type Error = CustomerPortalMemberNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for CustomerPortalMemberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
