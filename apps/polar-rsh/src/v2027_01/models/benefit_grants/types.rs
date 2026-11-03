#[derive(Debug, Clone)]
pub struct BenefitCustomSubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitCustomSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCustomSubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("custom"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitCustomSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitCustomSubscriber {
    pub fn new(required: BenefitCustomSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitCustomSubscriberPropertiesRequired {
    pub note: Option<String>,
}
/// Properties available to subscribers for a benefit of type `custom`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCustomSubscriberProperties {
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub note: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitCustomSubscriberProperties {
    pub fn new(required: BenefitCustomSubscriberPropertiesRequired) -> Self {
        Self { note: required.note }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordSubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitDiscordSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscordSubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("discord"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitDiscordSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordSubscriber {
    pub fn new(required: BenefitDiscordSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordSubscriberPropertiesRequired {
    /// The ID of the Discord server.
    pub guild_id: String,
}
/// Properties available to subscribers for a benefit of type `discord`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscordSubscriberProperties {
    /// The ID of the Discord server.
    pub guild_id: String,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordSubscriberProperties {
    pub fn new(required: BenefitDiscordSubscriberPropertiesRequired) -> Self {
        Self { guild_id: required.guild_id }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesSubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitDownloadablesSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadablesSubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("downloadables"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitDownloadablesSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesSubscriber {
    pub fn new(required: BenefitDownloadablesSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesSubscriberPropertiesRequired {
    pub active_files: Vec<
        crate::v2027_01::models::common::ids::BenefitDownloadablesSubscriberPropertiesActiveFilesId,
    >,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadablesSubscriberProperties {
    pub active_files: Vec<
        crate::v2027_01::models::common::ids::BenefitDownloadablesSubscriberPropertiesActiveFilesId,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesSubscriberProperties {
    pub fn new(required: BenefitDownloadablesSubscriberPropertiesRequired) -> Self {
        Self { active_files: required.active_files }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitFeatureFlagSubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitFeatureFlagSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitFeatureFlagSubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("feature_flag"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitFeatureFlagSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagSubscriber {
    pub fn new(required: BenefitFeatureFlagSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
/// Properties available to subscribers for a benefit of type `feature_flag`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitFeatureFlagSubscriberProperties {}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagSubscriberProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositorySubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitGitHubRepositorySubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepositorySubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("github_repository"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitGitHubRepositorySubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositorySubscriber {
    pub fn new(required: BenefitGitHubRepositorySubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositorySubscriberPropertiesRequired {
    /// The owner of the repository.
    pub repository_owner: String,
    /// The name of the repository.
    pub repository_name: String,
}
/// Properties available to subscribers for a benefit of type
/// `github_repository`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepositorySubscriberProperties {
    /// The owner of the repository.
    pub repository_owner: String,
    /// The name of the repository.
    pub repository_name: String,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositorySubscriberProperties {
    pub fn new(required: BenefitGitHubRepositorySubscriberPropertiesRequired) -> Self {
        Self {
            repository_owner: required.repository_owner,
            repository_name: required.repository_name,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantMeterCreditProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub last_credited_meter_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub last_credited_units: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub last_credited_at: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantMeterCreditProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn last_credited_meter_id(mut self, value: impl Into<String>) -> Self {
        self.last_credited_meter_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn last_credited_units(mut self, value: i64) -> Self {
        self.last_credited_units = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn last_credited_at(mut self, value: impl Into<String>) -> Self {
        self.last_credited_at = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeysSubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitLicenseKeysSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeysSubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("license_keys"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitLicenseKeysSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeysSubscriber {
    pub fn new(required: BenefitLicenseKeysSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeysSubscriberPropertiesRequired {
    pub prefix: Option<String>,
    pub expires:
        Option<crate::v2027_01::models::common::types::BenefitLicenseKeyExpirationProperties>,
    pub activations:
        Option<crate::v2027_01::models::common::types::BenefitLicenseKeyActivationProperties>,
    pub limit_usage: Option<i64>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeysSubscriberProperties {
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub prefix: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub expires:
        Option<crate::v2027_01::models::common::types::BenefitLicenseKeyExpirationProperties>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub activations:
        Option<crate::v2027_01::models::common::types::BenefitLicenseKeyActivationProperties>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_usage: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeysSubscriberProperties {
    pub fn new(required: BenefitLicenseKeysSubscriberPropertiesRequired) -> Self {
        Self {
            prefix: required.prefix,
            expires: required.expires,
            activations: required.activations,
            limit_usage: required.limit_usage,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditSubscriberRequired {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitMeterCreditSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditSubscriber {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("meter_credit"),
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// Whether the benefit is deleted.
    pub is_deleted: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitMeterCreditSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditSubscriber {
    pub fn new(required: BenefitMeterCreditSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditSubscriberPropertiesRequired {
    pub units: i64,
    pub rollover: bool,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
/// Properties available to subscribers for a benefit of type `meter_unit`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditSubscriberProperties {
    pub units: i64,
    pub rollover: bool,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditSubscriberProperties {
    pub fn new(required: BenefitMeterCreditSubscriberPropertiesRequired) -> Self {
        Self { units: required.units, rollover: required.rollover, meter_id: required.meter_id }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSlackSharedChannelSubscriberRequired {
    ///The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    ///The description of the benefit.
    pub description: String,
    ///Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    ///Whether the benefit is deletable.
    pub deletable: bool,
    ///Whether the benefit is deleted.
    pub is_deleted: bool,
    ///The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties: crate::v2027_01::models::benefit_grants::types::BenefitSlackSharedChannelSubscriberProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSlackSharedChannelSubscriber {
    ///The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("slack_shared_channel"),
    ///The description of the benefit.
    pub description: String,
    ///Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    ///Whether the benefit is deletable.
    pub deletable: bool,
    ///Whether the benefit is deleted.
    pub is_deleted: bool,
    ///The ID of the organization owning the benefit.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub organization: crate::v2027_01::models::benefit_grants::types::BenefitSubscriberOrganization,
    pub properties: crate::v2027_01::models::benefit_grants::types::BenefitSlackSharedChannelSubscriberProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelSubscriber {
    pub fn new(required: BenefitSlackSharedChannelSubscriberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            r#type: Default::default(),
            description: required.description,
            selectable: required.selectable,
            deletable: required.deletable,
            is_deleted: required.is_deleted,
            organization_id: required.organization_id,
            organization: required.organization,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitSlackSharedChannelSubscriberProperties {}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelSubscriberProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSubscriberOrganizationRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: String,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    pub avatar_url: Option<String>,
    /// Proration behavior applied when customer updates their subscription from
    /// the portal.
    pub proration_behavior: crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSubscriberOrganization {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: String,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
    /// Proration behavior applied when customer updates their subscription from
    /// the portal.
    pub proration_behavior: crate::v2027_01::models::common::enums::SubscriptionProrationBehavior,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitSubscriberOrganization {
    pub fn new(required: BenefitSubscriberOrganizationRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            slug: required.slug,
            avatar_url: required.avatar_url,
            proration_behavior: required.proration_behavior,
            allow_customer_updates: required.allow_customer_updates,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantCustomRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitCustomSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantCustomProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantCustom {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitCustomSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantCustomProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantCustom {
    pub fn new(required: CustomerBenefitGrantCustomRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerBenefitGrantCustomUpdate {
    pub benefit_type: monostate::MustBe!("custom"),
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantCustomUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantDiscordRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitDiscordSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantDiscordProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantDiscord {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<
        crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantMember,
    >,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitDiscordSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantDiscordProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantDiscord {
    pub fn new(required: CustomerBenefitGrantDiscordRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(
        mut self,
        value: crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantMember,
    ) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantDiscordPropertiesUpdateRequired {
    pub account_id: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantDiscordPropertiesUpdate {
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub account_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantDiscordPropertiesUpdate {
    pub fn new(required: CustomerBenefitGrantDiscordPropertiesUpdateRequired) -> Self {
        Self { account_id: required.account_id }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantDiscordUpdateRequired {
    pub properties:
        crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantDiscordPropertiesUpdate,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantDiscordUpdate {
    pub benefit_type: monostate::MustBe!("discord"),
    pub properties:
        crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantDiscordPropertiesUpdate,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantDiscordUpdate {
    pub fn new(required: CustomerBenefitGrantDiscordUpdateRequired) -> Self {
        Self { benefit_type: Default::default(), properties: required.properties }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantDownloadablesRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitDownloadablesSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantDownloadablesProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantDownloadables {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitDownloadablesSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantDownloadablesProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantDownloadables {
    pub fn new(required: CustomerBenefitGrantDownloadablesRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerBenefitGrantDownloadablesUpdate {
    pub benefit_type: monostate::MustBe!("downloadables"),
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantDownloadablesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantFeatureFlagRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitFeatureFlagSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantFeatureFlagProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantFeatureFlag {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitFeatureFlagSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantFeatureFlagProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantFeatureFlag {
    pub fn new(required: CustomerBenefitGrantFeatureFlagRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerBenefitGrantFeatureFlagUpdate {
    pub benefit_type: monostate::MustBe!("feature_flag"),
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantFeatureFlagUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantGitHubRepositoryRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitGitHubRepositorySubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantGitHubRepositoryProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantGitHubRepository {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<
        crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantMember,
    >,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitGitHubRepositorySubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantGitHubRepositoryProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantGitHubRepository {
    pub fn new(required: CustomerBenefitGrantGitHubRepositoryRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(
        mut self,
        value: crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantMember,
    ) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantGitHubRepositoryPropertiesUpdateRequired {
    pub account_id: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantGitHubRepositoryPropertiesUpdate {
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub account_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantGitHubRepositoryPropertiesUpdate {
    pub fn new(required: CustomerBenefitGrantGitHubRepositoryPropertiesUpdateRequired) -> Self {
        Self { account_id: required.account_id }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantGitHubRepositoryUpdateRequired {
    pub properties: crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantGitHubRepositoryPropertiesUpdate,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantGitHubRepositoryUpdate {
    pub benefit_type: monostate::MustBe!("github_repository"),
    pub properties: crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantGitHubRepositoryPropertiesUpdate,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantGitHubRepositoryUpdate {
    pub fn new(required: CustomerBenefitGrantGitHubRepositoryUpdateRequired) -> Self {
        Self { benefit_type: Default::default(), properties: required.properties }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantLicenseKeysRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitLicenseKeysSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantLicenseKeysProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantLicenseKeys {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitLicenseKeysSubscriber,
    pub properties: crate::v2027_01::models::common::types::BenefitGrantLicenseKeysProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantLicenseKeys {
    pub fn new(required: CustomerBenefitGrantLicenseKeysRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerBenefitGrantLicenseKeysUpdate {
    pub benefit_type: monostate::MustBe!("license_keys"),
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantLicenseKeysUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantMemberRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MemberId,
    pub oauth_accounts: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::types::CustomerPortalOAuthAccount,
    >,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantMember {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MemberId,
    pub oauth_accounts: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::types::CustomerPortalOAuthAccount,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantMember {
    pub fn new(required: CustomerBenefitGrantMemberRequired) -> Self {
        Self { id: required.id, oauth_accounts: required.oauth_accounts }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantMeterCreditRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitMeterCreditSubscriber,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitGrantMeterCreditProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantMeterCredit {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit: crate::v2027_01::models::benefit_grants::types::BenefitMeterCreditSubscriber,
    pub properties:
        crate::v2027_01::models::benefit_grants::types::BenefitGrantMeterCreditProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantMeterCredit {
    pub fn new(required: CustomerBenefitGrantMeterCreditRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerBenefitGrantMeterCreditUpdate {
    pub benefit_type: monostate::MustBe!("meter_credit"),
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantMeterCreditUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantSlackSharedChannelRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    pub granted_at: Option<jiff::Timestamp>,
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit:
        crate::v2027_01::models::benefit_grants::types::BenefitSlackSharedChannelSubscriber,
    pub properties:
        crate::v2027_01::models::common::types::BenefitGrantSlackSharedChannelProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantSlackSharedChannel {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub granted_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    pub is_granted: bool,
    pub is_revoked: bool,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::types::CustomerPortalCustomer,
    pub benefit:
        crate::v2027_01::models::benefit_grants::types::BenefitSlackSharedChannelSubscriber,
    pub properties:
        crate::v2027_01::models::common::types::BenefitGrantSlackSharedChannelProperties,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantSlackSharedChannel {
    pub fn new(required: CustomerBenefitGrantSlackSharedChannelRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: required.granted_at,
            revoked_at: required.revoked_at,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            is_granted: required.is_granted,
            is_revoked: required.is_revoked,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantSlackSharedChannelPropertiesUpdateRequired {
    pub invited_email: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantSlackSharedChannelPropertiesUpdate {
    pub invited_email: String,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantSlackSharedChannelPropertiesUpdate {
    pub fn new(required: CustomerBenefitGrantSlackSharedChannelPropertiesUpdateRequired) -> Self {
        Self { invited_email: required.invited_email }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerBenefitGrantSlackSharedChannelUpdateRequired {
    pub properties: crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantSlackSharedChannelPropertiesUpdate,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerBenefitGrantSlackSharedChannelUpdate {
    pub benefit_type: monostate::MustBe!("slack_shared_channel"),
    pub properties: crate::v2027_01::models::benefit_grants::types::CustomerBenefitGrantSlackSharedChannelPropertiesUpdate,
}
#[allow(clippy::type_complexity)]
impl CustomerBenefitGrantSlackSharedChannelUpdate {
    pub fn new(required: CustomerBenefitGrantSlackSharedChannelUpdateRequired) -> Self {
        Self { benefit_type: Default::default(), properties: required.properties }
    }
}
