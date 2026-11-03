#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerSeatAssign {
    /// Subscription ID. Required if neither order_id nor checkout_id is
    /// provided.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub subscription_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::SubscriptionId>,
    /// Order ID for one-time purchases. Required if subscription_id is not
    /// provided.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub order_id: presence_rs::Presence<crate::v2027_01::models::common::ids::OrderId>,
    /// Email of the customer to assign the seat to
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    /// External customer ID for the seat assignment
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_customer_id: presence_rs::Presence<String>,
    /// Customer ID for the seat assignment
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_id: presence_rs::Presence<crate::v2027_01::models::common::ids::CustomerId>,
    /// External member ID for the seat assignment. Can be used alone (lookup
    /// existing member) or with email (create/validate member).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Member ID for the seat assignment.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// Additional metadata for the seat (max 10 keys, 1KB total)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub metadata: presence_rs::Presence<std::collections::BTreeMap<String, serde_json::Value>>,
    /// If true, the seat will be immediately claimed without sending an
    /// invitation email. API-only feature.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub immediate_claim: Option<bool>,
    /// Checkout ID. Resolves to the subscription or order produced by the
    /// checkout.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub checkout_id: presence_rs::Presence<crate::v2027_01::models::common::ids::CheckoutId>,
}
#[allow(clippy::type_complexity)]
impl CustomerSeatAssign {
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscription ID. Required if neither order_id nor checkout_id is
    /// provided.
    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(
        mut self,
        value: crate::v2027_01::models::common::ids::SubscriptionId,
    ) -> Self {
        self.subscription_id = presence_rs::Presence::Some(value);
        self
    }

    /// Order ID for one-time purchases. Required if subscription_id is not
    /// provided.
    #[allow(clippy::should_implement_trait)]
    pub fn order_id(mut self, value: crate::v2027_01::models::common::ids::OrderId) -> Self {
        self.order_id = presence_rs::Presence::Some(value);
        self
    }

    /// Email of the customer to assign the seat to
    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    /// External customer ID for the seat assignment
    #[allow(clippy::should_implement_trait)]
    pub fn external_customer_id(mut self, value: impl Into<String>) -> Self {
        self.external_customer_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Customer ID for the seat assignment
    #[allow(clippy::should_implement_trait)]
    pub fn customer_id(mut self, value: crate::v2027_01::models::common::ids::CustomerId) -> Self {
        self.customer_id = presence_rs::Presence::Some(value);
        self
    }

    /// External member ID for the seat assignment. Can be used alone (lookup
    /// existing member) or with email (create/validate member).
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Member ID for the seat assignment.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// Additional metadata for the seat (max 10 keys, 1KB total)
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.metadata = presence_rs::Presence::Some(value);
        self
    }

    /// If true, the seat will be immediately claimed without sending an
    /// invitation email. API-only feature.
    #[allow(clippy::should_implement_trait)]
    pub fn immediate_claim(mut self, value: bool) -> Self {
        self.immediate_claim = Some(value);
        self
    }

    /// Checkout ID. Resolves to the subscription or order produced by the
    /// checkout.
    #[allow(clippy::should_implement_trait)]
    pub fn checkout_id(mut self, value: crate::v2027_01::models::common::ids::CheckoutId) -> Self {
        self.checkout_id = presence_rs::Presence::Some(value);
        self
    }
}
