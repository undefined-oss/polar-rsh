#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RefundCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_04::models::refunds::newtypes::RefundMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
