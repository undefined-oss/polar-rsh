#[derive(Debug, Clone)]
pub struct CardPaymentRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::PaymentId,
    /// The payment processor.
    pub processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// The payment status.
    pub status: crate::v2026_10::models::payments::enums::PaymentStatus,
    /// The payment amount in cents.
    pub amount: i64,
    /// The payment currency
    pub currency: String,
    /// What initiated this payment attempt, e.g. initial purchase, subscription
    /// renewal, or an automated dunning retry.
    pub trigger: Option<crate::v2026_10::models::payments::enums::PaymentTrigger>,
    /// Error code, if the payment was declined.
    pub decline_reason: Option<String>,
    /// Human-readable error message, if the payment was declined.
    pub decline_message: Option<String>,
    /// The ID of the organization that owns the payment.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the checkout session associated with this payment.
    pub checkout_id: Option<crate::v2026_10::models::common::ids::CheckoutId>,
    /// The ID of the order associated with this payment.
    pub order_id: Option<crate::v2026_10::models::common::ids::OrderId>,
    /// Additional metadata for the card payment method.
    pub method_metadata: crate::v2026_10::models::payments::types::CardPaymentMetadata,
}
/// Schema of a payment with a card payment method.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CardPayment {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::PaymentId,
    /// The payment processor.
    pub processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// The payment status.
    pub status: crate::v2026_10::models::payments::enums::PaymentStatus,
    /// The payment amount in cents.
    pub amount: i64,
    /// The payment currency
    pub currency: String,
    /// The payment method used.
    pub method: monostate::MustBe!("card"),
    /// What initiated this payment attempt, e.g. initial purchase, subscription
    /// renewal, or an automated dunning retry.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trigger: Option<crate::v2026_10::models::payments::enums::PaymentTrigger>,
    /// Error code, if the payment was declined.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub decline_reason: Option<String>,
    /// Human-readable error message, if the payment was declined.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub decline_message: Option<String>,
    /// The ID of the organization that owns the payment.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the checkout session associated with this payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_10::models::common::ids::CheckoutId>,
    /// The ID of the order associated with this payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_10::models::common::ids::OrderId>,
    /// Additional metadata from the payment processor for internal use.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub processor_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Additional metadata for the card payment method.
    pub method_metadata: crate::v2026_10::models::payments::types::CardPaymentMetadata,
}
#[allow(clippy::type_complexity)]
impl CardPayment {
    pub fn new(required: CardPaymentRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            processor: required.processor,
            status: required.status,
            amount: required.amount,
            currency: required.currency,
            method: Default::default(),
            trigger: required.trigger,
            decline_reason: required.decline_reason,
            decline_message: required.decline_message,
            organization_id: required.organization_id,
            checkout_id: required.checkout_id,
            order_id: required.order_id,
            processor_metadata: None,
            method_metadata: required.method_metadata,
        }
    }

    /// Additional metadata from the payment processor for internal use.
    #[allow(clippy::should_implement_trait)]
    pub fn processor_metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.processor_metadata = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CardPaymentMetadataRequired {
    /// The brand of the card used for the payment.
    pub brand: String,
    /// The last 4 digits of the card number.
    pub last4: String,
}
/// Additional metadata for a card payment method.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CardPaymentMetadata {
    /// The brand of the card used for the payment.
    pub brand: String,
    /// The last 4 digits of the card number.
    pub last4: String,
}
#[allow(clippy::type_complexity)]
impl CardPaymentMetadata {
    pub fn new(required: CardPaymentMetadataRequired) -> Self {
        Self { brand: required.brand, last4: required.last4 }
    }
}
#[derive(Debug, Clone)]
pub struct GenericPaymentRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::PaymentId,
    /// The payment processor.
    pub processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// The payment status.
    pub status: crate::v2026_10::models::payments::enums::PaymentStatus,
    /// The payment amount in cents.
    pub amount: i64,
    /// The payment currency
    pub currency: String,
    /// The payment method used.
    pub method: String,
    /// What initiated this payment attempt, e.g. initial purchase, subscription
    /// renewal, or an automated dunning retry.
    pub trigger: Option<crate::v2026_10::models::payments::enums::PaymentTrigger>,
    /// Error code, if the payment was declined.
    pub decline_reason: Option<String>,
    /// Human-readable error message, if the payment was declined.
    pub decline_message: Option<String>,
    /// The ID of the organization that owns the payment.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the checkout session associated with this payment.
    pub checkout_id: Option<crate::v2026_10::models::common::ids::CheckoutId>,
    /// The ID of the order associated with this payment.
    pub order_id: Option<crate::v2026_10::models::common::ids::OrderId>,
}
/// Schema of a payment with a generic payment method.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct GenericPayment {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::PaymentId,
    /// The payment processor.
    pub processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// The payment status.
    pub status: crate::v2026_10::models::payments::enums::PaymentStatus,
    /// The payment amount in cents.
    pub amount: i64,
    /// The payment currency
    pub currency: String,
    /// The payment method used.
    pub method: String,
    /// What initiated this payment attempt, e.g. initial purchase, subscription
    /// renewal, or an automated dunning retry.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trigger: Option<crate::v2026_10::models::payments::enums::PaymentTrigger>,
    /// Error code, if the payment was declined.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub decline_reason: Option<String>,
    /// Human-readable error message, if the payment was declined.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub decline_message: Option<String>,
    /// The ID of the organization that owns the payment.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the checkout session associated with this payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_10::models::common::ids::CheckoutId>,
    /// The ID of the order associated with this payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_10::models::common::ids::OrderId>,
    /// Additional metadata from the payment processor for internal use.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub processor_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
#[allow(clippy::type_complexity)]
impl GenericPayment {
    pub fn new(required: GenericPaymentRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            processor: required.processor,
            status: required.status,
            amount: required.amount,
            currency: required.currency,
            method: required.method,
            trigger: required.trigger,
            decline_reason: required.decline_reason,
            decline_message: required.decline_message,
            organization_id: required.organization_id,
            checkout_id: required.checkout_id,
            order_id: required.order_id,
            processor_metadata: None,
        }
    }

    /// Additional metadata from the payment processor for internal use.
    #[allow(clippy::should_implement_trait)]
    pub fn processor_metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.processor_metadata = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct KrCardPaymentRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::PaymentId,
    /// The payment processor.
    pub processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// The payment status.
    pub status: crate::v2026_10::models::payments::enums::PaymentStatus,
    /// The payment amount in cents.
    pub amount: i64,
    /// The payment currency
    pub currency: String,
    /// What initiated this payment attempt, e.g. initial purchase, subscription
    /// renewal, or an automated dunning retry.
    pub trigger: Option<crate::v2026_10::models::payments::enums::PaymentTrigger>,
    /// Error code, if the payment was declined.
    pub decline_reason: Option<String>,
    /// Human-readable error message, if the payment was declined.
    pub decline_message: Option<String>,
    /// The ID of the organization that owns the payment.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the checkout session associated with this payment.
    pub checkout_id: Option<crate::v2026_10::models::common::ids::CheckoutId>,
    /// The ID of the order associated with this payment.
    pub order_id: Option<crate::v2026_10::models::common::ids::OrderId>,
    /// Additional metadata for the South Korean card payment method.
    pub method_metadata: crate::v2026_10::models::payments::types::KrCardPaymentMetadata,
}
/// Schema of a payment with a South Korean card payment method.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct KrCardPayment {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::PaymentId,
    /// The payment processor.
    pub processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// The payment status.
    pub status: crate::v2026_10::models::payments::enums::PaymentStatus,
    /// The payment amount in cents.
    pub amount: i64,
    /// The payment currency
    pub currency: String,
    /// The payment method used.
    pub method: monostate::MustBe!("kr_card"),
    /// What initiated this payment attempt, e.g. initial purchase, subscription
    /// renewal, or an automated dunning retry.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trigger: Option<crate::v2026_10::models::payments::enums::PaymentTrigger>,
    /// Error code, if the payment was declined.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub decline_reason: Option<String>,
    /// Human-readable error message, if the payment was declined.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub decline_message: Option<String>,
    /// The ID of the organization that owns the payment.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the checkout session associated with this payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_10::models::common::ids::CheckoutId>,
    /// The ID of the order associated with this payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2026_10::models::common::ids::OrderId>,
    /// Additional metadata from the payment processor for internal use.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub processor_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Additional metadata for the South Korean card payment method.
    pub method_metadata: crate::v2026_10::models::payments::types::KrCardPaymentMetadata,
}
#[allow(clippy::type_complexity)]
impl KrCardPayment {
    pub fn new(required: KrCardPaymentRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            processor: required.processor,
            status: required.status,
            amount: required.amount,
            currency: required.currency,
            method: Default::default(),
            trigger: required.trigger,
            decline_reason: required.decline_reason,
            decline_message: required.decline_message,
            organization_id: required.organization_id,
            checkout_id: required.checkout_id,
            order_id: required.order_id,
            processor_metadata: None,
            method_metadata: required.method_metadata,
        }
    }

    /// Additional metadata from the payment processor for internal use.
    #[allow(clippy::should_implement_trait)]
    pub fn processor_metadata(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.processor_metadata = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct KrCardPaymentMetadataRequired {
    /// The local South Korean card brand used for the payment.
    pub brand: Option<String>,
    /// The last 4 digits of the card number.
    pub last4: Option<String>,
}
/// Additional metadata for a South Korean card payment method.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct KrCardPaymentMetadata {
    /// The local South Korean card brand used for the payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub brand: Option<String>,
    /// The last 4 digits of the card number.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last4: Option<String>,
}
#[allow(clippy::type_complexity)]
impl KrCardPaymentMetadata {
    pub fn new(required: KrCardPaymentMetadataRequired) -> Self {
        Self { brand: required.brand, last4: required.last4 }
    }
}
