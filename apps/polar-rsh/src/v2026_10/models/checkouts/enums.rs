use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum BillingAddressFieldMode {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "disabled")]
    Disabled,
}
impl BillingAddressFieldMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Disabled => "disabled",
        }
    }
}
impl ::std::fmt::Display for BillingAddressFieldMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for BillingAddressFieldMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CheckoutSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "expires_at")]
    ExpiresAt,
    #[serde(rename = "status")]
    Status,
}
impl CheckoutSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::ExpiresAt => "expires_at",
            Self::Status => "status",
        }
    }
}
impl ::std::fmt::Display for CheckoutSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CheckoutSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CheckoutStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}
impl CheckoutStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Expired => "expired",
            Self::Confirmed => "confirmed",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}
impl ::std::fmt::Display for CheckoutStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CheckoutStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum TaxBehavior {
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "exclusive")]
    Exclusive,
}
impl TaxBehavior {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inclusive => "inclusive",
            Self::Exclusive => "exclusive",
        }
    }
}
impl ::std::fmt::Display for TaxBehavior {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for TaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
