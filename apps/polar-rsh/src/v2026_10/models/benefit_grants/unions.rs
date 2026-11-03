#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerBenefitGrant {
    CustomerBenefitGrantDiscord(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantDiscord,
    ),
    CustomerBenefitGrantGitHubRepository(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantGitHubRepository,
    ),
    CustomerBenefitGrantDownloadables(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantDownloadables,
    ),
    CustomerBenefitGrantLicenseKeys(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantLicenseKeys,
    ),
    CustomerBenefitGrantCustom(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantCustom,
    ),
    CustomerBenefitGrantMeterCredit(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantMeterCredit,
    ),
    CustomerBenefitGrantFeatureFlag(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantFeatureFlag,
    ),
    CustomerBenefitGrantSlackSharedChannel(
        crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantSlackSharedChannel,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "benefit_type", deny_unknown_fields)]
pub enum CustomerBenefitGrantUpdate {
    #[serde(rename = "discord")]
    Discord {
        properties: crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantDiscordPropertiesUpdate,
    },
    #[serde(rename = "github_repository")]
    GithubRepository {
        properties: crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantGitHubRepositoryPropertiesUpdate,
    },
    #[serde(rename = "downloadables")]
    Downloadables {},
    #[serde(rename = "license_keys")]
    LicenseKeys {},
    #[serde(rename = "custom")]
    Custom {},
    #[serde(rename = "meter_credit")]
    MeterCredit {},
    #[serde(rename = "feature_flag")]
    FeatureFlag {},
    #[serde(rename = "slack_shared_channel")]
    SlackSharedChannel {
        properties: crate::v2026_10::models::benefit_grants::types::CustomerBenefitGrantSlackSharedChannelPropertiesUpdate,
    },
}
