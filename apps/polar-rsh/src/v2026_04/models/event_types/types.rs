#[derive(Debug, Clone)]
pub struct EventTypeRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::EventTypeId,
    /// The name of the event type.
    pub name: String,
    /// The label for the event type.
    pub label: String,
    /// The ID of the organization owning the event type.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventType {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::EventTypeId,
    /// The name of the event type.
    pub name: String,
    /// The label for the event type.
    pub label: String,
    /// Property path to extract dynamic label from event metadata.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label_property_selector: presence_rs::Presence<String>,
    /// The ID of the organization owning the event type.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl EventType {
    pub fn new(required: EventTypeRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            label: required.label,
            label_property_selector: presence_rs::Presence::Absent,
            organization_id: required.organization_id,
        }
    }

    /// Property path to extract dynamic label from event metadata.
    #[allow(clippy::should_implement_trait)]
    pub fn label_property_selector(mut self, value: impl Into<String>) -> Self {
        self.label_property_selector = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct EventTypeUpdateRequired {
    /// The label for the event type.
    pub label: crate::v2026_04::models::event_types::newtypes::EventTypeLabel,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct EventTypeUpdate {
    /// The label for the event type.
    pub label: crate::v2026_04::models::event_types::newtypes::EventTypeLabel,
    /// Property path to extract dynamic label from event metadata (e.g.,
    /// 'subject' or 'metadata.subject').
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label_property_selector: presence_rs::Presence<
        crate::v2026_04::models::event_types::newtypes::EventTypeLabelPropertySelector,
    >,
}
#[allow(clippy::type_complexity)]
impl EventTypeUpdate {
    pub fn new(required: EventTypeUpdateRequired) -> Self {
        Self { label: required.label, label_property_selector: presence_rs::Presence::Absent }
    }

    /// Property path to extract dynamic label from event metadata (e.g.,
    /// 'subject' or 'metadata.subject').
    #[allow(clippy::should_implement_trait)]
    pub fn label_property_selector(
        mut self,
        value: crate::v2026_04::models::event_types::newtypes::EventTypeLabelPropertySelector,
    ) -> Self {
        self.label_property_selector = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct EventTypeWithStatsRequired {
    /// The name of the event type.
    pub name: String,
    /// The label for the event type.
    pub label: String,
    /// The ID of the organization owning the event type.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The source of the events (system or user).
    pub source: crate::v2026_04::models::common::enums::EventSource,
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
pub struct EventTypeWithStats {
    /// The ID of the event type. Null for system event types.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub id: presence_rs::Presence<crate::v2026_04::models::common::ids::EventTypeId>,
    /// Creation timestamp of the event type. Null for system event types.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub created_at: presence_rs::Presence<jiff::Timestamp>,
    /// Last modification timestamp of the event type. Null for system event
    /// types.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub modified_at: presence_rs::Presence<jiff::Timestamp>,
    /// The name of the event type.
    pub name: String,
    /// The label for the event type.
    pub label: String,
    /// Property path to extract dynamic label from event metadata.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label_property_selector: presence_rs::Presence<String>,
    /// The ID of the organization owning the event type.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The source of the events (system or user).
    pub source: crate::v2026_04::models::common::enums::EventSource,
    /// Number of times the event has occurred.
    pub occurrences: i64,
    /// The first time the event occurred.
    pub first_seen: jiff::Timestamp,
    /// The last time the event occurred.
    pub last_seen: jiff::Timestamp,
}
#[allow(clippy::type_complexity)]
impl EventTypeWithStats {
    pub fn new(required: EventTypeWithStatsRequired) -> Self {
        Self {
            id: presence_rs::Presence::Absent,
            created_at: presence_rs::Presence::Absent,
            modified_at: presence_rs::Presence::Absent,
            name: required.name,
            label: required.label,
            label_property_selector: presence_rs::Presence::Absent,
            organization_id: required.organization_id,
            source: required.source,
            occurrences: required.occurrences,
            first_seen: required.first_seen,
            last_seen: required.last_seen,
        }
    }

    /// The ID of the event type. Null for system event types.
    #[allow(clippy::should_implement_trait)]
    pub fn id(mut self, value: crate::v2026_04::models::common::ids::EventTypeId) -> Self {
        self.id = presence_rs::Presence::Some(value);
        self
    }

    /// Creation timestamp of the event type. Null for system event types.
    #[allow(clippy::should_implement_trait)]
    pub fn created_at(mut self, value: jiff::Timestamp) -> Self {
        self.created_at = presence_rs::Presence::Some(value);
        self
    }

    /// Last modification timestamp of the event type. Null for system event
    /// types.
    #[allow(clippy::should_implement_trait)]
    pub fn modified_at(mut self, value: jiff::Timestamp) -> Self {
        self.modified_at = presence_rs::Presence::Some(value);
        self
    }

    /// Property path to extract dynamic label from event metadata.
    #[allow(clippy::should_implement_trait)]
    pub fn label_property_selector(mut self, value: impl Into<String>) -> Self {
        self.label_property_selector = presence_rs::Presence::Some(value.into());
        self
    }
}
