#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitGrantWebhook {
    BenefitGrantDiscordWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantDiscordWebhook,
    ),
    BenefitGrantCustomWebhook(crate::v2026_04::models::webhooks::types::BenefitGrantCustomWebhook),
    BenefitGrantGitHubRepositoryWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantGitHubRepositoryWebhook,
    ),
    BenefitGrantDownloadablesWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantDownloadablesWebhook,
    ),
    BenefitGrantLicenseKeysWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantLicenseKeysWebhook,
    ),
    BenefitGrantMeterCreditWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantMeterCreditWebhook,
    ),
    BenefitGrantFeatureFlagWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantFeatureFlagWebhook,
    ),
    BenefitGrantSlackSharedChannelWebhook(
        crate::v2026_04::models::webhooks::types::BenefitGrantSlackSharedChannelWebhook,
    ),
}
