use derive_more::{Debug, Deref, Display, Eq, From, PartialEq};
use serde::{Deserialize, Serialize};

pub mod enums;
pub mod errors;

/// Number of items per page, between 1 and 100 inclusive.
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Deref, Deserialize, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct PageLimit(u8);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("page limit must be between 1 and 100 inclusive, got {0}")]
pub struct PageLimitValidationError(pub u8);

impl TryFrom<u8> for PageLimit {
    type Error = PageLimitValidationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 1 || value > 100 {
            return Err(PageLimitValidationError(value));
        }

        Ok(Self(value))
    }
}

impl From<PageLimit> for u8 {
    fn from(value: PageLimit) -> Self {
        value.0
    }
}

impl Default for PageLimit {
    fn default() -> Self {
        Self(10)
    }
}

#[cfg(test)]
mod page_limit_tests {
    use super::{PageLimit, PageLimitValidationError};

    #[test]
    fn accepts_only_limits_in_the_supported_range() {
        for value in u8::MIN..=u8::MAX {
            let result = PageLimit::try_from(value);
            if (1..=100).contains(&value) {
                let limit = result.unwrap();
                assert_eq!(*limit, value);
                assert_eq!(u8::from(limit), value);
                assert_eq!(limit.to_string(), value.to_string());
            } else {
                assert_eq!(result, Err(PageLimitValidationError(value)));
            }
        }
        assert_eq!(*PageLimit::default(), 10);
    }

    #[test]
    fn serde_validates_limits_and_preserves_numeric_encoding() {
        for value in [1, 10, 100] {
            let json = serde_json::json!(value);
            let limit: PageLimit = serde_json::from_value(json.clone()).unwrap();
            assert_eq!(*limit, value);
            assert_eq!(serde_json::to_value(limit).unwrap(), json);
        }
        for json in ["0", "101", "255", "256", "-1", "1.5", "null", "\"10\""] {
            assert!(serde_json::from_str::<PageLimit>(json).is_err(), "accepted {json}");
        }
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
