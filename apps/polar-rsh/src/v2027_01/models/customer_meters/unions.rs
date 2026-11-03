#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerMetersCreditsRequestUnion {
    CustomerMeterCreditsIdCreate(
        crate::v2027_01::models::customer_meters::types::CustomerMeterCreditsIdCreate,
    ),
    CustomerMeterCreditsExternalIdCreate(
        crate::v2027_01::models::customer_meters::types::CustomerMeterCreditsExternalIdCreate,
    ),
}
