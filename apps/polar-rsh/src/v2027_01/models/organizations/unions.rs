#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum OrganizationCreateLegalEntityAnyOf0Union {
    #[serde(rename = "individual")]
    Individual {},
    #[serde(rename = "company")]
    Company { registered_name: String },
}
