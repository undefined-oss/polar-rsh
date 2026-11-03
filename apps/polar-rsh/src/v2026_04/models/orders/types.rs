#[derive(Debug, Clone)]
pub struct CustomerOrderRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrderId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub status: crate::v2026_04::models::orders::enums::OrderStatus,
    /// Whether the order has been paid for.
    pub paid: bool,
    /// Amount in cents, before discounts and taxes.
    pub subtotal_amount: i64,
    /// Discount amount in cents.
    pub discount_amount: i64,
    /// Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    /// Sales tax amount in cents.
    pub tax_amount: i64,
    /// Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    /// Customer's balance amount applied to this invoice. Can increase the
    /// total amount paid, if the customer has a negative balance,  or decrease
    /// it, if the customer has a positive balance.Amount in cents.
    pub applied_balance_amount: i64,
    /// Amount in cents that is due for this order.
    pub due_amount: i64,
    /// Amount refunded in cents.
    pub refunded_amount: i64,
    /// Sales tax refunded in cents.
    pub refunded_tax_amount: i64,
    pub currency: String,
    pub billing_reason: crate::v2026_04::models::orders::enums::OrderBillingReason,
    /// The name of the customer that should appear on the invoice.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    /// The invoice number associated with this order. `null` while the order is
    /// in `draft` status; assigned at finalize.
    pub invoice_number: Option<String>,
    /// Whether an invoice has been generated for this order.
    pub is_invoice_generated: bool,
    /// The receipt number for this order. Set once the order is paid for
    /// organizations with receipts enabled. When set, a downloadable receipt
    /// PDF can be obtained via the receipt endpoint.
    pub receipt_number: Option<String>,
    /// Number of units purchased (for unit-based pricing).
    pub units: Option<i64>,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub product_id: Option<crate::v2026_04::models::common::ids::ProductId>,
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    pub product: Option<crate::v2026_04::models::orders::types::CustomerOrderProduct>,
    pub subscription: Option<crate::v2026_04::models::orders::types::CustomerOrderSubscription>,
    /// Line items composing the order.
    pub items: Vec<crate::v2026_04::models::orders::types::OrderItemSchema>,
    /// A summary description of the order.
    pub description: String,
    /// Amount in cents that can still be refunded (net, before taxes). Accounts
    /// for any applied customer balance and previous refunds.
    pub refundable_amount: i64,
    /// Sales tax in cents that would be refunded if the full refundable amount
    /// is refunded.
    pub refundable_tax_amount: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrder {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrderId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub status: crate::v2026_04::models::orders::enums::OrderStatus,
    /// Whether the order has been paid for.
    pub paid: bool,
    /// Amount in cents, before discounts and taxes.
    pub subtotal_amount: i64,
    /// Discount amount in cents.
    pub discount_amount: i64,
    /// Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    /// Sales tax amount in cents.
    pub tax_amount: i64,
    /// Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    /// Customer's balance amount applied to this invoice. Can increase the
    /// total amount paid, if the customer has a negative balance,  or decrease
    /// it, if the customer has a positive balance.Amount in cents.
    pub applied_balance_amount: i64,
    /// Amount in cents that is due for this order.
    pub due_amount: i64,
    /// Amount refunded in cents.
    pub refunded_amount: i64,
    /// Sales tax refunded in cents.
    pub refunded_tax_amount: i64,
    pub currency: String,
    pub billing_reason: crate::v2026_04::models::orders::enums::OrderBillingReason,
    /// The name of the customer that should appear on the invoice.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    /// The invoice number associated with this order. `null` while the order is
    /// in `draft` status; assigned at finalize.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub invoice_number: Option<String>,
    /// Whether an invoice has been generated for this order.
    pub is_invoice_generated: bool,
    /// The receipt number for this order. Set once the order is paid for
    /// organizations with receipts enabled. When set, a downloadable receipt
    /// PDF can be obtained via the receipt endpoint.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub receipt_number: Option<String>,
    /// Number of seats purchased (for seat-based one-time orders).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    /// Number of units purchased (for unit-based pricing).
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_id: Option<crate::v2026_04::models::common::ids::ProductId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    /// When the next automatic payment retry is scheduled. `null` if the order
    /// is not in dunning or all retries have been exhausted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub next_payment_attempt_at: presence_rs::Presence<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product: Option<crate::v2026_04::models::orders::types::CustomerOrderProduct>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription: Option<crate::v2026_04::models::orders::types::CustomerOrderSubscription>,
    /// Line items composing the order.
    pub items: Vec<crate::v2026_04::models::orders::types::OrderItemSchema>,
    /// A summary description of the order.
    pub description: String,
    /// Amount in cents that can still be refunded (net, before taxes). Accounts
    /// for any applied customer balance and previous refunds.
    pub refundable_amount: i64,
    /// Sales tax in cents that would be refunded if the full refundable amount
    /// is refunded.
    pub refundable_tax_amount: i64,
}
#[allow(clippy::type_complexity)]
impl CustomerOrder {
    pub fn new(required: CustomerOrderRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            status: required.status,
            paid: required.paid,
            subtotal_amount: required.subtotal_amount,
            discount_amount: required.discount_amount,
            net_amount: required.net_amount,
            tax_amount: required.tax_amount,
            total_amount: required.total_amount,
            applied_balance_amount: required.applied_balance_amount,
            due_amount: required.due_amount,
            refunded_amount: required.refunded_amount,
            refunded_tax_amount: required.refunded_tax_amount,
            currency: required.currency,
            billing_reason: required.billing_reason,
            billing_name: required.billing_name,
            billing_address: required.billing_address,
            invoice_number: required.invoice_number,
            is_invoice_generated: required.is_invoice_generated,
            receipt_number: required.receipt_number,
            seats: presence_rs::Presence::Absent,
            units: required.units,
            customer_id: required.customer_id,
            product_id: required.product_id,
            discount_id: required.discount_id,
            subscription_id: required.subscription_id,
            checkout_id: required.checkout_id,
            next_payment_attempt_at: presence_rs::Presence::Absent,
            product: required.product,
            subscription: required.subscription,
            items: required.items,
            description: required.description,
            refundable_amount: required.refundable_amount,
            refundable_tax_amount: required.refundable_tax_amount,
        }
    }

    /// Number of seats purchased (for seat-based one-time orders).
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// When the next automatic payment retry is scheduled. `null` if the order
    /// is not in dunning or all retries have been exhausted.
    #[allow(clippy::should_implement_trait)]
    pub fn next_payment_attempt_at(mut self, value: jiff::Timestamp) -> Self {
        self.next_payment_attempt_at = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to confirm a retry payment using either a saved payment method or a
/// new confirmation token.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerOrderConfirmPayment {
    /// ID of the Stripe confirmation token for new payment methods.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub confirmation_token_id: presence_rs::Presence<String>,
    /// ID of an existing saved payment method.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub payment_method_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::PaymentMethodId>,
    /// Payment processor used.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub payment_processor: Option<crate::v2026_04::models::common::enums::PaymentProcessor>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderConfirmPayment {
    pub fn new() -> Self {
        Self::default()
    }

    /// ID of the Stripe confirmation token for new payment methods.
    #[allow(clippy::should_implement_trait)]
    pub fn confirmation_token_id(mut self, value: impl Into<String>) -> Self {
        self.confirmation_token_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// ID of an existing saved payment method.
    #[allow(clippy::should_implement_trait)]
    pub fn payment_method_id(
        mut self,
        value: crate::v2026_04::models::common::ids::PaymentMethodId,
    ) -> Self {
        self.payment_method_id = presence_rs::Presence::Some(value);
        self
    }

    /// Payment processor used.
    #[allow(clippy::should_implement_trait)]
    pub fn payment_processor(
        mut self,
        value: crate::v2026_04::models::common::enums::PaymentProcessor,
    ) -> Self {
        self.payment_processor = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrderInvoiceRequired {
    /// The URL to the invoice.
    pub url: String,
}
/// Order's invoice data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrderInvoice {
    /// The URL to the invoice.
    pub url: String,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderInvoice {
    pub fn new(required: CustomerOrderInvoiceRequired) -> Self {
        Self { url: required.url }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrderPaymentConfirmationRequired {
    /// Payment status after confirmation.
    pub status: String,
}
/// Response after confirming a retry payment.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrderPaymentConfirmation {
    /// Payment status after confirmation.
    pub status: String,
    /// Client secret for handling additional actions.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub client_secret: presence_rs::Presence<String>,
    /// Error message if confirmation failed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderPaymentConfirmation {
    pub fn new(required: CustomerOrderPaymentConfirmationRequired) -> Self {
        Self {
            status: required.status,
            client_secret: presence_rs::Presence::Absent,
            error: presence_rs::Presence::Absent,
        }
    }

    /// Client secret for handling additional actions.
    #[allow(clippy::should_implement_trait)]
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = presence_rs::Presence::Some(value.into());
        self
    }

    /// Error message if confirmation failed.
    #[allow(clippy::should_implement_trait)]
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrderPaymentStatusRequired {
    /// Current payment status.
    pub status: String,
}
/// Payment status for an order.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrderPaymentStatus {
    /// Current payment status.
    pub status: String,
    /// Error message if payment failed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderPaymentStatus {
    pub fn new(required: CustomerOrderPaymentStatusRequired) -> Self {
        Self { status: required.status, error: presence_rs::Presence::Absent }
    }

    /// Error message if payment failed.
    #[allow(clippy::should_implement_trait)]
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrderProductRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2026_04::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_04::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// List of prices for this product.
    pub prices: Vec<crate::v2026_04::models::orders::unions::CustomerOrderProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_04::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2026_04::models::common::types::ProductMediaFileRead>,
    pub organization: crate::v2026_04::models::common::types::CustomerOrganization,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrderProduct {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2026_04::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_04::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// List of prices for this product.
    pub prices: Vec<crate::v2026_04::models::orders::unions::CustomerOrderProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_04::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2026_04::models::common::types::ProductMediaFileRead>,
    pub organization: crate::v2026_04::models::common::types::CustomerOrganization,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderProduct {
    pub fn new(required: CustomerOrderProductRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            trial_interval: required.trial_interval,
            trial_interval_count: required.trial_interval_count,
            name: required.name,
            description: required.description,
            visibility: required.visibility,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            meter_interval: required.meter_interval,
            meter_interval_count: required.meter_interval_count,
            is_recurring: required.is_recurring,
            is_archived: required.is_archived,
            organization_id: required.organization_id,
            prices: required.prices,
            benefits: required.benefits,
            medias: required.medias,
            organization: required.organization,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrderReceiptRequired {
    /// The URL to the receipt PDF.
    pub url: String,
}
/// Order's receipt data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrderReceipt {
    /// The URL to the receipt PDF.
    pub url: String,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderReceipt {
    pub fn new(required: CustomerOrderReceiptRequired) -> Self {
        Self { url: required.url }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrderSubscriptionRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2026_04::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2026_04::models::common::enums::SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    /// The start timestamp of the current meter period, if the product has a
    /// meter cycle set. Metered credits are granted and overage is settled on
    /// this cadence.
    pub current_meter_period_start: Option<jiff::Timestamp>,
    /// The end timestamp of the current meter period, if the product has a
    /// meter cycle set. This is when credits next renew.
    pub current_meter_period_end: Option<jiff::Timestamp>,
    /// The start timestamp of the trial period, if any.
    pub trial_start: Option<jiff::Timestamp>,
    /// The end timestamp of the trial period, if any.
    pub trial_end: Option<jiff::Timestamp>,
    /// Whether the subscription will be canceled at the end of the current
    /// period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might
    /// still be active if `cancel_at_period_end` is `true`.
    pub canceled_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription started.
    pub started_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription will end.
    pub ends_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription ended.
    pub ended_at: Option<jiff::Timestamp>,
    /// Whether the subscription will be paused at the end of the current
    /// period.
    pub pause_at_period_end: bool,
    /// The timestamp when the subscription was paused.
    pub paused_at: Option<jiff::Timestamp>,
    /// The timestamp when a paused subscription is scheduled to automatically
    /// resume, if set.
    pub resumes_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed customer.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    pub units: Option<i64>,
    pub customer_cancellation_reason:
        Option<crate::v2026_04::models::common::enums::CustomerCancellationReason>,
    pub customer_cancellation_comment: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrderSubscription {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2026_04::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2026_04::models::common::enums::SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    /// The start timestamp of the current meter period, if the product has a
    /// meter cycle set. Metered credits are granted and overage is settled on
    /// this cadence.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_start: Option<jiff::Timestamp>,
    /// The end timestamp of the current meter period, if the product has a
    /// meter cycle set. This is when credits next renew.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_end: Option<jiff::Timestamp>,
    /// The start timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_start: Option<jiff::Timestamp>,
    /// The end timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    /// Whether the subscription will be canceled at the end of the current
    /// period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might
    /// still be active if `cancel_at_period_end` is `true`.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub canceled_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription started.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub started_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription will end.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription ended.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ended_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription entered `past_due` status.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub past_due_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the subscription will be paused at the end of the current
    /// period.
    pub pause_at_period_end: bool,
    /// The timestamp when the subscription was paused.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub paused_at: Option<jiff::Timestamp>,
    /// The timestamp when a paused subscription is scheduled to automatically
    /// resume, if set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub resumes_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed customer.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_reason:
        Option<crate::v2026_04::models::common::enums::CustomerCancellationReason>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_comment: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderSubscription {
    pub fn new(required: CustomerOrderSubscriptionRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            status: required.status,
            current_period_start: required.current_period_start,
            current_period_end: required.current_period_end,
            current_meter_period_start: required.current_meter_period_start,
            current_meter_period_end: required.current_meter_period_end,
            trial_start: required.trial_start,
            trial_end: required.trial_end,
            cancel_at_period_end: required.cancel_at_period_end,
            canceled_at: required.canceled_at,
            started_at: required.started_at,
            ends_at: required.ends_at,
            ended_at: required.ended_at,
            past_due_at: presence_rs::Presence::Absent,
            pause_at_period_end: required.pause_at_period_end,
            paused_at: required.paused_at,
            resumes_at: required.resumes_at,
            customer_id: required.customer_id,
            product_id: required.product_id,
            discount_id: required.discount_id,
            checkout_id: required.checkout_id,
            seats: presence_rs::Presence::Absent,
            units: required.units,
            customer_cancellation_reason: required.customer_cancellation_reason,
            customer_cancellation_comment: required.customer_cancellation_comment,
        }
    }

    /// The timestamp when the subscription entered `past_due` status.
    #[allow(clippy::should_implement_trait)]
    pub fn past_due_at(mut self, value: jiff::Timestamp) -> Self {
        self.past_due_at = presence_rs::Presence::Some(value);
        self
    }

    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update an order.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerOrderUpdate {
    /// The name of the customer that should appear on the invoice.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_name: presence_rs::Presence<String>,
    /// The address of the customer that should appear on the invoice. Country
    /// and state fields cannot be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address:
        presence_rs::Presence<crate::v2026_04::models::common::types::AddressInput>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrderUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// The name of the customer that should appear on the invoice.
    #[allow(clippy::should_implement_trait)]
    pub fn billing_name(mut self, value: impl Into<String>) -> Self {
        self.billing_name = presence_rs::Presence::Some(value.into());
        self
    }

    /// The address of the customer that should appear on the invoice. Country
    /// and state fields cannot be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ManualRetryLimitExceededRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct ManualRetryLimitExceeded {
    pub error: monostate::MustBe!("ManualRetryLimitExceeded"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl ManualRetryLimitExceeded {
    pub fn new(required: ManualRetryLimitExceededRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct MissingInvoiceBillingDetailsRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct MissingInvoiceBillingDetails {
    pub error: monostate::MustBe!("MissingInvoiceBillingDetails"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl MissingInvoiceBillingDetails {
    pub fn new(required: MissingInvoiceBillingDetailsRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OffSessionChargesNotEnabledRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct OffSessionChargesNotEnabled {
    pub error: monostate::MustBe!("OffSessionChargesNotEnabled"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl OffSessionChargesNotEnabled {
    pub fn new(required: OffSessionChargesNotEnabledRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OrderRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrderId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub status: crate::v2026_04::models::orders::enums::OrderStatus,
    /// Whether the order has been paid for.
    pub paid: bool,
    /// Amount in cents, before discounts and taxes.
    pub subtotal_amount: i64,
    /// Discount amount in cents.
    pub discount_amount: i64,
    /// Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    /// Sales tax amount in cents.
    pub tax_amount: i64,
    /// Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    /// Customer's balance amount applied to this invoice. Can increase the
    /// total amount paid, if the customer has a negative balance,  or decrease
    /// it, if the customer has a positive balance.Amount in cents.
    pub applied_balance_amount: i64,
    /// Amount in cents that is due for this order.
    pub due_amount: i64,
    /// Amount refunded in cents.
    pub refunded_amount: i64,
    /// Sales tax refunded in cents.
    pub refunded_tax_amount: i64,
    pub currency: String,
    pub billing_reason: crate::v2026_04::models::orders::enums::OrderBillingReason,
    /// The name of the customer that should appear on the invoice.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    /// The invoice number associated with this order. `null` while the order is
    /// in `draft` status; assigned at finalize.
    pub invoice_number: Option<String>,
    /// Whether an invoice has been generated for this order.
    pub is_invoice_generated: bool,
    /// The receipt number for this order. Set once the order is paid for
    /// organizations with receipts enabled. When set, a downloadable receipt
    /// PDF can be obtained via the receipt endpoint.
    pub receipt_number: Option<String>,
    /// Number of units purchased (for unit-based pricing).
    pub units: Option<i64>,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub product_id: Option<crate::v2026_04::models::common::ids::ProductId>,
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// Platform fee amount in cents.
    pub platform_fee_amount: i64,
    /// Currency of the platform fee.
    pub platform_fee_currency: Option<String>,
    pub customer: crate::v2026_04::models::orders::types::OrderCustomer,
    pub product: Option<crate::v2026_04::models::orders::types::OrderProduct>,
    pub discount: Option<crate::v2026_04::models::orders::unions::OrderDiscountAnyOf0Union>,
    pub subscription: Option<crate::v2026_04::models::orders::types::OrderSubscription>,
    /// Line items composing the order.
    pub items: Vec<crate::v2026_04::models::orders::types::OrderItemSchema>,
    /// A summary description of the order.
    pub description: String,
    /// Amount in cents that can still be refunded (net, before taxes). Accounts
    /// for any applied customer balance and previous refunds.
    pub refundable_amount: i64,
    /// Sales tax in cents that would be refunded if the full refundable amount
    /// is refunded.
    pub refundable_tax_amount: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Order {
    ///The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrderId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub status: crate::v2026_04::models::orders::enums::OrderStatus,
    ///Whether the order has been paid for.
    pub paid: bool,
    ///Amount in cents, before discounts and taxes.
    pub subtotal_amount: i64,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents.
    pub tax_amount: i64,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Customer's balance amount applied to this invoice. Can increase the total amount paid, if the customer has a negative balance,  or decrease it, if the customer has a positive balance.Amount in cents.
    pub applied_balance_amount: i64,
    ///Amount in cents that is due for this order.
    pub due_amount: i64,
    ///Amount refunded in cents.
    pub refunded_amount: i64,
    ///Sales tax refunded in cents.
    pub refunded_tax_amount: i64,
    pub currency: String,
    pub billing_reason: crate::v2026_04::models::orders::enums::OrderBillingReason,
    ///The name of the customer that should appear on the invoice.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    ///The invoice number associated with this order. `null` while the order is in `draft` status; assigned at finalize.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub invoice_number: Option<String>,
    ///Whether an invoice has been generated for this order.
    pub is_invoice_generated: bool,
    ///The receipt number for this order. Set once the order is paid for organizations with receipts enabled. When set, a downloadable receipt PDF can be obtained via the receipt endpoint.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub receipt_number: Option<String>,
    ///Number of seats purchased (for seat-based one-time orders).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    ///Number of units purchased (for unit-based pricing).
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_id: Option<crate::v2026_04::models::common::ids::ProductId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_04::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    ///When the next automatic payment retry is scheduled. `null` if the order is not in dunning or all retries have been exhausted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub next_payment_attempt_at: presence_rs::Presence<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_04::models::orders::unions::OrderCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///Platform fee amount in cents.
    pub platform_fee_amount: i64,
    ///Currency of the platform fee.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub platform_fee_currency: Option<String>,
    pub customer: crate::v2026_04::models::orders::types::OrderCustomer,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product: Option<crate::v2026_04::models::orders::types::OrderProduct>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount: Option<
        crate::v2026_04::models::orders::unions::OrderDiscountAnyOf0Union,
    >,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription: Option<crate::v2026_04::models::orders::types::OrderSubscription>,
    ///Line items composing the order.
    pub items: Vec<crate::v2026_04::models::orders::types::OrderItemSchema>,
    ///A summary description of the order.
    pub description: String,
    ///Amount in cents that can still be refunded (net, before taxes). Accounts for any applied customer balance and previous refunds.
    pub refundable_amount: i64,
    ///Sales tax in cents that would be refunded if the full refundable amount is refunded.
    pub refundable_tax_amount: i64,
}
#[allow(clippy::type_complexity)]
impl Order {
    pub fn new(required: OrderRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            status: required.status,
            paid: required.paid,
            subtotal_amount: required.subtotal_amount,
            discount_amount: required.discount_amount,
            net_amount: required.net_amount,
            tax_amount: required.tax_amount,
            total_amount: required.total_amount,
            applied_balance_amount: required.applied_balance_amount,
            due_amount: required.due_amount,
            refunded_amount: required.refunded_amount,
            refunded_tax_amount: required.refunded_tax_amount,
            currency: required.currency,
            billing_reason: required.billing_reason,
            billing_name: required.billing_name,
            billing_address: required.billing_address,
            invoice_number: required.invoice_number,
            is_invoice_generated: required.is_invoice_generated,
            receipt_number: required.receipt_number,
            seats: presence_rs::Presence::Absent,
            units: required.units,
            customer_id: required.customer_id,
            product_id: required.product_id,
            discount_id: required.discount_id,
            subscription_id: required.subscription_id,
            checkout_id: required.checkout_id,
            next_payment_attempt_at: presence_rs::Presence::Absent,
            metadata: required.metadata,
            custom_field_data: None,
            platform_fee_amount: required.platform_fee_amount,
            platform_fee_currency: required.platform_fee_currency,
            customer: required.customer,
            product: required.product,
            discount: required.discount,
            subscription: required.subscription,
            items: required.items,
            description: required.description,
            refundable_amount: required.refundable_amount,
            refundable_tax_amount: required.refundable_tax_amount,
        }
    }

    /// Number of seats purchased (for seat-based one-time orders).
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// When the next automatic payment retry is scheduled. `null` if the order
    /// is not in dunning or all retries have been exhausted.
    #[allow(clippy::should_implement_trait)]
    pub fn next_payment_attempt_at(mut self, value: jiff::Timestamp) -> Self {
        self.next_payment_attempt_at = presence_rs::Presence::Some(value);
        self
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_04::models::orders::unions::OrderCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderCreateRequired {
    /// The ID of the customer the order is for. Must belong to the order's
    /// organization.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the one-time product to charge for. Must belong to the order's
    /// organization. Only fixed-price, free and unit-based products are
    /// supported.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
}
/// Schema to create a draft order for an off-session charge.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderCreate {
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_04::models::orders::unions::OrderCreateCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///Key-value object allowing you to store additional information.
    ///
    ///The key must be a string with a maximum length of **40 characters**.
    ///The value must be either:
    ///
    ///* A string with a maximum length of **500 characters**
    ///* An integer
    ///* A floating-point number
    ///* A boolean
    ///
    ///You can store up to **50 key-value pairs**.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::orders::newtypes::OrderMetadataKey,
            crate::v2026_04::models::orders::unions::OrderCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The ID of the organization the order belongs to. **Required unless you use an organization token.** The customer and product must belong to this organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    ///The ID of the customer the order is for. Must belong to the order's organization.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    ///The ID of the one-time product to charge for. Must belong to the order's organization. Only fixed-price, free and unit-based products are supported.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    ///The currency to charge in (ISO 4217, lowercase, e.g. `usd`). Defaults to the organization's default currency; specify it to force a different one, or when the product isn't priced in the organization's default currency.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<String>,
    ///A custom amount to charge, in the smallest currency unit. Overrides the product's price; defaults to the product's configured price (0 for free products). A positive amount must be at least the currency's minimum. Can't be combined with `units`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2026_04::models::orders::newtypes::OrderAmount,
    >,
    ///The number of units to charge for. Required when the product has unit-based pricing, and rejected otherwise. The amount comes from the price's tiers. Can't be combined with `amount`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_04::models::orders::newtypes::OrderUnits,
    >,
    ///A custom description for the order's line item, shown on the invoice and receipt (e.g. `5,000 tokens`). Defaults to the product name.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2026_04::models::orders::newtypes::OrderDescription,
    >,
}
#[allow(clippy::type_complexity)]
impl OrderCreate {
    pub fn new(required: OrderCreateRequired) -> Self {
        Self {
            custom_field_data: None,
            metadata: None,
            organization_id: presence_rs::Presence::Absent,
            customer_id: required.customer_id,
            product_id: required.product_id,
            currency: presence_rs::Presence::Absent,
            amount: presence_rs::Presence::Absent,
            units: presence_rs::Presence::Absent,
            description: presence_rs::Presence::Absent,
        }
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_04::models::orders::unions::OrderCreateCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
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
            crate::v2026_04::models::orders::newtypes::OrderMetadataKey,
            crate::v2026_04::models::orders::unions::OrderCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization the order belongs to. **Required unless you
    /// use an organization token.** The customer and product must belong to
    /// this organization.
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The currency to charge in (ISO 4217, lowercase, e.g. `usd`). Defaults to
    /// the organization's default currency; specify it to force a different
    /// one, or when the product isn't priced in the organization's default
    /// currency.
    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = presence_rs::Presence::Some(value.into());
        self
    }

    /// A custom amount to charge, in the smallest currency unit. Overrides the
    /// product's price; defaults to the product's configured price (0 for free
    /// products). A positive amount must be at least the currency's minimum.
    /// Can't be combined with `units`.
    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: crate::v2026_04::models::orders::newtypes::OrderAmount) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    /// The number of units to charge for. Required when the product has
    /// unit-based pricing, and rejected otherwise. The amount comes from the
    /// price's tiers. Can't be combined with `amount`.
    #[allow(clippy::should_implement_trait)]
    pub fn units(mut self, value: crate::v2026_04::models::orders::newtypes::OrderUnits) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    /// A custom description for the order's line item, shown on the invoice and
    /// receipt (e.g. `5,000 tokens`). Defaults to the product name.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2026_04::models::orders::newtypes::OrderDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderCustomerRequired {
    /// The ID of the customer.
    pub id: crate::v2026_04::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The type of customer: 'individual' for single users, 'team' for
    /// customers with multiple members.
    pub r#type: crate::v2026_04::models::common::enums::CustomerType,
    /// The name of the customer.
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
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
pub struct OrderCustomer {
    /// The ID of the customer.
    pub id: crate::v2026_04::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
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
    pub r#type: crate::v2026_04::models::common::enums::CustomerType,
    /// The name of the customer.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl OrderCustomer {
    pub fn new(required: OrderCustomerRequired) -> Self {
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
        value: crate::v2026_04::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to finalize a draft order and trigger an off-session charge.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrderFinalize {
    /// ID of the payment method to charge. Must belong to the order's customer.
    /// Falls back to the customer's default payment method when unset.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub payment_method_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::PaymentMethodId>,
}
#[allow(clippy::type_complexity)]
impl OrderFinalize {
    pub fn new() -> Self {
        Self::default()
    }

    /// ID of the payment method to charge. Must belong to the order's customer.
    /// Falls back to the customer's default payment method when unset.
    #[allow(clippy::should_implement_trait)]
    pub fn payment_method_id(
        mut self,
        value: crate::v2026_04::models::common::ids::PaymentMethodId,
    ) -> Self {
        self.payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderInvoiceRequired {
    /// The URL to the invoice.
    pub url: String,
}
/// Order's invoice data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderInvoice {
    /// The URL to the invoice.
    pub url: String,
}
#[allow(clippy::type_complexity)]
impl OrderInvoice {
    pub fn new(required: OrderInvoiceRequired) -> Self {
        Self { url: required.url }
    }
}
#[derive(Debug, Clone)]
pub struct OrderItemSchemaRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrderItemSchemaId,
    /// Description of the line item charge.
    pub label: String,
    /// Amount in cents, before discounts and taxes.
    pub amount: i64,
    /// Sales tax amount in cents.
    pub tax_amount: i64,
    /// Whether this charge is due to a proration.
    pub proration: bool,
    /// Associated price ID, if any.
    pub product_price_id: Option<crate::v2026_04::models::common::ids::ProductPriceId>,
    /// Start of the period covered by this line item, if any.
    pub start_timestamp: Option<jiff::Timestamp>,
    /// End of the period covered by this line item, if any.
    pub end_timestamp: Option<jiff::Timestamp>,
}
/// An order line item.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderItemSchema {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrderItemSchemaId,
    /// Description of the line item charge.
    pub label: String,
    /// Amount in cents, before discounts and taxes.
    pub amount: i64,
    /// Sales tax amount in cents.
    pub tax_amount: i64,
    /// Whether this charge is due to a proration.
    pub proration: bool,
    /// Associated price ID, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price_id: Option<crate::v2026_04::models::common::ids::ProductPriceId>,
    /// Start of the period covered by this line item, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub start_timestamp: Option<jiff::Timestamp>,
    /// End of the period covered by this line item, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub end_timestamp: Option<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl OrderItemSchema {
    pub fn new(required: OrderItemSchemaRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            label: required.label,
            amount: required.amount,
            tax_amount: required.tax_amount,
            proration: required.proration,
            product_price_id: required.product_price_id,
            start_timestamp: required.start_timestamp,
            end_timestamp: required.end_timestamp,
        }
    }
}
#[derive(Debug, Clone)]
pub struct OrderNotDraftRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct OrderNotDraft {
    pub error: monostate::MustBe!("OrderNotDraft"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl OrderNotDraft {
    pub fn new(required: OrderNotDraftRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OrderNotEligibleForInvoiceRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct OrderNotEligibleForInvoice {
    pub error: monostate::MustBe!("OrderNotEligibleForInvoice"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl OrderNotEligibleForInvoice {
    pub fn new(required: OrderNotEligibleForInvoiceRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OrderNotEligibleForRetryRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct OrderNotEligibleForRetry {
    pub error: monostate::MustBe!("OrderNotEligibleForRetry"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl OrderNotEligibleForRetry {
    pub fn new(required: OrderNotEligibleForRetryRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OrderProductRequired {
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2026_04::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_04::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderProduct {
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2026_04::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_04::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl OrderProduct {
    pub fn new(required: OrderProductRequired) -> Self {
        Self {
            metadata: required.metadata,
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            trial_interval: required.trial_interval,
            trial_interval_count: required.trial_interval_count,
            name: required.name,
            description: required.description,
            visibility: required.visibility,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            meter_interval: required.meter_interval,
            meter_interval_count: required.meter_interval_count,
            is_recurring: required.is_recurring,
            is_archived: required.is_archived,
            organization_id: required.organization_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct OrderReceiptRequired {
    /// The URL to the receipt PDF.
    pub url: String,
}
/// Order's receipt data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderReceipt {
    /// The URL to the receipt PDF.
    pub url: String,
}
#[allow(clippy::type_complexity)]
impl OrderReceipt {
    pub fn new(required: OrderReceiptRequired) -> Self {
        Self { url: required.url }
    }
}
#[derive(Debug, Clone)]
pub struct OrderSubscriptionRequired {
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2026_04::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2026_04::models::common::enums::SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    /// The start timestamp of the current meter period, if the product has a
    /// meter cycle set. Metered credits are granted and overage is settled on
    /// this cadence.
    pub current_meter_period_start: Option<jiff::Timestamp>,
    /// The end timestamp of the current meter period, if the product has a
    /// meter cycle set. This is when credits next renew.
    pub current_meter_period_end: Option<jiff::Timestamp>,
    /// The start timestamp of the trial period, if any.
    pub trial_start: Option<jiff::Timestamp>,
    /// The end timestamp of the trial period, if any.
    pub trial_end: Option<jiff::Timestamp>,
    /// Whether the subscription will be canceled at the end of the current
    /// period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might
    /// still be active if `cancel_at_period_end` is `true`.
    pub canceled_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription started.
    pub started_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription will end.
    pub ends_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription ended.
    pub ended_at: Option<jiff::Timestamp>,
    /// Whether the subscription will be paused at the end of the current
    /// period.
    pub pause_at_period_end: bool,
    /// The timestamp when the subscription was paused.
    pub paused_at: Option<jiff::Timestamp>,
    /// The timestamp when a paused subscription is scheduled to automatically
    /// resume, if set.
    pub resumes_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed customer.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    pub units: Option<i64>,
    pub customer_cancellation_reason:
        Option<crate::v2026_04::models::common::enums::CustomerCancellationReason>,
    pub customer_cancellation_comment: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderSubscription {
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2026_04::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2026_04::models::common::enums::SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    /// The start timestamp of the current meter period, if the product has a
    /// meter cycle set. Metered credits are granted and overage is settled on
    /// this cadence.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_start: Option<jiff::Timestamp>,
    /// The end timestamp of the current meter period, if the product has a
    /// meter cycle set. This is when credits next renew.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_end: Option<jiff::Timestamp>,
    /// The start timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_start: Option<jiff::Timestamp>,
    /// The end timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    /// Whether the subscription will be canceled at the end of the current
    /// period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might
    /// still be active if `cancel_at_period_end` is `true`.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub canceled_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription started.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub started_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription will end.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription ended.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ended_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription entered `past_due` status.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub past_due_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the subscription will be paused at the end of the current
    /// period.
    pub pause_at_period_end: bool,
    /// The timestamp when the subscription was paused.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub paused_at: Option<jiff::Timestamp>,
    /// The timestamp when a paused subscription is scheduled to automatically
    /// resume, if set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub resumes_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed customer.
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2026_04::models::common::ids::CheckoutId>,
    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_reason:
        Option<crate::v2026_04::models::common::enums::CustomerCancellationReason>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_comment: Option<String>,
}
#[allow(clippy::type_complexity)]
impl OrderSubscription {
    pub fn new(required: OrderSubscriptionRequired) -> Self {
        Self {
            metadata: required.metadata,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            status: required.status,
            current_period_start: required.current_period_start,
            current_period_end: required.current_period_end,
            current_meter_period_start: required.current_meter_period_start,
            current_meter_period_end: required.current_meter_period_end,
            trial_start: required.trial_start,
            trial_end: required.trial_end,
            cancel_at_period_end: required.cancel_at_period_end,
            canceled_at: required.canceled_at,
            started_at: required.started_at,
            ends_at: required.ends_at,
            ended_at: required.ended_at,
            past_due_at: presence_rs::Presence::Absent,
            pause_at_period_end: required.pause_at_period_end,
            paused_at: required.paused_at,
            resumes_at: required.resumes_at,
            customer_id: required.customer_id,
            product_id: required.product_id,
            discount_id: required.discount_id,
            checkout_id: required.checkout_id,
            seats: presence_rs::Presence::Absent,
            units: required.units,
            customer_cancellation_reason: required.customer_cancellation_reason,
            customer_cancellation_comment: required.customer_cancellation_comment,
        }
    }

    /// The timestamp when the subscription entered `past_due` status.
    #[allow(clippy::should_implement_trait)]
    pub fn past_due_at(mut self, value: jiff::Timestamp) -> Self {
        self.past_due_at = presence_rs::Presence::Some(value);
        self
    }

    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update an order.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrderUpdate {
    /// The name of the customer that should appear on the invoice.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_name: presence_rs::Presence<String>,
    /// The address of the customer that should appear on the invoice. Country
    /// and state fields cannot be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address:
        presence_rs::Presence<crate::v2026_04::models::common::types::AddressInput>,
}
#[allow(clippy::type_complexity)]
impl OrderUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// The name of the customer that should appear on the invoice.
    #[allow(clippy::should_implement_trait)]
    pub fn billing_name(mut self, value: impl Into<String>) -> Self {
        self.billing_name = presence_rs::Presence::Some(value.into());
        self
    }

    /// The address of the customer that should appear on the invoice. Country
    /// and state fields cannot be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationNotReadyForPaymentsRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct OrganizationNotReadyForPayments {
    pub error: monostate::MustBe!("OrganizationNotReadyForPayments"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl OrganizationNotReadyForPayments {
    pub fn new(required: OrganizationNotReadyForPaymentsRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentAlreadyInProgressRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentAlreadyInProgress {
    pub error: monostate::MustBe!("PaymentAlreadyInProgress"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentAlreadyInProgress {
    pub fn new(required: PaymentAlreadyInProgressRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
