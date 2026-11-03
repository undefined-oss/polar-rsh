#[derive(Debug, Clone)]
pub struct AmbiguousExternalMeterIdRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct AmbiguousExternalMeterId {
    pub error: monostate::MustBe!("AmbiguousExternalMeterID"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl AmbiguousExternalMeterId {
    pub fn new(required: AmbiguousExternalMeterIdRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct MeterCreateRequired {
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: crate::v2027_01::models::meters::newtypes::MeterName,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: crate::v2027_01::models::common::types::Filter,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: crate::v2027_01::models::meters::unions::MeterCreateAggregationUnion,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterCreate {
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::meters::newtypes::MeterMetadataKey,
            crate::v2027_01::models::meters::unions::MeterCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: crate::v2027_01::models::meters::newtypes::MeterName,
    /// The unit of the meter.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub unit: Option<crate::v2027_01::models::common::enums::MeterUnit>,
    /// The label for the custom unit, e.g. 'request'. Required when unit is
    /// 'custom'.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub custom_label: presence_rs::Presence<String>,
    /// The multiplier to convert from the base unit to display scale, e.g. 1000
    /// to display per 1000 units. Defaults to 1 when not provided.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub custom_multiplier:
        presence_rs::Presence<crate::v2027_01::models::meters::newtypes::MeterCustomMultiplier>,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: crate::v2027_01::models::common::types::Filter,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: crate::v2027_01::models::meters::unions::MeterCreateAggregationUnion,
    /// The ID of the organization owning the meter. **Required unless you use
    /// an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::OrganizationId>,
}
#[allow(clippy::type_complexity)]
impl MeterCreate {
    pub fn new(required: MeterCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            name: required.name,
            unit: None,
            custom_label: presence_rs::Presence::Absent,
            custom_multiplier: presence_rs::Presence::Absent,
            filter: required.filter,
            aggregation: required.aggregation,
            organization_id: presence_rs::Presence::Absent,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
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
            crate::v2027_01::models::meters::newtypes::MeterMetadataKey,
            crate::v2027_01::models::meters::unions::MeterCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The unit of the meter.
    #[allow(clippy::should_implement_trait)]
    pub fn unit(mut self, value: crate::v2027_01::models::common::enums::MeterUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// The label for the custom unit, e.g. 'request'. Required when unit is
    /// 'custom'.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_label(mut self, value: impl Into<String>) -> Self {
        self.custom_label = presence_rs::Presence::Some(value.into());
        self
    }

    /// The multiplier to convert from the base unit to display scale, e.g. 1000
    /// to display per 1000 units. Defaults to 1 when not provided.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_multiplier(
        mut self,
        value: crate::v2027_01::models::meters::newtypes::MeterCustomMultiplier,
    ) -> Self {
        self.custom_multiplier = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the organization owning the meter. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MeterQuantitiesRequired {
    pub quantities: Vec<crate::v2027_01::models::meters::types::MeterQuantity>,
    /// The total quantity for the period.
    pub total: f64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterQuantities {
    pub quantities: Vec<crate::v2027_01::models::meters::types::MeterQuantity>,
    /// The total quantity for the period.
    pub total: f64,
}
#[allow(clippy::type_complexity)]
impl MeterQuantities {
    pub fn new(required: MeterQuantitiesRequired) -> Self {
        Self { quantities: required.quantities, total: required.total }
    }
}
#[derive(Debug, Clone)]
pub struct MeterQuantityRequired {
    /// The timestamp for the current period.
    pub timestamp: jiff::Timestamp,
    /// The quantity for the current period.
    pub quantity: f64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterQuantity {
    /// The timestamp for the current period.
    pub timestamp: jiff::Timestamp,
    /// The quantity for the current period.
    pub quantity: f64,
}
#[allow(clippy::type_complexity)]
impl MeterQuantity {
    pub fn new(required: MeterQuantityRequired) -> Self {
        Self { timestamp: required.timestamp, quantity: required.quantity }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct MeterUpdate {
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::meters::newtypes::MeterMetadataKey,
            crate::v2027_01::models::meters::unions::MeterUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<crate::v2027_01::models::meters::newtypes::MeterName>,
    /// The unit of the meter.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub unit: presence_rs::Presence<crate::v2027_01::models::common::enums::MeterUnit>,
    /// The label for the custom unit. Required when unit is 'custom'.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub custom_label: presence_rs::Presence<String>,
    /// The multiplier to convert from base unit to display scale. Required when
    /// unit is 'custom'.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub custom_multiplier:
        presence_rs::Presence<crate::v2027_01::models::meters::newtypes::MeterCustomMultiplier>,
    /// The filter to apply on events that'll be used to calculate the meter.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub filter: presence_rs::Presence<crate::v2027_01::models::common::types::Filter>,
    /// The aggregation to apply on the filtered events to calculate the meter.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub aggregation: presence_rs::Presence<
        crate::v2027_01::models::meters::unions::MeterUpdateAggregationAnyOf0Union,
    >,
    /// Whether the meter is archived. Archived meters are no longer used for
    /// billing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub is_archived: presence_rs::Presence<bool>,
}
#[allow(clippy::type_complexity)]
impl MeterUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
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
            crate::v2027_01::models::meters::newtypes::MeterMetadataKey,
            crate::v2027_01::models::meters::unions::MeterUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The name of the meter. Will be shown on customer's invoices and usage.
    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: crate::v2027_01::models::meters::newtypes::MeterName) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// The unit of the meter.
    #[allow(clippy::should_implement_trait)]
    pub fn unit(mut self, value: crate::v2027_01::models::common::enums::MeterUnit) -> Self {
        self.unit = presence_rs::Presence::Some(value);
        self
    }

    /// The label for the custom unit. Required when unit is 'custom'.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_label(mut self, value: impl Into<String>) -> Self {
        self.custom_label = presence_rs::Presence::Some(value.into());
        self
    }

    /// The multiplier to convert from base unit to display scale. Required when
    /// unit is 'custom'.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_multiplier(
        mut self,
        value: crate::v2027_01::models::meters::newtypes::MeterCustomMultiplier,
    ) -> Self {
        self.custom_multiplier = presence_rs::Presence::Some(value);
        self
    }

    /// The filter to apply on events that'll be used to calculate the meter.
    #[allow(clippy::should_implement_trait)]
    pub fn filter(mut self, value: crate::v2027_01::models::common::types::Filter) -> Self {
        self.filter = presence_rs::Presence::Some(value);
        self
    }

    /// The aggregation to apply on the filtered events to calculate the meter.
    #[allow(clippy::should_implement_trait)]
    pub fn aggregation(
        mut self,
        value: crate::v2027_01::models::meters::unions::MeterUpdateAggregationAnyOf0Union,
    ) -> Self {
        self.aggregation = presence_rs::Presence::Some(value);
        self
    }

    /// Whether the meter is archived. Archived meters are no longer used for
    /// billing.
    #[allow(clippy::should_implement_trait)]
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = presence_rs::Presence::Some(value);
        self
    }
}
