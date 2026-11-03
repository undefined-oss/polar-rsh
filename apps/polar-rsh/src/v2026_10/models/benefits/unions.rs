#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum BenefitCreate {
    #[serde(rename = "custom")]
    Custom {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitCustomMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitCustomCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitCustomDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitCustomCreateProperties,
    },
    #[serde(rename = "discord")]
    Discord {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitDiscordMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitDiscordCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitDiscordDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitDiscordCreateProperties,
    },
    #[serde(rename = "github_repository")]
    GithubRepository {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitGitHubRepositoryMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitGitHubRepositoryCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitGitHubRepositoryDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitGitHubRepositoryCreateProperties,
    },
    #[serde(rename = "downloadables")]
    Downloadables {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitDownloadablesMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitDownloadablesCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitDownloadablesDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitDownloadablesCreateProperties,
    },
    #[serde(rename = "license_keys")]
    LicenseKeys {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitLicenseKeysMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitLicenseKeysCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitLicenseKeysDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitLicenseKeysCreateProperties,
    },
    #[serde(rename = "meter_credit")]
    MeterCredit {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitMeterCreditMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitMeterCreditCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitMeterCreditDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitMeterCreditCreateProperties,
    },
    #[serde(rename = "feature_flag")]
    FeatureFlag {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitFeatureFlagMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitFeatureFlagCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitFeatureFlagDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitFeatureFlagCreateProperties,
    },
    #[serde(rename = "slack_shared_channel")]
    SlackSharedChannel {
        ///Key-value object allowing you to store additional information.
        ///
        ///The key must be a string with a maximum length of **40 characters**.
        ///The value must be either:
        ///
        ///* A string with a maximum length of **500 characters**
        ///* An integer
        ///* A floating-point number
        ///* A boolean
        ///
        ///You can store up to **50 key-value pairs**.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
        )]
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2026_10::models::benefits::newtypes::BenefitSlackSharedChannelMetadataKey,
                crate::v2026_10::models::benefits::unions::BenefitSlackSharedChannelCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///The description of the benefit. Will be displayed on products having this benefit.
        description: crate::v2026_10::models::benefits::newtypes::BenefitSlackSharedChannelDescription,
        ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        ///The visibility of the benefit in the customer portal.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        visibility: presence_rs::Presence<
            crate::v2026_10::models::common::enums::BenefitVisibility,
        >,
        properties: crate::v2026_10::models::benefits::types::BenefitSlackSharedChannelCreateProperties,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitCustomCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitCustomMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitCustomUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitCustomMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitDiscordCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitDiscordMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitDiscordUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitDiscordMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitDownloadablesCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitDownloadablesMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitDownloadablesUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitDownloadablesMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitFeatureFlagCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitFeatureFlagMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitFeatureFlagUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitFeatureFlagMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitGitHubRepositoryCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitGitHubRepositoryMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitGitHubRepositoryUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitGitHubRepositoryMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitLicenseKeysCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitLicenseKeysMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitLicenseKeysUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitLicenseKeysMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitMeterCreditCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitMeterCreditMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitMeterCreditUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::benefits::newtypes::BenefitMeterCreditMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitSlackSharedChannelCreateMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::benefits::newtypes::BenefitSlackSharedChannelMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitSlackSharedChannelUpdateMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::benefits::newtypes::BenefitSlackSharedChannelMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitsUpdateRequestUnion {
    BenefitCustomUpdate(crate::v2026_10::models::benefits::types::BenefitCustomUpdate),
    BenefitDiscordUpdate(crate::v2026_10::models::benefits::types::BenefitDiscordUpdate),
    BenefitGitHubRepositoryUpdate(
        crate::v2026_10::models::benefits::types::BenefitGitHubRepositoryUpdate,
    ),
    BenefitDownloadablesUpdate(
        crate::v2026_10::models::benefits::types::BenefitDownloadablesUpdate,
    ),
    BenefitLicenseKeysUpdate(crate::v2026_10::models::benefits::types::BenefitLicenseKeysUpdate),
    BenefitMeterCreditUpdate(crate::v2026_10::models::benefits::types::BenefitMeterCreditUpdate),
    BenefitFeatureFlagUpdate(crate::v2026_10::models::benefits::types::BenefitFeatureFlagUpdate),
    BenefitSlackSharedChannelUpdate(
        crate::v2026_10::models::benefits::types::BenefitSlackSharedChannelUpdate,
    ),
}
