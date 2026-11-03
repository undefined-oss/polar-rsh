#[derive(Debug, Clone)]
pub struct RefundRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::RefundId,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    pub status: crate::v2026_04::models::refunds::enums::RefundStatus,
    pub reason: crate::v2026_04::models::refunds::enums::RefundReason,
    pub amount: i64,
    pub tax_amount: i64,
    pub currency: String,
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    pub order_id: crate::v2026_04::models::common::ids::OrderId,
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub revoke_benefits: bool,
    pub dispute: Option<crate::v2026_04::models::refunds::types::RefundDispute>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Refund {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::RefundId,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    pub status: crate::v2026_04::models::refunds::enums::RefundStatus,
    pub reason: crate::v2026_04::models::refunds::enums::RefundReason,
    pub amount: i64,
    pub tax_amount: i64,
    pub currency: String,
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    pub order_id: crate::v2026_04::models::common::ids::OrderId,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub revoke_benefits: bool,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub dispute: Option<crate::v2026_04::models::refunds::types::RefundDispute>,
}
#[allow(clippy::type_complexity)]
impl Refund {
    pub fn new(required: RefundRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            metadata: required.metadata,
            status: required.status,
            reason: required.reason,
            amount: required.amount,
            tax_amount: required.tax_amount,
            currency: required.currency,
            organization_id: required.organization_id,
            order_id: required.order_id,
            subscription_id: required.subscription_id,
            customer_id: required.customer_id,
            revoke_benefits: required.revoke_benefits,
            dispute: required.dispute,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RefundCreateRequired {
    pub order_id: crate::v2026_04::models::common::ids::OrderId,
    /// Reason for the refund.
    pub reason: crate::v2026_04::models::refunds::enums::RefundCreateReason,
    /// Amount to refund in cents. Minimum is 1.
    pub amount: crate::v2026_04::models::refunds::newtypes::RefundAmount,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct RefundCreate {
    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::refunds::newtypes::RefundMetadataKey,
            crate::v2026_04::models::refunds::unions::RefundCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub order_id: crate::v2026_04::models::common::ids::OrderId,
    /// Reason for the refund.
    pub reason: crate::v2026_04::models::refunds::enums::RefundCreateReason,
    /// Amount to refund in cents. Minimum is 1.
    pub amount: crate::v2026_04::models::refunds::newtypes::RefundAmount,
    /// An internal comment about the refund.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub comment: presence_rs::Presence<String>,
    /// Should this refund trigger the associated customer benefits to be
    /// revoked?
    ///
    /// **Note:**
    /// Only allowed in case the `order` is a one-time purchase.
    /// Subscriptions automatically revoke customer benefits once the
    /// subscription itself is revoked, i.e fully canceled.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub revoke_benefits: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl RefundCreate {
    pub fn new(required: RefundCreateRequired) -> Self {
        Self {
            metadata: None,
            order_id: required.order_id,
            reason: required.reason,
            amount: required.amount,
            comment: presence_rs::Presence::Absent,
            revoke_benefits: None,
        }
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_04::models::refunds::newtypes::RefundMetadataKey,
            crate::v2026_04::models::refunds::unions::RefundCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// An internal comment about the refund.
    #[allow(clippy::should_implement_trait)]
    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = presence_rs::Presence::Some(value.into());
        self
    }

    /// Should this refund trigger the associated customer benefits to be
    /// revoked?
    ///
    /// **Note:**
    /// Only allowed in case the `order` is a one-time purchase.
    /// Subscriptions automatically revoke customer benefits once the
    /// subscription itself is revoked, i.e fully canceled.
    #[allow(clippy::should_implement_trait)]
    pub fn revoke_benefits(mut self, value: bool) -> Self {
        self.revoke_benefits = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct RefundDisputeRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::DisputeId,
    /// Status of the dispute. `prevented` means we issued a refund before the
    /// dispute was escalated, avoiding any fees.
    pub status: crate::v2026_04::models::common::enums::DisputeStatus,
    /// Whether the dispute has been resolved (won or lost).
    pub resolved: bool,
    /// Whether the dispute is closed (prevented, won, or lost).
    pub closed: bool,
    /// Amount in cents disputed.
    pub amount: i64,
    /// Tax amount in cents disputed.
    pub tax_amount: i64,
    /// Currency code of the dispute.
    pub currency: String,
    /// The reason for the dispute as reported by the card network (e.g.
    /// `fraudulent`, `product_not_received`). `None` until the processor
    /// reports it.
    pub reason: Option<String>,
    /// Deadline to submit evidence in response to the dispute. `None` when no
    /// response is required.
    pub evidence_due_by: Option<jiff::Timestamp>,
    /// Whether the evidence submission deadline has passed.
    pub past_due: bool,
    /// The ID of the order associated with the dispute.
    pub order_id: crate::v2026_04::models::common::ids::OrderId,
    /// The ID of the payment associated with the dispute.
    pub payment_id: crate::v2026_04::models::common::ids::PaymentId,
}
/// Dispute associated with a refund,
/// in case we prevented a dispute by issuing a refund.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct RefundDispute {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::DisputeId,
    /// Status of the dispute. `prevented` means we issued a refund before the
    /// dispute was escalated, avoiding any fees.
    pub status: crate::v2026_04::models::common::enums::DisputeStatus,
    /// Whether the dispute has been resolved (won or lost).
    pub resolved: bool,
    /// Whether the dispute is closed (prevented, won, or lost).
    pub closed: bool,
    /// Amount in cents disputed.
    pub amount: i64,
    /// Tax amount in cents disputed.
    pub tax_amount: i64,
    /// Currency code of the dispute.
    pub currency: String,
    /// The reason for the dispute as reported by the card network (e.g.
    /// `fraudulent`, `product_not_received`). `None` until the processor
    /// reports it.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub reason: Option<String>,
    /// Deadline to submit evidence in response to the dispute. `None` when no
    /// response is required.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub evidence_due_by: Option<jiff::Timestamp>,
    /// Whether the evidence submission deadline has passed.
    pub past_due: bool,
    /// The ID of the order associated with the dispute.
    pub order_id: crate::v2026_04::models::common::ids::OrderId,
    /// The ID of the payment associated with the dispute.
    pub payment_id: crate::v2026_04::models::common::ids::PaymentId,
}
#[allow(clippy::type_complexity)]
impl RefundDispute {
    pub fn new(required: RefundDisputeRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            status: required.status,
            resolved: required.resolved,
            closed: required.closed,
            amount: required.amount,
            tax_amount: required.tax_amount,
            currency: required.currency,
            reason: required.reason,
            evidence_due_by: required.evidence_due_by,
            past_due: required.past_due,
            order_id: required.order_id,
            payment_id: required.payment_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RefundedAlreadyRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct RefundedAlready {
    pub error: monostate::MustBe!("RefundedAlready"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl RefundedAlready {
    pub fn new(required: RefundedAlreadyRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
