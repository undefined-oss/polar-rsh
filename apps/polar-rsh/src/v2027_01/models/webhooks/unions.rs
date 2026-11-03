#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitGrantWebhook {
    BenefitGrantDiscordWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantDiscordWebhook,
    ),
    BenefitGrantCustomWebhook(crate::v2027_01::models::webhooks::types::BenefitGrantCustomWebhook),
    BenefitGrantGitHubRepositoryWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantGitHubRepositoryWebhook,
    ),
    BenefitGrantDownloadablesWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantDownloadablesWebhook,
    ),
    BenefitGrantLicenseKeysWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantLicenseKeysWebhook,
    ),
    BenefitGrantMeterCreditWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantMeterCreditWebhook,
    ),
    BenefitGrantFeatureFlagWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantFeatureFlagWebhook,
    ),
    BenefitGrantSlackSharedChannelWebhook(
        crate::v2027_01::models::webhooks::types::BenefitGrantSlackSharedChannelWebhook,
    ),
}
