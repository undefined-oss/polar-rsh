#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LicenseKeyActivateConditionsAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::license_keys::newtypes::LicenseKeyActivateConditionsValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LicenseKeyActivateMetaAdditionalPropertiesUnion {
    String(crate::v2026_10::models::license_keys::newtypes::LicenseKeyActivateMetaValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LicenseKeyActivationBaseMetaAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LicenseKeyActivationCreatedMetaAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LicenseKeyActivationReadMetaAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LicenseKeyValidateConditionsAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::license_keys::newtypes::LicenseKeyValidateConditionsValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
