#[derive(Debug, Clone)]
pub struct BenefitCustomCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitCustomDescription,
    pub properties: crate::v2027_01::models::benefits::types::BenefitCustomCreateProperties,
}
/// Schema to create a benefit of type `custom`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitCustomCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitCustomMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitCustomCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("custom"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitCustomDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitCustomCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitCustomCreate {
    pub fn new(required: BenefitCustomCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitCustomMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitCustomCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
/// Properties for creating a benefit of type `custom`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitCustomCreateProperties {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub note: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl BenefitCustomCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitCustomUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitCustomMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitCustomUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitCustomDescription,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub r#type: monostate::MustBe!("custom"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::common::types::BenefitCustomProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitCustomUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitCustomMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitCustomUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitCustomDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitCustomProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitDiscordDescription,
    pub properties: crate::v2027_01::models::benefits::types::BenefitDiscordCreateProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscordCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDiscordMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDiscordCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("discord"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitDiscordDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitDiscordCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordCreate {
    pub fn new(required: BenefitDiscordCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDiscordMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDiscordCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDiscordCreatePropertiesRequired {
    /// The ID of the Discord server.
    pub guild_id: String,
    /// The ID of the Discord role to grant.
    pub role_id: String,
    /// Whether to kick the member from the Discord server on revocation.
    pub kick_member: bool,
}
/// Properties to create a benefit of type `discord`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDiscordCreateProperties {
    /// The ID of the Discord server.
    pub guild_id: String,
    /// The ID of the Discord role to grant.
    pub role_id: String,
    /// Whether to kick the member from the Discord server on revocation.
    pub kick_member: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordCreateProperties {
    pub fn new(required: BenefitDiscordCreatePropertiesRequired) -> Self {
        Self {
            guild_id: required.guild_id,
            role_id: required.role_id,
            kick_member: required.kick_member,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitDiscordUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDiscordMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDiscordUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitDiscordDescription,
    >,
    pub r#type: monostate::MustBe!("discord"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitDiscordCreateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitDiscordUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDiscordMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDiscordUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitDiscordDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitDiscordCreateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadableFileRequired {
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
    pub flagged_malicious_at: Option<jiff::Timestamp>,
    /// Number of distinct customers or members who downloaded the file.
    pub downloaders: i64,
    /// Total number of downloads for the file.
    pub downloads: i64,
    pub size_readable: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadableFile {
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
    pub service: monostate::MustBe!("downloadable"),
    pub is_uploaded: bool,
    pub created_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub flagged_malicious_at: Option<jiff::Timestamp>,
    /// Number of distinct customers or members who downloaded the file.
    pub downloaders: i64,
    /// Total number of downloads for the file.
    pub downloads: i64,
    pub size_readable: String,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadableFile {
    pub fn new(required: BenefitDownloadableFileRequired) -> Self {
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
            flagged_malicious_at: required.flagged_malicious_at,
            downloaders: required.downloaders,
            downloads: required.downloads,
            size_readable: required.size_readable,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesDescription,
    pub properties: crate::v2027_01::models::benefits::types::BenefitDownloadablesCreateProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadablesCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDownloadablesCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("downloadables"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitDownloadablesCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesCreate {
    pub fn new(required: BenefitDownloadablesCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDownloadablesCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitDownloadablesCreatePropertiesRequired {
    pub files: Vec<crate::v2027_01::models::common::ids::FileId>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitDownloadablesCreateProperties {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub archived: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::common::ids::BenefitDownloadablesCreatePropertiesArchivedId,
            bool,
        >,
    >,
    pub files: Vec<crate::v2027_01::models::common::ids::FileId>,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesCreateProperties {
    pub fn new(required: BenefitDownloadablesCreatePropertiesRequired) -> Self {
        Self { archived: None, files: required.files }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn archived(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::common::ids::BenefitDownloadablesCreatePropertiesArchivedId,
            bool,
        >,
    ) -> Self {
        self.archived = Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitDownloadablesUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDownloadablesUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesDescription,
    >,
    pub r#type: monostate::MustBe!("downloadables"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitDownloadablesCreateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitDownloadablesUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitDownloadablesUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitDownloadablesDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitDownloadablesCreateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitFeatureFlagCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagDescription,
    pub properties: crate::v2027_01::models::benefits::types::BenefitFeatureFlagCreateProperties,
}
/// Schema to create a benefit of type `feature_flag`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitFeatureFlagCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitFeatureFlagCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("feature_flag"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitFeatureFlagCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagCreate {
    pub fn new(required: BenefitFeatureFlagCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitFeatureFlagCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
/// Properties for creating a benefit of type `feature_flag`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitFeatureFlagCreateProperties {}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitFeatureFlagUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitFeatureFlagUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagDescription,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub r#type: monostate::MustBe!("feature_flag"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::common::types::BenefitFeatureFlagProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitFeatureFlagUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitFeatureFlagUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitFeatureFlagDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitFeatureFlagProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositoryCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description:
        crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryDescription,
    pub properties:
        crate::v2027_01::models::benefits::types::BenefitGitHubRepositoryCreateProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepositoryCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitGitHubRepositoryCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("github_repository"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitGitHubRepositoryCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositoryCreate {
    pub fn new(required: BenefitGitHubRepositoryCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitGitHubRepositoryCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitGitHubRepositoryCreatePropertiesRequired {
    /// The owner of the repository.
    pub repository_owner: String,
    /// The name of the repository.
    pub repository_name: String,
    /// The permission level to grant. Read more about roles and their permissions on [GitHub documentation](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization#permissions-for-each-role).
    pub permission:
        crate::v2027_01::models::benefits::enums::BenefitGitHubRepositoryCreatePropertiesPermission,
}
/// Properties to create a benefit of type `github_repository`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitGitHubRepositoryCreateProperties {
    /// The owner of the repository.
    pub repository_owner: String,
    /// The name of the repository.
    pub repository_name: String,
    /// The permission level to grant. Read more about roles and their permissions on [GitHub documentation](https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization#permissions-for-each-role).
    pub permission:
        crate::v2027_01::models::benefits::enums::BenefitGitHubRepositoryCreatePropertiesPermission,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositoryCreateProperties {
    pub fn new(required: BenefitGitHubRepositoryCreatePropertiesRequired) -> Self {
        Self {
            repository_owner: required.repository_owner,
            repository_name: required.repository_name,
            permission: required.permission,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitGitHubRepositoryUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitGitHubRepositoryUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryDescription,
    >,
    pub r#type: monostate::MustBe!("github_repository"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitGitHubRepositoryCreateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitGitHubRepositoryUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitGitHubRepositoryUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitGitHubRepositoryDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitGitHubRepositoryCreateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeyActivationCreatePropertiesRequired {
    pub limit: crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeyActivationCreatePropertiesLimit,
    pub enable_customer_admin: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeyActivationCreateProperties {
    pub limit: crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeyActivationCreatePropertiesLimit,
    pub enable_customer_admin: bool,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeyActivationCreateProperties {
    pub fn new(required: BenefitLicenseKeyActivationCreatePropertiesRequired) -> Self {
        Self { limit: required.limit, enable_customer_admin: required.enable_customer_admin }
    }
}
#[derive(Debug, Clone)]
pub struct BenefitLicenseKeysCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysDescription,
    pub properties: crate::v2027_01::models::benefits::types::BenefitLicenseKeysCreateProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitLicenseKeysCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitLicenseKeysCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("license_keys"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitLicenseKeysCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeysCreate {
    pub fn new(required: BenefitLicenseKeysCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitLicenseKeysCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitLicenseKeysCreateProperties {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub prefix: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub expires: presence_rs::Presence<
        crate::v2027_01::models::common::types::BenefitLicenseKeyExpirationProperties,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub activations: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitLicenseKeyActivationCreateProperties,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub limit_usage: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysCreatePropertiesLimitUsage,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeysCreateProperties {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn prefix(mut self, value: impl Into<String>) -> Self {
        self.prefix = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn expires(
        mut self,
        value: crate::v2027_01::models::common::types::BenefitLicenseKeyExpirationProperties,
    ) -> Self {
        self.expires = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn activations(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitLicenseKeyActivationCreateProperties,
    ) -> Self {
        self.activations = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn limit_usage(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysCreatePropertiesLimitUsage,
    ) -> Self {
        self.limit_usage = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitLicenseKeysUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitLicenseKeysUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysDescription,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub r#type: monostate::MustBe!("license_keys"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitLicenseKeysCreateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitLicenseKeysUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitLicenseKeysUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitLicenseKeysDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitLicenseKeysCreateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditDescription,
    pub properties: crate::v2027_01::models::benefits::types::BenefitMeterCreditCreateProperties,
}
/// Schema to create a benefit of type `meter_unit`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitMeterCreditCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("meter_credit"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitMeterCreditCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditCreate {
    pub fn new(required: BenefitMeterCreditCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitMeterCreditCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitMeterCreditCreatePropertiesRequired {
    pub units: crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditCreatePropertiesUnits,
    pub rollover: bool,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
/// Properties for creating a benefit of type `meter_unit`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitMeterCreditCreateProperties {
    pub units: crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditCreatePropertiesUnits,
    pub rollover: bool,
    pub meter_id: crate::v2027_01::models::common::ids::MeterId,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditCreateProperties {
    pub fn new(required: BenefitMeterCreditCreatePropertiesRequired) -> Self {
        Self { units: required.units, rollover: required.rollover, meter_id: required.meter_id }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitMeterCreditUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitMeterCreditUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditDescription,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub r#type: monostate::MustBe!("meter_credit"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitMeterCreditCreateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitMeterCreditUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitMeterCreditUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitMeterCreditDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitMeterCreditCreateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSlackSharedChannelCreateRequired {
    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    pub description:
        crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelDescription,
    pub properties:
        crate::v2027_01::models::benefits::types::BenefitSlackSharedChannelCreateProperties,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSlackSharedChannelCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitSlackSharedChannelCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("slack_shared_channel"),
    ///The description of the benefit. Will be displayed on products having this benefit.
    pub description: crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelDescription,
    ///The ID of the organization owning the benefit. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The visibility of the benefit in the customer portal.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::BenefitVisibility,
    >,
    pub properties: crate::v2027_01::models::benefits::types::BenefitSlackSharedChannelCreateProperties,
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelCreate {
    pub fn new(required: BenefitSlackSharedChannelCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            r#type: Default::default(),
            description: required.description,
            organization_id: presence_rs::Presence::Absent,
            visibility: presence_rs::Presence::Absent,
            properties: required.properties,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitSlackSharedChannelCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the benefit. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the benefit in the customer portal.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::BenefitVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct BenefitSlackSharedChannelCreatePropertiesRequired {
    ///Polar Slack integration to use for this benefit.
    pub slack_integration_id: crate::v2027_01::models::common::ids::SlackIntegrationId,
    pub channel_name_template: crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct BenefitSlackSharedChannelCreateProperties {
    ///Polar Slack integration to use for this benefit.
    pub slack_integration_id: crate::v2027_01::models::common::ids::SlackIntegrationId,
    pub channel_name_template: crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelCreatePropertiesChannelNameTemplate,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub welcome_message: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelCreatePropertiesWelcomeMessage,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub archive_on_revoke: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub team_invitees: Option<Vec<String>>,
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelCreateProperties {
    pub fn new(required: BenefitSlackSharedChannelCreatePropertiesRequired) -> Self {
        Self {
            slack_integration_id: required.slack_integration_id,
            channel_name_template: required.channel_name_template,
            private: None,
            welcome_message: presence_rs::Presence::Absent,
            archive_on_revoke: None,
            team_invitees: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn private(mut self, value: bool) -> Self {
        self.private = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn welcome_message(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelCreatePropertiesWelcomeMessage,
    ) -> Self {
        self.welcome_message = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn archive_on_revoke(mut self, value: bool) -> Self {
        self.archive_on_revoke = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn team_invitees(mut self, value: Vec<String>) -> Self {
        self.team_invitees = Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct BenefitSlackSharedChannelUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitSlackSharedChannelUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The description of the benefit. Will be displayed on products having this benefit.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<
        crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelDescription,
    >,
    pub r#type: monostate::MustBe!("slack_shared_channel"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2027_01::models::benefits::types::BenefitSlackSharedChannelCreateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl BenefitSlackSharedChannelUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information.
    ///
    /// The key must be a string with a maximum length of **40 characters**.
    /// The value must be either:
    ///
    /// * A string with a maximum length of **500 characters**
    /// * An integer
    /// * A floating-point number
    /// * A boolean
    ///
    /// You can store up to **50 key-value pairs**.
    #[allow(clippy::should_implement_trait)]
    pub fn metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelMetadataKey,
            crate::v2027_01::models::benefits::unions::BenefitSlackSharedChannelUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the benefit. Will be displayed on products having
    /// this benefit.
    #[allow(clippy::should_implement_trait)]
    pub fn description(
        mut self,
        value: crate::v2027_01::models::benefits::newtypes::BenefitSlackSharedChannelDescription,
    ) -> Self {
        self.description = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2027_01::models::benefits::types::BenefitSlackSharedChannelCreateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
