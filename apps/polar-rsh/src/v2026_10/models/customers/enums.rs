use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "name")]
    Name,
}
impl CustomerSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Email => "email",
            Self::Name => "name",
        }
    }
}
impl ::std::fmt::Display for CustomerSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerStateSubscriptionStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "trialing")]
    Trialing,
}
impl CustomerStateSubscriptionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Trialing => "trialing",
        }
    }
}
impl ::std::fmt::Display for CustomerStateSubscriptionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerStateSubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
/// The role of the member within the customer. To assign or transfer ownership,
/// use the member update endpoint.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default,
)]
pub enum MemberCreateFromCustomerRole {
    #[default]
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "billing_manager")]
    BillingManager,
}
impl MemberCreateFromCustomerRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Member => "member",
            Self::BillingManager => "billing_manager",
        }
    }
}
impl ::std::fmt::Display for MemberCreateFromCustomerRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for MemberCreateFromCustomerRole {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum MemberSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl MemberSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
        }
    }
}
impl ::std::fmt::Display for MemberSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for MemberSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
