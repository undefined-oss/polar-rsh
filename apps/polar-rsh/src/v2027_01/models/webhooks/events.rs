/// An incoming webhook payload.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum WebhookPayload {
    /// Sent when a new checkout is created.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "checkout.created")]
    CheckoutCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::checkouts::types::Checkout>,
    },
    /// Sent when a checkout is updated.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "checkout.updated")]
    CheckoutUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::checkouts::types::Checkout>,
    },
    /// Sent when a checkout expires.
    ///
    /// This event fires when a checkout reaches its expiration time without
    /// being completed. Developers can use this to send reminder emails or
    /// track checkout abandonment.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "checkout.expired")]
    CheckoutExpired {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::checkouts::types::Checkout>,
    },
    /// Sent when a new customer is created.
    ///
    /// A customer can be created:
    ///
    /// * After a successful checkout.
    /// * Programmatically via the API.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "customer.created")]
    CustomerCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::unions::Customer>,
    },
    /// Sent when a customer is updated.
    ///
    /// This event is fired when the customer details are updated.
    ///
    /// If you want to be notified when a customer subscription or benefit state
    /// changes, you should listen to the `customer_state_changed` event.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "customer.updated")]
    CustomerUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::unions::Customer>,
    },
    /// Sent when a customer is deleted.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "customer.deleted")]
    CustomerDeleted {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::unions::Customer>,
    },
    /// Sent when a customer state has changed.
    ///
    /// It's triggered when:
    ///
    /// * Customer is created, updated or deleted.
    /// * A subscription is created or updated.
    /// * A benefit is granted or revoked.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "customer.state_changed")]
    CustomerStateChanged {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::customers::unions::CustomerState>,
    },
    /// Sent when a new customer seat is assigned.
    ///
    /// This event is triggered when a seat is assigned to a customer by the
    /// organization. The customer will receive an invitation email to claim
    /// the seat.
    #[serde(rename = "customer_seat.assigned")]
    CustomerSeatAssigned {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::CustomerSeat>,
    },
    /// Sent when a customer seat is claimed.
    ///
    /// This event is triggered when a customer accepts the seat invitation and
    /// claims their access.
    #[serde(rename = "customer_seat.claimed")]
    CustomerSeatClaimed {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::CustomerSeat>,
    },
    /// Sent when a customer seat is revoked.
    ///
    /// This event is triggered when access to a seat is revoked, either
    /// manually by the organization or automatically when a subscription is
    /// canceled.
    #[serde(rename = "customer_seat.revoked")]
    CustomerSeatRevoked {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::CustomerSeat>,
    },
    /// Sent when a new member is created.
    ///
    /// A member represents an individual within a customer (team).
    /// This event is triggered when a member is added to a customer,
    /// either programmatically via the API or when an owner is automatically
    /// created for a new customer.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "member.created")]
    MemberCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::Member>,
    },
    /// Sent when a member is updated.
    ///
    /// This event is triggered when member details are updated,
    /// such as their name or role within the customer.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "member.updated")]
    MemberUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::Member>,
    },
    /// Sent when a member is deleted.
    ///
    /// This event is triggered when a member is removed from a customer.
    /// Any active seats assigned to the member will be automatically revoked.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "member.deleted")]
    MemberDeleted {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::Member>,
    },
    /// Sent when a new order is created.
    ///
    /// A new order is created when:
    ///
    /// * A customer purchases a one-time product. In this case,
    ///   `billing_reason` is set to `purchase`.
    /// * A customer starts a subscription. In this case, `billing_reason` is
    ///   set to `subscription_create`.
    /// * A subscription is renewed. In this case, `billing_reason` is set to
    ///   `subscription_cycle`.
    /// * A subscription is upgraded or downgraded with an immediate proration
    ///   invoice. In this case, `billing_reason` is set to
    ///   `subscription_update`.
    ///
    /// > [!WARNING]
    /// > The order might not be paid yet, so the `status` field might be
    /// > `pending`.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "order.created")]
    OrderCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::orders::types::Order>,
    },
    /// Sent when an order is updated.
    ///
    /// An order is updated when:
    ///
    /// * Its status changes, e.g. from `pending` to `paid`.
    /// * It's refunded, partially or fully.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "order.updated")]
    OrderUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::orders::types::Order>,
    },
    /// Sent when an order is paid.
    ///
    /// When you receive this event, the order is fully processed and payment
    /// has been received.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "order.paid")]
    OrderPaid {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::orders::types::Order>,
    },
    /// Sent when an order is fully or partially refunded.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "order.refunded")]
    OrderRefunded {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::orders::types::Order>,
    },
    /// Sent when a new subscription is created.
    ///
    /// When this event occurs, the subscription `status` might not be `active`
    /// yet, as we can still have to wait for the first payment to be processed.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.created")]
    SubscriptionCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription is updated. This event fires for all changes to
    /// the subscription, including renewals.
    ///
    /// If you want more specific events, you can listen to
    /// `subscription.active`, `subscription.canceled`, `subscription.past_due`,
    /// and `subscription.revoked`.
    ///
    /// To listen specifically for renewals, listen to `subscription.cycled`.
    ///
    /// **Discord & Slack support:** On cancellation, past due, and revocation.
    /// Renewals are skipped.
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription becomes active,
    /// whether because it's a new paid subscription or because payment was
    /// recovered.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.active")]
    SubscriptionActive {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription is canceled.
    /// Customers might still have access until the end of the current period.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.canceled")]
    SubscriptionCanceled {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a customer revokes a pending cancellation.
    ///
    /// When a customer cancels with "at period end", they retain access until
    /// the subscription would renew. During this time, they can change
    /// their mind and undo the cancellation. This event is triggered when
    /// they do so.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.uncanceled")]
    SubscriptionUncanceled {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription enters a new billing period.
    ///
    /// The payload carries the new `current_period_start` and
    /// `current_period_end`. It fires when the period rolls over, before
    /// the renewal order exists and regardless of whether the renewal
    /// payment succeeds — listen to `order.paid` if you need the payment.
    ///
    /// A trial converting to a paid subscription starts a new period, so it
    /// fires there too. Read `status` to tell the two apart.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "subscription.cycled")]
    SubscriptionCycled {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription is revoked and the user loses access
    /// immediately. Happens when the subscription is canceled or payment
    /// retries are exhausted (status becomes `unpaid`).
    ///
    /// For payment failures that can still be recovered, see
    /// `subscription.past_due`.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.revoked")]
    SubscriptionRevoked {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription payment fails and the subscription enters
    /// `past_due` status.
    ///
    /// This is a recoverable state - the customer can update their payment
    /// method to restore the subscription. Benefits may be revoked
    /// depending on the organization's grace period settings.
    ///
    /// If payment retries are exhausted, a `subscription.revoked` event will be
    /// sent.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.past_due")]
    SubscriptionPastDue {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a subscription is paused and the customer temporarily loses
    /// access.
    ///
    /// No order is created while paused. The subscription resumes either on its
    /// scheduled resume date or when resumed manually, starting a new billing
    /// period.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when a paused subscription resumes, restoring the customer's
    /// access.
    ///
    /// Resuming starts a new billing period and charges the customer
    /// immediately.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
    },
    /// Sent when Polar takes over billing of a subscription migrated from
    /// another provider.
    ///
    /// This fires at cutover, once the subscription is live on Polar.
    /// `provider` and `provider_subscription_id` identify the subscription
    /// on the billing provider so you can correlate the two.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "subscription.migrated")]
    SubscriptionMigrated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::subscriptions::types::Subscription>,
        /// The billing provider the subscription was migrated from.
        provider: String,
        /// The identifier of the subscription on the billing provider.
        provider_subscription_id: String,
    },
    /// Sent when a refund is created regardless of status.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "refund.created")]
    RefundCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::refunds::types::Refund>,
    },
    /// Sent when a refund is updated.
    ///
    /// **Discord & Slack support:** Full
    #[serde(rename = "refund.updated")]
    RefundUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::refunds::types::Refund>,
    },
    /// Sent when a new product is created.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "product.created")]
    ProductCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::Product>,
    },
    /// Sent when a product is updated.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "product.updated")]
    ProductUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::types::Product>,
    },
    /// Sent when a new discount is created.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "discount.created")]
    DiscountCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::discounts::unions::Discount>,
    },
    /// Sent when a discount is updated.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "discount.updated")]
    DiscountUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::discounts::unions::Discount>,
    },
    /// Sent when a discount is deleted.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "discount.deleted")]
    DiscountDeleted {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::discounts::unions::Discount>,
    },
    /// Sent when a organization is updated.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "organization.updated")]
    OrganizationUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::organizations::types::Organization>,
    },
    /// Sent when a new benefit is created.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "benefit.created")]
    BenefitCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::unions::Benefit>,
    },
    /// Sent when a benefit is updated.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "benefit.updated")]
    BenefitUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::common::unions::Benefit>,
    },
    /// Sent when a new benefit grant is created.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "benefit_grant.created")]
    BenefitGrantCreated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::webhooks::unions::BenefitGrantWebhook>,
    },
    /// Sent when a benefit grant is updated.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "benefit_grant.updated")]
    BenefitGrantUpdated {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::webhooks::unions::BenefitGrantWebhook>,
    },
    /// Sent when a benefit grant is cycled,
    /// meaning the related subscription has been renewed for another period.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "benefit_grant.cycled")]
    BenefitGrantCycled {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::webhooks::unions::BenefitGrantWebhook>,
    },
    /// Sent when a benefit grant is revoked.
    ///
    /// **Discord & Slack support:** Basic
    #[serde(rename = "benefit_grant.revoked")]
    BenefitGrantRevoked {
        /// The ID of the event. Use it to discard duplicate deliveries.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        event_id: Option<crate::v2027_01::models::common::ids::EventId>,
        timestamp: jiff::Timestamp,
        api_version: String,
        data: Box<crate::v2027_01::models::webhooks::unions::BenefitGrantWebhook>,
    },
}
