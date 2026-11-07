#[derive(Debug, Clone)]
pub struct AddressDictRequired {
    pub country: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AddressDict {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub line1: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub line2: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub postal_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub city: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub state: Option<String>,
    pub country: String,
}
#[allow(clippy::type_complexity)]
impl AddressDict {
    pub fn new(required: AddressDictRequired) -> Self {
        Self {
            line1: None,
            line2: None,
            postal_code: None,
            city: None,
            state: None,
            country: required.country,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceCreditOrderEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BalanceCreditOrderMetadata,
}
/// An event created by Polar when an order is paid via customer balance.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceCreditOrderEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("balance.credit_order"),
    pub metadata: crate::v2027_01::models::events::types::BalanceCreditOrderMetadata,
}
#[allow(clippy::type_complexity)]
impl BalanceCreditOrderEvent {
    pub fn new(required: BalanceCreditOrderEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceCreditOrderMetadataRequired {
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
    pub tax_amount: i64,
    pub fee: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceCreditOrderMetadata {
    pub order_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub subscription_id: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub tax_amount: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_state: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_country: presence_rs::Presence<String>,
    pub fee: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub exchange_rate: Option<f64>,
}
#[allow(clippy::type_complexity)]
impl BalanceCreditOrderMetadata {
    pub fn new(required: BalanceCreditOrderMetadataRequired) -> Self {
        Self {
            order_id: required.order_id,
            product_id: None,
            subscription_id: None,
            amount: required.amount,
            currency: required.currency,
            tax_amount: required.tax_amount,
            tax_state: presence_rs::Presence::Absent,
            tax_country: presence_rs::Presence::Absent,
            fee: required.fee,
            exchange_rate: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_state(mut self, value: impl Into<String>) -> Self {
        self.tax_state = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_country(mut self, value: impl Into<String>) -> Self {
        self.tax_country = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceDisputeEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BalanceDisputeMetadata,
}
/// An event created by Polar when an order is disputed.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceDisputeEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("balance.dispute"),
    pub metadata: crate::v2027_01::models::events::types::BalanceDisputeMetadata,
}
#[allow(clippy::type_complexity)]
impl BalanceDisputeEvent {
    pub fn new(required: BalanceDisputeEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceDisputeMetadataRequired {
    pub transaction_id: String,
    pub dispute_id: String,
    pub amount: i64,
    pub currency: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    pub tax_amount: i64,
    pub fee: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceDisputeMetadata {
    pub transaction_id: String,
    pub dispute_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub order_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub order_created_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub subscription_id: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    pub tax_amount: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_state: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_country: presence_rs::Presence<String>,
    pub fee: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub exchange_rate: Option<f64>,
}
#[allow(clippy::type_complexity)]
impl BalanceDisputeMetadata {
    pub fn new(required: BalanceDisputeMetadataRequired) -> Self {
        Self {
            transaction_id: required.transaction_id,
            dispute_id: required.dispute_id,
            order_id: None,
            order_created_at: None,
            product_id: None,
            subscription_id: None,
            amount: required.amount,
            currency: required.currency,
            presentment_amount: required.presentment_amount,
            presentment_currency: required.presentment_currency,
            tax_amount: required.tax_amount,
            tax_state: presence_rs::Presence::Absent,
            tax_country: presence_rs::Presence::Absent,
            fee: required.fee,
            exchange_rate: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn order_created_at(mut self, value: impl Into<String>) -> Self {
        self.order_created_at = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_state(mut self, value: impl Into<String>) -> Self {
        self.tax_state = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_country(mut self, value: impl Into<String>) -> Self {
        self.tax_country = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceDisputeReversalEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BalanceDisputeMetadata,
}
/// An event created by Polar when a dispute is won and funds are reinstated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceDisputeReversalEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("balance.dispute_reversal"),
    pub metadata: crate::v2027_01::models::events::types::BalanceDisputeMetadata,
}
#[allow(clippy::type_complexity)]
impl BalanceDisputeReversalEvent {
    pub fn new(required: BalanceDisputeReversalEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceOrderEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BalanceOrderMetadata,
}
/// An event created by Polar when an order is paid.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceOrderEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("balance.order"),
    pub metadata: crate::v2027_01::models::events::types::BalanceOrderMetadata,
}
#[allow(clippy::type_complexity)]
impl BalanceOrderEvent {
    pub fn new(required: BalanceOrderEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceOrderMetadataRequired {
    pub transaction_id: String,
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    pub tax_amount: i64,
    pub fee: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceOrderMetadata {
    pub transaction_id: String,
    pub order_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub subscription_id: Option<String>,
    pub amount: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub net_amount: Option<i64>,
    pub currency: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    pub tax_amount: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_state: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_country: presence_rs::Presence<String>,
    pub fee: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub exchange_rate: Option<f64>,
}
#[allow(clippy::type_complexity)]
impl BalanceOrderMetadata {
    pub fn new(required: BalanceOrderMetadataRequired) -> Self {
        Self {
            transaction_id: required.transaction_id,
            order_id: required.order_id,
            product_id: None,
            subscription_id: None,
            amount: required.amount,
            net_amount: None,
            currency: required.currency,
            presentment_amount: required.presentment_amount,
            presentment_currency: required.presentment_currency,
            tax_amount: required.tax_amount,
            tax_state: presence_rs::Presence::Absent,
            tax_country: presence_rs::Presence::Absent,
            fee: required.fee,
            exchange_rate: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_amount(mut self, value: i64) -> Self {
        self.net_amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_state(mut self, value: impl Into<String>) -> Self {
        self.tax_state = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_country(mut self, value: impl Into<String>) -> Self {
        self.tax_country = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceRefundEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BalanceRefundMetadata,
}
/// An event created by Polar when an order is refunded.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceRefundEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("balance.refund"),
    pub metadata: crate::v2027_01::models::events::types::BalanceRefundMetadata,
}
#[allow(clippy::type_complexity)]
impl BalanceRefundEvent {
    pub fn new(required: BalanceRefundEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceRefundMetadataRequired {
    pub transaction_id: String,
    pub refund_id: String,
    pub amount: i64,
    pub currency: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    pub tax_amount: i64,
    pub fee: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceRefundMetadata {
    pub transaction_id: String,
    pub refund_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub order_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub order_created_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub subscription_id: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub presentment_amount: i64,
    pub presentment_currency: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub refundable_amount: Option<i64>,
    pub tax_amount: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_state: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_country: presence_rs::Presence<String>,
    pub fee: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub exchange_rate: Option<f64>,
}
#[allow(clippy::type_complexity)]
impl BalanceRefundMetadata {
    pub fn new(required: BalanceRefundMetadataRequired) -> Self {
        Self {
            transaction_id: required.transaction_id,
            refund_id: required.refund_id,
            order_id: None,
            order_created_at: None,
            product_id: None,
            subscription_id: None,
            amount: required.amount,
            currency: required.currency,
            presentment_amount: required.presentment_amount,
            presentment_currency: required.presentment_currency,
            refundable_amount: None,
            tax_amount: required.tax_amount,
            tax_state: presence_rs::Presence::Absent,
            tax_country: presence_rs::Presence::Absent,
            fee: required.fee,
            exchange_rate: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn order_created_at(mut self, value: impl Into<String>) -> Self {
        self.order_created_at = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn refundable_amount(mut self, value: i64) -> Self {
        self.refundable_amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_state(mut self, value: impl Into<String>) -> Self {
        self.tax_state = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_country(mut self, value: impl Into<String>) -> Self {
        self.tax_country = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BalanceRefundReversalEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BalanceRefundMetadata,
}
/// An event created by Polar when a refund is reverted.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BalanceRefundReversalEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("balance.refund_reversal"),
    pub metadata: crate::v2027_01::models::events::types::BalanceRefundMetadata,
}
#[allow(clippy::type_complexity)]
impl BalanceRefundReversalEvent {
    pub fn new(required: BalanceRefundReversalEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitCycledEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
/// An event created by Polar when a benefit is cycled.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCycledEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("benefit.cycled"),
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
#[allow(clippy::type_complexity)]
impl BenefitCycledEvent {
    pub fn new(required: BenefitCycledEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantMetadataRequired {
    pub benefit_id: String,
    pub benefit_grant_id: String,
    pub benefit_type: crate::v2027_01::models::common::enums::BenefitType,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantMetadata {
    pub benefit_id: String,
    pub benefit_grant_id: String,
    pub benefit_type: crate::v2027_01::models::common::enums::BenefitType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub member_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantMetadata {
    pub fn new(required: BenefitGrantMetadataRequired) -> Self {
        Self {
            benefit_id: required.benefit_id,
            benefit_grant_id: required.benefit_grant_id,
            benefit_type: required.benefit_type,
            member_id: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
/// An event created by Polar when a benefit is granted to a customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("benefit.granted"),
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantedEvent {
    pub fn new(required: BenefitGrantedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitRevokedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
/// An event created by Polar when a benefit is revoked from a customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitRevokedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("benefit.revoked"),
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
#[allow(clippy::type_complexity)]
impl BenefitRevokedEvent {
    pub fn new(required: BenefitRevokedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
/// An event created by Polar when a benefit is updated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("benefit.updated"),
    pub metadata: crate::v2027_01::models::events::types::BenefitGrantMetadata,
}
#[allow(clippy::type_complexity)]
impl BenefitUpdatedEvent {
    pub fn new(required: BenefitUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutCreatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::CheckoutCreatedMetadata,
}
/// An event created by Polar when a checkout is created.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutCreatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("checkout.created"),
    pub metadata: crate::v2027_01::models::events::types::CheckoutCreatedMetadata,
}
#[allow(clippy::type_complexity)]
impl CheckoutCreatedEvent {
    pub fn new(required: CheckoutCreatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutCreatedMetadataRequired {
    pub checkout_id: String,
    pub checkout_status: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutCreatedMetadata {
    pub checkout_id: String,
    pub checkout_status: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CheckoutCreatedMetadata {
    pub fn new(required: CheckoutCreatedMetadataRequired) -> Self {
        Self {
            checkout_id: required.checkout_id,
            checkout_status: required.checkout_status,
            product_id: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct CostMetadataInputRequired {
    /// The amount in cents.
    pub amount: crate::v2027_01::models::events::unions::CostMetadataInputAmountUnion,
    /// The currency. Currently, only `usd` is supported.
    pub currency: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CostMetadataInput {
    /// The amount in cents.
    pub amount: crate::v2027_01::models::events::unions::CostMetadataInputAmountUnion,
    /// The currency. Currently, only `usd` is supported.
    pub currency: String,
}
#[allow(clippy::type_complexity)]
impl CostMetadataInput {
    pub fn new(required: CostMetadataInputRequired) -> Self {
        Self { amount: required.amount, currency: required.currency }
    }
}
#[derive(Debug, Clone)]
pub struct CostMetadataOutputRequired {
    /// The amount in cents.
    pub amount: String,
    /// The currency. Currently, only `usd` is supported.
    pub currency: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CostMetadataOutput {
    /// The amount in cents.
    pub amount: String,
    /// The currency. Currently, only `usd` is supported.
    pub currency: String,
}
#[allow(clippy::type_complexity)]
impl CostMetadataOutput {
    pub fn new(required: CostMetadataOutputRequired) -> Self {
        Self { amount: required.amount, currency: required.currency }
    }
}
#[derive(Debug, Clone)]
pub struct CursorPaginationRequired {
    pub has_next_page: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CursorPagination {
    pub has_next_page: bool,
}
#[allow(clippy::type_complexity)]
impl CursorPagination {
    pub fn new(required: CursorPaginationRequired) -> Self {
        Self { has_next_page: required.has_next_page }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerCreatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::CustomerCreatedMetadata,
}
/// An event created by Polar when a customer is created.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerCreatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("customer.created"),
    pub metadata: crate::v2027_01::models::events::types::CustomerCreatedMetadata,
}
#[allow(clippy::type_complexity)]
impl CustomerCreatedEvent {
    pub fn new(required: CustomerCreatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerCreatedMetadataRequired {
    pub customer_id: String,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_external_id: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerCreatedMetadata {
    pub customer_id: String,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_external_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerCreatedMetadata {
    pub fn new(required: CustomerCreatedMetadataRequired) -> Self {
        Self {
            customer_id: required.customer_id,
            customer_email: required.customer_email,
            customer_name: required.customer_name,
            customer_external_id: required.customer_external_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerDeletedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::CustomerDeletedMetadata,
}
/// An event created by Polar when a customer is deleted.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerDeletedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("customer.deleted"),
    pub metadata: crate::v2027_01::models::events::types::CustomerDeletedMetadata,
}
#[allow(clippy::type_complexity)]
impl CustomerDeletedEvent {
    pub fn new(required: CustomerDeletedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerDeletedMetadataRequired {
    pub customer_id: String,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_external_id: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerDeletedMetadata {
    pub customer_id: String,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_external_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerDeletedMetadata {
    pub fn new(required: CustomerDeletedMetadataRequired) -> Self {
        Self {
            customer_id: required.customer_id,
            customer_email: required.customer_email,
            customer_name: required.customer_name,
            customer_external_id: required.customer_external_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::CustomerUpdatedMetadata,
}
/// An event created by Polar when a customer is updated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("customer.updated"),
    pub metadata: crate::v2027_01::models::events::types::CustomerUpdatedMetadata,
}
#[allow(clippy::type_complexity)]
impl CustomerUpdatedEvent {
    pub fn new(required: CustomerUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerUpdatedFields {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_name: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address: presence_rs::Presence<
        crate::v2027_01::models::events::types::AddressDict,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub metadata: presence_rs::Presence<
        std::collections::BTreeMap<
            String,
            crate::v2027_01::models::events::unions::CustomerUpdatedFieldsMetadataAnyOf0AdditionalPropertiesUnion,
        >,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomerUpdatedFields {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_name(mut self, value: impl Into<String>) -> Self {
        self.billing_name = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2027_01::models::events::types::AddressDict,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::v2027_01::models::events::unions::CustomerUpdatedFieldsMetadataAnyOf0AdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerUpdatedMetadataRequired {
    pub customer_id: String,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_external_id: Option<String>,
    pub updated_fields: crate::v2027_01::models::events::types::CustomerUpdatedFields,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerUpdatedMetadata {
    pub customer_id: String,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_external_id: Option<String>,
    pub updated_fields: crate::v2027_01::models::events::types::CustomerUpdatedFields,
}
#[allow(clippy::type_complexity)]
impl CustomerUpdatedMetadata {
    pub fn new(required: CustomerUpdatedMetadataRequired) -> Self {
        Self {
            customer_id: required.customer_id,
            customer_email: required.customer_email,
            customer_name: required.customer_name,
            customer_external_id: required.customer_external_id,
            updated_fields: required.updated_fields,
        }
    }
}
#[derive(Debug, Clone)]
pub struct EventCreateCustomerRequired {
    /// The name of the event.
    pub name: crate::v2027_01::models::events::newtypes::EventCreateCustomerName,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventCreateCustomer {
    /// The timestamp of the event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub timestamp: Option<jiff::Timestamp>,
    /// The name of the event.
    pub name: crate::v2027_01::models::events::newtypes::EventCreateCustomerName,
    /// The ID of the organization owning the event. **Required unless you use
    /// an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::OrganizationId>,
    /// Your unique identifier for this event. Useful for deduplication and
    /// parent-child relationships.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    /// The ID of the parent event. Can be either a Polar event ID (UUID) or an
    /// external event ID.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<String>,
    /// Key-value object allowing you to store additional information about the
    /// event. Some keys like `_llm` are structured data that are handled
    /// specially by Polar.
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<crate::v2027_01::models::events::types::EventMetadataInput>,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// ID of the member within the customer's organization who performed the
    /// action. Used for member-level attribution in B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
}
#[allow(clippy::type_complexity)]
impl EventCreateCustomer {
    pub fn new(required: EventCreateCustomerRequired) -> Self {
        Self {
            timestamp: None,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            external_id: presence_rs::Presence::Absent,
            parent_id: presence_rs::Presence::Absent,
            metadata: None,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp of the event.
    #[allow(clippy::should_implement_trait)]
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// The ID of the organization owning the event. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// Your unique identifier for this event. Useful for deduplication and
    /// parent-child relationships.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the parent event. Can be either a Polar event ID (UUID) or an
    /// external event ID.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information about the
    /// event. Some keys like `_llm` are structured data that are handled
    /// specially by Polar.
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
        value: crate::v2027_01::models::events::types::EventMetadataInput,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// ID of the member within the customer's organization who performed the
    /// action. Used for member-level attribution in B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct EventCreateExternalCustomerRequired {
    /// The name of the event.
    pub name: crate::v2027_01::models::events::newtypes::EventCreateExternalCustomerName,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventCreateExternalCustomer {
    /// The timestamp of the event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub timestamp: Option<jiff::Timestamp>,
    /// The name of the event.
    pub name: crate::v2027_01::models::events::newtypes::EventCreateExternalCustomerName,
    /// The ID of the organization owning the event. **Required unless you use
    /// an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::OrganizationId>,
    /// Your unique identifier for this event. Useful for deduplication and
    /// parent-child relationships.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    /// The ID of the parent event. Can be either a Polar event ID (UUID) or an
    /// external event ID.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<String>,
    /// Key-value object allowing you to store additional information about the
    /// event. Some keys like `_llm` are structured data that are handled
    /// specially by Polar.
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<crate::v2027_01::models::events::types::EventMetadataInput>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: String,
    /// ID of the member in your system within the customer's organization who
    /// performed the action. Used for member-level attribution in B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl EventCreateExternalCustomer {
    pub fn new(required: EventCreateExternalCustomerRequired) -> Self {
        Self {
            timestamp: None,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            external_id: presence_rs::Presence::Absent,
            parent_id: presence_rs::Presence::Absent,
            metadata: None,
            external_customer_id: required.external_customer_id,
            external_member_id: presence_rs::Presence::Absent,
        }
    }

    /// The timestamp of the event.
    #[allow(clippy::should_implement_trait)]
    pub fn timestamp(mut self, value: jiff::Timestamp) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// The ID of the organization owning the event. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// Your unique identifier for this event. Useful for deduplication and
    /// parent-child relationships.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the parent event. Can be either a Polar event ID (UUID) or an
    /// external event ID.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information about the
    /// event. Some keys like `_llm` are structured data that are handled
    /// specially by Polar.
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
        value: crate::v2027_01::models::events::types::EventMetadataInput,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action. Used for member-level attribution in B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[derive(Default)]
pub struct EventMetadataInput {
    #[serde(rename = "_cost")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub cost: Option<crate::v2027_01::models::events::types::CostMetadataInput>,
    #[serde(rename = "_llm")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub llm: Option<crate::v2027_01::models::events::types::LlmMetadata>,
    #[serde(flatten, default)]
    pub additional_properties: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::events::unions::EventMetadataInputAdditionalPropertiesUnion,
    >,
}
#[allow(clippy::type_complexity)]
impl EventMetadataInput {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cost(
        mut self,
        value: crate::v2027_01::models::events::types::CostMetadataInput,
    ) -> Self {
        self.cost = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn llm(mut self, value: crate::v2027_01::models::events::types::LlmMetadata) -> Self {
        self.llm = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::v2027_01::models::events::unions::EventMetadataInputAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[derive(Default)]
pub struct EventMetadataOutput {
    #[serde(rename = "_cost")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub cost: Option<crate::v2027_01::models::events::types::CostMetadataOutput>,
    #[serde(rename = "_llm")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub llm: Option<crate::v2027_01::models::events::types::LlmMetadata>,
    #[serde(flatten, default)]
    pub additional_properties: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::events::unions::EventMetadataOutputAdditionalPropertiesUnion,
    >,
}
#[allow(clippy::type_complexity)]
impl EventMetadataOutput {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cost(
        mut self,
        value: crate::v2027_01::models::events::types::CostMetadataOutput,
    ) -> Self {
        self.cost = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn llm(mut self, value: crate::v2027_01::models::events::types::LlmMetadata) -> Self {
        self.llm = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<
            String,
            crate::v2027_01::models::events::unions::EventMetadataOutputAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}
#[derive(Debug, Clone)]
pub struct EventNameRequired {
    /// The name of the event.
    pub name: String,
    /// Human readable label of the event.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: crate::v2027_01::models::common::enums::EventSource,
    /// Number of times the event has occurred.
    pub occurrences: i64,
    /// The first time the event occurred.
    pub first_seen: jiff::Timestamp,
    /// The last time the event occurred.
    pub last_seen: jiff::Timestamp,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventName {
    /// The name of the event.
    pub name: String,
    /// Human readable label of the event.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: crate::v2027_01::models::common::enums::EventSource,
    /// Number of times the event has occurred.
    pub occurrences: i64,
    /// The first time the event occurred.
    pub first_seen: jiff::Timestamp,
    /// The last time the event occurred.
    pub last_seen: jiff::Timestamp,
}
#[allow(clippy::type_complexity)]
impl EventName {
    pub fn new(required: EventNameRequired) -> Self {
        Self {
            name: required.name,
            label: required.label,
            source: required.source,
            occurrences: required.occurrences,
            first_seen: required.first_seen,
            last_seen: required.last_seen,
        }
    }
}
#[derive(Debug, Clone)]
pub struct EventsIngestRequired {
    /// List of events to ingest.
    pub events: Vec<crate::v2027_01::models::events::unions::EventsIngestEventsItemsUnion>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventsIngest {
    /// List of events to ingest.
    pub events: Vec<crate::v2027_01::models::events::unions::EventsIngestEventsItemsUnion>,
}
#[allow(clippy::type_complexity)]
impl EventsIngest {
    pub fn new(required: EventsIngestRequired) -> Self {
        Self { events: required.events }
    }
}
#[derive(Debug, Clone)]
pub struct EventsIngestResponseRequired {
    /// Number of events inserted.
    pub inserted: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventsIngestResponse {
    /// Number of events inserted.
    pub inserted: i64,
    /// Number of duplicate events skipped.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub duplicates: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl EventsIngestResponse {
    pub fn new(required: EventsIngestResponseRequired) -> Self {
        Self { inserted: required.inserted, duplicates: None }
    }

    /// Number of duplicate events skipped.
    #[allow(clippy::should_implement_trait)]
    pub fn duplicates(mut self, value: i64) -> Self {
        self.duplicates = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ListResourceWithCursorPaginationEventRequired {
    pub items: Vec<crate::v2027_01::models::events::unions::Event>,
    pub pagination: crate::v2027_01::models::events::types::CursorPagination,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ListResourceWithCursorPaginationEvent {
    pub items: Vec<crate::v2027_01::models::events::unions::Event>,
    pub pagination: crate::v2027_01::models::events::types::CursorPagination,
}
#[allow(clippy::type_complexity)]
impl ListResourceWithCursorPaginationEvent {
    pub fn new(required: ListResourceWithCursorPaginationEventRequired) -> Self {
        Self { items: required.items, pagination: required.pagination }
    }
}
#[derive(Debug, Clone)]
pub struct LlmMetadataRequired {
    /// The vendor of the event.
    pub vendor: String,
    /// The model used for the event.
    pub model: String,
    /// The number of LLM input tokens used for the event.
    pub input_tokens: crate::v2027_01::models::events::newtypes::LlmMetadataInputTokens,
    /// The number of LLM output tokens used for the event.
    pub output_tokens: crate::v2027_01::models::events::newtypes::LlmMetadataOutputTokens,
    /// The total number of LLM tokens used for the event.
    pub total_tokens: crate::v2027_01::models::events::newtypes::LlmMetadataTotalTokens,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LlmMetadata {
    /// The vendor of the event.
    pub vendor: String,
    /// The model used for the event.
    pub model: String,
    /// The LLM prompt used for the event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub prompt: presence_rs::Presence<String>,
    /// The LLM response used for the event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub response: presence_rs::Presence<String>,
    /// The number of LLM input tokens used for the event.
    pub input_tokens: crate::v2027_01::models::events::newtypes::LlmMetadataInputTokens,
    /// The number of LLM cached tokens that were used for the event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub cached_input_tokens:
        Option<crate::v2027_01::models::events::newtypes::LlmMetadataCachedInputTokens>,
    /// The number of LLM output tokens used for the event.
    pub output_tokens: crate::v2027_01::models::events::newtypes::LlmMetadataOutputTokens,
    /// The total number of LLM tokens used for the event.
    pub total_tokens: crate::v2027_01::models::events::newtypes::LlmMetadataTotalTokens,
}
#[allow(clippy::type_complexity)]
impl LlmMetadata {
    pub fn new(required: LlmMetadataRequired) -> Self {
        Self {
            vendor: required.vendor,
            model: required.model,
            prompt: presence_rs::Presence::Absent,
            response: presence_rs::Presence::Absent,
            input_tokens: required.input_tokens,
            cached_input_tokens: None,
            output_tokens: required.output_tokens,
            total_tokens: required.total_tokens,
        }
    }

    /// The LLM prompt used for the event.
    #[allow(clippy::should_implement_trait)]
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = presence_rs::Presence::Some(value.into());
        self
    }

    /// The LLM response used for the event.
    #[allow(clippy::should_implement_trait)]
    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = presence_rs::Presence::Some(value.into());
        self
    }

    /// The number of LLM cached tokens that were used for the event.
    #[allow(clippy::should_implement_trait)]
    pub fn cached_input_tokens(
        mut self,
        value: crate::v2027_01::models::events::newtypes::LlmMetadataCachedInputTokens,
    ) -> Self {
        self.cached_input_tokens = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MeterResetEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::MeterResetMetadata,
}
/// An event created by Polar when a customer meter is reset.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterResetEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("meter.reset"),
    pub metadata: crate::v2027_01::models::events::types::MeterResetMetadata,
}
#[allow(clippy::type_complexity)]
impl MeterResetEvent {
    pub fn new(required: MeterResetEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MeterResetMetadataRequired {
    pub meter_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterResetMetadata {
    pub meter_id: String,
}
#[allow(clippy::type_complexity)]
impl MeterResetMetadata {
    pub fn new(required: MeterResetMetadataRequired) -> Self {
        Self { meter_id: required.meter_id }
    }
}
#[derive(Debug, Clone)]
pub struct OrderPaidEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::OrderPaidMetadata,
}
/// An event created by Polar when an order is paid.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderPaidEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("order.paid"),
    pub metadata: crate::v2027_01::models::events::types::OrderPaidMetadata,
}
#[allow(clippy::type_complexity)]
impl OrderPaidEvent {
    pub fn new(required: OrderPaidEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderPaidMetadataRequired {
    pub order_id: String,
    pub amount: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderPaidMetadata {
    pub order_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub billing_type: Option<String>,
    pub amount: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub net_amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub tax_amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub applied_balance_amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub discount_amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub discount_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub platform_fee: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub subscription_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl OrderPaidMetadata {
    pub fn new(required: OrderPaidMetadataRequired) -> Self {
        Self {
            order_id: required.order_id,
            product_id: None,
            billing_type: None,
            amount: required.amount,
            currency: None,
            net_amount: None,
            tax_amount: None,
            applied_balance_amount: None,
            discount_amount: None,
            discount_id: None,
            platform_fee: None,
            subscription_id: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_type(mut self, value: impl Into<String>) -> Self {
        self.billing_type = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn net_amount(mut self, value: i64) -> Self {
        self.net_amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_amount(mut self, value: i64) -> Self {
        self.tax_amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn applied_balance_amount(mut self, value: i64) -> Self {
        self.applied_balance_amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn discount_amount(mut self, value: i64) -> Self {
        self.discount_amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: impl Into<String>) -> Self {
        self.discount_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn platform_fee(mut self, value: i64) -> Self {
        self.platform_fee = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderRefundedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::OrderRefundedMetadata,
}
/// An event created by Polar when an order is refunded.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderRefundedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("order.refunded"),
    pub metadata: crate::v2027_01::models::events::types::OrderRefundedMetadata,
}
#[allow(clippy::type_complexity)]
impl OrderRefundedEvent {
    pub fn new(required: OrderRefundedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderRefundedMetadataRequired {
    pub order_id: String,
    pub refunded_amount: i64,
    pub currency: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderRefundedMetadata {
    pub order_id: String,
    pub refunded_amount: i64,
    pub currency: String,
}
#[allow(clippy::type_complexity)]
impl OrderRefundedMetadata {
    pub fn new(required: OrderRefundedMetadataRequired) -> Self {
        Self {
            order_id: required.order_id,
            refunded_amount: required.refunded_amount,
            currency: required.currency,
        }
    }
}
#[derive(Debug, Clone)]
pub struct OrderUnvoidedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::OrderUnvoidedMetadata,
}
/// An event created by Polar when an order is unvoided.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderUnvoidedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("order.unvoided"),
    pub metadata: crate::v2027_01::models::events::types::OrderUnvoidedMetadata,
}
#[allow(clippy::type_complexity)]
impl OrderUnvoidedEvent {
    pub fn new(required: OrderUnvoidedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderUnvoidedMetadataRequired {
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderUnvoidedMetadata {
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
}
#[allow(clippy::type_complexity)]
impl OrderUnvoidedMetadata {
    pub fn new(required: OrderUnvoidedMetadataRequired) -> Self {
        Self { order_id: required.order_id, amount: required.amount, currency: required.currency }
    }
}
#[derive(Debug, Clone)]
pub struct OrderVoidedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::OrderVoidedMetadata,
}
/// An event created by Polar when an order is voided.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderVoidedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("order.voided"),
    pub metadata: crate::v2027_01::models::events::types::OrderVoidedMetadata,
}
#[allow(clippy::type_complexity)]
impl OrderVoidedEvent {
    pub fn new(required: OrderVoidedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrderVoidedMetadataRequired {
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrderVoidedMetadata {
    pub order_id: String,
    pub amount: i64,
    pub currency: String,
}
#[allow(clippy::type_complexity)]
impl OrderVoidedMetadata {
    pub fn new(required: OrderVoidedMetadataRequired) -> Self {
        Self { order_id: required.order_id, amount: required.amount, currency: required.currency }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionBillingPeriodUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionBillingPeriodUpdatedMetadata,
}
/// An event created by Polar when a subscription billing period is updated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionBillingPeriodUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.billing_period_updated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionBillingPeriodUpdatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionBillingPeriodUpdatedEvent {
    pub fn new(required: SubscriptionBillingPeriodUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionBillingPeriodUpdatedMetadataRequired {
    pub subscription_id: String,
    pub old_period_end: String,
    pub new_period_end: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionBillingPeriodUpdatedMetadata {
    pub subscription_id: String,
    pub old_period_end: String,
    pub new_period_end: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionBillingPeriodUpdatedMetadata {
    pub fn new(required: SubscriptionBillingPeriodUpdatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            old_period_end: required.old_period_end,
            new_period_end: required.new_period_end,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCanceledEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionCanceledMetadata,
}
/// An event created by Polar when a subscription is canceled.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCanceledEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.canceled"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionCanceledMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCanceledEvent {
    pub fn new(required: SubscriptionCanceledEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCanceledMetadataRequired {
    pub subscription_id: String,
    pub amount: i64,
    pub currency: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    pub canceled_at: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCanceledMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub customer_cancellation_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub customer_cancellation_comment: Option<String>,
    pub canceled_at: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub ends_at: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub cancel_at_period_end: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCanceledMetadata {
    pub fn new(required: SubscriptionCanceledMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            customer_cancellation_reason: None,
            customer_cancellation_comment: None,
            canceled_at: required.canceled_at,
            ends_at: None,
            cancel_at_period_end: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_cancellation_reason(mut self, value: impl Into<String>) -> Self {
        self.customer_cancellation_reason = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_cancellation_comment(mut self, value: impl Into<String>) -> Self {
        self.customer_cancellation_comment = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ends_at(mut self, value: impl Into<String>) -> Self {
        self.ends_at = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCreatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionCreatedMetadata,
}
/// An event created by Polar when a subscription is created.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCreatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.created"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionCreatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCreatedEvent {
    pub fn new(required: SubscriptionCreatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCreatedMetadataRequired {
    pub subscription_id: String,
    pub product_id: String,
    pub amount: i64,
    pub currency: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    pub started_at: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCreatedMetadata {
    pub subscription_id: String,
    pub product_id: String,
    pub amount: i64,
    pub currency: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
    pub started_at: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCreatedMetadata {
    pub fn new(required: SubscriptionCreatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: required.product_id,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            started_at: required.started_at,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCycledEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionCycledMetadata,
}
/// An event created by Polar when a subscription is cycled.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCycledEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.cycled"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionCycledMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCycledEvent {
    pub fn new(required: SubscriptionCycledEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionCycledMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionCycledMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionCycledMetadata {
    pub fn new(required: SubscriptionCycledMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionMigratedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionMigratedMetadata,
}
/// An event created by Polar when a subscription is migrated to Polar.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionMigratedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.migrated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionMigratedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionMigratedEvent {
    pub fn new(required: SubscriptionMigratedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionMigratedMetadataRequired {
    pub subscription_id: String,
    pub provider: String,
    pub provider_subscription_id: String,
    pub product_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionMigratedMetadata {
    pub subscription_id: String,
    pub provider: String,
    pub provider_subscription_id: String,
    pub product_id: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionMigratedMetadata {
    pub fn new(required: SubscriptionMigratedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            provider: required.provider,
            provider_subscription_id: required.provider_subscription_id,
            product_id: required.product_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionPastDueEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionPastDueMetadata,
}
/// An event created by Polar when a subscription becomes past due.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionPastDueEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.past_due"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionPastDueMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionPastDueEvent {
    pub fn new(required: SubscriptionPastDueEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionPastDueMetadataRequired {
    pub subscription_id: String,
    pub past_due_at: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionPastDueMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    pub past_due_at: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionPastDueMetadata {
    pub fn new(required: SubscriptionPastDueMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            past_due_at: required.past_due_at,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionPausedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionPausedMetadata,
}
/// An event created by Polar when a subscription is paused.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionPausedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.paused"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionPausedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionPausedEvent {
    pub fn new(required: SubscriptionPausedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionPausedMetadataRequired {
    pub subscription_id: String,
    pub paused_at: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionPausedMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
    pub paused_at: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub resumes_at: Option<String>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionPausedMetadata {
    pub fn new(required: SubscriptionPausedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
            paused_at: required.paused_at,
            resumes_at: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn resumes_at(mut self, value: impl Into<String>) -> Self {
        self.resumes_at = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionProductUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionProductUpdatedMetadata,
}
/// An event created by Polar when a subscription changes the product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionProductUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.product_updated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionProductUpdatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionProductUpdatedEvent {
    pub fn new(required: SubscriptionProductUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionProductUpdatedMetadataRequired {
    pub subscription_id: String,
    pub old_product_id: String,
    pub new_product_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionProductUpdatedMetadata {
    pub subscription_id: String,
    pub old_product_id: String,
    pub new_product_id: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionProductUpdatedMetadata {
    pub fn new(required: SubscriptionProductUpdatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            old_product_id: required.old_product_id,
            new_product_id: required.new_product_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionReactivatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionReactivatedMetadata,
}
/// An event created by Polar when a past due subscription is recovered.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionReactivatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.reactivated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionReactivatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionReactivatedEvent {
    pub fn new(required: SubscriptionReactivatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionReactivatedMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionReactivatedMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionReactivatedMetadata {
    pub fn new(required: SubscriptionReactivatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionReinstatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionReinstatedMetadata,
}
/// An event created by Polar when a canceled subscription is reinstated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionReinstatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.reinstated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionReinstatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionReinstatedEvent {
    pub fn new(required: SubscriptionReinstatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionReinstatedMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionReinstatedMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionReinstatedMetadata {
    pub fn new(required: SubscriptionReinstatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionResumedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionResumedMetadata,
}
/// An event created by Polar when a paused subscription is resumed.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionResumedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.resumed"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionResumedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionResumedEvent {
    pub fn new(required: SubscriptionResumedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionResumedMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionResumedMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionResumedMetadata {
    pub fn new(required: SubscriptionResumedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionRevokedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionRevokedMetadata,
}
/// An event created by Polar when a subscription is revoked from a customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionRevokedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.revoked"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionRevokedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionRevokedEvent {
    pub fn new(required: SubscriptionRevokedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionRevokedMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionRevokedMetadata {
    pub subscription_id: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub amount: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub currency: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl SubscriptionRevokedMetadata {
    pub fn new(required: SubscriptionRevokedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: None,
            amount: None,
            currency: None,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: impl Into<String>) -> Self {
        self.recurring_interval = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: i64) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionSeatsUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionSeatsUpdatedMetadata,
}
/// An event created by Polar when a the seats on a subscription is changed.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionSeatsUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.seats_updated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionSeatsUpdatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionSeatsUpdatedEvent {
    pub fn new(required: SubscriptionSeatsUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionSeatsUpdatedMetadataRequired {
    pub subscription_id: String,
    pub old_seats: i64,
    pub new_seats: i64,
    pub proration_behavior: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionSeatsUpdatedMetadata {
    pub subscription_id: String,
    pub old_seats: i64,
    pub new_seats: i64,
    pub proration_behavior: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionSeatsUpdatedMetadata {
    pub fn new(required: SubscriptionSeatsUpdatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            old_seats: required.old_seats,
            new_seats: required.new_seats,
            proration_behavior: required.proration_behavior,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUncanceledEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUncanceledMetadata,
}
/// An event created by Polar when a subscription cancellation is reversed.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUncanceledEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.uncanceled"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUncanceledMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUncanceledEvent {
    pub fn new(required: SubscriptionUncanceledEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUncanceledMetadataRequired {
    pub subscription_id: String,
    pub product_id: String,
    pub amount: i64,
    pub currency: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUncanceledMetadata {
    pub subscription_id: String,
    pub product_id: String,
    pub amount: i64,
    pub currency: String,
    pub recurring_interval: String,
    pub recurring_interval_count: i64,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUncanceledMetadata {
    pub fn new(required: SubscriptionUncanceledMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            product_id: required.product_id,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUnitsUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUnitsUpdatedMetadata,
}
/// An event created by Polar when the units on a subscription are changed.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUnitsUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.units_updated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUnitsUpdatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUnitsUpdatedEvent {
    pub fn new(required: SubscriptionUnitsUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUnitsUpdatedMetadataRequired {
    pub subscription_id: String,
    pub old_units: i64,
    pub new_units: i64,
    pub proration_behavior: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUnitsUpdatedMetadata {
    pub subscription_id: String,
    pub old_units: i64,
    pub new_units: i64,
    pub proration_behavior: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUnitsUpdatedMetadata {
    pub fn new(required: SubscriptionUnitsUpdatedMetadataRequired) -> Self {
        Self {
            subscription_id: required.subscription_id,
            old_units: required.old_units,
            new_units: required.new_units,
            proration_behavior: required.proration_behavior,
        }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdateClearedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUpdateClearedMetadata,
}
/// An event created by Polar when a pending subscription update is cleared
/// without being applied.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdateClearedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.update_cleared"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUpdateClearedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateClearedEvent {
    pub fn new(required: SubscriptionUpdateClearedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdateClearedMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdateClearedMetadata {
    pub subscription_id: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdateClearedMetadata {
    pub fn new(required: SubscriptionUpdateClearedMetadataRequired) -> Self {
        Self { subscription_id: required.subscription_id }
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdatedEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUpdatedMetadata,
}
/// An event created by Polar when a subscription is updated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdatedEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("subscription.updated"),
    pub metadata: crate::v2027_01::models::events::types::SubscriptionUpdatedMetadata,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdatedEvent {
    pub fn new(required: SubscriptionUpdatedEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptionUpdatedMetadataRequired {
    pub subscription_id: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionUpdatedMetadata {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub product_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub proration_behavior:
        Option<crate::v2027_01::models::common::enums::SubscriptionProrationBehavior>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub trial_end: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub seats: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub units: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub billing_period_end: Option<String>,
    pub subscription_id: String,
}
#[allow(clippy::type_complexity)]
impl SubscriptionUpdatedMetadata {
    pub fn new(required: SubscriptionUpdatedMetadataRequired) -> Self {
        Self {
            product_id: None,
            proration_behavior: None,
            discount_id: presence_rs::Presence::Absent,
            trial_end: None,
            seats: None,
            units: None,
            billing_period_end: None,
            subscription_id: required.subscription_id,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn proration_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    ) -> Self {
        self.proration_behavior = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: impl Into<String>) -> Self {
        self.discount_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn trial_end(mut self, value: impl Into<String>) -> Self {
        self.trial_end = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_period_end(mut self, value: impl Into<String>) -> Self {
        self.billing_period_end = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct UserEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    /// The name of the event.
    pub name: String,
    pub metadata: crate::v2027_01::models::events::types::EventMetadataOutput,
}
/// An event you created through the ingestion API.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct UserEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The name of the event.
    pub name: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("user"),
    pub metadata: crate::v2027_01::models::events::types::EventMetadataOutput,
}
#[allow(clippy::type_complexity)]
impl UserEvent {
    pub fn new(required: UserEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            name: required.name,
            source: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
