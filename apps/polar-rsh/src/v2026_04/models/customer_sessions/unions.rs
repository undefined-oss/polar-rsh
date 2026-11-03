#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerSessionsCreateRequestUnion {
    CustomerSessionCustomerIdCreate(
        crate::v2026_04::models::customer_sessions::types::CustomerSessionCustomerIdCreate,
    ),
    CustomerSessionCustomerExternalIdCreate(
        crate::v2026_04::models::customer_sessions::types::CustomerSessionCustomerExternalIdCreate,
    ),
}
