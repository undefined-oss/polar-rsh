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
pub enum SubscriptionExportColumn {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "started_at")]
    StartedAt,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "currency")]
    Currency,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "recurring_interval")]
    RecurringInterval,
    #[serde(rename = "customer_name")]
    CustomerName,
    #[serde(rename = "billing_name")]
    BillingName,
    #[serde(rename = "billing_country")]
    BillingCountry,
    #[serde(rename = "net_amount")]
    NetAmount,
    #[serde(rename = "discount")]
    Discount,
    #[serde(rename = "seats")]
    Seats,
    #[serde(rename = "current_period_start")]
    CurrentPeriodStart,
    #[serde(rename = "current_period_end")]
    CurrentPeriodEnd,
    #[serde(rename = "cancel_at_period_end")]
    CancelAtPeriodEnd,
    #[serde(rename = "canceled_at")]
    CanceledAt,
    #[serde(rename = "ends_at")]
    EndsAt,
    #[serde(rename = "ended_at")]
    EndedAt,
    #[serde(rename = "cancellation_reason")]
    CancellationReason,
    #[serde(rename = "trial_start")]
    TrialStart,
    #[serde(rename = "trial_end")]
    TrialEnd,
}
impl SubscriptionExportColumn {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::StartedAt => "started_at",
            Self::Product => "product",
            Self::Amount => "amount",
            Self::Currency => "currency",
            Self::Status => "status",
            Self::RecurringInterval => "recurring_interval",
            Self::CustomerName => "customer_name",
            Self::BillingName => "billing_name",
            Self::BillingCountry => "billing_country",
            Self::NetAmount => "net_amount",
            Self::Discount => "discount",
            Self::Seats => "seats",
            Self::CurrentPeriodStart => "current_period_start",
            Self::CurrentPeriodEnd => "current_period_end",
            Self::CancelAtPeriodEnd => "cancel_at_period_end",
            Self::CanceledAt => "canceled_at",
            Self::EndsAt => "ends_at",
            Self::EndedAt => "ended_at",
            Self::CancellationReason => "cancellation_reason",
            Self::TrialStart => "trial_start",
            Self::TrialEnd => "trial_end",
        }
    }
}
impl ::std::fmt::Display for SubscriptionExportColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for SubscriptionExportColumn {
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
