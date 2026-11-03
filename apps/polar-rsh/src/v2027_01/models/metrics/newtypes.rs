/// Display name for the dashboard.
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
pub struct MetricDashboardCreateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MetricDashboardCreateNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl MetricDashboardCreateName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, MetricDashboardCreateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MetricDashboardCreateName {
    type Error = MetricDashboardCreateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(MetricDashboardCreateNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<MetricDashboardCreateName> for String {
    fn from(value: MetricDashboardCreateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MetricDashboardCreateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MetricDashboardCreateName {
    type Error = MetricDashboardCreateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MetricDashboardCreateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
/// Display name for the dashboard.
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
pub struct MetricDashboardUpdateName(String);
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MetricDashboardUpdateNameValidationError {
    #[error("must contain at least 1 characters")]
    TooShort,
}
impl MetricDashboardUpdateName {
    /// Validate a value.
    pub fn try_new(
        value: impl Into<String>,
    ) -> Result<Self, MetricDashboardUpdateNameValidationError> {
        Self::try_from(value.into())
    }

    /// Take the underlying value.
    pub fn into_inner(self) -> String {
        self.0
    }
}
impl TryFrom<String> for MetricDashboardUpdateName {
    type Error = MetricDashboardUpdateNameValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let length = value.chars().count() as u64;
        if length < 1u64 {
            return Err(MetricDashboardUpdateNameValidationError::TooShort);
        }
        Ok(Self(value))
    }
}
impl From<MetricDashboardUpdateName> for String {
    fn from(value: MetricDashboardUpdateName) -> Self {
        value.0
    }
}
impl<'de> serde::Deserialize<'de> for MetricDashboardUpdateName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::try_from(value).map_err(serde::de::Error::custom)
    }
}
impl TryFrom<&str> for MetricDashboardUpdateName {
    type Error = MetricDashboardUpdateNameValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}
impl AsRef<str> for MetricDashboardUpdateName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
