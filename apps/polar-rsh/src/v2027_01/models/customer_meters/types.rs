#[derive(Debug, Clone)]
pub struct CustomerCustomerMeterRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    pub meter: crate::v2027_01::models::customer_meters::types::CustomerCustomerMeterMeter,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerCustomerMeter {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    pub meter: crate::v2027_01::models::customer_meters::types::CustomerCustomerMeterMeter,
}
#[allow(clippy::type_complexity)]
impl CustomerCustomerMeter {
    pub fn new(required: CustomerCustomerMeterRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            customer_id: required.customer_id,
            meter_id: required.meter_id,
            consumed_units: required.consumed_units,
            credited_units: required.credited_units,
            balance: required.balance,
            meter: required.meter,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerCustomerMeterMeterRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerCustomerMeterMeter {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
}
#[allow(clippy::type_complexity)]
impl CustomerCustomerMeterMeter {
    pub fn new(required: CustomerCustomerMeterMeterRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerMeterRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomerMeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    /// The customer associated with this meter.
    pub customer: crate::v2027_01::models::common::unions::Customer,
    /// The meter associated with this customer.
    pub meter: crate::v2027_01::models::common::types::Meter,
}
/// An active customer meter, with current consumed and credited units.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerMeter {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomerMeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    /// The customer associated with this meter.
    pub customer: crate::v2027_01::models::common::unions::Customer,
    /// The meter associated with this customer.
    pub meter: crate::v2027_01::models::common::types::Meter,
}
#[allow(clippy::type_complexity)]
impl CustomerMeter {
    pub fn new(required: CustomerMeterRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            customer_id: required.customer_id,
            meter_id: required.meter_id,
            consumed_units: required.consumed_units,
            credited_units: required.credited_units,
            balance: required.balance,
            customer: required.customer,
            meter: required.meter,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerMeterCreditExternalIdConflictRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct CustomerMeterCreditExternalIdConflict {
    pub error: monostate::MustBe!("CustomerMeterCreditExternalIDConflict"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl CustomerMeterCreditExternalIdConflict {
    pub fn new(required: CustomerMeterCreditExternalIdConflictRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerMeterCreditNotReversibleRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct CustomerMeterCreditNotReversible {
    pub error: monostate::MustBe!("CustomerMeterCreditNotReversible"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl CustomerMeterCreditNotReversible {
    pub fn new(required: CustomerMeterCreditNotReversibleRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerMeterCreditsExternalIdCreateRequired {
    ///The number of units to credit.
    pub units: crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsExternalIdCreateUnits,
    ///The external ID of the customer.
    pub external_customer_id: String,
    ///The external ID of the meter, as set by config as code.
    pub external_meter_id: crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsExternalIdCreateExternalMeterId,
}
/// Schema to manually grant credits on a customer meter using external IDs.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerMeterCreditsExternalIdCreate {
    ///The number of units to credit.
    pub units: crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsExternalIdCreateUnits,
    ///Whether unused credits carry over to the next meter period.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub rollover: Option<bool>,
    ///Your unique identifier for this credit. Retrying with the same `external_id` doesn't grant the credit twice.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<
        crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsExternalIdCreateExternalId,
    >,
    ///The external ID of the customer.
    pub external_customer_id: String,
    ///The external ID of the meter, as set by config as code.
    pub external_meter_id: crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsExternalIdCreateExternalMeterId,
}
#[allow(clippy::type_complexity)]
impl CustomerMeterCreditsExternalIdCreate {
    pub fn new(required: CustomerMeterCreditsExternalIdCreateRequired) -> Self {
        Self {
            units: required.units,
            rollover: None,
            external_id: presence_rs::Presence::Absent,
            external_customer_id: required.external_customer_id,
            external_meter_id: required.external_meter_id,
        }
    }

    /// Whether unused credits carry over to the next meter period.
    #[allow(clippy::should_implement_trait)]
    pub fn rollover(mut self, value: bool) -> Self {
        self.rollover = Some(value);
        self
    }

    /// Your unique identifier for this credit. Retrying with the same
    /// `external_id` doesn't grant the credit twice.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(
        mut self,
        value: crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsExternalIdCreateExternalId,
    ) -> Self {
        self.external_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerMeterCreditsIdCreateRequired {
    /// The number of units to credit.
    pub units:
        crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsIdCreateUnits,
    /// The ID of the customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
/// Schema to manually grant credits on a customer meter using Polar IDs.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerMeterCreditsIdCreate {
    /// The number of units to credit.
    pub units:
        crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsIdCreateUnits,
    /// Whether unused credits carry over to the next meter period.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub rollover: Option<bool>,
    /// Your unique identifier for this credit. Retrying with the same
    /// `external_id` doesn't grant the credit twice.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<
        crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsIdCreateExternalId,
    >,
    /// The ID of the customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
#[allow(clippy::type_complexity)]
impl CustomerMeterCreditsIdCreate {
    pub fn new(required: CustomerMeterCreditsIdCreateRequired) -> Self {
        Self {
            units: required.units,
            rollover: None,
            external_id: presence_rs::Presence::Absent,
            customer_id: required.customer_id,
            meter_id: required.meter_id,
        }
    }

    /// Whether unused credits carry over to the next meter period.
    #[allow(clippy::should_implement_trait)]
    pub fn rollover(mut self, value: bool) -> Self {
        self.rollover = Some(value);
        self
    }

    /// Your unique identifier for this credit. Retrying with the same
    /// `external_id` doesn't grant the credit twice.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(
        mut self,
        value: crate::v2027_01::models::customer_meters::newtypes::CustomerMeterCreditsIdCreateExternalId,
    ) -> Self {
        self.external_id = presence_rs::Presence::Some(value);
        self
    }
}
