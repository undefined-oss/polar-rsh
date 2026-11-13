#[derive(Debug, Clone)]
pub struct AddressRequired {
    pub country: crate::v2027_01::models::common::enums::CountryAlpha2,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Address {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub line1: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub line2: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub postal_code: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub city: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub state: presence_rs::Presence<String>,
    pub country: crate::v2027_01::models::common::enums::CountryAlpha2,
}
#[allow(clippy::type_complexity)]
impl Address {
    pub fn new(required: AddressRequired) -> Self {
        Self {
            line1: presence_rs::Presence::Absent,
            line2: presence_rs::Presence::Absent,
            postal_code: presence_rs::Presence::Absent,
            city: presence_rs::Presence::Absent,
            state: presence_rs::Presence::Absent,
            country: required.country,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct AddressInputRequired {
    pub country: crate::v2027_01::models::common::enums::CountryAlpha2Input,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AddressInput {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub line1: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub line2: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub postal_code: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub city: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub state: presence_rs::Presence<String>,
    pub country: crate::v2027_01::models::common::enums::CountryAlpha2Input,
}
#[allow(clippy::type_complexity)]
impl AddressInput {
    pub fn new(required: AddressInputRequired) -> Self {
        Self {
            line1: presence_rs::Presence::Absent,
            line2: presence_rs::Presence::Absent,
            postal_code: presence_rs::Presence::Absent,
            city: presence_rs::Presence::Absent,
            state: presence_rs::Presence::Absent,
            country: required.country,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct AmbiguousExternalCustomerIdRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct AmbiguousExternalCustomerId {
    pub error: monostate::MustBe!("AmbiguousExternalCustomerID"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl AmbiguousExternalCustomerId {
    pub fn new(required: AmbiguousExternalCustomerIdRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct AttachedCustomFieldRequired {
    /// ID of the custom field.
    pub custom_field_id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub custom_field: crate::v2027_01::models::common::unions::CustomField,
    /// Order of the custom field in the resource.
    pub order: i64,
    /// Whether the value is required for this custom field.
    pub required: bool,
}
/// Schema of a custom field attached to a resource.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AttachedCustomField {
    /// ID of the custom field.
    pub custom_field_id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub custom_field: crate::v2027_01::models::common::unions::CustomField,
    /// Order of the custom field in the resource.
    pub order: i64,
    /// Whether the value is required for this custom field.
    pub required: bool,
}
#[allow(clippy::type_complexity)]
impl AttachedCustomField {
    pub fn new(required: AttachedCustomFieldRequired) -> Self {
        Self {
            custom_field_id: required.custom_field_id,
            custom_field: required.custom_field,
            order: required.order,
            required: required.required,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitCustomRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitCustomProperties,
    pub visibility_configurable: bool,
}
/// A benefit of type `custom`.
///
/// Use it to grant any kind of benefit that doesn't fit in the other types.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCustom {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitCustomProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitCustom {
    pub fn new(required: BenefitCustomRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitCustomPropertiesRequired {
    pub note: Option<String>,
}
/// Properties for a benefit of type `custom`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCustomProperties {
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub note: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitCustomProperties {
    pub fn new(required: BenefitCustomPropertiesRequired) -> Self {
        Self { note: required.note }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitCustomPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCustomPublic {
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
}
#[allow(clippy::type_complexity)]
impl BenefitCustomPublic {
    pub fn new(required: BenefitCustomPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitDiscordProperties,
    pub visibility_configurable: bool,
}
/// A benefit of type `discord`.
///
/// Use it to automatically invite your backers to a Discord server.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscord {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitDiscordProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscord {
    pub fn new(required: BenefitDiscordRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordPropertiesRequired {
    /// The ID of the Discord server.
    pub guild_id: String,
    /// The ID of the Discord role to grant.
    pub role_id: String,
    /// Whether to kick the member from the Discord server on revocation.
    pub kick_member: bool,
}
/// Properties for a benefit of type `discord`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscordProperties {
    /// The ID of the Discord server.
    pub guild_id: String,
    /// The ID of the Discord role to grant.
    pub role_id: String,
    /// Whether to kick the member from the Discord server on revocation.
    pub kick_member: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordProperties {
    pub fn new(required: BenefitDiscordPropertiesRequired) -> Self {
        Self {
            guild_id: required.guild_id,
            role_id: required.role_id,
            kick_member: required.kick_member,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscordPublic {
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
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordPublic {
    pub fn new(required: BenefitDiscordPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitDownloadablesProperties,
    pub visibility_configurable: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadables {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitDownloadablesProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadables {
    pub fn new(required: BenefitDownloadablesRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesPropertiesRequired {
    pub archived: std::collections::BTreeMap<
        crate::v2027_01::models::common::ids::BenefitDownloadablesPropertiesArchivedId,
        bool,
    >,
    pub files: Vec<crate::v2027_01::models::common::ids::FileId>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadablesProperties {
    pub archived: std::collections::BTreeMap<
        crate::v2027_01::models::common::ids::BenefitDownloadablesPropertiesArchivedId,
        bool,
    >,
    pub files: Vec<crate::v2027_01::models::common::ids::FileId>,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesProperties {
    pub fn new(required: BenefitDownloadablesPropertiesRequired) -> Self {
        Self { archived: required.archived, files: required.files }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadablesPublic {
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
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesPublic {
    pub fn new(required: BenefitDownloadablesPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitFeatureFlagRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitFeatureFlagProperties,
    pub visibility_configurable: bool,
}
/// A benefit of type `feature_flag`.
///
/// Use it to grant feature flags with key-value metadata
/// that can be queried via the API and webhooks.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitFeatureFlag {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitFeatureFlagProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlag {
    pub fn new(required: BenefitFeatureFlagRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
/// Properties for a benefit of type `feature_flag`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitFeatureFlagProperties {}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct BenefitFeatureFlagPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitFeatureFlagPublic {
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
}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagPublic {
    pub fn new(required: BenefitFeatureFlagPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositoryRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitGitHubRepositoryProperties,
    pub visibility_configurable: bool,
}
/// A benefit of type `github_repository`.
///
/// Use it to automatically invite your backers to a private GitHub repository.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepository {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitGitHubRepositoryProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepository {
    pub fn new(required: BenefitGitHubRepositoryRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositoryPropertiesRequired {
    /// The owner of the repository.
    pub repository_owner: String,
    /// The name of the repository.
    pub repository_name: String,
    /// The permission level to grant. Read more about roles and their permissions on [GitHub documentation](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization#permissions-for-each-role).
    pub permission:
        crate::v2027_01::models::common::enums::BenefitGitHubRepositoryPropertiesPermission,
}
/// Properties for a benefit of type `github_repository`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepositoryProperties {
    /// The owner of the repository.
    pub repository_owner: String,
    /// The name of the repository.
    pub repository_name: String,
    /// The permission level to grant. Read more about roles and their permissions on [GitHub documentation](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization#permissions-for-each-role).
    pub permission:
        crate::v2027_01::models::common::enums::BenefitGitHubRepositoryPropertiesPermission,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositoryProperties {
    pub fn new(required: BenefitGitHubRepositoryPropertiesRequired) -> Self {
        Self {
            repository_owner: required.repository_owner,
            repository_name: required.repository_name,
            permission: required.permission,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositoryPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepositoryPublic {
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
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositoryPublic {
    pub fn new(required: BenefitGitHubRepositoryPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantRequired {
    /// When the manual grant was requested, if applicable.
    pub manually_granted_at: Option<jiff::Timestamp>,
    /// When this grant was permanently revoked manually, if applicable.
    pub manually_revoked_at: Option<jiff::Timestamp>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    pub customer: crate::v2027_01::models::common::unions::Customer,
    pub benefit: crate::v2027_01::models::common::unions::Benefit,
    pub properties: crate::v2027_01::models::common::unions::BenefitGrantPropertiesUnion,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrant {
    /// When the manual grant was requested, if applicable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub manually_granted_at: Option<jiff::Timestamp>,
    /// When this grant was permanently revoked manually, if applicable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub manually_revoked_at: Option<jiff::Timestamp>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the grant.
    pub id: crate::v2027_01::models::common::ids::BenefitGrantId,
    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub granted_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is granted.
    pub is_granted: bool,
    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub revoked_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the benefit is revoked.
    pub is_revoked: bool,
    /// The ID of the subscription that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    /// The ID of the order that granted this benefit.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    /// The ID of the customer concerned by this grant.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the member concerned by this grant.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2027_01::models::common::ids::BenefitId,
    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub error: presence_rs::Presence<crate::v2027_01::models::common::types::BenefitGrantError>,
    pub customer: crate::v2027_01::models::common::unions::Customer,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member: presence_rs::Presence<crate::v2027_01::models::common::types::Member>,
    pub benefit: crate::v2027_01::models::common::unions::Benefit,
    pub properties: crate::v2027_01::models::common::unions::BenefitGrantPropertiesUnion,
}
#[allow(clippy::type_complexity)]
impl BenefitGrant {
    pub fn new(required: BenefitGrantRequired) -> Self {
        Self {
            manually_granted_at: required.manually_granted_at,
            manually_revoked_at: required.manually_revoked_at,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            granted_at: presence_rs::Presence::Absent,
            is_granted: required.is_granted,
            revoked_at: presence_rs::Presence::Absent,
            is_revoked: required.is_revoked,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            customer_id: required.customer_id,
            member_id: presence_rs::Presence::Absent,
            benefit_id: required.benefit_id,
            error: presence_rs::Presence::Absent,
            customer: required.customer,
            member: presence_rs::Presence::Absent,
            benefit: required.benefit,
            properties: required.properties,
        }
    }

    /// The timestamp when the benefit was granted. If `None`, the benefit is
    /// not granted.
    #[allow(clippy::should_implement_trait)]
    pub fn granted_at(mut self, value: jiff::Timestamp) -> Self {
        self.granted_at = presence_rs::Presence::Some(value);
        self
    }

    /// The timestamp when the benefit was revoked. If `None`, the benefit is
    /// not revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn revoked_at(mut self, value: jiff::Timestamp) -> Self {
        self.revoked_at = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member concerned by this grant.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// The error information if the benefit grant failed with an unrecoverable
    /// error.
    #[allow(clippy::should_implement_trait)]
    pub fn error(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitGrantError,
    ) -> Self {
        self.error = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn member(mut self, value: crate::v2027_01::models::common::types::Member) -> Self {
        self.member = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantCustomProperties {}
#[allow(clippy::type_complexity)]
impl BenefitGrantCustomProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantDiscordProperties {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub account_id: presence_rs::Presence<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub guild_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub role_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub granted_account_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantDiscordProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn guild_id(mut self, value: impl Into<String>) -> Self {
        self.guild_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn role_id(mut self, value: impl Into<String>) -> Self {
        self.role_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn granted_account_id(mut self, value: impl Into<String>) -> Self {
        self.granted_account_id = Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantDownloadablesProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub files: Option<Vec<String>>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantDownloadablesProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn files(mut self, value: Vec<String>) -> Self {
        self.files = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGrantErrorRequired {
    pub message: String,
    pub r#type: String,
    pub timestamp: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGrantError {
    pub message: String,
    pub r#type: String,
    pub timestamp: String,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantError {
    pub fn new(required: BenefitGrantErrorRequired) -> Self {
        Self { message: required.message, r#type: required.r#type, timestamp: required.timestamp }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantFeatureFlagProperties {}
#[allow(clippy::type_complexity)]
impl BenefitGrantFeatureFlagProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantGitHubRepositoryProperties {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub account_id: presence_rs::Presence<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub repository_owner: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub repository_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub permission: Option<
        crate::v2027_01::models::common::enums::BenefitGrantGitHubRepositoryPropertiesPermission,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub granted_account_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantGitHubRepositoryProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn repository_owner(mut self, value: impl Into<String>) -> Self {
        self.repository_owner = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn repository_name(mut self, value: impl Into<String>) -> Self {
        self.repository_name = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn permission(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitGrantGitHubRepositoryPropertiesPermission,
    ) -> Self {
        self.permission = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn granted_account_id(mut self, value: impl Into<String>) -> Self {
        self.granted_account_id = Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantLicenseKeysProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub user_provided_key: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub license_key_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub display_key: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantLicenseKeysProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn user_provided_key(mut self, value: impl Into<String>) -> Self {
        self.user_provided_key = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn license_key_id(mut self, value: impl Into<String>) -> Self {
        self.license_key_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn display_key(mut self, value: impl Into<String>) -> Self {
        self.display_key = Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGrantSlackSharedChannelProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub invited_email: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub channel_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub invite_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub invite_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub connected_team_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitGrantSlackSharedChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn invited_email(mut self, value: impl Into<String>) -> Self {
        self.invited_email = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn channel_name(mut self, value: impl Into<String>) -> Self {
        self.channel_name = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn invite_id(mut self, value: impl Into<String>) -> Self {
        self.invite_id = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn invite_url(mut self, value: impl Into<String>) -> Self {
        self.invite_url = Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn connected_team_id(mut self, value: impl Into<String>) -> Self {
        self.connected_team_id = Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeyActivationPropertiesRequired {
    pub limit: i64,
    pub enable_customer_admin: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeyActivationProperties {
    pub limit: i64,
    pub enable_customer_admin: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeyActivationProperties {
    pub fn new(required: BenefitLicenseKeyActivationPropertiesRequired) -> Self {
        Self { limit: required.limit, enable_customer_admin: required.enable_customer_admin }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeyExpirationPropertiesRequired {
    pub ttl: crate::v2027_01::models::common::newtypes::BenefitLicenseKeyExpirationPropertiesTtl,
    pub timeframe:
        crate::v2027_01::models::common::enums::BenefitLicenseKeyExpirationPropertiesTimeframe,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeyExpirationProperties {
    pub ttl: crate::v2027_01::models::common::newtypes::BenefitLicenseKeyExpirationPropertiesTtl,
    pub timeframe:
        crate::v2027_01::models::common::enums::BenefitLicenseKeyExpirationPropertiesTimeframe,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeyExpirationProperties {
    pub fn new(required: BenefitLicenseKeyExpirationPropertiesRequired) -> Self {
        Self { ttl: required.ttl, timeframe: required.timeframe }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeysRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitLicenseKeysProperties,
    pub visibility_configurable: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeys {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitLicenseKeysProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeys {
    pub fn new(required: BenefitLicenseKeysRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeysPropertiesRequired {
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
pub struct BenefitLicenseKeysProperties {
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
impl BenefitLicenseKeysProperties {
    pub fn new(required: BenefitLicenseKeysPropertiesRequired) -> Self {
        Self {
            prefix: required.prefix,
            expires: required.expires,
            activations: required.activations,
            limit_usage: required.limit_usage,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeysPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeysPublic {
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
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeysPublic {
    pub fn new(required: BenefitLicenseKeysPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitMeterCreditProperties,
    pub visibility_configurable: bool,
}
/// A benefit of type `meter_unit`.
///
/// Use it to grant a number of units on a specific meter.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCredit {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitMeterCreditProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCredit {
    pub fn new(required: BenefitMeterCreditRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditPropertiesRequired {
    pub units: i64,
    pub rollover: bool,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
/// Properties for a benefit of type `meter_unit`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditProperties {
    pub units: i64,
    pub rollover: bool,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditProperties {
    pub fn new(required: BenefitMeterCreditPropertiesRequired) -> Self {
        Self { units: required.units, rollover: required.rollover, meter_id: required.meter_id }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditPublicRequired {
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
    pub properties: crate::v2027_01::models::common::types::BenefitMeterCreditPublicProperties,
}
/// A benefit of type `meter_credit`.
///
/// Grants a number of units on a specific meter.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditPublic {
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
    pub properties: crate::v2027_01::models::common::types::BenefitMeterCreditPublicProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditPublic {
    pub fn new(required: BenefitMeterCreditPublicRequired) -> Self {
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
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditPublicPropertiesRequired {
    pub units: i64,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
/// Properties for a benefit of type `meter_credit`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditPublicProperties {
    pub units: i64,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditPublicProperties {
    pub fn new(required: BenefitMeterCreditPublicPropertiesRequired) -> Self {
        Self { units: required.units, meter_id: required.meter_id }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSlackSharedChannelRequired {
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitSlackSharedChannelProperties,
    pub visibility_configurable: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSlackSharedChannel {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("slack_shared_channel"),
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
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The visibility of the benefit in the customer portal.
    pub visibility: crate::v2027_01::models::common::enums::BenefitVisibility,
    /// ID of the catalog that introduced this version of the benefit. None for
    /// benefits not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the benefit.
    pub is_current: bool,
    pub properties: crate::v2027_01::models::common::types::BenefitSlackSharedChannelProperties,
    pub visibility_configurable: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannel {
    pub fn new(required: BenefitSlackSharedChannelRequired) -> Self {
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
            external_id: required.external_id,
            metadata: required.metadata,
            visibility: required.visibility,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            properties: required.properties,
            visibility_configurable: required.visibility_configurable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSlackSharedChannelPropertiesRequired {
    ///Polar Slack integration linked to this benefit.
    pub slack_integration_id: crate::v2027_01::models::common::ids::SlackIntegrationId,
    ///Template for the channel name. Supports placeholders: {customer_name}, {customer_email_local}, and {metadata.<key>} for any value stored in customer user metadata.
    pub channel_name_template: crate::v2027_01::models::common::newtypes::BenefitSlackSharedChannelPropertiesChannelNameTemplate,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSlackSharedChannelProperties {
    ///Polar Slack integration linked to this benefit.
    pub slack_integration_id: crate::v2027_01::models::common::ids::SlackIntegrationId,
    ///Template for the channel name. Supports placeholders: {customer_name}, {customer_email_local}, and {metadata.<key>} for any value stored in customer user metadata.
    pub channel_name_template: crate::v2027_01::models::common::newtypes::BenefitSlackSharedChannelPropertiesChannelNameTemplate,
    ///Create the channel as private (recommended).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub private: Option<bool>,
    ///Optional message posted to the channel right after creation.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub welcome_message: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::BenefitSlackSharedChannelPropertiesWelcomeMessage,
    >,
    ///Archive the channel when the benefit is revoked.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub archive_on_revoke: Option<bool>,
    ///Slack user IDs from the merchant workspace to invite to every channel created for this benefit.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub team_invitees: Option<Vec<String>>,
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelProperties {
    pub fn new(required: BenefitSlackSharedChannelPropertiesRequired) -> Self {
        Self {
            slack_integration_id: required.slack_integration_id,
            channel_name_template: required.channel_name_template,
            private: None,
            welcome_message: presence_rs::Presence::Absent,
            archive_on_revoke: None,
            team_invitees: None,
        }
    }

    /// Create the channel as private (recommended).
    #[allow(clippy::should_implement_trait)]
    pub fn private(mut self, value: bool) -> Self {
        self.private = Some(value);
        self
    }

    /// Optional message posted to the channel right after creation.
    #[allow(clippy::should_implement_trait)]
    pub fn welcome_message(
        mut self,
        value: crate::v2027_01::models::common::newtypes::BenefitSlackSharedChannelPropertiesWelcomeMessage,
    ) -> Self {
        self.welcome_message = presence_rs::Presence::Some(value);
        self
    }

    /// Archive the channel when the benefit is revoked.
    #[allow(clippy::should_implement_trait)]
    pub fn archive_on_revoke(mut self, value: bool) -> Self {
        self.archive_on_revoke = Some(value);
        self
    }

    /// Slack user IDs from the merchant workspace to invite to every channel
    /// created for this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn team_invitees(mut self, value: Vec<String>) -> Self {
        self.team_invitees = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSlackSharedChannelPublicRequired {
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
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSlackSharedChannelPublic {
    /// The ID of the benefit.
    pub id: crate::v2027_01::models::common::ids::BenefitId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub r#type: monostate::MustBe!("slack_shared_channel"),
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
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelPublic {
    pub fn new(required: BenefitSlackSharedChannelPublicRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CountAggregation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub func: Option<monostate::MustBe!("count")>,
}
#[allow(clippy::type_complexity)]
impl CountAggregation {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn func(mut self, value: monostate::MustBe!("count")) -> Self {
        self.func = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldCheckboxRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldCheckboxProperties,
}
/// Schema for a custom field of type checkbox.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldCheckbox {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    pub r#type: monostate::MustBe!("checkbox"),
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldCheckboxProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCheckbox {
    pub fn new(required: CustomFieldCheckboxRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            metadata: required.metadata,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: required.organization_id,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldCheckboxProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_label:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldCheckboxPropertiesFormLabel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_help_text: Option<
        crate::v2027_01::models::common::newtypes::CustomFieldCheckboxPropertiesFormHelpText,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_placeholder: Option<
        crate::v2027_01::models::common::newtypes::CustomFieldCheckboxPropertiesFormPlaceholder,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCheckboxProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_label(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldCheckboxPropertiesFormLabel,
    ) -> Self {
        self.form_label = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_help_text(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldCheckboxPropertiesFormHelpText,
    ) -> Self {
        self.form_help_text = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_placeholder(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldCheckboxPropertiesFormPlaceholder,
    ) -> Self {
        self.form_placeholder = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldDateRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldDateProperties,
}
/// Schema for a custom field of type date.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldDate {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    pub r#type: monostate::MustBe!("date"),
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldDateProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldDate {
    pub fn new(required: CustomFieldDateRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            metadata: required.metadata,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: required.organization_id,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldDateProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_label:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesFormLabel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_help_text:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesFormHelpText>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_placeholder:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesFormPlaceholder>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub ge: Option<crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesGe>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub le: Option<crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesLe>,
}
#[allow(clippy::type_complexity)]
impl CustomFieldDateProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_label(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesFormLabel,
    ) -> Self {
        self.form_label = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_help_text(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesFormHelpText,
    ) -> Self {
        self.form_help_text = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_placeholder(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesFormPlaceholder,
    ) -> Self {
        self.form_placeholder = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ge(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesGe,
    ) -> Self {
        self.ge = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn le(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldDatePropertiesLe,
    ) -> Self {
        self.le = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldNumberRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldNumberProperties,
}
/// Schema for a custom field of type number.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldNumber {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    pub r#type: monostate::MustBe!("number"),
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldNumberProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldNumber {
    pub fn new(required: CustomFieldNumberRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            metadata: required.metadata,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: required.organization_id,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldNumberProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_label:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesFormLabel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_help_text:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesFormHelpText>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_placeholder: Option<
        crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesFormPlaceholder,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub ge: Option<crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesGe>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub le: Option<crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesLe>,
}
#[allow(clippy::type_complexity)]
impl CustomFieldNumberProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_label(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesFormLabel,
    ) -> Self {
        self.form_label = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_help_text(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesFormHelpText,
    ) -> Self {
        self.form_help_text = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_placeholder(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesFormPlaceholder,
    ) -> Self {
        self.form_placeholder = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ge(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesGe,
    ) -> Self {
        self.ge = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn le(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldNumberPropertiesLe,
    ) -> Self {
        self.le = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldSelectRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldSelectProperties,
}
/// Schema for a custom field of type select.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldSelect {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    pub r#type: monostate::MustBe!("select"),
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldSelectProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldSelect {
    pub fn new(required: CustomFieldSelectRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            metadata: required.metadata,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: required.organization_id,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldSelectOptionRequired {
    pub value: crate::v2027_01::models::common::newtypes::CustomFieldSelectOptionValue,
    pub label: crate::v2027_01::models::common::newtypes::CustomFieldSelectOptionLabel,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldSelectOption {
    pub value: crate::v2027_01::models::common::newtypes::CustomFieldSelectOptionValue,
    pub label: crate::v2027_01::models::common::newtypes::CustomFieldSelectOptionLabel,
}
#[allow(clippy::type_complexity)]
impl CustomFieldSelectOption {
    pub fn new(required: CustomFieldSelectOptionRequired) -> Self {
        Self { value: required.value, label: required.label }
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldSelectPropertiesRequired {
    pub options: Vec<crate::v2027_01::models::common::types::CustomFieldSelectOption>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldSelectProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_label:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldSelectPropertiesFormLabel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_help_text:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldSelectPropertiesFormHelpText>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_placeholder: Option<
        crate::v2027_01::models::common::newtypes::CustomFieldSelectPropertiesFormPlaceholder,
    >,
    pub options: Vec<crate::v2027_01::models::common::types::CustomFieldSelectOption>,
}
#[allow(clippy::type_complexity)]
impl CustomFieldSelectProperties {
    pub fn new(required: CustomFieldSelectPropertiesRequired) -> Self {
        Self {
            form_label: None,
            form_help_text: None,
            form_placeholder: None,
            options: required.options,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_label(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldSelectPropertiesFormLabel,
    ) -> Self {
        self.form_label = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_help_text(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldSelectPropertiesFormHelpText,
    ) -> Self {
        self.form_help_text = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_placeholder(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldSelectPropertiesFormPlaceholder,
    ) -> Self {
        self.form_placeholder = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldTextRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldTextProperties,
}
/// Schema for a custom field of type text.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldText {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomFieldId,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    pub r#type: monostate::MustBe!("text"),
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub properties: crate::v2027_01::models::common::types::CustomFieldTextProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldText {
    pub fn new(required: CustomFieldTextRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            metadata: required.metadata,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: required.organization_id,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldTextProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_label:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesFormLabel>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_help_text:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesFormHelpText>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub form_placeholder:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesFormPlaceholder>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub textarea: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub min_length:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesMinLength>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub max_length:
        Option<crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesMaxLength>,
}
#[allow(clippy::type_complexity)]
impl CustomFieldTextProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_label(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesFormLabel,
    ) -> Self {
        self.form_label = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_help_text(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesFormHelpText,
    ) -> Self {
        self.form_help_text = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn form_placeholder(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesFormPlaceholder,
    ) -> Self {
        self.form_placeholder = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn textarea(mut self, value: bool) -> Self {
        self.textarea = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn min_length(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesMinLength,
    ) -> Self {
        self.min_length = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn max_length(
        mut self,
        value: crate::v2027_01::models::common::newtypes::CustomFieldTextPropertiesMaxLength,
    ) -> Self {
        self.max_length = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerIndividualRequired {
    /// The ID of the customer.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The email address of the customer. This must be unique within the
    /// organization.
    pub email: String,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The name of the customer.
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    pub first_user_event_at: Option<jiff::Timestamp>,
    pub avatar_url: Option<String>,
}
/// A customer in an organization.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerIndividual {
    /// The ID of the customer.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    /// The email address of the customer. This must be unique within the
    /// organization.
    pub email: String,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The type of customer.
    pub r#type: monostate::MustBe!("individual"),
    /// The name of the customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerIndividual {
    pub fn new(required: CustomerIndividualRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            metadata: required.metadata,
            external_id: presence_rs::Presence::Absent,
            email: required.email,
            email_verified: required.email_verified,
            r#type: Default::default(),
            name: required.name,
            billing_name: required.billing_name,
            billing_address: required.billing_address,
            tax_id: required.tax_id,
            locale: presence_rs::Presence::Absent,
            organization_id: required.organization_id,
            default_payment_method_id: presence_rs::Presence::Absent,
            deleted_at: required.deleted_at,
            first_user_event_at: required.first_user_event_at,
            avatar_url: required.avatar_url,
        }
    }

    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[allow(clippy::should_implement_trait)]
    pub fn default_payment_method_id(
        mut self,
        value: crate::v2027_01::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrganizationRequired {
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
    /// Settings related to the customer portal
    pub customer_portal_settings:
        crate::v2027_01::models::common::types::OrganizationCustomerPortalSettings,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrganization {
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
    /// Settings related to the customer portal
    pub customer_portal_settings:
        crate::v2027_01::models::common::types::OrganizationCustomerPortalSettings,
    /// Feature flags for the customer portal.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub organization_features:
        Option<crate::v2027_01::models::common::types::CustomerOrganizationFeatureSettings>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrganization {
    pub fn new(required: CustomerOrganizationRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            slug: required.slug,
            avatar_url: required.avatar_url,
            proration_behavior: required.proration_behavior,
            allow_customer_updates: required.allow_customer_updates,
            customer_portal_settings: required.customer_portal_settings,
            organization_features: None,
        }
    }

    /// Feature flags for the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn organization_features(
        mut self,
        value: crate::v2027_01::models::common::types::CustomerOrganizationFeatureSettings,
    ) -> Self {
        self.organization_features = Some(value);
        self
    }
}
/// Feature flags exposed to the customer portal.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerOrganizationFeatureSettings {
    /// Whether localization is enabled for this organization.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub checkout_localization_enabled: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrganizationFeatureSettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether localization is enabled for this organization.
    #[allow(clippy::should_implement_trait)]
    pub fn checkout_localization_enabled(mut self, value: bool) -> Self {
        self.checkout_localization_enabled = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPortalCustomerRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    pub email: Option<String>,
    pub email_verified: bool,
    pub name: Option<String>,
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    pub oauth_accounts: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::types::CustomerPortalOAuthAccount,
    >,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPortalCustomer {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub email: Option<String>,
    pub email_verified: bool,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    pub oauth_accounts: std::collections::BTreeMap<
        String,
        crate::v2027_01::models::common::types::CustomerPortalOAuthAccount,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::DefaultPaymentMethodId>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub r#type: presence_rs::Presence<crate::v2027_01::models::common::enums::CustomerType>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalCustomer {
    pub fn new(required: CustomerPortalCustomerRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            email: required.email,
            email_verified: required.email_verified,
            name: required.name,
            billing_name: required.billing_name,
            billing_address: required.billing_address,
            tax_id: required.tax_id,
            oauth_accounts: required.oauth_accounts,
            default_payment_method_id: presence_rs::Presence::Absent,
            r#type: presence_rs::Presence::Absent,
            locale: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default_payment_method_id(
        mut self,
        value: crate::v2027_01::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn r#type(mut self, value: crate::v2027_01::models::common::enums::CustomerType) -> Self {
        self.r#type = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerPortalCustomerSettings {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub allow_email_change: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalCustomerSettings {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn allow_email_change(mut self, value: bool) -> Self {
        self.allow_email_change = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPortalOAuthAccountRequired {
    pub account_id: String,
    pub account_username: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPortalOAuthAccount {
    pub account_id: String,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub account_username: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalOAuthAccount {
    pub fn new(required: CustomerPortalOAuthAccountRequired) -> Self {
        Self { account_id: required.account_id, account_username: required.account_username }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPortalSubscriptionSettingsRequired {
    pub update_seats: bool,
    pub update_plan: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPortalSubscriptionSettings {
    pub update_seats: bool,
    pub update_plan: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub update_units: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub pause: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalSubscriptionSettings {
    pub fn new(required: CustomerPortalSubscriptionSettingsRequired) -> Self {
        Self {
            update_seats: required.update_seats,
            update_plan: required.update_plan,
            update_units: None,
            pause: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn update_units(mut self, value: bool) -> Self {
        self.update_units = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn pause(mut self, value: bool) -> Self {
        self.pause = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPortalUsageSettingsRequired {
    pub show: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPortalUsageSettings {
    pub show: bool,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalUsageSettings {
    pub fn new(required: CustomerPortalUsageSettingsRequired) -> Self {
        Self { show: required.show }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSeatRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The seat ID
    pub id: crate::v2027_01::models::common::ids::SeatId,
    /// The subscription ID (for recurring seats)
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    /// The order ID (for one-time purchase seats)
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    /// Status of the seat
    pub status: crate::v2027_01::models::common::enums::SeatStatus,
    /// The customer ID. When member_model_enabled is true, this is the billing
    /// customer (purchaser). When false, this is the seat member customer.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The member ID of the seat occupant
    pub member_id: Option<crate::v2027_01::models::common::ids::MemberId>,
    /// The member associated with this seat
    pub member: Option<crate::v2027_01::models::common::types::Member>,
    /// Email of the seat member (set when member_model_enabled is true)
    pub email: Option<String>,
    /// The assigned customer email
    pub customer_email: Option<String>,
    /// When the invitation token expires
    pub invitation_token_expires_at: Option<jiff::Timestamp>,
    /// When the seat was claimed
    pub claimed_at: Option<jiff::Timestamp>,
    /// When the seat was revoked
    pub revoked_at: Option<jiff::Timestamp>,
    /// Additional metadata for the seat
    pub seat_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSeat {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The seat ID
    pub id: crate::v2027_01::models::common::ids::SeatId,
    /// The subscription ID (for recurring seats)
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2027_01::models::common::ids::SubscriptionId>,
    /// The order ID (for one-time purchase seats)
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub order_id: Option<crate::v2027_01::models::common::ids::OrderId>,
    /// Status of the seat
    pub status: crate::v2027_01::models::common::enums::SeatStatus,
    /// The customer ID. When member_model_enabled is true, this is the billing
    /// customer (purchaser). When false, this is the seat member customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The member ID of the seat occupant
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member_id: Option<crate::v2027_01::models::common::ids::MemberId>,
    /// The member associated with this seat
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member: Option<crate::v2027_01::models::common::types::Member>,
    /// Email of the seat member (set when member_model_enabled is true)
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub email: Option<String>,
    /// The assigned customer email
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    /// When the invitation token expires
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub invitation_token_expires_at: Option<jiff::Timestamp>,
    /// When the seat was claimed
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub claimed_at: Option<jiff::Timestamp>,
    /// When the seat was revoked
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub revoked_at: Option<jiff::Timestamp>,
    /// Additional metadata for the seat
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub seat_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
#[allow(clippy::type_complexity)]
impl CustomerSeat {
    pub fn new(required: CustomerSeatRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            subscription_id: required.subscription_id,
            order_id: required.order_id,
            status: required.status,
            customer_id: required.customer_id,
            member_id: required.member_id,
            member: required.member,
            email: required.email,
            customer_email: required.customer_email,
            invitation_token_expires_at: required.invitation_token_expires_at,
            claimed_at: required.claimed_at,
            revoked_at: required.revoked_at,
            seat_metadata: required.seat_metadata,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2027_01::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2027_01::models::common::enums::SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    /// The start timestamp of the current meter period, if the product has a
    /// meter cycle set. Metered credits are granted and overage is settled on
    /// this cadence.
    pub current_meter_period_start: Option<jiff::Timestamp>,
    /// The end timestamp of the current meter period, if the product has a
    /// meter cycle set. This is when credits next renew.
    pub current_meter_period_end: Option<jiff::Timestamp>,
    /// The start timestamp of the trial period, if any.
    pub trial_start: Option<jiff::Timestamp>,
    /// The end timestamp of the trial period, if any.
    pub trial_end: Option<jiff::Timestamp>,
    /// Whether the subscription will be canceled at the end of the current
    /// period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might
    /// still be active if `cancel_at_period_end` is `true`.
    pub canceled_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription started.
    pub started_at: jiff::Timestamp,
    /// The timestamp when the subscription will end.
    pub ends_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription ended.
    pub ended_at: Option<jiff::Timestamp>,
    /// Whether the subscription will be paused at the end of the current
    /// period.
    pub pause_at_period_end: bool,
    /// The timestamp when the subscription was paused.
    pub paused_at: Option<jiff::Timestamp>,
    /// The timestamp when a paused subscription is scheduled to automatically
    /// resume, if set.
    pub resumes_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    pub discount_id: Option<crate::v2027_01::models::common::ids::DiscountId>,
    pub checkout_id: Option<crate::v2027_01::models::common::ids::CheckoutId>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    pub units: Option<i64>,
    pub customer_cancellation_reason:
        Option<crate::v2027_01::models::common::enums::CustomerCancellationReason>,
    pub customer_cancellation_comment: Option<String>,
    pub product: crate::v2027_01::models::common::types::CustomerSubscriptionProduct,
    /// List of enabled prices for the subscription.
    pub prices: Vec<crate::v2027_01::models::common::unions::ProductPrice>,
    /// List of meters associated with the subscription.
    pub meters: Vec<crate::v2027_01::models::common::types::CustomerSubscriptionMeter>,
    /// Pending subscription update that will be applied at the beginning of the
    /// next period. If `null`, there is no pending update.
    pub pending_update: Option<crate::v2027_01::models::common::types::PendingSubscriptionUpdate>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscription {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::SubscriptionId,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2027_01::models::common::enums::RecurringInterval,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    pub recurring_interval_count: i64,
    /// The status of the subscription.
    pub status: crate::v2027_01::models::common::enums::SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    /// The start timestamp of the current meter period, if the product has a
    /// meter cycle set. Metered credits are granted and overage is settled on
    /// this cadence.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_start: Option<jiff::Timestamp>,
    /// The end timestamp of the current meter period, if the product has a
    /// meter cycle set. This is when credits next renew.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub current_meter_period_end: Option<jiff::Timestamp>,
    /// The start timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_start: Option<jiff::Timestamp>,
    /// The end timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    /// Whether the subscription will be canceled at the end of the current
    /// period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might
    /// still be active if `cancel_at_period_end` is `true`.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub canceled_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription started.
    pub started_at: jiff::Timestamp,
    /// The timestamp when the subscription will end.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription ended.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ended_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription entered `past_due` status.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub past_due_at: presence_rs::Presence<jiff::Timestamp>,
    /// Whether the subscription will be paused at the end of the current
    /// period.
    pub pause_at_period_end: bool,
    /// The timestamp when the subscription was paused.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub paused_at: Option<jiff::Timestamp>,
    /// The timestamp when a paused subscription is scheduled to automatically
    /// resume, if set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub resumes_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed customer.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The ID of the subscribed product.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2027_01::models::common::ids::DiscountId>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checkout_id: Option<crate::v2027_01::models::common::ids::CheckoutId>,
    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_reason:
        Option<crate::v2027_01::models::common::enums::CustomerCancellationReason>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_cancellation_comment: Option<String>,
    pub product: crate::v2027_01::models::common::types::CustomerSubscriptionProduct,
    /// List of enabled prices for the subscription.
    pub prices: Vec<crate::v2027_01::models::common::unions::ProductPrice>,
    /// List of meters associated with the subscription.
    pub meters: Vec<crate::v2027_01::models::common::types::CustomerSubscriptionMeter>,
    /// Pending subscription update that will be applied at the beginning of the
    /// next period. If `null`, there is no pending update.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub pending_update: Option<crate::v2027_01::models::common::types::PendingSubscriptionUpdate>,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscription {
    pub fn new(required: CustomerSubscriptionRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            status: required.status,
            current_period_start: required.current_period_start,
            current_period_end: required.current_period_end,
            current_meter_period_start: required.current_meter_period_start,
            current_meter_period_end: required.current_meter_period_end,
            trial_start: required.trial_start,
            trial_end: required.trial_end,
            cancel_at_period_end: required.cancel_at_period_end,
            canceled_at: required.canceled_at,
            started_at: required.started_at,
            ends_at: required.ends_at,
            ended_at: required.ended_at,
            past_due_at: presence_rs::Presence::Absent,
            pause_at_period_end: required.pause_at_period_end,
            paused_at: required.paused_at,
            resumes_at: required.resumes_at,
            customer_id: required.customer_id,
            product_id: required.product_id,
            discount_id: required.discount_id,
            checkout_id: required.checkout_id,
            seats: presence_rs::Presence::Absent,
            units: required.units,
            customer_cancellation_reason: required.customer_cancellation_reason,
            customer_cancellation_comment: required.customer_cancellation_comment,
            product: required.product,
            prices: required.prices,
            meters: required.meters,
            pending_update: required.pending_update,
        }
    }

    /// The timestamp when the subscription entered `past_due` status.
    #[allow(clippy::should_implement_trait)]
    pub fn past_due_at(mut self, value: jiff::Timestamp) -> Self {
        self.past_due_at = presence_rs::Presence::Some(value);
        self
    }

    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionMeterRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: f64,
    /// The number of credited units so far in this billing period.
    pub credited_units: i64,
    /// The amount due in cents so far in this billing period.
    pub amount: i64,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    pub meter: crate::v2027_01::models::common::types::CustomerSubscriptionMeterMeter,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionMeter {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: f64,
    /// The number of credited units so far in this billing period.
    pub credited_units: i64,
    /// The amount due in cents so far in this billing period.
    pub amount: i64,
    /// The ID of the meter.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    pub meter: crate::v2027_01::models::common::types::CustomerSubscriptionMeterMeter,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionMeter {
    pub fn new(required: CustomerSubscriptionMeterRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            consumed_units: required.consumed_units,
            credited_units: required.credited_units,
            amount: required.amount,
            meter_id: required.meter_id,
            meter: required.meter,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionMeterMeterRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionMeterMeter {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionMeterMeter {
    pub fn new(required: CustomerSubscriptionMeterMeterRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerSubscriptionProductRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2027_01::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2027_01::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// List of prices for this product.
    pub prices: Vec<crate::v2027_01::models::common::unions::ProductPrice>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2027_01::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2027_01::models::common::types::ProductMediaFileRead>,
    pub organization: crate::v2027_01::models::common::types::CustomerOrganization,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerSubscriptionProduct {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2027_01::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2027_01::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// List of prices for this product.
    pub prices: Vec<crate::v2027_01::models::common::unions::ProductPrice>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2027_01::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2027_01::models::common::types::ProductMediaFileRead>,
    pub organization: crate::v2027_01::models::common::types::CustomerOrganization,
}
#[allow(clippy::type_complexity)]
impl CustomerSubscriptionProduct {
    pub fn new(required: CustomerSubscriptionProductRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            trial_interval: required.trial_interval,
            trial_interval_count: required.trial_interval_count,
            name: required.name,
            description: required.description,
            visibility: required.visibility,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            meter_interval: required.meter_interval,
            meter_interval_count: required.meter_interval_count,
            is_recurring: required.is_recurring,
            is_archived: required.is_archived,
            organization_id: required.organization_id,
            prices: required.prices,
            benefits: required.benefits,
            medias: required.medias,
            organization: required.organization,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerTeamRequired {
    /// The ID of the customer.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The name of the customer.
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    pub first_user_event_at: Option<jiff::Timestamp>,
    pub avatar_url: Option<String>,
}
/// A team customer in an organization.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerTeam {
    /// The ID of the customer.
    pub id: crate::v2027_01::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    /// The email address of the customer. This must be unique within the
    /// organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The type of customer. Team customers can have multiple members.
    pub r#type: monostate::MustBe!("team"),
    /// The name of the customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2027_01::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2027_01::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2027_01::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerTeam {
    pub fn new(required: CustomerTeamRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            metadata: required.metadata,
            external_id: presence_rs::Presence::Absent,
            email: presence_rs::Presence::Absent,
            email_verified: required.email_verified,
            r#type: Default::default(),
            name: required.name,
            billing_name: required.billing_name,
            billing_address: required.billing_address,
            tax_id: required.tax_id,
            locale: presence_rs::Presence::Absent,
            organization_id: required.organization_id,
            default_payment_method_id: presence_rs::Presence::Absent,
            deleted_at: required.deleted_at,
            first_user_event_at: required.first_user_event_at,
            avatar_url: required.avatar_url,
        }
    }

    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// The email address of the customer. This must be unique within the
    /// organization.
    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[allow(clippy::should_implement_trait)]
    pub fn default_payment_method_id(
        mut self,
        value: crate::v2027_01::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct DiscountFixedOnceForeverDurationBaseRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountFixedOnceForeverDurationBase {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl DiscountFixedOnceForeverDurationBase {
    pub fn new(required: DiscountFixedOnceForeverDurationBaseRequired) -> Self {
        Self {
            duration: required.duration,
            r#type: required.r#type,
            amounts: required.amounts,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountFixedRepeatDurationBaseRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountFixedRepeatDurationBase {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl DiscountFixedRepeatDurationBase {
    pub fn new(required: DiscountFixedRepeatDurationBaseRequired) -> Self {
        Self {
            duration: required.duration,
            duration_in_months: required.duration_in_months,
            r#type: required.r#type,
            amounts: required.amounts,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountPercentageOnceForeverDurationBaseRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountPercentageOnceForeverDurationBase {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl DiscountPercentageOnceForeverDurationBase {
    pub fn new(required: DiscountPercentageOnceForeverDurationBaseRequired) -> Self {
        Self {
            duration: required.duration,
            r#type: required.r#type,
            basis_points: required.basis_points,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountPercentageRepeatDurationBaseRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountPercentageRepeatDurationBase {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl DiscountPercentageRepeatDurationBase {
    pub fn new(required: DiscountPercentageRepeatDurationBaseRequired) -> Self {
        Self {
            duration: required.duration,
            duration_in_months: required.duration_in_months,
            r#type: required.r#type,
            basis_points: required.basis_points,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct FilterRequired {
    pub conjunction: crate::v2027_01::models::common::enums::FilterConjunction,
    pub clauses: Vec<crate::v2027_01::models::common::unions::FilterClausesItemsUnion>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    pub conjunction: crate::v2027_01::models::common::enums::FilterConjunction,
    pub clauses: Vec<crate::v2027_01::models::common::unions::FilterClausesItemsUnion>,
}
#[allow(clippy::type_complexity)]
impl Filter {
    pub fn new(required: FilterRequired) -> Self {
        Self { conjunction: required.conjunction, clauses: required.clauses }
    }
}
#[derive(Debug, Clone)]
pub struct FilterClauseRequired {
    pub property: String,
    pub operator: crate::v2027_01::models::common::enums::FilterOperator,
    pub value: crate::v2027_01::models::common::unions::FilterClauseValueUnion,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct FilterClause {
    pub property: String,
    pub operator: crate::v2027_01::models::common::enums::FilterOperator,
    pub value: crate::v2027_01::models::common::unions::FilterClauseValueUnion,
}
#[allow(clippy::type_complexity)]
impl FilterClause {
    pub fn new(required: FilterClauseRequired) -> Self {
        Self { property: required.property, operator: required.operator, value: required.value }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default, thiserror::Error)]
#[error("{self:?}")]
pub struct HttpValidationError {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub detail: Option<Vec<crate::v2027_01::models::common::types::ValidationError>>,
}
#[allow(clippy::type_complexity)]
impl HttpValidationError {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn detail(
        mut self,
        value: Vec<crate::v2027_01::models::common::types::ValidationError>,
    ) -> Self {
        self.detail = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MemberRequired {
    /// The ID of the member.
    pub id: crate::v2027_01::models::common::ids::MemberId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer this member belongs to.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The email address of the member.
    pub email: String,
    /// The name of the member.
    pub name: Option<String>,
    /// The ID of the member in your system. This must be unique within the
    /// customer.
    pub external_id: Option<String>,
    /// The role of the member within the customer.
    pub role: crate::v2027_01::models::common::enums::MemberRole,
}
/// A member of a customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Member {
    /// The ID of the member.
    pub id: crate::v2027_01::models::common::ids::MemberId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the customer this member belongs to.
    pub customer_id: crate::v2027_01::models::common::ids::CustomerId,
    /// The email address of the member.
    pub email: String,
    /// The name of the member.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The ID of the member in your system. This must be unique within the
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    /// The role of the member within the customer.
    pub role: crate::v2027_01::models::common::enums::MemberRole,
}
#[allow(clippy::type_complexity)]
impl Member {
    pub fn new(required: MemberRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            customer_id: required.customer_id,
            email: required.email,
            name: required.name,
            external_id: required.external_id,
            role: required.role,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MeterRequired {
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
    /// The unit of the meter.
    pub unit: crate::v2027_01::models::common::enums::MeterUnit,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: crate::v2027_01::models::common::types::Filter,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: crate::v2027_01::models::common::unions::MeterAggregationUnion,
    /// The ID of the organization owning the meter.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Meter {
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
    /// The unit of the meter.
    pub unit: crate::v2027_01::models::common::enums::MeterUnit,
    /// The label for the custom unit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub custom_label: presence_rs::Presence<String>,
    /// The multiplier to convert from base unit to display scale.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub custom_multiplier: presence_rs::Presence<i64>,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: crate::v2027_01::models::common::types::Filter,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: crate::v2027_01::models::common::unions::MeterAggregationUnion,
    /// The ID of the organization owning the meter.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Whether the meter is archived and the time it was archived.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub archived_at: presence_rs::Presence<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl Meter {
    pub fn new(required: MeterRequired) -> Self {
        Self {
            external_id: required.external_id,
            metadata: required.metadata,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            unit: required.unit,
            custom_label: presence_rs::Presence::Absent,
            custom_multiplier: presence_rs::Presence::Absent,
            filter: required.filter,
            aggregation: required.aggregation,
            organization_id: required.organization_id,
            archived_at: presence_rs::Presence::Absent,
        }
    }

    /// The label for the custom unit.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_label(mut self, value: impl Into<String>) -> Self {
        self.custom_label = presence_rs::Presence::Some(value.into());
        self
    }

    /// The multiplier to convert from base unit to display scale.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_multiplier(mut self, value: i64) -> Self {
        self.custom_multiplier = presence_rs::Presence::Some(value);
        self
    }

    /// Whether the meter is archived and the time it was archived.
    #[allow(clippy::should_implement_trait)]
    pub fn archived_at(mut self, value: jiff::Timestamp) -> Self {
        self.archived_at = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MeterCreditEventRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    pub external_customer_id: Option<String>,
    /// Human readable label of the event type.
    pub label: String,
    pub metadata: crate::v2027_01::models::common::types::MeterCreditedMetadata,
}
/// An event created by Polar when credits are added to a customer meter.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterCreditEvent {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::EventId,
    /// The timestamp of the event.
    pub timestamp: jiff::Timestamp,
    /// The ID of the organization owning the event.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// ID of the customer in your Polar organization associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2027_01::models::common::ids::CustomerId>,
    /// The customer associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer: Option<crate::v2027_01::models::common::unions::Customer>,
    /// ID of the customer in your system associated with the event.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub member_id: presence_rs::Presence<crate::v2027_01::models::common::ids::MemberId>,
    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_member_id: presence_rs::Presence<String>,
    /// Number of direct child events linked to this event.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub child_count: Option<i64>,
    /// The ID of the parent event.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub parent_id: presence_rs::Presence<crate::v2027_01::models::common::ids::EventId>,
    /// Human readable label of the event type.
    pub label: String,
    /// The source of the event. `system` events are created by Polar. `user`
    /// events are the one you create through our ingestion API.
    pub source: monostate::MustBe!("system"),
    /// The name of the event.
    pub name: monostate::MustBe!("meter.credited"),
    pub metadata: crate::v2027_01::models::common::types::MeterCreditedMetadata,
}
#[allow(clippy::type_complexity)]
impl MeterCreditEvent {
    pub fn new(required: MeterCreditEventRequired) -> Self {
        Self {
            id: required.id,
            timestamp: required.timestamp,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            external_customer_id: required.external_customer_id,
            member_id: presence_rs::Presence::Absent,
            external_member_id: presence_rs::Presence::Absent,
            child_count: None,
            parent_id: presence_rs::Presence::Absent,
            label: required.label,
            source: Default::default(),
            name: Default::default(),
            metadata: required.metadata,
        }
    }

    /// ID of the member within the customer's organization who performed the
    /// action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn member_id(mut self, value: crate::v2027_01::models::common::ids::MemberId) -> Self {
        self.member_id = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the member in your system within the customer's organization who
    /// performed the action inside B2B.
    #[allow(clippy::should_implement_trait)]
    pub fn external_member_id(mut self, value: impl Into<String>) -> Self {
        self.external_member_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Number of direct child events linked to this event.
    #[allow(clippy::should_implement_trait)]
    pub fn child_count(mut self, value: i64) -> Self {
        self.child_count = Some(value);
        self
    }

    /// The ID of the parent event.
    #[allow(clippy::should_implement_trait)]
    pub fn parent_id(mut self, value: crate::v2027_01::models::common::ids::EventId) -> Self {
        self.parent_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MeterCreditedMetadataRequired {
    pub meter_id: String,
    pub units: i64,
    pub rollover: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MeterCreditedMetadata {
    pub meter_id: String,
    pub units: i64,
    pub rollover: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub reason: Option<crate::v2027_01::models::common::enums::MeterCreditedMetadataReason>,
}
#[allow(clippy::type_complexity)]
impl MeterCreditedMetadata {
    pub fn new(required: MeterCreditedMetadataRequired) -> Self {
        Self {
            meter_id: required.meter_id,
            units: required.units,
            rollover: required.rollover,
            reason: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn reason(
        mut self,
        value: crate::v2027_01::models::common::enums::MeterCreditedMetadataReason,
    ) -> Self {
        self.reason = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct NotPermittedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct NotPermitted {
    pub error: monostate::MustBe!("NotPermitted"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl NotPermitted {
    pub fn new(required: NotPermittedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationCustomerPortalSettingsRequired {
    pub usage: crate::v2027_01::models::common::types::CustomerPortalUsageSettings,
    pub subscription: crate::v2027_01::models::common::types::CustomerPortalSubscriptionSettings,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationCustomerPortalSettings {
    pub usage: crate::v2027_01::models::common::types::CustomerPortalUsageSettings,
    pub subscription: crate::v2027_01::models::common::types::CustomerPortalSubscriptionSettings,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub customer: Option<crate::v2027_01::models::common::types::CustomerPortalCustomerSettings>,
}
#[allow(clippy::type_complexity)]
impl OrganizationCustomerPortalSettings {
    pub fn new(required: OrganizationCustomerPortalSettingsRequired) -> Self {
        Self { usage: required.usage, subscription: required.subscription, customer: None }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer(
        mut self,
        value: crate::v2027_01::models::common::types::CustomerPortalCustomerSettings,
    ) -> Self {
        self.customer = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationNotificationSettingsRequired {
    pub new_order: bool,
    pub new_subscription: bool,
    pub chargeback_prevention: bool,
    pub subscription_renewal: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationNotificationSettings {
    pub new_order: bool,
    pub new_subscription: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub new_trial: Option<bool>,
    pub chargeback_prevention: bool,
    pub subscription_renewal: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub subscription_cancellation: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub exclude_free_products: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl OrganizationNotificationSettings {
    pub fn new(required: OrganizationNotificationSettingsRequired) -> Self {
        Self {
            new_order: required.new_order,
            new_subscription: required.new_subscription,
            new_trial: None,
            chargeback_prevention: required.chargeback_prevention,
            subscription_renewal: required.subscription_renewal,
            subscription_cancellation: None,
            exclude_free_products: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn new_trial(mut self, value: bool) -> Self {
        self.new_trial = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_cancellation(mut self, value: bool) -> Self {
        self.subscription_cancellation = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn exclude_free_products(mut self, value: bool) -> Self {
        self.exclude_free_products = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct PaymentActionRequiredRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentActionRequired {
    pub error: monostate::MustBe!("PaymentActionRequired"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentActionRequired {
    pub fn new(required: PaymentActionRequiredRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentFailedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentFailed {
    pub error: monostate::MustBe!("PaymentFailed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentFailed {
    pub fn new(required: PaymentFailedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentNotReadyRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentNotReady {
    pub error: monostate::MustBe!("PaymentNotReady"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentNotReady {
    pub fn new(required: PaymentNotReadyRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PendingSubscriptionUpdateRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::PendingSubscriptionUpdateId,
    /// The date and time when the subscription update will be applied.
    pub applies_at: jiff::Timestamp,
    /// ID of the new product to apply to the subscription. If `null`, the
    /// product won't be changed.
    pub product_id: Option<crate::v2027_01::models::common::ids::ProductId>,
    /// Number of seats to apply to the subscription. If `null`, the number of
    /// seats won't be changed.
    pub seats: Option<i64>,
    /// Number of units to apply to the subscription. If `null`, the number of
    /// units won't be changed.
    pub units: Option<i64>,
    /// ID of the new discount to apply to the subscription. If `null`, the
    /// subscription keeps its current discount.
    pub discount_id: Option<crate::v2027_01::models::common::ids::DiscountId>,
    /// Whether the subscription's current discount will be removed.
    pub discount_unset: bool,
}
/// Pending update to be applied to a subscription at the beginning of the next
/// period.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PendingSubscriptionUpdate {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::PendingSubscriptionUpdateId,
    /// The date and time when the subscription update will be applied.
    pub applies_at: jiff::Timestamp,
    /// ID of the new product to apply to the subscription. If `null`, the
    /// product won't be changed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_id: Option<crate::v2027_01::models::common::ids::ProductId>,
    /// Number of seats to apply to the subscription. If `null`, the number of
    /// seats won't be changed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub seats: Option<i64>,
    /// Number of units to apply to the subscription. If `null`, the number of
    /// units won't be changed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    /// ID of the new discount to apply to the subscription. If `null`, the
    /// subscription keeps its current discount.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2027_01::models::common::ids::DiscountId>,
    /// Whether the subscription's current discount will be removed.
    pub discount_unset: bool,
}
#[allow(clippy::type_complexity)]
impl PendingSubscriptionUpdate {
    pub fn new(required: PendingSubscriptionUpdateRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            applies_at: required.applies_at,
            product_id: required.product_id,
            seats: required.seats,
            units: required.units,
            discount_id: required.discount_id,
            discount_unset: required.discount_unset,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2027_01::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2027_01::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Whether customers must provide a payment method to start a trial. If
    /// `false`, customers can start a trial without entering payment details.
    pub trial_payment_setup_required: bool,
    /// ID of the catalog that introduced this version of the product. None for
    /// products not managed by declarative config.
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the product.
    pub is_current: bool,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Whether the product can be permanently deleted. Products referenced by
    /// an order, subscription, trial or discount cannot be deleted.
    pub is_deletable: bool,
    /// List of prices for this product.
    pub prices: Vec<crate::v2027_01::models::common::unions::ProductPrice>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2027_01::models::common::unions::Benefit>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2027_01::models::common::types::ProductMediaFileRead>,
    /// List of custom fields attached to the product.
    pub attached_custom_fields: Vec<crate::v2027_01::models::common::types::AttachedCustomField>,
}
/// A product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Product {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2027_01::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2027_01::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    /// Whether customers must provide a payment method to start a trial. If
    /// `false`, customers can start a trial without entering payment details.
    pub trial_payment_setup_required: bool,
    /// ID of the catalog that introduced this version of the product. None for
    /// products not managed by declarative config.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub catalog_id: Option<crate::v2027_01::models::common::ids::CatalogId>,
    /// Whether this is the current version of the product.
    pub is_current: bool,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Whether the product can be permanently deleted. Products referenced by
    /// an order, subscription, trial or discount cannot be deleted.
    pub is_deletable: bool,
    /// List of prices for this product.
    pub prices: Vec<crate::v2027_01::models::common::unions::ProductPrice>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2027_01::models::common::unions::Benefit>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2027_01::models::common::types::ProductMediaFileRead>,
    /// List of custom fields attached to the product.
    pub attached_custom_fields: Vec<crate::v2027_01::models::common::types::AttachedCustomField>,
}
#[allow(clippy::type_complexity)]
impl Product {
    pub fn new(required: ProductRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            trial_interval: required.trial_interval,
            trial_interval_count: required.trial_interval_count,
            name: required.name,
            description: required.description,
            visibility: required.visibility,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: required.recurring_interval_count,
            meter_interval: required.meter_interval,
            meter_interval_count: required.meter_interval_count,
            is_recurring: required.is_recurring,
            is_archived: required.is_archived,
            organization_id: required.organization_id,
            trial_payment_setup_required: required.trial_payment_setup_required,
            catalog_id: required.catalog_id,
            is_current: required.is_current,
            external_id: required.external_id,
            metadata: required.metadata,
            is_deletable: required.is_deletable,
            prices: required.prices,
            benefits: required.benefits,
            medias: required.medias,
            attached_custom_fields: required.attached_custom_fields,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductMediaFileReadRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::FileId,
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<jiff::Timestamp>,
    pub version: Option<String>,
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub size_readable: String,
    pub public_url: String,
}
/// File to be used as a product media file.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductMediaFileRead {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::FileId,
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: i64,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub storage_version: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_etag: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_base64: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub checksum_sha256_hex: Option<String>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_modified_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub version: Option<String>,
    pub service: monostate::MustBe!("product_media"),
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    pub size_readable: String,
    pub public_url: String,
}
#[allow(clippy::type_complexity)]
impl ProductMediaFileRead {
    pub fn new(required: ProductMediaFileReadRequired) -> Self {
        Self {
            id: required.id,
            organization_id: required.organization_id,
            name: required.name,
            path: required.path,
            mime_type: required.mime_type,
            size: required.size,
            storage_version: required.storage_version,
            checksum_etag: required.checksum_etag,
            checksum_sha256_base64: required.checksum_sha256_base64,
            checksum_sha256_hex: required.checksum_sha256_hex,
            last_modified_at: required.last_modified_at,
            version: required.version,
            service: Default::default(),
            is_uploaded: required.is_uploaded,
            created_at: required.created_at,
            size_readable: required.size_readable,
            public_url: required.public_url,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceCustomRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The minimum amount the customer can pay. If 0, the price is 'free or pay
    /// what you want'.
    pub minimum_amount: i64,
    /// The maximum amount the customer can pay.
    pub maximum_amount: Option<i64>,
    /// The initial amount shown to the customer.
    pub preset_amount: Option<i64>,
}
/// A pay-what-you-want price for a product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceCustom {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    pub amount_type: monostate::MustBe!("custom"),
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The minimum amount the customer can pay. If 0, the price is 'free or pay
    /// what you want'.
    pub minimum_amount: i64,
    /// The maximum amount the customer can pay.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub maximum_amount: Option<i64>,
    /// The initial amount shown to the customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub preset_amount: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl ProductPriceCustom {
    pub fn new(required: ProductPriceCustomRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            source: required.source,
            amount_type: Default::default(),
            price_currency: required.price_currency,
            tax_behavior: required.tax_behavior,
            is_archived: required.is_archived,
            product_id: required.product_id,
            minimum_amount: required.minimum_amount,
            maximum_amount: required.maximum_amount,
            preset_amount: required.preset_amount,
        }
    }
}
/// Schema to create a pay-what-you-want price.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct ProductPriceCustomCreate {
    pub amount_type: monostate::MustBe!("custom"),
    /// The currency in which the customer will be charged.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_behavior:
        presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// The minimum amount the customer can pay. If set to 0, the price is 'free
    /// or pay what you want' and $0 is accepted. If set to a value below the
    /// minimum price amount for the currency, it will be rejected. Defaults to
    /// the minimum price amount for the currency. Minimum per currency:
    /// - USD: 0.5
    /// - AED: 2
    /// - ALL: 50
    /// - AMD: 200
    /// - AOA: 500
    /// - ARS: 750
    /// - AUD: 0.7
    /// - AWG: 1
    /// - AZN: 1
    /// - BAM: 1
    /// - BBD: 2
    /// - BDT: 70
    /// - BIF: 2,000
    /// - BMD: 1
    /// - BND: 1
    /// - BOB: 5
    /// - BRL: 2.5
    /// - BSD: 1
    /// - BWP: 10
    /// - BZD: 2
    /// - CAD: 0.7
    /// - CDF: 2,000
    /// - CHF: 0.5
    /// - CLP: 500
    /// - CNY: 5
    /// - COP: 2,000
    /// - CRC: 300
    /// - CVE: 50
    /// - CZK: 15
    /// - DJF: 100
    /// - DKK: 3.2
    /// - DOP: 40
    /// - DZD: 70
    /// - EGP: 30
    /// - ETB: 80
    /// - EUR: 0.5
    /// - FJD: 2
    /// - FKP: 1
    /// - GBP: 0.4
    /// - GEL: 2
    /// - GNF: 5,000
    /// - GIP: 1
    /// - GMD: 40
    /// - GTQ: 5
    /// - GYD: 200
    /// - HKD: 4
    /// - HNL: 20
    /// - HTG: 70
    /// - HUF: 175
    /// - IDR: 9,000
    /// - ILS: 1.5
    /// - INR: 60
    /// - ISK: 70
    /// - JMD: 80
    /// - JPY: 80
    /// - KES: 70
    /// - KGS: 50
    /// - KHR: 3,000
    /// - KMF: 500
    /// - KRW: 800
    /// - KYD: 1
    /// - KZT: 300
    /// - LAK: 20,000
    /// - LKR: 200
    /// - LRD: 100
    /// - LSL: 10
    /// - MAD: 5
    /// - MDL: 10
    /// - MGA: 3,000
    /// - MKD: 50
    /// - MNT: 2,000
    /// - MOP: 5
    /// - MUR: 50
    /// - MVR: 8
    /// - MXN: 9
    /// - MWK: 1,000
    /// - MYR: 2
    /// - MZN: 50
    /// - NAD: 10
    /// - NGN: 700
    /// - NIO: 20
    /// - NOK: 5
    /// - NPR: 80
    /// - NZD: 0.9
    /// - PAB: 1
    /// - PEN: 2
    /// - PGK: 3
    /// - PHP: 35
    /// - PKR: 200
    /// - PLN: 2
    /// - PYG: 4,000
    /// - QAR: 2
    /// - RON: 2.5
    /// - RSD: 60
    /// - RWF: 1,000
    /// - SAR: 2
    /// - SBD: 4
    /// - SCR: 8
    /// - SEK: 5
    /// - SGD: 0.7
    /// - SHP: 1
    /// - SOS: 500
    /// - SRD: 20
    /// - SZL: 10
    /// - THB: 20
    /// - TJS: 5
    /// - TOP: 2
    /// - TRY: 30
    /// - TTD: 4
    /// - TWD: 20
    /// - TZS: 2,000
    /// - UAH: 30
    /// - UGX: 2,000
    /// - UYU: 20
    /// - UZS: 7,000
    /// - VND: 20,000
    /// - VUV: 100
    /// - WST: 2
    /// - XAF: 500
    /// - XCD: 2
    /// - XCG: 1
    /// - XOF: 500
    /// - XPF: 100
    /// - YER: 200
    /// - ZAR: 9
    /// - ZMW: 10
    /// - Other currencies: 50 minor units
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub minimum_amount:
        Option<crate::v2027_01::models::common::newtypes::ProductPriceCustomMinimumAmount>,
    /// The maximum amount the customer can pay. Maximum per currency:
    /// - USD: 999,999.99
    /// - EUR: 999,999.99
    /// - GBP: 999,999.99
    /// - ARS: 1,400,000
    /// - CDF: 2,800,000
    /// - COP: 4,000,000
    /// - IDR: 16,000,000
    /// - KHR: 4,000,000
    /// - LAK: 21,000,000
    /// - MNT: 3,500,000
    /// - MWK: 1,750,000
    /// - NGN: 1,550,000
    /// - TZS: 2,500,000
    /// - UGX: 3,700,000
    /// - UZS: 12,500,000
    /// - Other currencies: 99,999,999 minor units
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub maximum_amount: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::ProductPriceCustomMaximumAmount,
    >,
    /// The initial amount shown to the customer. If 0, the customer will see $0
    /// as the default. If set to a value below the minimum price amount for the
    /// currency, it will be rejected.Minimum per currency:
    /// - USD: 0.5
    /// - AED: 2
    /// - ALL: 50
    /// - AMD: 200
    /// - AOA: 500
    /// - ARS: 750
    /// - AUD: 0.7
    /// - AWG: 1
    /// - AZN: 1
    /// - BAM: 1
    /// - BBD: 2
    /// - BDT: 70
    /// - BIF: 2,000
    /// - BMD: 1
    /// - BND: 1
    /// - BOB: 5
    /// - BRL: 2.5
    /// - BSD: 1
    /// - BWP: 10
    /// - BZD: 2
    /// - CAD: 0.7
    /// - CDF: 2,000
    /// - CHF: 0.5
    /// - CLP: 500
    /// - CNY: 5
    /// - COP: 2,000
    /// - CRC: 300
    /// - CVE: 50
    /// - CZK: 15
    /// - DJF: 100
    /// - DKK: 3.2
    /// - DOP: 40
    /// - DZD: 70
    /// - EGP: 30
    /// - ETB: 80
    /// - EUR: 0.5
    /// - FJD: 2
    /// - FKP: 1
    /// - GBP: 0.4
    /// - GEL: 2
    /// - GNF: 5,000
    /// - GIP: 1
    /// - GMD: 40
    /// - GTQ: 5
    /// - GYD: 200
    /// - HKD: 4
    /// - HNL: 20
    /// - HTG: 70
    /// - HUF: 175
    /// - IDR: 9,000
    /// - ILS: 1.5
    /// - INR: 60
    /// - ISK: 70
    /// - JMD: 80
    /// - JPY: 80
    /// - KES: 70
    /// - KGS: 50
    /// - KHR: 3,000
    /// - KMF: 500
    /// - KRW: 800
    /// - KYD: 1
    /// - KZT: 300
    /// - LAK: 20,000
    /// - LKR: 200
    /// - LRD: 100
    /// - LSL: 10
    /// - MAD: 5
    /// - MDL: 10
    /// - MGA: 3,000
    /// - MKD: 50
    /// - MNT: 2,000
    /// - MOP: 5
    /// - MUR: 50
    /// - MVR: 8
    /// - MXN: 9
    /// - MWK: 1,000
    /// - MYR: 2
    /// - MZN: 50
    /// - NAD: 10
    /// - NGN: 700
    /// - NIO: 20
    /// - NOK: 5
    /// - NPR: 80
    /// - NZD: 0.9
    /// - PAB: 1
    /// - PEN: 2
    /// - PGK: 3
    /// - PHP: 35
    /// - PKR: 200
    /// - PLN: 2
    /// - PYG: 4,000
    /// - QAR: 2
    /// - RON: 2.5
    /// - RSD: 60
    /// - RWF: 1,000
    /// - SAR: 2
    /// - SBD: 4
    /// - SCR: 8
    /// - SEK: 5
    /// - SGD: 0.7
    /// - SHP: 1
    /// - SOS: 500
    /// - SRD: 20
    /// - SZL: 10
    /// - THB: 20
    /// - TJS: 5
    /// - TOP: 2
    /// - TRY: 30
    /// - TTD: 4
    /// - TWD: 20
    /// - TZS: 2,000
    /// - UAH: 30
    /// - UGX: 2,000
    /// - UYU: 20
    /// - UZS: 7,000
    /// - VND: 20,000
    /// - VUV: 100
    /// - WST: 2
    /// - XAF: 500
    /// - XCD: 2
    /// - XCG: 1
    /// - XOF: 500
    /// - XPF: 100
    /// - YER: 200
    /// - ZAR: 9
    /// - ZMW: 10
    /// - Other currencies: 50 minor units
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub preset_amount: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::ProductPriceCustomPresetAmount,
    >,
}
#[allow(clippy::type_complexity)]
impl ProductPriceCustomCreate {
    pub fn new() -> Self {
        Self::default()
    }

    /// The currency in which the customer will be charged.
    #[allow(clippy::should_implement_trait)]
    pub fn price_currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.price_currency = Some(value);
        self
    }

    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn tax_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.tax_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// The minimum amount the customer can pay. If set to 0, the price is 'free
    /// or pay what you want' and $0 is accepted. If set to a value below the
    /// minimum price amount for the currency, it will be rejected. Defaults to
    /// the minimum price amount for the currency. Minimum per currency:
    /// - USD: 0.5
    /// - AED: 2
    /// - ALL: 50
    /// - AMD: 200
    /// - AOA: 500
    /// - ARS: 750
    /// - AUD: 0.7
    /// - AWG: 1
    /// - AZN: 1
    /// - BAM: 1
    /// - BBD: 2
    /// - BDT: 70
    /// - BIF: 2,000
    /// - BMD: 1
    /// - BND: 1
    /// - BOB: 5
    /// - BRL: 2.5
    /// - BSD: 1
    /// - BWP: 10
    /// - BZD: 2
    /// - CAD: 0.7
    /// - CDF: 2,000
    /// - CHF: 0.5
    /// - CLP: 500
    /// - CNY: 5
    /// - COP: 2,000
    /// - CRC: 300
    /// - CVE: 50
    /// - CZK: 15
    /// - DJF: 100
    /// - DKK: 3.2
    /// - DOP: 40
    /// - DZD: 70
    /// - EGP: 30
    /// - ETB: 80
    /// - EUR: 0.5
    /// - FJD: 2
    /// - FKP: 1
    /// - GBP: 0.4
    /// - GEL: 2
    /// - GNF: 5,000
    /// - GIP: 1
    /// - GMD: 40
    /// - GTQ: 5
    /// - GYD: 200
    /// - HKD: 4
    /// - HNL: 20
    /// - HTG: 70
    /// - HUF: 175
    /// - IDR: 9,000
    /// - ILS: 1.5
    /// - INR: 60
    /// - ISK: 70
    /// - JMD: 80
    /// - JPY: 80
    /// - KES: 70
    /// - KGS: 50
    /// - KHR: 3,000
    /// - KMF: 500
    /// - KRW: 800
    /// - KYD: 1
    /// - KZT: 300
    /// - LAK: 20,000
    /// - LKR: 200
    /// - LRD: 100
    /// - LSL: 10
    /// - MAD: 5
    /// - MDL: 10
    /// - MGA: 3,000
    /// - MKD: 50
    /// - MNT: 2,000
    /// - MOP: 5
    /// - MUR: 50
    /// - MVR: 8
    /// - MXN: 9
    /// - MWK: 1,000
    /// - MYR: 2
    /// - MZN: 50
    /// - NAD: 10
    /// - NGN: 700
    /// - NIO: 20
    /// - NOK: 5
    /// - NPR: 80
    /// - NZD: 0.9
    /// - PAB: 1
    /// - PEN: 2
    /// - PGK: 3
    /// - PHP: 35
    /// - PKR: 200
    /// - PLN: 2
    /// - PYG: 4,000
    /// - QAR: 2
    /// - RON: 2.5
    /// - RSD: 60
    /// - RWF: 1,000
    /// - SAR: 2
    /// - SBD: 4
    /// - SCR: 8
    /// - SEK: 5
    /// - SGD: 0.7
    /// - SHP: 1
    /// - SOS: 500
    /// - SRD: 20
    /// - SZL: 10
    /// - THB: 20
    /// - TJS: 5
    /// - TOP: 2
    /// - TRY: 30
    /// - TTD: 4
    /// - TWD: 20
    /// - TZS: 2,000
    /// - UAH: 30
    /// - UGX: 2,000
    /// - UYU: 20
    /// - UZS: 7,000
    /// - VND: 20,000
    /// - VUV: 100
    /// - WST: 2
    /// - XAF: 500
    /// - XCD: 2
    /// - XCG: 1
    /// - XOF: 500
    /// - XPF: 100
    /// - YER: 200
    /// - ZAR: 9
    /// - ZMW: 10
    /// - Other currencies: 50 minor units
    #[allow(clippy::should_implement_trait)]
    pub fn minimum_amount(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceCustomMinimumAmount,
    ) -> Self {
        self.minimum_amount = Some(value);
        self
    }

    /// The maximum amount the customer can pay. Maximum per currency:
    /// - USD: 999,999.99
    /// - EUR: 999,999.99
    /// - GBP: 999,999.99
    /// - ARS: 1,400,000
    /// - CDF: 2,800,000
    /// - COP: 4,000,000
    /// - IDR: 16,000,000
    /// - KHR: 4,000,000
    /// - LAK: 21,000,000
    /// - MNT: 3,500,000
    /// - MWK: 1,750,000
    /// - NGN: 1,550,000
    /// - TZS: 2,500,000
    /// - UGX: 3,700,000
    /// - UZS: 12,500,000
    /// - Other currencies: 99,999,999 minor units
    #[allow(clippy::should_implement_trait)]
    pub fn maximum_amount(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceCustomMaximumAmount,
    ) -> Self {
        self.maximum_amount = presence_rs::Presence::Some(value);
        self
    }

    /// The initial amount shown to the customer. If 0, the customer will see $0
    /// as the default. If set to a value below the minimum price amount for the
    /// currency, it will be rejected.Minimum per currency:
    /// - USD: 0.5
    /// - AED: 2
    /// - ALL: 50
    /// - AMD: 200
    /// - AOA: 500
    /// - ARS: 750
    /// - AUD: 0.7
    /// - AWG: 1
    /// - AZN: 1
    /// - BAM: 1
    /// - BBD: 2
    /// - BDT: 70
    /// - BIF: 2,000
    /// - BMD: 1
    /// - BND: 1
    /// - BOB: 5
    /// - BRL: 2.5
    /// - BSD: 1
    /// - BWP: 10
    /// - BZD: 2
    /// - CAD: 0.7
    /// - CDF: 2,000
    /// - CHF: 0.5
    /// - CLP: 500
    /// - CNY: 5
    /// - COP: 2,000
    /// - CRC: 300
    /// - CVE: 50
    /// - CZK: 15
    /// - DJF: 100
    /// - DKK: 3.2
    /// - DOP: 40
    /// - DZD: 70
    /// - EGP: 30
    /// - ETB: 80
    /// - EUR: 0.5
    /// - FJD: 2
    /// - FKP: 1
    /// - GBP: 0.4
    /// - GEL: 2
    /// - GNF: 5,000
    /// - GIP: 1
    /// - GMD: 40
    /// - GTQ: 5
    /// - GYD: 200
    /// - HKD: 4
    /// - HNL: 20
    /// - HTG: 70
    /// - HUF: 175
    /// - IDR: 9,000
    /// - ILS: 1.5
    /// - INR: 60
    /// - ISK: 70
    /// - JMD: 80
    /// - JPY: 80
    /// - KES: 70
    /// - KGS: 50
    /// - KHR: 3,000
    /// - KMF: 500
    /// - KRW: 800
    /// - KYD: 1
    /// - KZT: 300
    /// - LAK: 20,000
    /// - LKR: 200
    /// - LRD: 100
    /// - LSL: 10
    /// - MAD: 5
    /// - MDL: 10
    /// - MGA: 3,000
    /// - MKD: 50
    /// - MNT: 2,000
    /// - MOP: 5
    /// - MUR: 50
    /// - MVR: 8
    /// - MXN: 9
    /// - MWK: 1,000
    /// - MYR: 2
    /// - MZN: 50
    /// - NAD: 10
    /// - NGN: 700
    /// - NIO: 20
    /// - NOK: 5
    /// - NPR: 80
    /// - NZD: 0.9
    /// - PAB: 1
    /// - PEN: 2
    /// - PGK: 3
    /// - PHP: 35
    /// - PKR: 200
    /// - PLN: 2
    /// - PYG: 4,000
    /// - QAR: 2
    /// - RON: 2.5
    /// - RSD: 60
    /// - RWF: 1,000
    /// - SAR: 2
    /// - SBD: 4
    /// - SCR: 8
    /// - SEK: 5
    /// - SGD: 0.7
    /// - SHP: 1
    /// - SOS: 500
    /// - SRD: 20
    /// - SZL: 10
    /// - THB: 20
    /// - TJS: 5
    /// - TOP: 2
    /// - TRY: 30
    /// - TTD: 4
    /// - TWD: 20
    /// - TZS: 2,000
    /// - UAH: 30
    /// - UGX: 2,000
    /// - UYU: 20
    /// - UZS: 7,000
    /// - VND: 20,000
    /// - VUV: 100
    /// - WST: 2
    /// - XAF: 500
    /// - XCD: 2
    /// - XCG: 1
    /// - XOF: 500
    /// - XPF: 100
    /// - YER: 200
    /// - ZAR: 9
    /// - ZMW: 10
    /// - Other currencies: 50 minor units
    #[allow(clippy::should_implement_trait)]
    pub fn preset_amount(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceCustomPresetAmount,
    ) -> Self {
        self.preset_amount = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceFixedRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The price in cents.
    pub price_amount: i64,
}
/// A fixed price for a product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceFixed {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    pub amount_type: monostate::MustBe!("fixed"),
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The price in cents.
    pub price_amount: i64,
}
#[allow(clippy::type_complexity)]
impl ProductPriceFixed {
    pub fn new(required: ProductPriceFixedRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            source: required.source,
            amount_type: Default::default(),
            price_currency: required.price_currency,
            tax_behavior: required.tax_behavior,
            is_archived: required.is_archived,
            product_id: required.product_id,
            price_amount: required.price_amount,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceFixedCreateRequired {
    /// The price in cents. Set to `0` for a free price.
    /// Minimum amounts per currency:
    /// - USD: 0.5
    /// - AED: 2
    /// - ALL: 50
    /// - AMD: 200
    /// - AOA: 500
    /// - ARS: 750
    /// - AUD: 0.7
    /// - AWG: 1
    /// - AZN: 1
    /// - BAM: 1
    /// - BBD: 2
    /// - BDT: 70
    /// - BIF: 2,000
    /// - BMD: 1
    /// - BND: 1
    /// - BOB: 5
    /// - BRL: 2.5
    /// - BSD: 1
    /// - BWP: 10
    /// - BZD: 2
    /// - CAD: 0.7
    /// - CDF: 2,000
    /// - CHF: 0.5
    /// - CLP: 500
    /// - CNY: 5
    /// - COP: 2,000
    /// - CRC: 300
    /// - CVE: 50
    /// - CZK: 15
    /// - DJF: 100
    /// - DKK: 3.2
    /// - DOP: 40
    /// - DZD: 70
    /// - EGP: 30
    /// - ETB: 80
    /// - EUR: 0.5
    /// - FJD: 2
    /// - FKP: 1
    /// - GBP: 0.4
    /// - GEL: 2
    /// - GNF: 5,000
    /// - GIP: 1
    /// - GMD: 40
    /// - GTQ: 5
    /// - GYD: 200
    /// - HKD: 4
    /// - HNL: 20
    /// - HTG: 70
    /// - HUF: 175
    /// - IDR: 9,000
    /// - ILS: 1.5
    /// - INR: 60
    /// - ISK: 70
    /// - JMD: 80
    /// - JPY: 80
    /// - KES: 70
    /// - KGS: 50
    /// - KHR: 3,000
    /// - KMF: 500
    /// - KRW: 800
    /// - KYD: 1
    /// - KZT: 300
    /// - LAK: 20,000
    /// - LKR: 200
    /// - LRD: 100
    /// - LSL: 10
    /// - MAD: 5
    /// - MDL: 10
    /// - MGA: 3,000
    /// - MKD: 50
    /// - MNT: 2,000
    /// - MOP: 5
    /// - MUR: 50
    /// - MVR: 8
    /// - MXN: 9
    /// - MWK: 1,000
    /// - MYR: 2
    /// - MZN: 50
    /// - NAD: 10
    /// - NGN: 700
    /// - NIO: 20
    /// - NOK: 5
    /// - NPR: 80
    /// - NZD: 0.9
    /// - PAB: 1
    /// - PEN: 2
    /// - PGK: 3
    /// - PHP: 35
    /// - PKR: 200
    /// - PLN: 2
    /// - PYG: 4,000
    /// - QAR: 2
    /// - RON: 2.5
    /// - RSD: 60
    /// - RWF: 1,000
    /// - SAR: 2
    /// - SBD: 4
    /// - SCR: 8
    /// - SEK: 5
    /// - SGD: 0.7
    /// - SHP: 1
    /// - SOS: 500
    /// - SRD: 20
    /// - SZL: 10
    /// - THB: 20
    /// - TJS: 5
    /// - TOP: 2
    /// - TRY: 30
    /// - TTD: 4
    /// - TWD: 20
    /// - TZS: 2,000
    /// - UAH: 30
    /// - UGX: 2,000
    /// - UYU: 20
    /// - UZS: 7,000
    /// - VND: 20,000
    /// - VUV: 100
    /// - WST: 2
    /// - XAF: 500
    /// - XCD: 2
    /// - XCG: 1
    /// - XOF: 500
    /// - XPF: 100
    /// - YER: 200
    /// - ZAR: 9
    /// - ZMW: 10
    /// - Other currencies: 50 minor units
    pub price_amount: crate::v2027_01::models::common::newtypes::ProductPriceFixedPriceAmount,
}
/// Schema to create a fixed price.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceFixedCreate {
    pub amount_type: monostate::MustBe!("fixed"),
    /// The currency in which the customer will be charged.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_behavior:
        presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// The price in cents. Set to `0` for a free price.
    /// Minimum amounts per currency:
    /// - USD: 0.5
    /// - AED: 2
    /// - ALL: 50
    /// - AMD: 200
    /// - AOA: 500
    /// - ARS: 750
    /// - AUD: 0.7
    /// - AWG: 1
    /// - AZN: 1
    /// - BAM: 1
    /// - BBD: 2
    /// - BDT: 70
    /// - BIF: 2,000
    /// - BMD: 1
    /// - BND: 1
    /// - BOB: 5
    /// - BRL: 2.5
    /// - BSD: 1
    /// - BWP: 10
    /// - BZD: 2
    /// - CAD: 0.7
    /// - CDF: 2,000
    /// - CHF: 0.5
    /// - CLP: 500
    /// - CNY: 5
    /// - COP: 2,000
    /// - CRC: 300
    /// - CVE: 50
    /// - CZK: 15
    /// - DJF: 100
    /// - DKK: 3.2
    /// - DOP: 40
    /// - DZD: 70
    /// - EGP: 30
    /// - ETB: 80
    /// - EUR: 0.5
    /// - FJD: 2
    /// - FKP: 1
    /// - GBP: 0.4
    /// - GEL: 2
    /// - GNF: 5,000
    /// - GIP: 1
    /// - GMD: 40
    /// - GTQ: 5
    /// - GYD: 200
    /// - HKD: 4
    /// - HNL: 20
    /// - HTG: 70
    /// - HUF: 175
    /// - IDR: 9,000
    /// - ILS: 1.5
    /// - INR: 60
    /// - ISK: 70
    /// - JMD: 80
    /// - JPY: 80
    /// - KES: 70
    /// - KGS: 50
    /// - KHR: 3,000
    /// - KMF: 500
    /// - KRW: 800
    /// - KYD: 1
    /// - KZT: 300
    /// - LAK: 20,000
    /// - LKR: 200
    /// - LRD: 100
    /// - LSL: 10
    /// - MAD: 5
    /// - MDL: 10
    /// - MGA: 3,000
    /// - MKD: 50
    /// - MNT: 2,000
    /// - MOP: 5
    /// - MUR: 50
    /// - MVR: 8
    /// - MXN: 9
    /// - MWK: 1,000
    /// - MYR: 2
    /// - MZN: 50
    /// - NAD: 10
    /// - NGN: 700
    /// - NIO: 20
    /// - NOK: 5
    /// - NPR: 80
    /// - NZD: 0.9
    /// - PAB: 1
    /// - PEN: 2
    /// - PGK: 3
    /// - PHP: 35
    /// - PKR: 200
    /// - PLN: 2
    /// - PYG: 4,000
    /// - QAR: 2
    /// - RON: 2.5
    /// - RSD: 60
    /// - RWF: 1,000
    /// - SAR: 2
    /// - SBD: 4
    /// - SCR: 8
    /// - SEK: 5
    /// - SGD: 0.7
    /// - SHP: 1
    /// - SOS: 500
    /// - SRD: 20
    /// - SZL: 10
    /// - THB: 20
    /// - TJS: 5
    /// - TOP: 2
    /// - TRY: 30
    /// - TTD: 4
    /// - TWD: 20
    /// - TZS: 2,000
    /// - UAH: 30
    /// - UGX: 2,000
    /// - UYU: 20
    /// - UZS: 7,000
    /// - VND: 20,000
    /// - VUV: 100
    /// - WST: 2
    /// - XAF: 500
    /// - XCD: 2
    /// - XCG: 1
    /// - XOF: 500
    /// - XPF: 100
    /// - YER: 200
    /// - ZAR: 9
    /// - ZMW: 10
    /// - Other currencies: 50 minor units
    pub price_amount: crate::v2027_01::models::common::newtypes::ProductPriceFixedPriceAmount,
}
#[allow(clippy::type_complexity)]
impl ProductPriceFixedCreate {
    pub fn new(required: ProductPriceFixedCreateRequired) -> Self {
        Self {
            amount_type: Default::default(),
            price_currency: None,
            tax_behavior: presence_rs::Presence::Absent,
            price_amount: required.price_amount,
        }
    }

    /// The currency in which the customer will be charged.
    #[allow(clippy::should_implement_trait)]
    pub fn price_currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.price_currency = Some(value);
        self
    }

    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn tax_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.tax_behavior = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceMeterRequired {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter.
    pub name: String,
    /// The unit of the meter.
    pub unit: crate::v2027_01::models::common::enums::MeterUnit,
    /// The label for the custom unit.
    pub custom_label: Option<String>,
    /// The multiplier to convert from base unit to display scale.
    pub custom_multiplier: Option<i64>,
}
/// A meter associated to a metered price.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceMeter {
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::MeterId,
    /// The name of the meter.
    pub name: String,
    /// The unit of the meter.
    pub unit: crate::v2027_01::models::common::enums::MeterUnit,
    /// The label for the custom unit.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub custom_label: Option<String>,
    /// The multiplier to convert from base unit to display scale.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub custom_multiplier: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl ProductPriceMeter {
    pub fn new(required: ProductPriceMeterRequired) -> Self {
        Self {
            id: required.id,
            name: required.name,
            unit: required.unit,
            custom_label: required.custom_label,
            custom_multiplier: required.custom_multiplier,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceMeteredTiersRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    pub cap_amount: Option<i64>,
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The meter associated to the price.
    pub meter: crate::v2027_01::models::common::types::ProductPriceMeter,
    /// The pricing tiers based on consumed units.
    pub tiers: crate::v2027_01::models::common::types::Tiers,
}
/// A metered, usage-based, price for a product, billed from tiers.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceMeteredTiers {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    pub amount_type: monostate::MustBe!("metered_tiers"),
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub cap_amount: Option<i64>,
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The meter associated to the price.
    pub meter: crate::v2027_01::models::common::types::ProductPriceMeter,
    /// The pricing tiers based on consumed units.
    pub tiers: crate::v2027_01::models::common::types::Tiers,
}
#[allow(clippy::type_complexity)]
impl ProductPriceMeteredTiers {
    pub fn new(required: ProductPriceMeteredTiersRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            source: required.source,
            amount_type: Default::default(),
            price_currency: required.price_currency,
            tax_behavior: required.tax_behavior,
            is_archived: required.is_archived,
            product_id: required.product_id,
            cap_amount: required.cap_amount,
            meter_id: required.meter_id,
            meter: required.meter,
            tiers: required.tiers,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceMeteredTiersCreateRequired {
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// Tiered pricing based on consumed units.
    pub tiers: crate::v2027_01::models::common::types::TiersInput,
}
/// Schema to create a metered price billed from tiers on consumed units.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceMeteredTiersCreate {
    pub amount_type: monostate::MustBe!("metered_tiers"),
    /// The currency in which the customer will be charged.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_behavior:
        presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// Tiered pricing based on consumed units.
    pub tiers: crate::v2027_01::models::common::types::TiersInput,
    /// Optional maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cap_amount: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::ProductPriceMeteredTiersCapAmount,
    >,
}
#[allow(clippy::type_complexity)]
impl ProductPriceMeteredTiersCreate {
    pub fn new(required: ProductPriceMeteredTiersCreateRequired) -> Self {
        Self {
            amount_type: Default::default(),
            price_currency: None,
            tax_behavior: presence_rs::Presence::Absent,
            meter_id: required.meter_id,
            tiers: required.tiers,
            cap_amount: presence_rs::Presence::Absent,
        }
    }

    /// The currency in which the customer will be charged.
    #[allow(clippy::should_implement_trait)]
    pub fn price_currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.price_currency = Some(value);
        self
    }

    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn tax_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.tax_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    #[allow(clippy::should_implement_trait)]
    pub fn cap_amount(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceMeteredTiersCapAmount,
    ) -> Self {
        self.cap_amount = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceMeteredUnitRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    pub cap_amount: Option<i64>,
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The meter associated to the price.
    pub meter: crate::v2027_01::models::common::types::ProductPriceMeter,
    /// The price per unit in cents.
    pub unit_amount: String,
}
/// A metered, usage-based, price for a product, with a fixed unit price.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceMeteredUnit {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    pub amount_type: monostate::MustBe!("metered_unit"),
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// The maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub cap_amount: Option<i64>,
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The meter associated to the price.
    pub meter: crate::v2027_01::models::common::types::ProductPriceMeter,
    /// The price per unit in cents.
    pub unit_amount: String,
}
#[allow(clippy::type_complexity)]
impl ProductPriceMeteredUnit {
    pub fn new(required: ProductPriceMeteredUnitRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            source: required.source,
            amount_type: Default::default(),
            price_currency: required.price_currency,
            tax_behavior: required.tax_behavior,
            is_archived: required.is_archived,
            product_id: required.product_id,
            cap_amount: required.cap_amount,
            meter_id: required.meter_id,
            meter: required.meter,
            unit_amount: required.unit_amount,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceMeteredUnitCreateRequired {
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The price per unit in cents. Supports up to 12 decimal places.
    pub unit_amount:
        crate::v2027_01::models::common::unions::ProductPriceMeteredUnitCreateUnitAmountUnion,
}
/// Schema to create a metered price with a fixed unit price.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceMeteredUnitCreate {
    pub amount_type: monostate::MustBe!("metered_unit"),
    /// The currency in which the customer will be charged.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_behavior:
        presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// The ID of the meter associated to the price.
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
    /// The price per unit in cents. Supports up to 12 decimal places.
    pub unit_amount:
        crate::v2027_01::models::common::unions::ProductPriceMeteredUnitCreateUnitAmountUnion,
    /// Optional maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub cap_amount: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::ProductPriceMeteredUnitCapAmount,
    >,
}
#[allow(clippy::type_complexity)]
impl ProductPriceMeteredUnitCreate {
    pub fn new(required: ProductPriceMeteredUnitCreateRequired) -> Self {
        Self {
            amount_type: Default::default(),
            price_currency: None,
            tax_behavior: presence_rs::Presence::Absent,
            meter_id: required.meter_id,
            unit_amount: required.unit_amount,
            cap_amount: presence_rs::Presence::Absent,
        }
    }

    /// The currency in which the customer will be charged.
    #[allow(clippy::should_implement_trait)]
    pub fn price_currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.price_currency = Some(value);
        self
    }

    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn tax_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.tax_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum amount in cents that can be charged, regardless of the
    /// number of units consumed.
    #[allow(clippy::should_implement_trait)]
    pub fn cap_amount(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceMeteredUnitCapAmount,
    ) -> Self {
        self.cap_amount = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceSeatBasedRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// Tiered pricing based on the purchased seat quantity.
    pub tiers: crate::v2027_01::models::common::types::Tiers,
    /// The minimum purchasable seat quantity (inclusive).
    pub minimum_units: Option<i64>,
    /// The maximum purchasable seat quantity, from the last tier's bound.
    /// `null` for unlimited.
    pub maximum_units: Option<i64>,
}
/// A seat-based price for a product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceSeatBased {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    pub amount_type: monostate::MustBe!("seat_based"),
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// Tiered pricing based on the purchased seat quantity.
    pub tiers: crate::v2027_01::models::common::types::Tiers,
    /// The minimum purchasable seat quantity (inclusive).
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub minimum_units: Option<i64>,
    /// The maximum purchasable seat quantity, from the last tier's bound.
    /// `null` for unlimited.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub maximum_units: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl ProductPriceSeatBased {
    pub fn new(required: ProductPriceSeatBasedRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            source: required.source,
            amount_type: Default::default(),
            price_currency: required.price_currency,
            tax_behavior: required.tax_behavior,
            is_archived: required.is_archived,
            product_id: required.product_id,
            tiers: required.tiers,
            minimum_units: required.minimum_units,
            maximum_units: required.maximum_units,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceSeatBasedCreateRequired {
    /// Tiered pricing based on the purchased seat quantity.
    pub tiers: crate::v2027_01::models::common::types::TiersInput,
}
/// Schema to create a seat-based price with volume-based tiers.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceSeatBasedCreate {
    pub amount_type: monostate::MustBe!("seat_based"),
    /// The currency in which the customer will be charged.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_behavior:
        presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Tiered pricing based on the purchased seat quantity.
    pub tiers: crate::v2027_01::models::common::types::TiersInput,
    /// The minimum purchasable seat quantity (inclusive). Defaults to 1 when
    /// not set.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub minimum_units: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::ProductPriceSeatBasedMinimumUnits,
    >,
}
#[allow(clippy::type_complexity)]
impl ProductPriceSeatBasedCreate {
    pub fn new(required: ProductPriceSeatBasedCreateRequired) -> Self {
        Self {
            amount_type: Default::default(),
            price_currency: None,
            tax_behavior: presence_rs::Presence::Absent,
            tiers: required.tiers,
            minimum_units: presence_rs::Presence::Absent,
        }
    }

    /// The currency in which the customer will be charged.
    #[allow(clippy::should_implement_trait)]
    pub fn price_currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.price_currency = Some(value);
        self
    }

    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn tax_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.tax_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// The minimum purchasable seat quantity (inclusive). Defaults to 1 when
    /// not set.
    #[allow(clippy::should_implement_trait)]
    pub fn minimum_units(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceSeatBasedMinimumUnits,
    ) -> Self {
        self.minimum_units = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceUnitBasedRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// Tiered pricing based on the purchased unit quantity.
    pub tiers: crate::v2027_01::models::common::types::Tiers,
    /// The minimum purchasable quantity (inclusive).
    pub minimum_units: Option<i64>,
    /// Per-locale unit nouns shown at checkout and on invoices. `null` defaults
    /// to "unit"/"units".
    pub unit_label:
        Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>>,
    /// The maximum purchasable quantity, from the last tier's bound. `null` for
    /// unlimited.
    pub maximum_units: Option<i64>,
}
/// A unit-based price for a product: the buyer picks a quantity of units,
/// pays for it up-front. On subscriptions, quantity changes are prorated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceUnitBased {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the price.
    pub id: crate::v2027_01::models::common::ids::ProductPriceId,
    /// The source of the price . `catalog` is a predefined price, while
    /// `ad_hoc` is a price created dynamically on a Checkout session.
    pub source: crate::v2027_01::models::common::enums::ProductPriceSource,
    pub amount_type: monostate::MustBe!("unit_based"),
    /// The currency in which the customer will be charged.
    pub price_currency: String,
    /// The tax behavior of the price. If null, it defaults to the
    /// organization's default tax behavior.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: crate::v2027_01::models::common::ids::ProductId,
    /// Tiered pricing based on the purchased unit quantity.
    pub tiers: crate::v2027_01::models::common::types::Tiers,
    /// The minimum purchasable quantity (inclusive).
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub minimum_units: Option<i64>,
    /// Per-locale unit nouns shown at checkout and on invoices. `null` defaults
    /// to "unit"/"units".
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub unit_label:
        Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>>,
    /// The maximum purchasable quantity, from the last tier's bound. `null` for
    /// unlimited.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub maximum_units: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl ProductPriceUnitBased {
    pub fn new(required: ProductPriceUnitBasedRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            source: required.source,
            amount_type: Default::default(),
            price_currency: required.price_currency,
            tax_behavior: required.tax_behavior,
            is_archived: required.is_archived,
            product_id: required.product_id,
            tiers: required.tiers,
            minimum_units: required.minimum_units,
            unit_label: required.unit_label,
            maximum_units: required.maximum_units,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProductPriceUnitBasedCreateRequired {
    /// Tiered pricing based on the purchased unit quantity.
    pub tiers: crate::v2027_01::models::common::types::TiersInput,
}
/// Schema to create a unit-based price: the buyer picks a quantity of units,
/// pays for it up-front. On subscriptions, quantity changes are prorated.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductPriceUnitBasedCreate {
    pub amount_type: monostate::MustBe!("unit_based"),
    /// The currency in which the customer will be charged.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_behavior:
        presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
    /// Tiered pricing based on the purchased unit quantity.
    pub tiers: crate::v2027_01::models::common::types::TiersInput,
    /// The minimum purchasable quantity (inclusive). Defaults to 1 when not
    /// set.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub minimum_units: presence_rs::Presence<
        crate::v2027_01::models::common::newtypes::ProductPriceUnitBasedMinimumUnits,
    >,
    /// Per-locale unit nouns shown at checkout and on invoices. `{"en": {"=1":
    /// "device", "other": "devices"}}`. Defaults to "unit"/"units" when unset.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub unit_label: presence_rs::Presence<
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    >,
}
#[allow(clippy::type_complexity)]
impl ProductPriceUnitBasedCreate {
    pub fn new(required: ProductPriceUnitBasedCreateRequired) -> Self {
        Self {
            amount_type: Default::default(),
            price_currency: None,
            tax_behavior: presence_rs::Presence::Absent,
            tiers: required.tiers,
            minimum_units: presence_rs::Presence::Absent,
            unit_label: presence_rs::Presence::Absent,
        }
    }

    /// The currency in which the customer will be charged.
    #[allow(clippy::should_implement_trait)]
    pub fn price_currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.price_currency = Some(value);
        self
    }

    /// The tax behavior of the price. If not set, it will default to the
    /// organization's default tax behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn tax_behavior(
        mut self,
        value: crate::v2027_01::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.tax_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// The minimum purchasable quantity (inclusive). Defaults to 1 when not
    /// set.
    #[allow(clippy::should_implement_trait)]
    pub fn minimum_units(
        mut self,
        value: crate::v2027_01::models::common::newtypes::ProductPriceUnitBasedMinimumUnits,
    ) -> Self {
        self.minimum_units = presence_rs::Presence::Some(value);
        self
    }

    /// Per-locale unit nouns shown at checkout and on invoices. `{"en": {"=1":
    /// "device", "other": "devices"}}`. Defaults to "unit"/"units" when unset.
    #[allow(clippy::should_implement_trait)]
    pub fn unit_label(
        mut self,
        value: std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
    ) -> Self {
        self.unit_label = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct PropertyAggregationRequired {
    pub func: crate::v2027_01::models::common::enums::PropertyAggregationFunc,
    pub property: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PropertyAggregation {
    pub func: crate::v2027_01::models::common::enums::PropertyAggregationFunc,
    pub property: String,
}
#[allow(clippy::type_complexity)]
impl PropertyAggregation {
    pub fn new(required: PropertyAggregationRequired) -> Self {
        Self { func: required.func, property: required.property }
    }
}
#[derive(Debug, Clone)]
pub struct ResourceNotFoundRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct ResourceNotFound {
    pub error: monostate::MustBe!("ResourceNotFound"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl ResourceNotFound {
    pub fn new(required: ResourceNotFoundRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct SeatsListRequired {
    /// List of seats
    pub seats: Vec<crate::v2027_01::models::common::types::CustomerSeat>,
    /// Number of available seats
    pub available_seats: i64,
    /// Total number of seats for the subscription
    pub total_seats: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct SeatsList {
    /// List of seats
    pub seats: Vec<crate::v2027_01::models::common::types::CustomerSeat>,
    /// Number of available seats
    pub available_seats: i64,
    /// Total number of seats for the subscription
    pub total_seats: i64,
}
#[allow(clippy::type_complexity)]
impl SeatsList {
    pub fn new(required: SeatsListRequired) -> Self {
        Self {
            seats: required.seats,
            available_seats: required.available_seats,
            total_seats: required.total_seats,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TierRequired {
    pub unit_amount: String,
}
/// A per-unit rate up to and including `bound`.
///
/// Each tier starts where the previous one ended. The first starts at
/// zero. `bound` is None on the last tier if it's unbounded. Rates are
/// in cents and may be fractional.
///
/// Rates carry no precision bound: this schema reads stored rows, and a
/// bound tightened later would stop them loading. `TierInput` holds the
/// rules new rates must meet.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Tier {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub bound: presence_rs::Presence<crate::v2027_01::models::common::newtypes::TierBound>,
    pub unit_amount: String,
}
#[allow(clippy::type_complexity)]
impl Tier {
    pub fn new(required: TierRequired) -> Self {
        Self { bound: presence_rs::Presence::Absent, unit_amount: required.unit_amount }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn bound(mut self, value: crate::v2027_01::models::common::newtypes::TierBound) -> Self {
        self.bound = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct TierInputRequired {
    pub unit_amount: crate::v2027_01::models::common::unions::TierInputUnitAmountUnion,
}
/// A tier submitted through the API. Rates stop at the reach of the
/// BigInteger amount columns, with 12 decimal places.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct TierInput {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub bound: presence_rs::Presence<crate::v2027_01::models::common::newtypes::TierInputBound>,
    pub unit_amount: crate::v2027_01::models::common::unions::TierInputUnitAmountUnion,
}
#[allow(clippy::type_complexity)]
impl TierInput {
    pub fn new(required: TierInputRequired) -> Self {
        Self { bound: presence_rs::Presence::Absent, unit_amount: required.unit_amount }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn bound(
        mut self,
        value: crate::v2027_01::models::common::newtypes::TierInputBound,
    ) -> Self {
        self.bound = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct TiersRequired {
    pub r#type: crate::v2027_01::models::common::enums::TierType,
    pub tiers: Vec<crate::v2027_01::models::common::types::Tier>,
}
/// The structure of the shared tiers JSONB column, used by every tiered
/// price type. Purchasable quantity bounds live in the `minimum_units` and
/// `maximum_units` columns, not here.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Tiers {
    pub r#type: crate::v2027_01::models::common::enums::TierType,
    pub tiers: Vec<crate::v2027_01::models::common::types::Tier>,
}
#[allow(clippy::type_complexity)]
impl Tiers {
    pub fn new(required: TiersRequired) -> Self {
        Self { r#type: required.r#type, tiers: required.tiers }
    }
}
#[derive(Debug, Clone)]
pub struct TiersInputRequired {
    pub r#type: crate::v2027_01::models::common::enums::TierType,
    pub tiers: Vec<crate::v2027_01::models::common::types::TierInput>,
}
/// Tiers submitted through the API. Kept apart from `Tiers` so tightening
/// a rule here never stops a stored row from loading.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct TiersInput {
    pub r#type: crate::v2027_01::models::common::enums::TierType,
    pub tiers: Vec<crate::v2027_01::models::common::types::TierInput>,
}
#[allow(clippy::type_complexity)]
impl TiersInput {
    pub fn new(required: TiersInputRequired) -> Self {
        Self { r#type: required.r#type, tiers: required.tiers }
    }
}
#[derive(Debug, Clone)]
pub struct TrialAlreadyRedeemedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct TrialAlreadyRedeemed {
    pub error: monostate::MustBe!("TrialAlreadyRedeemed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl TrialAlreadyRedeemed {
    pub fn new(required: TrialAlreadyRedeemedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct UniqueAggregationRequired {
    pub property: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct UniqueAggregation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub func: Option<monostate::MustBe!("unique")>,
    pub property: String,
}
#[allow(clippy::type_complexity)]
impl UniqueAggregation {
    pub fn new(required: UniqueAggregationRequired) -> Self {
        Self { func: None, property: required.property }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn func(mut self, value: monostate::MustBe!("unique")) -> Self {
        self.func = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ValidationErrorRequired {
    pub loc: Vec<crate::v2027_01::models::common::unions::ValidationErrorLocItemsUnion>,
    pub msg: String,
    pub r#type: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ValidationError {
    pub loc: Vec<crate::v2027_01::models::common::unions::ValidationErrorLocItemsUnion>,
    pub msg: String,
    pub r#type: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub input: presence_rs::Presence<serde_json::Value>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub ctx: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}
#[allow(clippy::type_complexity)]
impl ValidationError {
    pub fn new(required: ValidationErrorRequired) -> Self {
        Self {
            loc: required.loc,
            msg: required.msg,
            r#type: required.r#type,
            input: presence_rs::Presence::Absent,
            ctx: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn input(mut self, value: serde_json::Value) -> Self {
        self.input = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn ctx(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.ctx = Some(value);
        self
    }
}
