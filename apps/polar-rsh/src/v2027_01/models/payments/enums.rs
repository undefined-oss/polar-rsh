use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PaymentSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "amount")]
    Amount,
    #[serde(rename = "method")]
    Method,
}
impl PaymentSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::Status => "status",
            Self::Amount => "amount",
            Self::Method => "method",
        }
    }
}
impl ::std::fmt::Display for PaymentSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for PaymentSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PaymentStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
}
impl PaymentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
        }
    }
}
impl ::std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for PaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PaymentTrigger {
    #[serde(rename = "purchase")]
    Purchase,
    #[serde(rename = "subscription_cycle")]
    SubscriptionCycle,
    #[serde(rename = "retry_dunning")]
    RetryDunning,
    #[serde(rename = "retry_customer")]
    RetryCustomer,
    #[serde(rename = "retry_payment_method_update")]
    RetryPaymentMethodUpdate,
    #[serde(rename = "retry_admin")]
    RetryAdmin,
}
impl PaymentTrigger {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Purchase => "purchase",
            Self::SubscriptionCycle => "subscription_cycle",
            Self::RetryDunning => "retry_dunning",
            Self::RetryCustomer => "retry_customer",
            Self::RetryPaymentMethodUpdate => "retry_payment_method_update",
            Self::RetryAdmin => "retry_admin",
        }
    }
}
impl ::std::fmt::Display for PaymentTrigger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for PaymentTrigger {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
