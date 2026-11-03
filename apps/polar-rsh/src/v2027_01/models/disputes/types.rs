#[derive(Debug, Clone)]
pub struct DisputeRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DisputeId,
    /// Status of the dispute. `prevented` means we issued a refund before the
    /// dispute was escalated, avoiding any fees.
    pub status: crate::v2027_01::models::common::enums::DisputeStatus,
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
    pub order_id: crate::v2027_01::models::common::ids::OrderId,
    /// The ID of the payment associated with the dispute.
    pub payment_id: crate::v2027_01::models::common::ids::PaymentId,
    /// The customer who was charged for the disputed payment.
    pub customer: crate::v2027_01::models::disputes::types::DisputeCustomer,
    /// The ID of the support case for this dispute, if one was opened.
    pub case_id: Option<crate::v2027_01::models::common::ids::CaseId>,
}
/// Schema representing a dispute.
///
/// A dispute is a challenge raised by a customer or their bank regarding a
/// payment.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Dispute {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DisputeId,
    /// Status of the dispute. `prevented` means we issued a refund before the
    /// dispute was escalated, avoiding any fees.
    pub status: crate::v2027_01::models::common::enums::DisputeStatus,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub reason: Option<String>,
    /// Deadline to submit evidence in response to the dispute. `None` when no
    /// response is required.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub evidence_due_by: Option<jiff::Timestamp>,
    /// Whether the evidence submission deadline has passed.
    pub past_due: bool,
    /// The ID of the order associated with the dispute.
    pub order_id: crate::v2027_01::models::common::ids::OrderId,
    /// The ID of the payment associated with the dispute.
    pub payment_id: crate::v2027_01::models::common::ids::PaymentId,
    /// The customer who was charged for the disputed payment.
    pub customer: crate::v2027_01::models::disputes::types::DisputeCustomer,
    /// The ID of the support case for this dispute, if one was opened.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub case_id: Option<crate::v2027_01::models::common::ids::CaseId>,
}
#[allow(clippy::type_complexity)]
impl Dispute {
    pub fn new(required: DisputeRequired) -> Self {
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
            customer: required.customer,
            case_id: required.case_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DisputeCustomerRequired {
    /// The ID of the customer.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The type of customer: 'individual' for single users, 'team' for
    /// customers with multiple members.
    pub r#type: crate::v2027_01::models::common::enums::CustomerType,
    /// The name of the customer.
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    pub first_user_event_at: Option<jiff::Timestamp>,
    pub avatar_url: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DisputeCustomer {
    /// The ID of the customer.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    /// The email address of the customer. This must be unique within the
    /// organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The type of customer: 'individual' for single users, 'team' for
    /// customers with multiple members.
    pub r#type: crate::v2027_01::models::common::enums::CustomerType,
    /// The name of the customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl DisputeCustomer {
    pub fn new(required: DisputeCustomerRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            metadata: required.metadata,
            external_id: presence_rs::Presence::Absent,
            email: presence_rs::Presence::Absent,
            email_verified: required.email_verified,
            r#type: required.r#type,
            name: required.name,
            billing_name: required.billing_name,
            billing_address: required.billing_address,
            tax_id: required.tax_id,
            locale: presence_rs::Presence::Absent,
            organization_id: required.organization_id,
            default_payment_method_id: presence_rs::Presence::Absent,
            deleted_at: required.deleted_at,
            first_user_event_at: required.first_user_event_at,
            avatar_url: required.avatar_url,
        }
    }

    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// The email address of the customer. This must be unique within the
    /// organization.
    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[allow(clippy::should_implement_trait)]
    pub fn default_payment_method_id(
        mut self,
        value: crate::v2027_01::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct DisputeNotOpenErrorRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct DisputeNotOpenError {
    pub error: monostate::MustBe!("DisputeNotOpenError"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl DisputeNotOpenError {
    pub fn new(required: DisputeNotOpenErrorRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
