use serde::{Deserialize, Serialize};
/// The API version that'll be used in event payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum WebhookEndpointCreateApiVersion {
    #[serde(rename = "2026-04")]
    Value2026Minus04,
}
impl WebhookEndpointCreateApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value2026Minus04 => "2026-04",
        }
    }
}
impl ::std::fmt::Display for WebhookEndpointCreateApiVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for WebhookEndpointCreateApiVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum WebhookEndpointUpdateApiVersion {
    #[serde(rename = "2026-04")]
    Value2026Minus04,
}
impl WebhookEndpointUpdateApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value2026Minus04 => "2026-04",
        }
    }
}
impl ::std::fmt::Display for WebhookEndpointUpdateApiVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for WebhookEndpointUpdateApiVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum WebhookEventType {
    #[serde(rename = "checkout.created")]
    CheckoutCreated,
    #[serde(rename = "checkout.updated")]
    CheckoutUpdated,
    #[serde(rename = "checkout.expired")]
    CheckoutExpired,
    #[serde(rename = "customer.created")]
    CustomerCreated,
    #[serde(rename = "customer.updated")]
    CustomerUpdated,
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,
    #[serde(rename = "customer.state_changed")]
    CustomerStateChanged,
    #[serde(rename = "customer_seat.assigned")]
    CustomerSeatAssigned,
    #[serde(rename = "customer_seat.claimed")]
    CustomerSeatClaimed,
    #[serde(rename = "customer_seat.revoked")]
    CustomerSeatRevoked,
    #[serde(rename = "member.created")]
    MemberCreated,
    #[serde(rename = "member.updated")]
    MemberUpdated,
    #[serde(rename = "member.deleted")]
    MemberDeleted,
    #[serde(rename = "order.created")]
    OrderCreated,
    #[serde(rename = "order.updated")]
    OrderUpdated,
    #[serde(rename = "order.paid")]
    OrderPaid,
    #[serde(rename = "order.refunded")]
    OrderRefunded,
    #[serde(rename = "subscription.created")]
    SubscriptionCreated,
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,
    #[serde(rename = "subscription.active")]
    SubscriptionActive,
    #[serde(rename = "subscription.canceled")]
    SubscriptionCanceled,
    #[serde(rename = "subscription.uncanceled")]
    SubscriptionUncanceled,
    #[serde(rename = "subscription.cycled")]
    SubscriptionCycled,
    #[serde(rename = "subscription.revoked")]
    SubscriptionRevoked,
    #[serde(rename = "subscription.past_due")]
    SubscriptionPastDue,
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused,
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed,
    #[serde(rename = "subscription.migrated")]
    SubscriptionMigrated,
    #[serde(rename = "refund.created")]
    RefundCreated,
    #[serde(rename = "refund.updated")]
    RefundUpdated,
    #[serde(rename = "product.created")]
    ProductCreated,
    #[serde(rename = "product.updated")]
    ProductUpdated,
    #[serde(rename = "discount.created")]
    DiscountCreated,
    #[serde(rename = "discount.updated")]
    DiscountUpdated,
    #[serde(rename = "discount.deleted")]
    DiscountDeleted,
    #[serde(rename = "benefit.created")]
    BenefitCreated,
    #[serde(rename = "benefit.updated")]
    BenefitUpdated,
    #[serde(rename = "benefit_grant.created")]
    BenefitGrantCreated,
    #[serde(rename = "benefit_grant.cycled")]
    BenefitGrantCycled,
    #[serde(rename = "benefit_grant.updated")]
    BenefitGrantUpdated,
    #[serde(rename = "benefit_grant.revoked")]
    BenefitGrantRevoked,
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,
}
impl WebhookEventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CheckoutCreated => "checkout.created",
            Self::CheckoutUpdated => "checkout.updated",
            Self::CheckoutExpired => "checkout.expired",
            Self::CustomerCreated => "customer.created",
            Self::CustomerUpdated => "customer.updated",
            Self::CustomerDeleted => "customer.deleted",
            Self::CustomerStateChanged => "customer.state_changed",
            Self::CustomerSeatAssigned => "customer_seat.assigned",
            Self::CustomerSeatClaimed => "customer_seat.claimed",
            Self::CustomerSeatRevoked => "customer_seat.revoked",
            Self::MemberCreated => "member.created",
            Self::MemberUpdated => "member.updated",
            Self::MemberDeleted => "member.deleted",
            Self::OrderCreated => "order.created",
            Self::OrderUpdated => "order.updated",
            Self::OrderPaid => "order.paid",
            Self::OrderRefunded => "order.refunded",
            Self::SubscriptionCreated => "subscription.created",
            Self::SubscriptionUpdated => "subscription.updated",
            Self::SubscriptionActive => "subscription.active",
            Self::SubscriptionCanceled => "subscription.canceled",
            Self::SubscriptionUncanceled => "subscription.uncanceled",
            Self::SubscriptionCycled => "subscription.cycled",
            Self::SubscriptionRevoked => "subscription.revoked",
            Self::SubscriptionPastDue => "subscription.past_due",
            Self::SubscriptionPaused => "subscription.paused",
            Self::SubscriptionResumed => "subscription.resumed",
            Self::SubscriptionMigrated => "subscription.migrated",
            Self::RefundCreated => "refund.created",
            Self::RefundUpdated => "refund.updated",
            Self::ProductCreated => "product.created",
            Self::ProductUpdated => "product.updated",
            Self::DiscountCreated => "discount.created",
            Self::DiscountUpdated => "discount.updated",
            Self::DiscountDeleted => "discount.deleted",
            Self::BenefitCreated => "benefit.created",
            Self::BenefitUpdated => "benefit.updated",
            Self::BenefitGrantCreated => "benefit_grant.created",
            Self::BenefitGrantCycled => "benefit_grant.cycled",
            Self::BenefitGrantUpdated => "benefit_grant.updated",
            Self::BenefitGrantRevoked => "benefit_grant.revoked",
            Self::OrganizationUpdated => "organization.updated",
        }
    }
}
impl ::std::fmt::Display for WebhookEventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for WebhookEventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum WebhookFormat {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "discord")]
    Discord,
    #[serde(rename = "slack")]
    Slack,
}
impl WebhookFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Discord => "discord",
            Self::Slack => "slack",
        }
    }
}
impl ::std::fmt::Display for WebhookFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for WebhookFormat {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum WebhooksListWebhookDeliveriesHttpCodeClass {
    #[serde(rename = "2xx")]
    Value2xx,
    #[serde(rename = "3xx")]
    Value3xx,
    #[serde(rename = "4xx")]
    Value4xx,
    #[serde(rename = "5xx")]
    Value5xx,
}
impl WebhooksListWebhookDeliveriesHttpCodeClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Value2xx => "2xx",
            Self::Value3xx => "3xx",
            Self::Value4xx => "4xx",
            Self::Value5xx => "5xx",
        }
    }
}
impl ::std::fmt::Display for WebhooksListWebhookDeliveriesHttpCodeClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for WebhooksListWebhookDeliveriesHttpCodeClass {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
