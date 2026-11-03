/// Size of the file. A maximum of 1 MB is allowed for this type of file.
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
pub struct OrganizationAvatarFileCreateSize(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrganizationAvatarFileCreateSizeValidationError {
    #[error("must be at most 1048576")]
    AboveMaximum,
}
impl OrganizationAvatarFileCreateSize {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, OrganizationAvatarFileCreateSizeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for OrganizationAvatarFileCreateSize {
    type Error = OrganizationAvatarFileCreateSizeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) > 1048576i128 {
            return Err(OrganizationAvatarFileCreateSizeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<OrganizationAvatarFileCreateSize> for i64 {
    fn from(value: OrganizationAvatarFileCreateSize) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationAvatarFileCreateSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Size of the file. A maximum of 10 MB is allowed for this type of file.
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
pub struct ProductMediaFileCreateSize(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProductMediaFileCreateSizeValidationError {
    #[error("must be at most 10485760")]
    AboveMaximum,
}
impl ProductMediaFileCreateSize {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, ProductMediaFileCreateSizeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for ProductMediaFileCreateSize {
    type Error = ProductMediaFileCreateSizeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) > 10485760i128 {
            return Err(ProductMediaFileCreateSizeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<ProductMediaFileCreateSize> for i64 {
    fn from(value: ProductMediaFileCreateSize) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for ProductMediaFileCreateSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
/// Size of the file. A maximum of 250 MB is allowed for this type of file.
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
pub struct SupportCaseAttachmentFileCreateSize(i64);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SupportCaseAttachmentFileCreateSizeValidationError {
    #[error("must be at most 262144000")]
    AboveMaximum,
}
impl SupportCaseAttachmentFileCreateSize {
    /// Validate a value.
    pub fn try_new(value: i64) -> Result<Self, SupportCaseAttachmentFileCreateSizeValidationError> {
        Self::try_from(value)
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> i64 {
        self.0
    }
}
impl TryFrom<i64> for SupportCaseAttachmentFileCreateSize {
    type Error = SupportCaseAttachmentFileCreateSizeValidationError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if i128::from(value) > 262144000i128 {
            return Err(SupportCaseAttachmentFileCreateSizeValidationError::AboveMaximum);
        }
        Ok(Self(value))
    }
}
impl From<SupportCaseAttachmentFileCreateSize> for i64 {
    fn from(value: SupportCaseAttachmentFileCreateSize) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for SupportCaseAttachmentFileCreateSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
