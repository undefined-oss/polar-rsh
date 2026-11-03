use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerCustomerMeterSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "modified_at")]
    ModifiedAt,
    #[serde(rename = "meter_id")]
    MeterId,
    #[serde(rename = "meter_name")]
    MeterName,
    #[serde(rename = "consumed_units")]
    ConsumedUnits,
    #[serde(rename = "credited_units")]
    CreditedUnits,
    #[serde(rename = "balance")]
    Balance,
}
impl CustomerCustomerMeterSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::ModifiedAt => "modified_at",
            Self::MeterId => "meter_id",
            Self::MeterName => "meter_name",
            Self::ConsumedUnits => "consumed_units",
            Self::CreditedUnits => "credited_units",
            Self::Balance => "balance",
        }
    }
}
impl ::std::fmt::Display for CustomerCustomerMeterSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerCustomerMeterSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CustomerMeterSortProperty {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "modified_at")]
    ModifiedAt,
    #[serde(rename = "customer_id")]
    CustomerId,
    #[serde(rename = "customer_name")]
    CustomerName,
    #[serde(rename = "meter_id")]
    MeterId,
    #[serde(rename = "meter_name")]
    MeterName,
    #[serde(rename = "consumed_units")]
    ConsumedUnits,
    #[serde(rename = "credited_units")]
    CreditedUnits,
    #[serde(rename = "balance")]
    Balance,
}
impl CustomerMeterSortProperty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CreatedAt => "created_at",
            Self::ModifiedAt => "modified_at",
            Self::CustomerId => "customer_id",
            Self::CustomerName => "customer_name",
            Self::MeterId => "meter_id",
            Self::MeterName => "meter_name",
            Self::ConsumedUnits => "consumed_units",
            Self::CreditedUnits => "credited_units",
            Self::Balance => "balance",
        }
    }
}
impl ::std::fmt::Display for CustomerMeterSortProperty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for CustomerMeterSortProperty {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
