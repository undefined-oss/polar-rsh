/// The amount in cents.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CostMetadataInputAmountUnion {
    Number(f64),
    String(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerUpdatedFieldsMetadataAnyOf0AdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Event {
    System(crate::v2026_10::models::events::unions::SystemEvent),
    User(crate::v2026_10::models::events::types::UserEvent),
}
impl<'de> serde::Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let tag = value
            .get("source")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                serde::de::Error::custom(concat!(
                    "missing or non-string discriminator ",
                    "source",
                    " for ",
                    "Event"
                ))
            })?
            .to_owned();
        match tag.as_str() {
            "system" => serde_json::from_value::<
                crate::v2026_10::models::events::unions::SystemEvent,
            >(value)
            .map(Self::System)
            .map_err(serde::de::Error::custom),
            "user" => {
                serde_json::from_value::<crate::v2026_10::models::events::types::UserEvent>(value)
                    .map(Self::User)
                    .map_err(serde::de::Error::custom)
            }
            _ => Err(serde::de::Error::custom(format!(
                "unknown {} discriminator {:?} for {}",
                "source", tag, "Event"
            ))),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventMetadataInputAdditionalPropertiesUnion {
    String(crate::v2026_10::models::events::newtypes::EventMetadataInputValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventMetadataOutputAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventsIngestEventsItemsUnion {
    EventCreateCustomer(crate::v2026_10::models::events::types::EventCreateCustomer),
    EventCreateExternalCustomer(
        crate::v2026_10::models::events::types::EventCreateExternalCustomer,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EventsListResponse200Union {
    ListResourceEvent(
        crate::v2026_10::models::pagination::ListResource<
            crate::v2026_10::models::events::unions::Event,
        >,
    ),
    ListResourceWithCursorPaginationEvent(
        crate::v2026_10::models::events::types::ListResourceWithCursorPaginationEvent,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", deny_unknown_fields)]
pub enum SystemEvent {
    #[serde(rename = "meter.credited")]
    MeterCredited {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::MeterCreditedMetadata,
    },
    #[serde(rename = "meter.reset")]
    MeterReset {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::MeterResetMetadata,
    },
    #[serde(rename = "benefit.granted")]
    BenefitGranted {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BenefitGrantMetadata,
    },
    #[serde(rename = "benefit.cycled")]
    BenefitCycled {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BenefitGrantMetadata,
    },
    #[serde(rename = "benefit.updated")]
    BenefitUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BenefitGrantMetadata,
    },
    #[serde(rename = "benefit.revoked")]
    BenefitRevoked {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BenefitGrantMetadata,
    },
    #[serde(rename = "subscription.created")]
    SubscriptionCreated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionCreatedMetadata,
    },
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionUpdatedMetadata,
    },
    #[serde(rename = "subscription.cycled")]
    SubscriptionCycled {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionCycledMetadata,
    },
    #[serde(rename = "subscription.canceled")]
    SubscriptionCanceled {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionCanceledMetadata,
    },
    #[serde(rename = "subscription.revoked")]
    SubscriptionRevoked {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionRevokedMetadata,
    },
    #[serde(rename = "subscription.past_due")]
    SubscriptionPastDue {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionPastDueMetadata,
    },
    #[serde(rename = "subscription.reactivated")]
    SubscriptionReactivated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionReactivatedMetadata,
    },
    #[serde(rename = "subscription.reinstated")]
    SubscriptionReinstated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionReinstatedMetadata,
    },
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionPausedMetadata,
    },
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionResumedMetadata,
    },
    #[serde(rename = "subscription.migrated")]
    SubscriptionMigrated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionMigratedMetadata,
    },
    #[serde(rename = "subscription.uncanceled")]
    SubscriptionUncanceled {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionUncanceledMetadata,
    },
    #[serde(rename = "subscription.product_updated")]
    SubscriptionProductUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionProductUpdatedMetadata,
    },
    #[serde(rename = "subscription.seats_updated")]
    SubscriptionSeatsUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionSeatsUpdatedMetadata,
    },
    #[serde(rename = "subscription.units_updated")]
    SubscriptionUnitsUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionUnitsUpdatedMetadata,
    },
    #[serde(rename = "subscription.billing_period_updated")]
    SubscriptionBillingPeriodUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionBillingPeriodUpdatedMetadata,
    },
    #[serde(rename = "subscription.update_cleared")]
    SubscriptionUpdateCleared {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::SubscriptionUpdateClearedMetadata,
    },
    #[serde(rename = "order.paid")]
    OrderPaid {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::OrderPaidMetadata,
    },
    #[serde(rename = "order.refunded")]
    OrderRefunded {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::OrderRefundedMetadata,
    },
    #[serde(rename = "order.voided")]
    OrderVoided {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::OrderVoidedMetadata,
    },
    #[serde(rename = "order.unvoided")]
    OrderUnvoided {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::OrderUnvoidedMetadata,
    },
    #[serde(rename = "checkout.created")]
    CheckoutCreated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::CheckoutCreatedMetadata,
    },
    #[serde(rename = "customer.created")]
    CustomerCreated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::CustomerCreatedMetadata,
    },
    #[serde(rename = "customer.updated")]
    CustomerUpdated {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::CustomerUpdatedMetadata,
    },
    #[serde(rename = "customer.deleted")]
    CustomerDeleted {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::CustomerDeletedMetadata,
    },
    #[serde(rename = "balance.order")]
    BalanceOrder {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BalanceOrderMetadata,
    },
    #[serde(rename = "balance.credit_order")]
    BalanceCreditOrder {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BalanceCreditOrderMetadata,
    },
    #[serde(rename = "balance.refund")]
    BalanceRefund {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BalanceRefundMetadata,
    },
    #[serde(rename = "balance.refund_reversal")]
    BalanceRefundReversal {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BalanceRefundMetadata,
    },
    #[serde(rename = "balance.dispute")]
    BalanceDispute {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BalanceDisputeMetadata,
    },
    #[serde(rename = "balance.dispute_reversal")]
    BalanceDisputeReversal {
        /// The ID of the object.
        id: crate::v2026_10::models::common::ids::EventId,
        /// The timestamp of the event.
        timestamp: jiff::Timestamp,
        /// The ID of the organization owning the event.
        organization_id: crate::v2026_10::models::common::ids::OrganizationId,
        /// ID of the customer in your Polar organization associated with the
        /// event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
        /// The customer associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        customer: Option<crate::v2026_10::models::common::unions::Customer>,
        /// ID of the customer in your system associated with the event.
        #[serde(
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_customer_id: Option<String>,
        /// ID of the member within the customer's organization who performed
        /// the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        member_id: presence_rs::Presence<crate::v2026_10::models::common::ids::MemberId>,
        /// ID of the member in your system within the customer's organization
        /// who performed the action inside B2B.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_member_id: presence_rs::Presence<String>,
        /// Number of direct child events linked to this event.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        child_count: Option<i64>,
        /// The ID of the parent event.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        parent_id: presence_rs::Presence<crate::v2026_10::models::common::ids::EventId>,
        /// Human readable label of the event type.
        label: String,
        /// The source of the event. `system` events are created by Polar.
        /// `user` events are the one you create through our ingestion API.
        source: monostate::MustBe!("system"),
        metadata: crate::v2026_10::models::events::types::BalanceDisputeMetadata,
    },
}
