#[derive(Debug, Clone)]
pub struct CustomerCustomerMeterRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::MeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2026_10::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    pub meter: crate::v2026_10::models::customer_meters::types::CustomerCustomerMeterMeter,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerCustomerMeter {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::MeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2026_10::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    pub meter: crate::v2026_10::models::customer_meters::types::CustomerCustomerMeterMeter,
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
    pub id: crate::v2026_10::models::common::ids::MeterId,
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
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::MeterId,
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
    pub id: crate::v2026_10::models::common::ids::CustomerMeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2026_10::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    /// The customer associated with this meter.
    pub customer: crate::v2026_10::models::common::unions::Customer,
    /// The meter associated with this customer.
    pub meter: crate::v2026_10::models::common::types::Meter,
}
/// An active customer meter, with current consumed and credited units.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerMeter {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CustomerMeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer.
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    /// The ID of the meter.
    pub meter_id: crate::v2026_10::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
    /// The customer associated with this meter.
    pub customer: crate::v2026_10::models::common::unions::Customer,
    /// The meter associated with this customer.
    pub meter: crate::v2026_10::models::common::types::Meter,
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
