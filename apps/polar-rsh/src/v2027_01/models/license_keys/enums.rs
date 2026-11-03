use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum LicenseKeyStatus {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "disabled")]
    Disabled,
}
impl LicenseKeyStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::Revoked => "revoked",
            Self::Disabled => "disabled",
        }
    }
}
impl ::std::fmt::Display for LicenseKeyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for LicenseKeyStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum RotatedLicenseKeyStatus {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "disabled")]
    Disabled,
}
impl RotatedLicenseKeyStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::Disabled => "disabled",
        }
    }
}
impl ::std::fmt::Display for RotatedLicenseKeyStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for RotatedLicenseKeyStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
