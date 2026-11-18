use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerSubscriptionSortProperty {
    #[serde(rename = "started_at")]
    StartedAt,
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "product")]
    Product,
}
impl CustomerSubscriptionSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::StartedAt => "started_at",
            Self::Amount => "amount",
            Self::Status => "status",
            Self::Organization => "organization",
            Self::Product => "product",
        }
    }
}
impl ::std::fmt::Display for CustomerSubscriptionSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerSubscriptionSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum SubscriptionSortProperty {
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "started_at")]
    StartedAt,
    #[serde(rename = "current_period_end")]
    CurrentPeriodEnd,
    #[serde(rename = "ended_at")]
    EndedAt,
    #[serde(rename = "ends_at")]
    EndsAt,
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "discount")]
    Discount,
}
impl SubscriptionSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Customer => "customer",
            Self::Status => "status",
            Self::StartedAt => "started_at",
            Self::CurrentPeriodEnd => "current_period_end",
            Self::EndedAt => "ended_at",
            Self::EndsAt => "ends_at",
            Self::Amount => "amount",
            Self::Product => "product",
            Self::Discount => "discount",
        }
    }
}
impl ::std::fmt::Display for SubscriptionSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SubscriptionSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
