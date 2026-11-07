#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Benefit {
    #[serde(rename = "custom")]
    Custom {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitCustomProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "discord")]
    Discord {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitDiscordProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "github_repository")]
    GithubRepository {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitGitHubRepositoryProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "downloadables")]
    Downloadables {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitDownloadablesProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "license_keys")]
    LicenseKeys {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitLicenseKeysProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "meter_credit")]
    MeterCredit {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitMeterCreditProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "feature_flag")]
    FeatureFlag {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitFeatureFlagProperties,
        visibility_configurable: bool,
    },
    #[serde(rename = "slack_shared_channel")]
    SlackSharedChannel {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// An ID from your own system to reference this resource. It must be
        /// unique within the organization for this type of resource.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        external_id: Option<String>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The visibility of the benefit in the customer portal.
        visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
        /// ID of the catalog that introduced this version of the benefit. None
        /// for benefits not managed by declarative config.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
        properties: crate::v2027_01::models::common::types::BenefitSlackSharedChannelProperties,
        visibility_configurable: bool,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BenefitGrantPropertiesUnion {
    BenefitGrantDiscordProperties(
        crate::v2027_01::models::common::types::BenefitGrantDiscordProperties,
    ),
    BenefitGrantGitHubRepositoryProperties(
        crate::v2027_01::models::common::types::BenefitGrantGitHubRepositoryProperties,
    ),
    BenefitGrantDownloadablesProperties(
        crate::v2027_01::models::common::types::BenefitGrantDownloadablesProperties,
    ),
    BenefitGrantLicenseKeysProperties(
        crate::v2027_01::models::common::types::BenefitGrantLicenseKeysProperties,
    ),
    BenefitGrantCustomProperties(
        crate::v2027_01::models::common::types::BenefitGrantCustomProperties,
    ),
    BenefitGrantFeatureFlagProperties(
        crate::v2027_01::models::common::types::BenefitGrantFeatureFlagProperties,
    ),
    BenefitGrantSlackSharedChannelProperties(
        crate::v2027_01::models::common::types::BenefitGrantSlackSharedChannelProperties,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum BenefitPublic {
    #[serde(rename = "custom")]
    Custom {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "discord")]
    Discord {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "github_repository")]
    GithubRepository {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "downloadables")]
    Downloadables {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "license_keys")]
    LicenseKeys {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "feature_flag")]
    FeatureFlag {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "slack_shared_channel")]
    SlackSharedChannel {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    },
    #[serde(rename = "meter_credit")]
    MeterCredit {
        /// The ID of the benefit.
        id: crate::v2027_01::models::common::ids::BenefitId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The description of the benefit.
        description: String,
        /// Whether the benefit is selectable when creating a product.
        selectable: bool,
        /// Whether the benefit is deletable.
        deletable: bool,
        /// Whether the benefit is deleted.
        is_deleted: bool,
        /// The ID of the organization owning the benefit.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        properties: crate::v2027_01::models::common::types::BenefitMeterCreditPublicProperties,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum CustomField {
    #[serde(rename = "text")]
    Text {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the object.
        id: crate::v2027_01::models::common::ids::CustomFieldId,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// Identifier of the custom field. It'll be used as key when storing
        /// the value.
        slug: String,
        /// Name of the custom field.
        name: String,
        /// The ID of the organization owning the custom field.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        properties: crate::v2027_01::models::common::types::CustomFieldTextProperties,
    },
    #[serde(rename = "number")]
    Number {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the object.
        id: crate::v2027_01::models::common::ids::CustomFieldId,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// Identifier of the custom field. It'll be used as key when storing
        /// the value.
        slug: String,
        /// Name of the custom field.
        name: String,
        /// The ID of the organization owning the custom field.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        properties: crate::v2027_01::models::common::types::CustomFieldNumberProperties,
    },
    #[serde(rename = "date")]
    Date {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the object.
        id: crate::v2027_01::models::common::ids::CustomFieldId,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// Identifier of the custom field. It'll be used as key when storing
        /// the value.
        slug: String,
        /// Name of the custom field.
        name: String,
        /// The ID of the organization owning the custom field.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        properties: crate::v2027_01::models::common::types::CustomFieldDateProperties,
    },
    #[serde(rename = "checkbox")]
    Checkbox {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the object.
        id: crate::v2027_01::models::common::ids::CustomFieldId,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// Identifier of the custom field. It'll be used as key when storing
        /// the value.
        slug: String,
        /// Name of the custom field.
        name: String,
        /// The ID of the organization owning the custom field.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        properties: crate::v2027_01::models::common::types::CustomFieldCheckboxProperties,
    },
    #[serde(rename = "select")]
    Select {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the object.
        id: crate::v2027_01::models::common::ids::CustomFieldId,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// Identifier of the custom field. It'll be used as key when storing
        /// the value.
        slug: String,
        /// Name of the custom field.
        name: String,
        /// The ID of the organization owning the custom field.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        properties: crate::v2027_01::models::common::types::CustomFieldSelectProperties,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Customer {
    #[serde(rename = "individual")]
    Individual {
        /// The ID of the customer.
        id: crate::v2027_01::models::common::ids::CustomerId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The ID of the customer in your system. This must be unique within
        /// the organization. Once set, it can't be updated.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_id: presence_rs::Presence<String>,
        /// The email address of the customer. This must be unique within the
        /// organization.
        email: String,
        /// Whether the customer email address is verified. The address is
        /// automatically verified when the customer accesses the customer
        /// portal using their email address.
        email_verified: bool,
        /// The name of the customer.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        name: Option<String>,
        /// The name that should appear on the customer's invoices. Falls back
        /// to the customer name when not explicitly set.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_name: Option<String>,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_address: Option<crate::v2027_01::models::common::types::Address>,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        locale: presence_rs::Presence<String>,
        /// The ID of the organization owning the customer.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// The ID of the customer's default payment method, if any. Use the
        /// payment methods endpoint to retrieve its details.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        default_payment_method_id:
            presence_rs::Presence<crate::v2027_01::models::common::ids::DefaultPaymentMethodId>,
        /// Timestamp for when the customer was soft deleted.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        deleted_at: Option<jiff::Timestamp>,
        /// Timestamp of the first event ingested for this customer. Can predate
        /// `created_at`, and is null if no event was ever ingested.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        first_user_event_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        avatar_url: Option<String>,
    },
    #[serde(rename = "team")]
    Team {
        /// The ID of the customer.
        id: crate::v2027_01::models::common::ids::CustomerId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
        /// The ID of the customer in your system. This must be unique within
        /// the organization. Once set, it can't be updated.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_id: presence_rs::Presence<String>,
        /// The email address of the customer. This must be unique within the
        /// organization.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        email: presence_rs::Presence<String>,
        /// Whether the customer email address is verified. The address is
        /// automatically verified when the customer accesses the customer
        /// portal using their email address.
        email_verified: bool,
        /// The name of the customer.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        name: Option<String>,
        /// The name that should appear on the customer's invoices. Falls back
        /// to the customer name when not explicitly set.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_name: Option<String>,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_address: Option<crate::v2027_01::models::common::types::Address>,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        locale: presence_rs::Presence<String>,
        /// The ID of the organization owning the customer.
        organization_id: crate::v2027_01::models::common::ids::OrganizationId,
        /// The ID of the customer's default payment method, if any. Use the
        /// payment methods endpoint to retrieve its details.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        default_payment_method_id:
            presence_rs::Presence<crate::v2027_01::models::common::ids::DefaultPaymentMethodId>,
        /// Timestamp for when the customer was soft deleted.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        deleted_at: Option<jiff::Timestamp>,
        /// Timestamp of the first event ingested for this customer. Can predate
        /// `created_at`, and is null if no event was ever ingested.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        first_user_event_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        avatar_url: Option<String>,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum FilterClauseValueUnion {
    String(crate::v2027_01::models::common::newtypes::FilterClauseValueString),
    Integer(crate::v2027_01::models::common::newtypes::FilterClauseValueInteger),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum FilterClausesItemsUnion {
    FilterClause(crate::v2027_01::models::common::types::FilterClause),
    Filter(crate::v2027_01::models::common::types::Filter),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MetadataOutputTypeAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MetadataQueryAnyOf0AdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    StringArray(Vec<String>),
    IntegerArray(Vec<i64>),
    BooleanArray(Vec<bool>),
}
/// The aggregation to apply on the filtered events to calculate the meter.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MeterAggregationUnion {
    Count(crate::v2027_01::models::common::types::CountAggregation),
    Avg(crate::v2027_01::models::common::types::PropertyAggregation),
    Unique(crate::v2027_01::models::common::types::UniqueAggregation),
}
impl<'de> serde::Deserialize<'de> for MeterAggregationUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let tag = value
            .get("func")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                serde::de::Error::custom(concat!(
                    "missing or non-string discriminator ",
                    "func",
                    " for ",
                    "MeterAggregationUnion"
                ))
            })?
            .to_owned();
        match tag.as_str() {
            "count" => serde_json::from_value::<
                crate::v2027_01::models::common::types::CountAggregation,
            >(value)
            .map(Self::Count)
            .map_err(serde::de::Error::custom),
            "avg" | "max" | "min" | "sum" => serde_json::from_value::<
                crate::v2027_01::models::common::types::PropertyAggregation,
            >(value)
            .map(Self::Avg)
            .map_err(serde::de::Error::custom),
            "unique" => serde_json::from_value::<
                crate::v2027_01::models::common::types::UniqueAggregation,
            >(value)
            .map(Self::Unique)
            .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "unknown {} discriminator {:?} for {}",
                "func", tag, "MeterAggregationUnion"
            ))),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "amount_type", deny_unknown_fields)]
pub enum ProductPrice {
    #[serde(rename = "fixed")]
    Fixed {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the price.
        id: crate::v2027_01::models::common::ids::ProductPriceId,
        /// The source of the price . `catalog` is a predefined price, while
        /// `ad_hoc` is a price created dynamically on a Checkout session.
        source: crate::v2027_01::models::common::enums::ProductPriceSource,
        /// The currency in which the customer will be charged.
        price_currency: String,
        /// The tax behavior of the price. If null, it defaults to the
        /// organization's default tax behavior.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Whether the price is archived and no longer available.
        is_archived: bool,
        /// The ID of the product owning the price.
        product_id: crate::v2027_01::models::common::ids::ProductId,
        /// The price in cents.
        price_amount: i64,
    },
    #[serde(rename = "custom")]
    Custom {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the price.
        id: crate::v2027_01::models::common::ids::ProductPriceId,
        /// The source of the price . `catalog` is a predefined price, while
        /// `ad_hoc` is a price created dynamically on a Checkout session.
        source: crate::v2027_01::models::common::enums::ProductPriceSource,
        /// The currency in which the customer will be charged.
        price_currency: String,
        /// The tax behavior of the price. If null, it defaults to the
        /// organization's default tax behavior.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Whether the price is archived and no longer available.
        is_archived: bool,
        /// The ID of the product owning the price.
        product_id: crate::v2027_01::models::common::ids::ProductId,
        /// The minimum amount the customer can pay. If 0, the price is 'free or
        /// pay what you want'.
        minimum_amount: i64,
        /// The maximum amount the customer can pay.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        maximum_amount: Option<i64>,
        /// The initial amount shown to the customer.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        preset_amount: Option<i64>,
    },
    #[serde(rename = "seat_based")]
    SeatBased {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the price.
        id: crate::v2027_01::models::common::ids::ProductPriceId,
        /// The source of the price . `catalog` is a predefined price, while
        /// `ad_hoc` is a price created dynamically on a Checkout session.
        source: crate::v2027_01::models::common::enums::ProductPriceSource,
        /// The currency in which the customer will be charged.
        price_currency: String,
        /// The tax behavior of the price. If null, it defaults to the
        /// organization's default tax behavior.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Whether the price is archived and no longer available.
        is_archived: bool,
        /// The ID of the product owning the price.
        product_id: crate::v2027_01::models::common::ids::ProductId,
        /// Tiered pricing based on the purchased seat quantity.
        tiers: crate::v2027_01::models::common::types::Tiers,
        /// The minimum purchasable seat quantity (inclusive).
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        minimum_units: Option<i64>,
        /// The maximum purchasable seat quantity, from the last tier's bound.
        /// `null` for unlimited.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        maximum_units: Option<i64>,
    },
    #[serde(rename = "unit_based")]
    UnitBased {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the price.
        id: crate::v2027_01::models::common::ids::ProductPriceId,
        /// The source of the price . `catalog` is a predefined price, while
        /// `ad_hoc` is a price created dynamically on a Checkout session.
        source: crate::v2027_01::models::common::enums::ProductPriceSource,
        /// The currency in which the customer will be charged.
        price_currency: String,
        /// The tax behavior of the price. If null, it defaults to the
        /// organization's default tax behavior.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Whether the price is archived and no longer available.
        is_archived: bool,
        /// The ID of the product owning the price.
        product_id: crate::v2027_01::models::common::ids::ProductId,
        /// Tiered pricing based on the purchased unit quantity.
        tiers: crate::v2027_01::models::common::types::Tiers,
        /// The minimum purchasable quantity (inclusive).
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        minimum_units: Option<i64>,
        /// Per-locale unit nouns shown at checkout and on invoices. `null`
        /// defaults to "unit"/"units".
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        unit_label:
            Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>>,
        /// The maximum purchasable quantity, from the last tier's bound. `null`
        /// for unlimited.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        maximum_units: Option<i64>,
    },
    #[serde(rename = "metered_unit")]
    MeteredUnit {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the price.
        id: crate::v2027_01::models::common::ids::ProductPriceId,
        /// The source of the price . `catalog` is a predefined price, while
        /// `ad_hoc` is a price created dynamically on a Checkout session.
        source: crate::v2027_01::models::common::enums::ProductPriceSource,
        /// The currency in which the customer will be charged.
        price_currency: String,
        /// The tax behavior of the price. If null, it defaults to the
        /// organization's default tax behavior.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Whether the price is archived and no longer available.
        is_archived: bool,
        /// The ID of the product owning the price.
        product_id: crate::v2027_01::models::common::ids::ProductId,
        /// The maximum amount in cents that can be charged, regardless of the
        /// number of units consumed.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        cap_amount: Option<i64>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// The meter associated to the price.
        meter: crate::v2027_01::models::common::types::ProductPriceMeter,
        /// The price per unit in cents.
        unit_amount: String,
    },
    #[serde(rename = "metered_tiers")]
    MeteredTiers {
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        /// The ID of the price.
        id: crate::v2027_01::models::common::ids::ProductPriceId,
        /// The source of the price . `catalog` is a predefined price, while
        /// `ad_hoc` is a price created dynamically on a Checkout session.
        source: crate::v2027_01::models::common::enums::ProductPriceSource,
        /// The currency in which the customer will be charged.
        price_currency: String,
        /// The tax behavior of the price. If null, it defaults to the
        /// organization's default tax behavior.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Whether the price is archived and no longer available.
        is_archived: bool,
        /// The ID of the product owning the price.
        product_id: crate::v2027_01::models::common::ids::ProductId,
        /// The maximum amount in cents that can be charged, regardless of the
        /// number of units consumed.
        #[serde(
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
        )]
        cap_amount: Option<i64>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// The meter associated to the price.
        meter: crate::v2027_01::models::common::types::ProductPriceMeter,
        /// The pricing tiers based on consumed units.
        tiers: crate::v2027_01::models::common::types::Tiers,
    },
}
/// The price per unit in cents. Supports up to 12 decimal places.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProductPriceMeteredUnitCreateUnitAmountUnion {
    Number(crate::v2027_01::models::common::newtypes::ProductPriceMeteredUnitUnitAmountNumber),
    String(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum TierInputUnitAmountUnion {
    Number(crate::v2027_01::models::common::newtypes::TierInputUnitAmountNumber),
    String(String),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ValidationErrorLocItemsUnion {
    String(String),
    Integer(i64),
}
