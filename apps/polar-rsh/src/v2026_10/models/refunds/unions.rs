#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RefundCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::refunds::newtypes::RefundMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
