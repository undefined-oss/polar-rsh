#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum OrganizationCreateLegalEntityAnyOf0Union {
    #[serde(rename = "individual")]
    Individual {},
    #[serde(rename = "company")]
    Company { registered_name: String },
}
/// Proration behavior applied when customer updates their subscription from the
/// portal.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrganizationProrationBehaviorUnion {
    SubscriptionProrationBehavior(
        crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    ),
    Auto(monostate::MustBe!("auto")),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrganizationSubscriptionSettingsProrationBehaviorUnion {
    String(
        crate::v2027_01::models::organizations::enums::OrganizationSubscriptionSettingsProrationBehaviorAnyOf0,
    ),
    Auto(monostate::MustBe!("auto")),
}
