use derive_more::{Debug, Deref, Display, Eq, From, PartialEq};
use serde::{Deserialize, Serialize};

pub mod enums;
pub mod errors;
pub(crate) mod serde_helpers;

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct OrganizationId(uuid::Uuid);

impl ::std::default::Default for OrganizationId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct AccountId(uuid::Uuid);

impl ::std::default::Default for AccountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    Eq,
    From,
    Ord,
    PartialEq,
    PartialOrd,
    Deref,
    Hash,
    Deserialize,
    Serialize,
)]
pub struct PayoutAccountId(uuid::Uuid);

impl ::std::default::Default for PayoutAccountId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Display, Deref, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrganizationSlug(String);

#[derive(thiserror::Error, Debug)]
pub enum OrganizationSlugValidationError {
    #[error("organization slug can't be null. Slug should be compose of 3 to 64 characters")]
    OrganizationSlugIsEmpty,
    #[error("organization slug should be compose at least of 3 characters")]
    OrganizationSlugIsNotLongEnough,
    #[error("organization slug can't exceed the maximum of 64 characters")]
    OrganizationSlugIsLongerThanTheMaximum,
}

impl OrganizationSlug {
    fn new_or_panic(slug: &str) -> Self {
        Self::try_new(slug).unwrap()
    }

    fn try_new(slug: &str) -> Result<Self, OrganizationSlugValidationError> {
        if slug.is_empty() {
            return Err(OrganizationSlugValidationError::OrganizationSlugIsEmpty);
        }

        if slug.len() < 3 {
            return Err(OrganizationSlugValidationError::OrganizationSlugIsNotLongEnough);
        }

        if slug.len() > 64 {
            return Err(OrganizationSlugValidationError::OrganizationSlugIsLongerThanTheMaximum);
        }

        return Ok(Self(slug.to_owned()));
    }
}

impl AsRef<str> for OrganizationSlug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for OrganizationSlug {
    type Error = OrganizationSlugValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        OrganizationSlug::try_new(value.as_ref())
    }
}

impl TryFrom<&str> for OrganizationSlug {
    type Error = OrganizationSlugValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        OrganizationSlug::try_new(value)
    }
}

#[derive(Debug, Clone, Display, Deref, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OrganizationName(String);

#[derive(thiserror::Error, Debug)]
pub enum OrganizationNameValidationError {
    #[error("organization slug can't be null. Slug should be compose at least of 3 characters")]
    OrganizationNameIsEmpty,
    #[error("organization slug should be compose at least of 3 characters")]
    OrganizationNameIsNotLongEnough,
}

impl OrganizationName {
    fn new_or_panic(slug: &str) -> Self {
        Self::try_new(slug).unwrap()
    }

    fn try_new(slug: &str) -> Result<Self, OrganizationNameValidationError> {
        if slug.is_empty() {
            return Err(OrganizationNameValidationError::OrganizationNameIsEmpty);
        }

        if slug.len() < 3 {
            return Err(OrganizationNameValidationError::OrganizationNameIsNotLongEnough);
        }

        return Ok(Self(slug.to_owned()));
    }
}

impl AsRef<str> for OrganizationName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for OrganizationName {
    type Error = OrganizationNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        OrganizationName::try_new(value.as_ref())
    }
}

impl TryFrom<&str> for OrganizationName {
    type Error = OrganizationNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        OrganizationName::try_new(value)
    }
}
