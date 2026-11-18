/// The ID of the customer in your system.
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
pub struct BenefitGrantCreateExternalExternalCustomerId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitGrantCreateExternalExternalCustomerIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl BenefitGrantCreateExternalExternalCustomerId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitGrantCreateExternalExternalCustomerIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitGrantCreateExternalExternalCustomerId {
    type Error = BenefitGrantCreateExternalExternalCustomerIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitGrantCreateExternalExternalCustomerIdValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<BenefitGrantCreateExternalExternalCustomerId> for String {
    fn from(value: BenefitGrantCreateExternalExternalCustomerId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitGrantCreateExternalExternalCustomerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitGrantCreateExternalExternalCustomerId {
    type Error = BenefitGrantCreateExternalExternalCustomerIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitGrantCreateExternalExternalCustomerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// The ID of the member in your system, scoped to the customer. If not
/// provided, the benefit is granted to the customer's owner.
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
pub struct BenefitGrantCreateExternalExternalMemberId(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BenefitGrantCreateExternalExternalMemberIdValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl BenefitGrantCreateExternalExternalMemberId {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, BenefitGrantCreateExternalExternalMemberIdValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for BenefitGrantCreateExternalExternalMemberId {
    type Error = BenefitGrantCreateExternalExternalMemberIdValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(BenefitGrantCreateExternalExternalMemberIdValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<BenefitGrantCreateExternalExternalMemberId> for String {
    fn from(value: BenefitGrantCreateExternalExternalMemberId) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for BenefitGrantCreateExternalExternalMemberId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for BenefitGrantCreateExternalExternalMemberId {
    type Error = BenefitGrantCreateExternalExternalMemberIdValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for BenefitGrantCreateExternalExternalMemberId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
