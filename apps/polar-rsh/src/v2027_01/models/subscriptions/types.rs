#[derive(Debug, Clone)]
pub struct AlreadyCanceledSubscriptionRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct AlreadyCanceledSubscription {
    pub error: monostate::MustBe!("AlreadyCanceledSubscription"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl AlreadyCanceledSubscription {
    pub fn new(required: AlreadyCanceledSubscriptionRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerSubscriptionCancel {
    /// Cancel an active subscription once the current period ends.
    ///
    /// Or uncancel a subscription currently set to be revoked at period end.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cancel_at_period_end: presence_rs::Presence<bool>,
    /// Customers reason for cancellation.
    ///
    /// * `too_expensive`: Too expensive for the customer.
    /// * `missing_features`: Customer is missing certain features.
    /// * `switched_service`: Customer switched to another service.
    /// * `unused`: Customer is not using it enough.
    /// * `customer_service`: Customer is not satisfied with the customer
    ///   service.
    /// * `low_quality`: Customer is unhappy with the quality.
    /// * `too_complex`: Customer considers the service too complicated.
    /// * `other`: Other reason(s).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cancellation_reason:
        presence_rs::Presence<crate::v2027_01::models::common::enums::CustomerCancellationReason>,
    /// Customer feedback and why they decided to cancel.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cancellation_comment: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionCancel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Cancel an active subscription once the current period ends.
    ///
    /// Or uncancel a subscription currently set to be revoked at period end.
    #[allow(clippy::should_implement_trait)]
    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = presence_rs::Presence::Some(value);
        self
    }

    /// Customers reason for cancellation.
    ///
    /// * `too_expensive`: Too expensive for the customer.
    /// * `missing_features`: Customer is missing certain features.
    /// * `switched_service`: Customer switched to another service.
    /// * `unused`: Customer is not using it enough.
    /// * `customer_service`: Customer is not satisfied with the customer
    ///   service.
    /// * `low_quality`: Customer is unhappy with the quality.
    /// * `too_complex`: Customer considers the service too complicated.
    /// * `other`: Other reason(s).
    #[allow(clippy::should_implement_trait)]
    pub fn cancellation_reason(
        mut self,
        value: crate::v2027_01::models::common::enums::CustomerCancellationReason,
    ) -> Self {
        self.cancellation_reason = presence_rs::Presence::Some(value);
        self
    }

    /// Customer feedback and why they decided to cancel.
    #[allow(clippy::should_implement_trait)]
    pub fn cancellation_comment(mut self, value: impl Into<String>) -> Self {
        self.cancellation_comment = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionPauseRequired {
    /// Pause an active subscription at the end of the current period.
    ///
    /// Or cancel a scheduled pause on a subscription set to be paused at
    /// period end.
    pub pause_at_period_end: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionPause {
    /// Pause an active subscription at the end of the current period.
    ///
    /// Or cancel a scheduled pause on a subscription set to be paused at
    /// period end.
    pub pause_at_period_end: bool,
    /// Date at which the paused subscription should automatically resume. If
    /// not set, it stays paused until resumed. Must be after the current period
    /// end.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub resumes_at: presence_rs::Presence<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionPause {
    pub fn new(required: CustomerSubscriptionPauseRequired) -> Self {
        Self {
            pause_at_period_end: required.pause_at_period_end,
            resumes_at: presence_rs::Presence::Absent,
        }
    }

    /// Date at which the paused subscription should automatically resume. If
    /// not set, it stays paused until resumed. Must be after the current period
    /// end.
    #[allow(clippy::should_implement_trait)]
    pub fn resumes_at(mut self, value: jiff::Timestamp) -> Self {
        self.resumes_at = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerSubscriptionResume {
    /// Resume a paused subscription immediately, starting a new billing period
    /// and charging the customer.
    pub resume: monostate::MustBe!(true),
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionResume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionUpdateClearRequired {
    /// Clear the pending subscription update.
    pub pending_update: (),
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionUpdateClear {
    /// Clear the pending subscription update.
    pub pending_update: (),
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionUpdateClear {
    pub fn new(required: CustomerSubscriptionUpdateClearRequired) -> Self {
        Self { pending_update: required.pending_update }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionUpdateProductRequired {
    /// Update subscription to another product.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionUpdateProduct {
    /// Update subscription to another product.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionUpdateProduct {
    pub fn new(required: CustomerSubscriptionUpdateProductRequired) -> Self {
        Self { product_id: required.product_id }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionUpdateSeatsRequired {
    /// Update the number of seats for this subscription.
    pub seats:
        crate::v2027_01::models::subscriptions::newtypes::CustomerSubscriptionUpdateSeatsSeats,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionUpdateSeats {
    /// Update the number of seats for this subscription.
    pub seats:
        crate::v2027_01::models::subscriptions::newtypes::CustomerSubscriptionUpdateSeatsSeats,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionUpdateSeats {
    pub fn new(required: CustomerSubscriptionUpdateSeatsRequired) -> Self {
        Self { seats: required.seats }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionUpdateUnitsRequired {
    /// Update the number of units for this subscription.
    pub units:
        crate::v2027_01::models::subscriptions::newtypes::CustomerSubscriptionUpdateUnitsUnits,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionUpdateUnits {
    /// Update the number of units for this subscription.
    pub units:
        crate::v2027_01::models::subscriptions::newtypes::CustomerSubscriptionUpdateUnitsUnits,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionUpdateUnits {
    pub fn new(required: CustomerSubscriptionUpdateUnitsRequired) -> Self {
        Self { units: required.units }
    }
}
#[derive(Debug, Clone)]
pub struct InactiveSubscriptionRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct InactiveSubscription {
    pub error: monostate::MustBe!("InactiveSubscription"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl InactiveSubscription {
    pub fn new(required: InactiveSubscriptionRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PauseResumeNotAllowedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PauseResumeNotAllowed {
    pub error: monostate::MustBe!("PauseResumeNotAllowed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PauseResumeNotAllowed {
    pub fn new(required: PauseResumeNotAllowedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodRequiredRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentMethodRequired {
    pub error: monostate::MustBe!("PaymentMethodRequired"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodRequired {
    pub fn new(required: PaymentMethodRequiredRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2027_01::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2027_01::models::common::enums::SubscriptionStatus,
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
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    pub discount_id: Option<crate::v2027_01::models::common::ids::DiscountId>,
    pub checkout_id: Option<crate::v2027_01::models::common::ids::CheckoutId>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    pub units: Option<i64>,
    pub customer_cancellation_reason:
        Option<crate::v2027_01::models::common::enums::CustomerCancellationReason>,
    pub customer_cancellation_comment: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    pub customer: crate::v2027_01::models::subscriptions::types::SubscriptionCustomer,
    pub product: crate::v2027_01::models::common::types::Product,
    pub discount:
        Option<crate::v2027_01::models::subscriptions::unions::SubscriptionDiscountAnyOf0Union>,
    /// List of enabled prices for the subscription.
    pub prices: Vec<crate::v2027_01::models::subscriptions::unions::SubscriptionPricesItemsUnion>,
    /// List of meters associated with the subscription.
    pub meters: Vec<crate::v2027_01::models::subscriptions::types::SubscriptionMeter>,
    /// Pending subscription update that will be applied at the beginning of the
    /// next period. If `null`, there is no pending update.
    pub pending_update: Option<crate::v2027_01::models::common::types::PendingSubscriptionUpdate>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Subscription {
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    ///The ID of the object.
    pub id: crate::v2027_01::models::common::ids::SubscriptionId,
    ///The amount of the subscription.
    pub amount: i64,
    ///The currency of the subscription.
    pub currency: String,
    ///The interval at which the subscription recurs.
    pub recurring_interval: crate::v2027_01::models::common::enums::RecurringInterval,
    ///Number of interval units of the subscription. If this is set to 1 the charge will happen every interval (e.g. every month), if set to 2 it will be every other month, and so on.
    pub recurring_interval_count: i64,
    ///The status of the subscription.
    pub status: crate::v2027_01::models::common::enums::SubscriptionStatus,
    ///The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    ///The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    ///The start timestamp of the current meter period, if the product has a meter cycle set. Metered credits are granted and overage is settled on this cadence.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_start: Option<jiff::Timestamp>,
    ///The end timestamp of the current meter period, if the product has a meter cycle set. This is when credits next renew.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_end: Option<jiff::Timestamp>,
    ///The start timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_start: Option<jiff::Timestamp>,
    ///The end timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    ///Whether the subscription will be canceled at the end of the current period.
    pub cancel_at_period_end: bool,
    ///The timestamp when the subscription was canceled. The subscription might still be active if `cancel_at_period_end` is `true`.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub canceled_at: Option<jiff::Timestamp>,
    ///The timestamp when the subscription started.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub started_at: Option<jiff::Timestamp>,
    ///The timestamp when the subscription will end.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    ///The timestamp when the subscription ended.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ended_at: Option<jiff::Timestamp>,
    ///The timestamp when the subscription entered `past_due` status.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub past_due_at: presence_rs::Presence<jiff::Timestamp>,
    ///Whether the subscription will be paused at the end of the current period.
    pub pause_at_period_end: bool,
    ///The timestamp when the subscription was paused.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub paused_at: Option<jiff::Timestamp>,
    ///The timestamp when a paused subscription is scheduled to automatically resume, if set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub resumes_at: Option<jiff::Timestamp>,
    ///The ID of the subscribed customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    ///The ID of the subscribed product.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    ///The ID of the applied discount, if any.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2027_01::models::common::ids::DiscountId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2027_01::models::common::ids::CheckoutId>,
    ///The number of seats for seat-based subscriptions. None for non-seat subscriptions.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    ///The number of units for unit-based subscriptions. None for non-unit subscriptions.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_reason: Option<
        crate::v2027_01::models::common::enums::CustomerCancellationReason,
    >,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_comment: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2027_01::models::subscriptions::unions::SubscriptionCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    pub customer: crate::v2027_01::models::subscriptions::types::SubscriptionCustomer,
    pub product: crate::v2027_01::models::common::types::Product,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount: Option<
        crate::v2027_01::models::subscriptions::unions::SubscriptionDiscountAnyOf0Union,
    >,
    ///List of enabled prices for the subscription.
    pub prices: Vec<
        crate::v2027_01::models::subscriptions::unions::SubscriptionPricesItemsUnion,
    >,
    ///List of meters associated with the subscription.
    pub meters: Vec<crate::v2027_01::models::subscriptions::types::SubscriptionMeter>,
    ///Pending subscription update that will be applied at the beginning of the next period. If `null`, there is no pending update.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub pending_update: Option<
        crate::v2027_01::models::common::types::PendingSubscriptionUpdate,
    >,
}
#[allow(clippy::type_complexity)]
impl Subscription {
    pub fn new(required: SubscriptionRequired) -> Self {
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
            metadata: required.metadata,
            custom_field_data: None,
            customer: required.customer,
            product: required.product,
            discount: required.discount,
            prices: required.prices,
            meters: required.meters,
            pending_update: required.pending_update,
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

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2027_01::models::subscriptions::unions::SubscriptionCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCancelRequired {
    /// Cancel an active subscription once the current period ends.
    ///
    /// Or uncancel a subscription currently set to be revoked at period end.
    pub cancel_at_period_end: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCancel {
    /// Customer reason for cancellation.
    ///
    /// Helpful to monitor reasons behind churn for future improvements.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you know based on direct conversations, i.e support, with
    /// the customer.
    ///
    /// * `too_expensive`: Too expensive for the customer.
    /// * `missing_features`: Customer is missing certain features.
    /// * `switched_service`: Customer switched to another service.
    /// * `unused`: Customer is not using it enough.
    /// * `customer_service`: Customer is not satisfied with the customer
    ///   service.
    /// * `low_quality`: Customer is unhappy with the quality.
    /// * `too_complex`: Customer considers the service too complicated.
    /// * `other`: Other reason(s).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_cancellation_reason:
        presence_rs::Presence<crate::v2027_01::models::common::enums::CustomerCancellationReason>,
    /// Customer feedback and why they decided to cancel.
    ///
    /// **IMPORTANT:**
    /// Do not use this to store internal notes! It's intended to be input
    /// from the customer and is therefore also available in their Polar
    /// purchases library.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you copy a message directly from a customer
    /// conversation, i.e support.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_cancellation_comment: presence_rs::Presence<String>,
    /// Cancel an active subscription once the current period ends.
    ///
    /// Or uncancel a subscription currently set to be revoked at period end.
    pub cancel_at_period_end: bool,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCancel {
    pub fn new(required: SubscriptionCancelRequired) -> Self {
        Self {
            customer_cancellation_reason: presence_rs::Presence::Absent,
            customer_cancellation_comment: presence_rs::Presence::Absent,
            cancel_at_period_end: required.cancel_at_period_end,
        }
    }

    /// Customer reason for cancellation.
    ///
    /// Helpful to monitor reasons behind churn for future improvements.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you know based on direct conversations, i.e support, with
    /// the customer.
    ///
    /// * `too_expensive`: Too expensive for the customer.
    /// * `missing_features`: Customer is missing certain features.
    /// * `switched_service`: Customer switched to another service.
    /// * `unused`: Customer is not using it enough.
    /// * `customer_service`: Customer is not satisfied with the customer
    ///   service.
    /// * `low_quality`: Customer is unhappy with the quality.
    /// * `too_complex`: Customer considers the service too complicated.
    /// * `other`: Other reason(s).
    #[allow(clippy::should_implement_trait)]
    pub fn customer_cancellation_reason(
        mut self,
        value: crate::v2027_01::models::common::enums::CustomerCancellationReason,
    ) -> Self {
        self.customer_cancellation_reason = presence_rs::Presence::Some(value);
        self
    }

    /// Customer feedback and why they decided to cancel.
    ///
    /// **IMPORTANT:**
    /// Do not use this to store internal notes! It's intended to be input
    /// from the customer and is therefore also available in their Polar
    /// purchases library.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you copy a message directly from a customer
    /// conversation, i.e support.
    #[allow(clippy::should_implement_trait)]
    pub fn customer_cancellation_comment(mut self, value: impl Into<String>) -> Self {
        self.customer_cancellation_comment = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCreateCustomerRequired {
    /// The ID of the recurring product to subscribe to. Must be a free product,
    /// otherwise the customer should go through a checkout flow.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The ID of the customer to create the subscription for.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
}
/// Create a subscription for an existing customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCreateCustomer {
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::subscriptions::newtypes::SubscriptionCreateCustomerMetadataKey,
            crate::v2027_01::models::subscriptions::unions::SubscriptionCreateCustomerMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The ID of the recurring product to subscribe to. Must be a free product, otherwise the customer should go through a checkout flow.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    ///The currency of the subscription. The product must have a free price in this currency. If not set, the organization's default currency is used.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<
        crate::v2027_01::models::common::enums::PresentmentCurrency,
    >,
    ///The ID of the customer to create the subscription for.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCreateCustomer {
    pub fn new(required: SubscriptionCreateCustomerRequired) -> Self {
        Self {
            metadata: None,
            product_id: required.product_id,
            currency: presence_rs::Presence::Absent,
            customer_id: required.customer_id,
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
            crate::v2027_01::models::subscriptions::newtypes::SubscriptionCreateCustomerMetadataKey,
            crate::v2027_01::models::subscriptions::unions::SubscriptionCreateCustomerMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The currency of the subscription. The product must have a free price in
    /// this currency. If not set, the organization's default currency is used.
    #[allow(clippy::should_implement_trait)]
    pub fn currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.currency = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCreateExternalCustomerRequired {
    /// The ID of the recurring product to subscribe to. Must be a free product,
    /// otherwise the customer should go through a checkout flow.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The ID of the customer in your system to create the subscription for. It
    /// must already exist in Polar.
    pub external_customer_id: String,
}
/// Create a subscription for an existing customer identified by an external ID.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCreateExternalCustomer {
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::subscriptions::newtypes::SubscriptionCreateExternalCustomerMetadataKey,
            crate::v2027_01::models::subscriptions::unions::SubscriptionCreateExternalCustomerMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The ID of the recurring product to subscribe to. Must be a free product, otherwise the customer should go through a checkout flow.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    ///The currency of the subscription. The product must have a free price in this currency. If not set, the organization's default currency is used.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<
        crate::v2027_01::models::common::enums::PresentmentCurrency,
    >,
    ///The ID of the customer in your system to create the subscription for. It must already exist in Polar.
    pub external_customer_id: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCreateExternalCustomer {
    pub fn new(required: SubscriptionCreateExternalCustomerRequired) -> Self {
        Self {
            metadata: None,
            product_id: required.product_id,
            currency: presence_rs::Presence::Absent,
            external_customer_id: required.external_customer_id,
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
            crate::v2027_01::models::subscriptions::newtypes::SubscriptionCreateExternalCustomerMetadataKey,
            crate::v2027_01::models::subscriptions::unions::SubscriptionCreateExternalCustomerMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The currency of the subscription. The product must have a free price in
    /// this currency. If not set, the organization's default currency is used.
    #[allow(clippy::should_implement_trait)]
    pub fn currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.currency = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCustomerRequired {
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
pub struct SubscriptionCustomer {
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
impl SubscriptionCustomer {
    pub fn new(required: SubscriptionCustomerRequired) -> Self {
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
pub struct SubscriptionLockedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct SubscriptionLocked {
    pub error: monostate::MustBe!("SubscriptionLocked"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionLocked {
    pub fn new(required: SubscriptionLockedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionMeterRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: f64,
    /// The number of credited units so far in this billing period.
    pub credited_units: i64,
    /// The amount due in cents so far in this billing period.
    pub amount: i64,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The meter associated with this subscription.
    pub meter: crate::v2027_01::models::common::types::Meter,
}
/// Current consumption and spending for a subscription meter.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionMeter {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: f64,
    /// The number of credited units so far in this billing period.
    pub credited_units: i64,
    /// The amount due in cents so far in this billing period.
    pub amount: i64,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The meter associated with this subscription.
    pub meter: crate::v2027_01::models::common::types::Meter,
}
#[allow(clippy::type_complexity)]
impl SubscriptionMeter {
    pub fn new(required: SubscriptionMeterRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            consumed_units: required.consumed_units,
            credited_units: required.credited_units,
            amount: required.amount,
            meter_id: required.meter_id,
            meter: required.meter,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionNotScheduledToCancelRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct SubscriptionNotScheduledToCancel {
    pub error: monostate::MustBe!("SubscriptionNotScheduledToCancel"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionNotScheduledToCancel {
    pub fn new(required: SubscriptionNotScheduledToCancelRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionPauseRequired {
    /// Pause an active subscription at the end of the current period.
    ///
    /// Or cancel a scheduled pause on a subscription set to be paused at
    /// period end.
    pub pause_at_period_end: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionPause {
    /// Pause an active subscription at the end of the current period.
    ///
    /// Or cancel a scheduled pause on a subscription set to be paused at
    /// period end.
    pub pause_at_period_end: bool,
    /// Date at which the paused subscription should automatically resume.
    ///
    /// If not set, the subscription stays paused until it is resumed manually.
    /// Must be after the current period end.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub resumes_at: presence_rs::Presence<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionPause {
    pub fn new(required: SubscriptionPauseRequired) -> Self {
        Self {
            pause_at_period_end: required.pause_at_period_end,
            resumes_at: presence_rs::Presence::Absent,
        }
    }

    /// Date at which the paused subscription should automatically resume.
    ///
    /// If not set, the subscription stays paused until it is resumed manually.
    /// Must be after the current period end.
    #[allow(clippy::should_implement_trait)]
    pub fn resumes_at(mut self, value: jiff::Timestamp) -> Self {
        self.resumes_at = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct SubscriptionResume {
    /// Resume a paused subscription immediately, starting a new billing period
    /// and charging the customer.
    pub resume: monostate::MustBe!(true),
}
#[allow(clippy::type_complexity)]
impl SubscriptionResume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct SubscriptionRevoke {
    /// Customer reason for cancellation.
    ///
    /// Helpful to monitor reasons behind churn for future improvements.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you know based on direct conversations, i.e support, with
    /// the customer.
    ///
    /// * `too_expensive`: Too expensive for the customer.
    /// * `missing_features`: Customer is missing certain features.
    /// * `switched_service`: Customer switched to another service.
    /// * `unused`: Customer is not using it enough.
    /// * `customer_service`: Customer is not satisfied with the customer
    ///   service.
    /// * `low_quality`: Customer is unhappy with the quality.
    /// * `too_complex`: Customer considers the service too complicated.
    /// * `other`: Other reason(s).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_cancellation_reason:
        presence_rs::Presence<crate::v2027_01::models::common::enums::CustomerCancellationReason>,
    /// Customer feedback and why they decided to cancel.
    ///
    /// **IMPORTANT:**
    /// Do not use this to store internal notes! It's intended to be input
    /// from the customer and is therefore also available in their Polar
    /// purchases library.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you copy a message directly from a customer
    /// conversation, i.e support.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_cancellation_comment: presence_rs::Presence<String>,
    /// Cancel and revoke an active subscription immediately
    pub revoke: monostate::MustBe!(true),
}
#[allow(clippy::type_complexity)]
impl SubscriptionRevoke {
    pub fn new() -> Self {
        Self::default()
    }

    /// Customer reason for cancellation.
    ///
    /// Helpful to monitor reasons behind churn for future improvements.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you know based on direct conversations, i.e support, with
    /// the customer.
    ///
    /// * `too_expensive`: Too expensive for the customer.
    /// * `missing_features`: Customer is missing certain features.
    /// * `switched_service`: Customer switched to another service.
    /// * `unused`: Customer is not using it enough.
    /// * `customer_service`: Customer is not satisfied with the customer
    ///   service.
    /// * `low_quality`: Customer is unhappy with the quality.
    /// * `too_complex`: Customer considers the service too complicated.
    /// * `other`: Other reason(s).
    #[allow(clippy::should_implement_trait)]
    pub fn customer_cancellation_reason(
        mut self,
        value: crate::v2027_01::models::common::enums::CustomerCancellationReason,
    ) -> Self {
        self.customer_cancellation_reason = presence_rs::Presence::Some(value);
        self
    }

    /// Customer feedback and why they decided to cancel.
    ///
    /// **IMPORTANT:**
    /// Do not use this to store internal notes! It's intended to be input
    /// from the customer and is therefore also available in their Polar
    /// purchases library.
    ///
    /// Only set this in case your own service is requesting the reason from the
    /// customer. Or you copy a message directly from a customer
    /// conversation, i.e support.
    #[allow(clippy::should_implement_trait)]
    pub fn customer_cancellation_comment(mut self, value: impl Into<String>) -> Self {
        self.customer_cancellation_comment = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct SubscriptionUpdateBase {
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::subscriptions::newtypes::SubscriptionUpdateBaseMetadataKey,
            crate::v2027_01::models::subscriptions::unions::SubscriptionUpdateBaseMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///Update subscription to another product.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::ProductId,
    >,
    ///Determine how to handle the proration billing. If not provided, will use the default organization setting.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub proration_behavior: presence_rs::Presence<
        crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    >,
    ///Update the subscription to apply a new discount. If set to `null`, the discount will be removed. The change will be applied on the next billing cycle.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::DiscountId,
    >,
    ///Set or extend the trial period of the subscription. If set to `now`, the trial will end immediately and the first billing cycle will be charged synchronously. The subscription remains trialing if the payment fails.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_end: presence_rs::Presence<
        crate::v2027_01::models::subscriptions::unions::SubscriptionUpdateBaseTrialEndUnion,
    >,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateBase {
    pub fn new() -> Self {
        Self::default()
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
            crate::v2027_01::models::subscriptions::newtypes::SubscriptionUpdateBaseMetadataKey,
            crate::v2027_01::models::subscriptions::unions::SubscriptionUpdateBaseMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Update subscription to another product.
    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: crate::v2027_01::models::common::ids::ProductId) -> Self {
        self.product_id = presence_rs::Presence::Some(value);
        self
    }

    /// Determine how to handle the proration billing. If not provided, will use
    /// the default organization setting.
    #[allow(clippy::should_implement_trait)]
    pub fn proration_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    ) -> Self {
        self.proration_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// Update the subscription to apply a new discount. If set to `null`, the
    /// discount will be removed. The change will be applied on the next billing
    /// cycle.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2027_01::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Set or extend the trial period of the subscription. If set to `now`, the
    /// trial will end immediately and the first billing cycle will be charged
    /// synchronously. The subscription remains trialing if the payment fails.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_end(
        mut self,
        value: crate::v2027_01::models::subscriptions::unions::SubscriptionUpdateBaseTrialEndUnion,
    ) -> Self {
        self.trial_end = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdateBillingPeriodRequired {
    /// Set a new date for the end of the current billing period. The
    /// subscription will renew on this date. The new date can be earlier or
    /// later than the current period end, as long as it's in the future.
    ///
    /// If the subscription is set to cancel at the end of the period, it'll end
    /// on this new date instead.
    ///
    /// It is not possible to update the current billing period on a
    /// subscription that's already revoked or not active.
    pub current_billing_period_end: jiff::Timestamp,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdateBillingPeriod {
    /// Set a new date for the end of the current billing period. The
    /// subscription will renew on this date. The new date can be earlier or
    /// later than the current period end, as long as it's in the future.
    ///
    /// If the subscription is set to cancel at the end of the period, it'll end
    /// on this new date instead.
    ///
    /// It is not possible to update the current billing period on a
    /// subscription that's already revoked or not active.
    pub current_billing_period_end: jiff::Timestamp,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateBillingPeriod {
    pub fn new(required: SubscriptionUpdateBillingPeriodRequired) -> Self {
        Self { current_billing_period_end: required.current_billing_period_end }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdateClearRequired {
    /// Clear the pending subscription update. Set to null to remove scheduled
    /// changes.
    pub pending_update: (),
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdateClear {
    /// Clear the pending subscription update. Set to null to remove scheduled
    /// changes.
    pub pending_update: (),
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateClear {
    pub fn new(required: SubscriptionUpdateClearRequired) -> Self {
        Self { pending_update: required.pending_update }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdateSeatsRequired {
    /// Update the number of seats for this subscription.
    pub seats: crate::v2027_01::models::subscriptions::newtypes::SubscriptionUpdateSeatsSeats,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdateSeats {
    /// Update the number of seats for this subscription.
    pub seats: crate::v2027_01::models::subscriptions::newtypes::SubscriptionUpdateSeatsSeats,
    /// Determine how to handle the proration billing. If not provided, will use
    /// the default organization setting.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub proration_behavior: presence_rs::Presence<
        crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    >,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateSeats {
    pub fn new(required: SubscriptionUpdateSeatsRequired) -> Self {
        Self { seats: required.seats, proration_behavior: presence_rs::Presence::Absent }
    }

    /// Determine how to handle the proration billing. If not provided, will use
    /// the default organization setting.
    #[allow(clippy::should_implement_trait)]
    pub fn proration_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    ) -> Self {
        self.proration_behavior = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdateUnitsRequired {
    /// Update the number of units for this subscription.
    pub units: crate::v2027_01::models::subscriptions::newtypes::SubscriptionUpdateUnitsUnits,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdateUnits {
    /// Update the number of units for this subscription.
    pub units: crate::v2027_01::models::subscriptions::newtypes::SubscriptionUpdateUnitsUnits,
    /// Determine how to handle the proration billing. If not provided, will use
    /// the default organization setting.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub proration_behavior: presence_rs::Presence<
        crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    >,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateUnits {
    pub fn new(required: SubscriptionUpdateUnitsRequired) -> Self {
        Self { units: required.units, proration_behavior: presence_rs::Presence::Absent }
    }

    /// Determine how to handle the proration billing. If not provided, will use
    /// the default organization setting.
    #[allow(clippy::should_implement_trait)]
    pub fn proration_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    ) -> Self {
        self.proration_behavior = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionPlanNotAllowedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct UpdateSubscriptionPlanNotAllowed {
    pub error: monostate::MustBe!("UpdateSubscriptionPlanNotAllowed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl UpdateSubscriptionPlanNotAllowed {
    pub fn new(required: UpdateSubscriptionPlanNotAllowedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionSeatsNotAllowedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct UpdateSubscriptionSeatsNotAllowed {
    pub error: monostate::MustBe!("UpdateSubscriptionSeatsNotAllowed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl UpdateSubscriptionSeatsNotAllowed {
    pub fn new(required: UpdateSubscriptionSeatsNotAllowedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionUnitsNotAllowedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct UpdateSubscriptionUnitsNotAllowed {
    pub error: monostate::MustBe!("UpdateSubscriptionUnitsNotAllowed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl UpdateSubscriptionUnitsNotAllowed {
    pub fn new(required: UpdateSubscriptionUnitsNotAllowedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
