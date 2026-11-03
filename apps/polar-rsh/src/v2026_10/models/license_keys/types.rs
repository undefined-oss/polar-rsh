#[derive(Debug, Clone)]
pub struct BadRequestRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct BadRequest {
    pub error: monostate::MustBe!("BadRequest"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl BadRequest {
    pub fn new(required: BadRequestRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct GrantedLicenseKeyRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::LicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub limit_activations: Option<i64>,
    pub usage: i64,
    pub limit_usage: Option<i64>,
    pub validations: i64,
    pub last_validated_at: Option<jiff::Timestamp>,
    pub expires_at: Option<jiff::Timestamp>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct GrantedLicenseKey {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::LicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: monostate::MustBe!("granted"),
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_activations: Option<i64>,
    pub usage: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_usage: Option<i64>,
    pub validations: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_validated_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub expires_at: Option<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl GrantedLicenseKey {
    pub fn new(required: GrantedLicenseKeyRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            member_id: required.member_id,
            member: required.member,
            benefit_id: required.benefit_id,
            key: required.key,
            display_key: required.display_key,
            status: Default::default(),
            limit_activations: required.limit_activations,
            usage: required.usage,
            limit_usage: required.limit_usage,
            validations: required.validations,
            last_validated_at: required.last_validated_at,
            expires_at: required.expires_at,
        }
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyActivateRequired {
    pub key: String,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub label: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyActivate {
    pub key: String,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub label: String,
    ///Key-value object allowing you to set conditions that must match when validating the license key.
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
    pub conditions: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::license_keys::newtypes::LicenseKeyActivateConditionsKey,
            crate::v2026_10::models::license_keys::unions::LicenseKeyActivateConditionsAdditionalPropertiesUnion,
        >,
    >,
    ///Key-value object allowing you to store additional information about the activation
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
    pub meta: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::license_keys::newtypes::LicenseKeyActivateMetaKey,
            crate::v2026_10::models::license_keys::unions::LicenseKeyActivateMetaAdditionalPropertiesUnion,
        >,
    >,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyActivate {
    pub fn new(required: LicenseKeyActivateRequired) -> Self {
        Self {
            key: required.key,
            organization_id: required.organization_id,
            label: required.label,
            conditions: None,
            meta: None,
        }
    }

    /// Key-value object allowing you to set conditions that must match when
    /// validating the license key.
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
    pub fn conditions(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_10::models::license_keys::newtypes::LicenseKeyActivateConditionsKey,
            crate::v2026_10::models::license_keys::unions::LicenseKeyActivateConditionsAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Key-value object allowing you to store additional information about the
    /// activation
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
    pub fn meta(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_10::models::license_keys::newtypes::LicenseKeyActivateMetaKey,
            crate::v2026_10::models::license_keys::unions::LicenseKeyActivateMetaAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.meta = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyActivationBaseRequired {
    pub id: crate::v2026_10::models::common::ids::ActivationId,
    pub license_key_id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub label: String,
    pub meta: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::license_keys::unions::LicenseKeyActivationBaseMetaAdditionalPropertiesUnion,
    >,
    pub created_at: jiff::Timestamp,
    pub modified_at: Option<jiff::Timestamp>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyActivationBase {
    pub id: crate::v2026_10::models::common::ids::ActivationId,
    pub license_key_id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub label: String,
    pub meta: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::license_keys::unions::LicenseKeyActivationBaseMetaAdditionalPropertiesUnion,
    >,
    pub created_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyActivationBase {
    pub fn new(required: LicenseKeyActivationBaseRequired) -> Self {
        Self {
            id: required.id,
            license_key_id: required.license_key_id,
            label: required.label,
            meta: required.meta,
            created_at: required.created_at,
            modified_at: required.modified_at,
        }
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyActivationCreatedRequired {
    pub id: crate::v2026_10::models::common::ids::LicenseKeyActivationCreatedId,
    pub license_key_id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub label: String,
    pub meta: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::license_keys::unions::LicenseKeyActivationCreatedMetaAdditionalPropertiesUnion,
    >,
    pub created_at: jiff::Timestamp,
    pub modified_at: Option<jiff::Timestamp>,
    pub license_key: crate::v2026_10::models::license_keys::types::GrantedLicenseKey,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyActivationCreated {
    pub id: crate::v2026_10::models::common::ids::LicenseKeyActivationCreatedId,
    pub license_key_id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub label: String,
    pub meta: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::license_keys::unions::LicenseKeyActivationCreatedMetaAdditionalPropertiesUnion,
    >,
    pub created_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub license_key: crate::v2026_10::models::license_keys::types::GrantedLicenseKey,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyActivationCreated {
    pub fn new(required: LicenseKeyActivationCreatedRequired) -> Self {
        Self {
            id: required.id,
            license_key_id: required.license_key_id,
            label: required.label,
            meta: required.meta,
            created_at: required.created_at,
            modified_at: required.modified_at,
            license_key: required.license_key,
        }
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyActivationReadRequired {
    pub id: crate::v2026_10::models::common::ids::ActivationId,
    pub license_key_id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub label: String,
    pub meta: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::license_keys::unions::LicenseKeyActivationReadMetaAdditionalPropertiesUnion,
    >,
    pub created_at: jiff::Timestamp,
    pub modified_at: Option<jiff::Timestamp>,
    pub license_key: crate::v2026_10::models::license_keys::types::LicenseKeyRead,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyActivationRead {
    pub id: crate::v2026_10::models::common::ids::ActivationId,
    pub license_key_id: crate::v2026_10::models::common::ids::LicenseKeyId,
    pub label: String,
    pub meta: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::license_keys::unions::LicenseKeyActivationReadMetaAdditionalPropertiesUnion,
    >,
    pub created_at: jiff::Timestamp,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub license_key: crate::v2026_10::models::license_keys::types::LicenseKeyRead,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyActivationRead {
    pub fn new(required: LicenseKeyActivationReadRequired) -> Self {
        Self {
            id: required.id,
            license_key_id: required.license_key_id,
            label: required.label,
            meta: required.meta,
            created_at: required.created_at,
            modified_at: required.modified_at,
            license_key: required.license_key,
        }
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyCustomerRequired {
    /// The ID of the customer.
    pub id: crate::v2026_10::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The type of customer: 'individual' for single users, 'team' for
    /// customers with multiple members.
    pub r#type: crate::v2026_10::models::common::enums::CustomerType,
    /// The name of the customer.
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2026_10::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2026_10::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    pub first_user_event_at: Option<jiff::Timestamp>,
    pub avatar_url: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyCustomer {
    /// The ID of the customer.
    pub id: crate::v2026_10::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
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
    /// The type of customer: 'individual' for single users, 'team' for
    /// customers with multiple members.
    pub r#type: crate::v2026_10::models::common::enums::CustomerType,
    /// The name of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2026_10::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2026_10::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2026_10::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyCustomer {
    pub fn new(required: LicenseKeyCustomerRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            metadata: required.metadata,
            external_id: presence_rs::Presence::Absent,
            email: presence_rs::Presence::Absent,
            email_verified: required.email_verified,
            r#type: required.r#type,
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
        value: crate::v2026_10::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyDeactivateRequired {
    pub key: String,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub activation_id: crate::v2026_10::models::common::ids::ActivationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyDeactivate {
    pub key: String,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub activation_id: crate::v2026_10::models::common::ids::ActivationId,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyDeactivate {
    pub fn new(required: LicenseKeyDeactivateRequired) -> Self {
        Self {
            key: required.key,
            organization_id: required.organization_id,
            activation_id: required.activation_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyMemberRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::MemberId,
    /// The email address of the seat member.
    pub email: String,
    /// The external ID of the seat member, if set.
    pub external_id: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyMember {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::MemberId,
    /// The email address of the seat member.
    pub email: String,
    /// The external ID of the seat member, if set.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyMember {
    pub fn new(required: LicenseKeyMemberRequired) -> Self {
        Self { id: required.id, email: required.email, external_id: required.external_id }
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyReadRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::LicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: crate::v2026_10::models::license_keys::enums::LicenseKeyStatus,
    pub limit_activations: Option<i64>,
    pub usage: i64,
    pub limit_usage: Option<i64>,
    pub validations: i64,
    pub last_validated_at: Option<jiff::Timestamp>,
    pub expires_at: Option<jiff::Timestamp>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyRead {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::LicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: crate::v2026_10::models::license_keys::enums::LicenseKeyStatus,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_activations: Option<i64>,
    pub usage: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_usage: Option<i64>,
    pub validations: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_validated_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub expires_at: Option<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyRead {
    pub fn new(required: LicenseKeyReadRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            member_id: required.member_id,
            member: required.member,
            benefit_id: required.benefit_id,
            key: required.key,
            display_key: required.display_key,
            status: required.status,
            limit_activations: required.limit_activations,
            usage: required.usage,
            limit_usage: required.limit_usage,
            validations: required.validations,
            last_validated_at: required.last_validated_at,
            expires_at: required.expires_at,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct LicenseKeyUpdate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub status:
        presence_rs::Presence<crate::v2026_10::models::license_keys::enums::LicenseKeyStatus>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub usage: Option<crate::v2026_10::models::license_keys::newtypes::LicenseKeyUpdateUsage>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub limit_activations: presence_rs::Presence<
        crate::v2026_10::models::license_keys::newtypes::LicenseKeyUpdateLimitActivations,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub limit_usage: presence_rs::Presence<
        crate::v2026_10::models::license_keys::newtypes::LicenseKeyUpdateLimitUsage,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub expires_at: presence_rs::Presence<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn status(
        mut self,
        value: crate::v2026_10::models::license_keys::enums::LicenseKeyStatus,
    ) -> Self {
        self.status = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn usage(
        mut self,
        value: crate::v2026_10::models::license_keys::newtypes::LicenseKeyUpdateUsage,
    ) -> Self {
        self.usage = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn limit_activations(
        mut self,
        value: crate::v2026_10::models::license_keys::newtypes::LicenseKeyUpdateLimitActivations,
    ) -> Self {
        self.limit_activations = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn limit_usage(
        mut self,
        value: crate::v2026_10::models::license_keys::newtypes::LicenseKeyUpdateLimitUsage,
    ) -> Self {
        self.limit_usage = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn expires_at(mut self, value: jiff::Timestamp) -> Self {
        self.expires_at = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyValidateRequired {
    pub key: String,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyValidate {
    pub key: String,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub activation_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ActivationId,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub benefit_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::BenefitId,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::CustomerId,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub increment_usage: presence_rs::Presence<
        crate::v2026_10::models::license_keys::newtypes::LicenseKeyValidateIncrementUsage,
    >,
    ///Key-value object allowing you to set conditions that must match when validating the license key.
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
    pub conditions: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::license_keys::newtypes::LicenseKeyValidateConditionsKey,
            crate::v2026_10::models::license_keys::unions::LicenseKeyValidateConditionsAdditionalPropertiesUnion,
        >,
    >,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyValidate {
    pub fn new(required: LicenseKeyValidateRequired) -> Self {
        Self {
            key: required.key,
            organization_id: required.organization_id,
            activation_id: presence_rs::Presence::Absent,
            benefit_id: presence_rs::Presence::Absent,
            customer_id: presence_rs::Presence::Absent,
            increment_usage: presence_rs::Presence::Absent,
            conditions: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn activation_id(
        mut self,
        value: crate::v2026_10::models::common::ids::ActivationId,
    ) -> Self {
        self.activation_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn benefit_id(mut self, value: crate::v2026_10::models::common::ids::BenefitId) -> Self {
        self.benefit_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_id(mut self, value: crate::v2026_10::models::common::ids::CustomerId) -> Self {
        self.customer_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn increment_usage(
        mut self,
        value: crate::v2026_10::models::license_keys::newtypes::LicenseKeyValidateIncrementUsage,
    ) -> Self {
        self.increment_usage = presence_rs::Presence::Some(value);
        self
    }

    /// Key-value object allowing you to set conditions that must match when
    /// validating the license key.
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
    pub fn conditions(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_10::models::license_keys::newtypes::LicenseKeyValidateConditionsKey,
            crate::v2026_10::models::license_keys::unions::LicenseKeyValidateConditionsAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.conditions = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct LicenseKeyWithActivationsRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::LicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: crate::v2026_10::models::license_keys::enums::LicenseKeyStatus,
    pub limit_activations: Option<i64>,
    pub usage: i64,
    pub limit_usage: Option<i64>,
    pub validations: i64,
    pub last_validated_at: Option<jiff::Timestamp>,
    pub expires_at: Option<jiff::Timestamp>,
    pub activations: Vec<crate::v2026_10::models::license_keys::types::LicenseKeyActivationBase>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct LicenseKeyWithActivations {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::LicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: crate::v2026_10::models::license_keys::enums::LicenseKeyStatus,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_activations: Option<i64>,
    pub usage: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_usage: Option<i64>,
    pub validations: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_validated_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub expires_at: Option<jiff::Timestamp>,
    pub activations: Vec<crate::v2026_10::models::license_keys::types::LicenseKeyActivationBase>,
}
#[allow(clippy::type_complexity)]
impl LicenseKeyWithActivations {
    pub fn new(required: LicenseKeyWithActivationsRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            member_id: required.member_id,
            member: required.member,
            benefit_id: required.benefit_id,
            key: required.key,
            display_key: required.display_key,
            status: required.status,
            limit_activations: required.limit_activations,
            usage: required.usage,
            limit_usage: required.limit_usage,
            validations: required.validations,
            last_validated_at: required.last_validated_at,
            expires_at: required.expires_at,
            activations: required.activations,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RotateNotPermittedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct RotateNotPermitted {
    pub error: monostate::MustBe!("RotateNotPermitted"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl RotateNotPermitted {
    pub fn new(required: RotateNotPermittedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct RotatedLicenseKeyRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::RotatedLicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: crate::v2026_10::models::license_keys::enums::RotatedLicenseKeyStatus,
    pub limit_activations: Option<i64>,
    pub usage: i64,
    pub limit_usage: Option<i64>,
    pub validations: i64,
    pub last_validated_at: Option<jiff::Timestamp>,
    pub expires_at: Option<jiff::Timestamp>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct RotatedLicenseKey {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::RotatedLicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: crate::v2026_10::models::license_keys::enums::RotatedLicenseKeyStatus,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_activations: Option<i64>,
    pub usage: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_usage: Option<i64>,
    pub validations: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_validated_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub expires_at: Option<jiff::Timestamp>,
}
#[allow(clippy::type_complexity)]
impl RotatedLicenseKey {
    pub fn new(required: RotatedLicenseKeyRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            member_id: required.member_id,
            member: required.member,
            benefit_id: required.benefit_id,
            key: required.key,
            display_key: required.display_key,
            status: required.status,
            limit_activations: required.limit_activations,
            usage: required.usage,
            limit_usage: required.limit_usage,
            validations: required.validations,
            last_validated_at: required.last_validated_at,
            expires_at: required.expires_at,
        }
    }
}
#[derive(Debug, Clone)]
pub struct UnauthorizedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct Unauthorized {
    pub error: monostate::MustBe!("Unauthorized"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl Unauthorized {
    pub fn new(required: UnauthorizedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct ValidatedLicenseKeyRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::ValidatedLicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub limit_activations: Option<i64>,
    pub usage: i64,
    pub limit_usage: Option<i64>,
    pub validations: i64,
    pub last_validated_at: Option<jiff::Timestamp>,
    pub expires_at: Option<jiff::Timestamp>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ValidatedLicenseKey {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::ValidatedLicenseKeyId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub customer_id: crate::v2026_10::models::common::ids::CustomerId,
    pub customer: crate::v2026_10::models::license_keys::types::LicenseKeyCustomer,
    /// The ID of the seat member holding this key, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member_id: Option<crate::v2026_10::models::common::ids::MemberId>,
    /// The seat member holding this key. Set for keys granted through a
    /// seat-based product; `null` for keys granted to the customer directly.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub member: Option<crate::v2026_10::models::license_keys::types::LicenseKeyMember>,
    /// The benefit ID.
    pub benefit_id: crate::v2026_10::models::common::ids::BenefitId,
    pub key: String,
    pub display_key: String,
    pub status: monostate::MustBe!("granted"),
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_activations: Option<i64>,
    pub usage: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub limit_usage: Option<i64>,
    pub validations: i64,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last_validated_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub expires_at: Option<jiff::Timestamp>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub activation: presence_rs::Presence<
        crate::v2026_10::models::license_keys::types::LicenseKeyActivationBase,
    >,
}
#[allow(clippy::type_complexity)]
impl ValidatedLicenseKey {
    pub fn new(required: ValidatedLicenseKeyRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            organization_id: required.organization_id,
            customer_id: required.customer_id,
            customer: required.customer,
            member_id: required.member_id,
            member: required.member,
            benefit_id: required.benefit_id,
            key: required.key,
            display_key: required.display_key,
            status: Default::default(),
            limit_activations: required.limit_activations,
            usage: required.usage,
            limit_usage: required.limit_usage,
            validations: required.validations,
            last_validated_at: required.last_validated_at,
            expires_at: required.expires_at,
            activation: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn activation(
        mut self,
        value: crate::v2026_10::models::license_keys::types::LicenseKeyActivationBase,
    ) -> Self {
        self.activation = presence_rs::Presence::Some(value);
        self
    }
}
