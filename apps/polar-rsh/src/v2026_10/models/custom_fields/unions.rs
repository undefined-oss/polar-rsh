#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum CustomFieldCreate {
    #[serde(rename = "text")]
    Text {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateTextMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldCreateTextMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
        slug: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateTextSlug,
        ///Name of the custom field.
        name: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateTextName,
        ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        properties: crate::v2026_10::models::common::types::CustomFieldTextProperties,
    },
    #[serde(rename = "number")]
    Number {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateNumberMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldCreateNumberMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
        slug: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateNumberSlug,
        ///Name of the custom field.
        name: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateNumberName,
        ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        properties: crate::v2026_10::models::common::types::CustomFieldNumberProperties,
    },
    #[serde(rename = "date")]
    Date {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateDateMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldCreateDateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
        slug: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateDateSlug,
        ///Name of the custom field.
        name: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateDateName,
        ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        properties: crate::v2026_10::models::common::types::CustomFieldDateProperties,
    },
    #[serde(rename = "checkbox")]
    Checkbox {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateCheckboxMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldCreateCheckboxMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
        slug: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateCheckboxSlug,
        ///Name of the custom field.
        name: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateCheckboxName,
        ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        properties: crate::v2026_10::models::common::types::CustomFieldCheckboxProperties,
    },
    #[serde(rename = "select")]
    Select {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateSelectMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldCreateSelectMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Identifier of the custom field. It'll be used as key when storing the value. Must be unique across the organization.It can only contain ASCII letters, numbers and hyphens.
        slug: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateSelectSlug,
        ///Name of the custom field.
        name: crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateSelectName,
        ///The ID of the organization owning the custom field. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2026_10::models::common::ids::OrganizationId,
        >,
        properties: crate::v2026_10::models::common::types::CustomFieldSelectProperties,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldCreateCheckboxMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateCheckboxMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldCreateDateMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateDateMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldCreateNumberMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateNumberMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldCreateSelectMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateSelectMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldCreateTextMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldCreateTextMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum CustomFieldUpdate {
    #[serde(rename = "text")]
    Text {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateTextMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldUpdateTextMetadataAdditionalPropertiesUnion,
            >,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        name: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateTextName,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        slug: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateTextSlug,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        properties: presence_rs::Presence<
            crate::v2026_10::models::common::types::CustomFieldTextProperties,
        >,
    },
    #[serde(rename = "number")]
    Number {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateNumberMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldUpdateNumberMetadataAdditionalPropertiesUnion,
            >,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        name: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateNumberName,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        slug: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateNumberSlug,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        properties: presence_rs::Presence<
            crate::v2026_10::models::common::types::CustomFieldNumberProperties,
        >,
    },
    #[serde(rename = "date")]
    Date {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateDateMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldUpdateDateMetadataAdditionalPropertiesUnion,
            >,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        name: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateDateName,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        slug: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateDateSlug,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        properties: presence_rs::Presence<
            crate::v2026_10::models::common::types::CustomFieldDateProperties,
        >,
    },
    #[serde(rename = "checkbox")]
    Checkbox {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateCheckboxMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldUpdateCheckboxMetadataAdditionalPropertiesUnion,
            >,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        name: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateCheckboxName,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        slug: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateCheckboxSlug,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        properties: presence_rs::Presence<
            crate::v2026_10::models::common::types::CustomFieldCheckboxProperties,
        >,
    },
    #[serde(rename = "select")]
    Select {
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
                crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateSelectMetadataKey,
                crate::v2026_10::models::custom_fields::unions::CustomFieldUpdateSelectMetadataAdditionalPropertiesUnion,
            >,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        name: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateSelectName,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        slug: presence_rs::Presence<
            crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateSelectSlug,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        properties: presence_rs::Presence<
            crate::v2026_10::models::common::types::CustomFieldSelectProperties,
        >,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldUpdateCheckboxMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateCheckboxMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldUpdateDateMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateDateMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldUpdateNumberMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateNumberMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldUpdateSelectMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateSelectMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomFieldUpdateTextMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::custom_fields::newtypes::CustomFieldUpdateTextMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
