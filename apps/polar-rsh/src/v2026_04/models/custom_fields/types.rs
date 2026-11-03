#[derive(Debug, Clone)]
pub struct CustomFieldCreateCheckboxRequired {
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value. Must be unique across the organization.It can only contain ASCII
    /// letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateCheckboxSlug,
    /// Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateCheckboxName,
    pub properties: crate::v2026_04::models::common::types::CustomFieldCheckboxProperties,
}
/// Schema to create a custom field of type checkbox.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldCreateCheckbox {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateCheckboxMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateCheckboxMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("checkbox"),
    ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateCheckboxSlug,
    ///Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateCheckboxName,
    ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    pub properties: crate::v2026_04::models::common::types::CustomFieldCheckboxProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCreateCheckbox {
    pub fn new(required: CustomFieldCreateCheckboxRequired) -> Self {
        Self {
            metadata: None,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            properties: required.properties,
        }
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateCheckboxMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateCheckboxMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the custom field. **Required unless
    /// you use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldCreateDateRequired {
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value. Must be unique across the organization.It can only contain ASCII
    /// letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateDateSlug,
    /// Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateDateName,
    pub properties: crate::v2026_04::models::common::types::CustomFieldDateProperties,
}
/// Schema to create a custom field of type date.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldCreateDate {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateDateMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateDateMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("date"),
    ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateDateSlug,
    ///Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateDateName,
    ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    pub properties: crate::v2026_04::models::common::types::CustomFieldDateProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCreateDate {
    pub fn new(required: CustomFieldCreateDateRequired) -> Self {
        Self {
            metadata: None,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            properties: required.properties,
        }
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateDateMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateDateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the custom field. **Required unless
    /// you use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldCreateNumberRequired {
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value. Must be unique across the organization.It can only contain ASCII
    /// letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateNumberSlug,
    /// Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateNumberName,
    pub properties: crate::v2026_04::models::common::types::CustomFieldNumberProperties,
}
/// Schema to create a custom field of type number.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldCreateNumber {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateNumberMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateNumberMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("number"),
    ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateNumberSlug,
    ///Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateNumberName,
    ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    pub properties: crate::v2026_04::models::common::types::CustomFieldNumberProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCreateNumber {
    pub fn new(required: CustomFieldCreateNumberRequired) -> Self {
        Self {
            metadata: None,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            properties: required.properties,
        }
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateNumberMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateNumberMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the custom field. **Required unless
    /// you use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldCreateSelectRequired {
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value. Must be unique across the organization.It can only contain ASCII
    /// letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateSelectSlug,
    /// Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateSelectName,
    pub properties: crate::v2026_04::models::common::types::CustomFieldSelectProperties,
}
/// Schema to create a custom field of type select.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldCreateSelect {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateSelectMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateSelectMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("select"),
    ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateSelectSlug,
    ///Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateSelectName,
    ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    pub properties: crate::v2026_04::models::common::types::CustomFieldSelectProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCreateSelect {
    pub fn new(required: CustomFieldCreateSelectRequired) -> Self {
        Self {
            metadata: None,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            properties: required.properties,
        }
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateSelectMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateSelectMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the custom field. **Required unless
    /// you use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomFieldCreateTextRequired {
    /// Identifier of the custom field. It'll be used as key when storing the
    /// value. Must be unique across the organization.It can only contain ASCII
    /// letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateTextSlug,
    /// Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateTextName,
    pub properties: crate::v2026_04::models::common::types::CustomFieldTextProperties,
}
/// Schema to create a custom field of type text.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomFieldCreateText {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateTextMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateTextMetadataAdditionalPropertiesUnion,
        >,
    >,
    pub r#type: monostate::MustBe!("text"),
    ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
    pub slug: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateTextSlug,
    ///Name of the custom field.
    pub name: crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateTextName,
    ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    pub properties: crate::v2026_04::models::common::types::CustomFieldTextProperties,
}
#[allow(clippy::type_complexity)]
impl CustomFieldCreateText {
    pub fn new(required: CustomFieldCreateTextRequired) -> Self {
        Self {
            metadata: None,
            r#type: Default::default(),
            slug: required.slug,
            name: required.name,
            organization_id: presence_rs::Presence::Absent,
            properties: required.properties,
        }
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldCreateTextMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldCreateTextMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the organization owning the custom field. **Required unless
    /// you use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update a custom field of type checkbox.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldUpdateCheckbox {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateCheckboxMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateCheckboxMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateCheckboxName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub slug: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateCheckboxSlug,
    >,
    pub r#type: monostate::MustBe!("checkbox"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::CustomFieldCheckboxProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomFieldUpdateCheckbox {
    pub fn new() -> Self {
        Self::default()
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateCheckboxMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateCheckboxMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateCheckboxName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn slug(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateCheckboxSlug,
    ) -> Self {
        self.slug = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2026_04::models::common::types::CustomFieldCheckboxProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update a custom field of type date.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldUpdateDate {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateDateMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateDateMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateDateName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub slug: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateDateSlug,
    >,
    pub r#type: monostate::MustBe!("date"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::CustomFieldDateProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomFieldUpdateDate {
    pub fn new() -> Self {
        Self::default()
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateDateMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateDateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateDateName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn slug(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateDateSlug,
    ) -> Self {
        self.slug = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2026_04::models::common::types::CustomFieldDateProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update a custom field of type number.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldUpdateNumber {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateNumberMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateNumberMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateNumberName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub slug: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateNumberSlug,
    >,
    pub r#type: monostate::MustBe!("number"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::CustomFieldNumberProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomFieldUpdateNumber {
    pub fn new() -> Self {
        Self::default()
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateNumberMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateNumberMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateNumberName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn slug(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateNumberSlug,
    ) -> Self {
        self.slug = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2026_04::models::common::types::CustomFieldNumberProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update a custom field of type select.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldUpdateSelect {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateSelectMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateSelectMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateSelectName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub slug: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateSelectSlug,
    >,
    pub r#type: monostate::MustBe!("select"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::CustomFieldSelectProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomFieldUpdateSelect {
    pub fn new() -> Self {
        Self::default()
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateSelectMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateSelectMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateSelectName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn slug(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateSelectSlug,
    ) -> Self {
        self.slug = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2026_04::models::common::types::CustomFieldSelectProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
/// Schema to update a custom field of type text.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomFieldUpdateText {
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateTextMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateTextMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateTextName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub slug: presence_rs::Presence<
        crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateTextSlug,
    >,
    pub r#type: monostate::MustBe!("text"),
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub properties: presence_rs::Presence<
        crate::v2026_04::models::common::types::CustomFieldTextProperties,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomFieldUpdateText {
    pub fn new() -> Self {
        Self::default()
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
            crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateTextMetadataKey,
            crate::v2026_04::models::custom_fields::unions::CustomFieldUpdateTextMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateTextName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn slug(
        mut self,
        value: crate::v2026_04::models::custom_fields::newtypes::CustomFieldUpdateTextSlug,
    ) -> Self {
        self.slug = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn properties(
        mut self,
        value: crate::v2026_04::models::common::types::CustomFieldTextProperties,
    ) -> Self {
        self.properties = presence_rs::Presence::Some(value);
        self
    }
}
