#[derive(Debug, Clone)]
pub struct CustomerEmailUpdateRequestRequired {
    pub email: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerEmailUpdateRequest {
    pub email: String,
}
#[allow(clippy::type_complexity)]
impl CustomerEmailUpdateRequest {
    pub fn new(required: CustomerEmailUpdateRequestRequired) -> Self {
        Self { email: required.email }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerEmailUpdateVerifyRequestRequired {
    pub token: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerEmailUpdateVerifyRequest {
    pub token: String,
}
#[allow(clippy::type_complexity)]
impl CustomerEmailUpdateVerifyRequest {
    pub fn new(required: CustomerEmailUpdateVerifyRequestRequired) -> Self {
        Self { token: required.token }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerEmailUpdateVerifyResponseRequired {
    pub token: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerEmailUpdateVerifyResponse {
    pub token: String,
}
#[allow(clippy::type_complexity)]
impl CustomerEmailUpdateVerifyResponse {
    pub fn new(required: CustomerEmailUpdateVerifyResponseRequired) -> Self {
        Self { token: required.token }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerIndividualCreateRequired {
    /// The email address of the customer. This must be unique within the
    /// organization.
    pub email: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerIndividualCreate {
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
            crate::v2026_04::models::customers::newtypes::CustomerIndividualMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerIndividualCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The ID of the customer in your system. This must be unique within the organization. Once set, it can't be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::customers::newtypes::CustomerIndividualName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address: presence_rs::Presence<
        crate::v2026_04::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///The ID of the organization owning the customer. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    ///Optional owner member to create with the customer. If not provided, an owner member will be automatically created using the customer's email and name.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub owner: presence_rs::Presence<
        crate::v2026_04::models::customers::types::MemberOwnerCreate,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub r#type: Option<monostate::MustBe!("individual")>,
    ///The email address of the customer. This must be unique within the organization.
    pub email: String,
}
#[allow(clippy::type_complexity)]
impl CustomerIndividualCreate {
    pub fn new(required: CustomerIndividualCreateRequired) -> Self {
        Self {
            metadata: None,
            external_id: presence_rs::Presence::Absent,
            name: presence_rs::Presence::Absent,
            billing_address: presence_rs::Presence::Absent,
            tax_id: presence_rs::Presence::Absent,
            locale: presence_rs::Presence::Absent,
            organization_id: presence_rs::Presence::Absent,
            owner: presence_rs::Presence::Absent,
            r#type: None,
            email: required.email,
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
            crate::v2026_04::models::customers::newtypes::CustomerIndividualMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerIndividualCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::customers::newtypes::CustomerIndividualName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the organization owning the customer. **Required unless you
    /// use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// Optional owner member to create with the customer. If not provided, an
    /// owner member will be automatically created using the customer's email
    /// and name.
    #[allow(clippy::should_implement_trait)]
    pub fn owner(
        mut self,
        value: crate::v2026_04::models::customers::types::MemberOwnerCreate,
    ) -> Self {
        self.owner = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn r#type(mut self, value: monostate::MustBe!("individual")) -> Self {
        self.r#type = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerNotReadyRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct CustomerNotReady {
    pub error: monostate::MustBe!("CustomerNotReady"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl CustomerNotReady {
    pub fn new(required: CustomerNotReadyRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodCardRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodCardMetadata,
    /// Whether this payment method is the customer's default payment method.
    pub is_default: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodCard {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: monostate::MustBe!("card"),
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodCardMetadata,
    /// Whether this payment method is the customer's default payment method.
    pub is_default: bool,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodCard {
    pub fn new(required: CustomerPaymentMethodCardRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            processor: required.processor,
            customer_id: required.customer_id,
            r#type: Default::default(),
            method_metadata: required.method_metadata,
            is_default: required.is_default,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodConfirmRequired {
    pub setup_intent_id: String,
    pub set_default: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodConfirm {
    pub setup_intent_id: String,
    pub set_default: bool,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodConfirm {
    pub fn new(required: CustomerPaymentMethodConfirmRequired) -> Self {
        Self { setup_intent_id: required.setup_intent_id, set_default: required.set_default }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodCreateRequired {
    pub confirmation_token_id: String,
    pub set_default: bool,
    pub return_url: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodCreate {
    pub confirmation_token_id: String,
    pub set_default: bool,
    pub return_url: String,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodCreate {
    pub fn new(required: CustomerPaymentMethodCreateRequired) -> Self {
        Self {
            confirmation_token_id: required.confirmation_token_id,
            set_default: required.set_default,
            return_url: required.return_url,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodCreateRequiresActionResponseRequired {
    pub client_secret: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodCreateRequiresActionResponse {
    pub status: monostate::MustBe!("requires_action"),
    pub client_secret: String,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodCreateRequiresActionResponse {
    pub fn new(required: CustomerPaymentMethodCreateRequiresActionResponseRequired) -> Self {
        Self { status: Default::default(), client_secret: required.client_secret }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodCreateSucceededResponseRequired {
    pub payment_method: crate::v2026_04::models::customers::unions::CustomerPaymentMethod,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodCreateSucceededResponse {
    pub status: monostate::MustBe!("succeeded"),
    pub payment_method: crate::v2026_04::models::customers::unions::CustomerPaymentMethod,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodCreateSucceededResponse {
    pub fn new(required: CustomerPaymentMethodCreateSucceededResponseRequired) -> Self {
        Self { status: Default::default(), payment_method: required.payment_method }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodGenericRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: String,
    /// Whether this payment method is the customer's default payment method.
    pub is_default: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodGeneric {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: String,
    /// Whether this payment method is the customer's default payment method.
    pub is_default: bool,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodGeneric {
    pub fn new(required: CustomerPaymentMethodGenericRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            processor: required.processor,
            customer_id: required.customer_id,
            r#type: required.r#type,
            is_default: required.is_default,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerPaymentMethodKrCardRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodKrCardMetadata,
    /// Whether this payment method is the customer's default payment method.
    pub is_default: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerPaymentMethodKrCard {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: monostate::MustBe!("kr_card"),
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodKrCardMetadata,
    /// Whether this payment method is the customer's default payment method.
    pub is_default: bool,
}
#[allow(clippy::type_complexity)]
impl CustomerPaymentMethodKrCard {
    pub fn new(required: CustomerPaymentMethodKrCardRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            processor: required.processor,
            customer_id: required.customer_id,
            r#type: Default::default(),
            method_metadata: required.method_metadata,
            is_default: required.is_default,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerPortalCustomerUpdate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_name: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address:
        presence_rs::Presence<crate::v2026_04::models::common::types::AddressInput>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::DefaultPaymentMethodId>,
}
#[allow(clippy::type_complexity)]
impl CustomerPortalCustomerUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_name(mut self, value: impl Into<String>) -> Self {
        self.billing_name = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn default_payment_method_id(
        mut self,
        value: crate::v2026_04::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerStateBenefitGrantRequired {
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::CustomerStateBenefitGrantId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The timestamp when the benefit was granted.
    pub granted_at: jiff::Timestamp,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The type of the benefit concerned by this grant.
    pub benefit_type: crate::v2026_04::models::common::enums::BenefitType,
    /// The metadata of the benefit concerned by this grant.
    pub benefit_metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    pub properties:
        crate::v2026_04::models::customers::unions::CustomerStateBenefitGrantPropertiesUnion,
}
/// An active benefit grant for a customer.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerStateBenefitGrant {
    /// The ID of the grant.
    pub id: crate::v2026_04::models::common::ids::CustomerStateBenefitGrantId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The timestamp when the benefit was granted.
    pub granted_at: jiff::Timestamp,
    /// The ID of the benefit concerned by this grant.
    pub benefit_id: crate::v2026_04::models::common::ids::BenefitId,
    /// The type of the benefit concerned by this grant.
    pub benefit_type: crate::v2026_04::models::common::enums::BenefitType,
    /// The metadata of the benefit concerned by this grant.
    pub benefit_metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    pub properties:
        crate::v2026_04::models::customers::unions::CustomerStateBenefitGrantPropertiesUnion,
}
#[allow(clippy::type_complexity)]
impl CustomerStateBenefitGrant {
    pub fn new(required: CustomerStateBenefitGrantRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            granted_at: required.granted_at,
            benefit_id: required.benefit_id,
            benefit_type: required.benefit_type,
            benefit_metadata: required.benefit_metadata,
            properties: required.properties,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerStateIndividualRequired {
    /// The ID of the customer.
    pub id: crate::v2026_04::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
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
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    pub first_user_event_at: Option<jiff::Timestamp>,
    pub avatar_url: Option<String>,
    /// The customer's active subscriptions.
    pub active_subscriptions:
        Vec<crate::v2026_04::models::customers::types::CustomerStateSubscription>,
    /// The customer's active benefit grants.
    pub granted_benefits: Vec<crate::v2026_04::models::customers::types::CustomerStateBenefitGrant>,
    /// The customer's active meters.
    pub active_meters: Vec<crate::v2026_04::models::customers::types::CustomerStateMeter>,
}
/// A customer along with additional state information:
///
/// * Active subscriptions
/// * Granted benefits
/// * Active meters
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerStateIndividual {
    /// The ID of the customer.
    pub id: crate::v2026_04::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
    /// The customer's active subscriptions.
    pub active_subscriptions:
        Vec<crate::v2026_04::models::customers::types::CustomerStateSubscription>,
    /// The customer's active benefit grants.
    pub granted_benefits: Vec<crate::v2026_04::models::customers::types::CustomerStateBenefitGrant>,
    /// The customer's active meters.
    pub active_meters: Vec<crate::v2026_04::models::customers::types::CustomerStateMeter>,
}
#[allow(clippy::type_complexity)]
impl CustomerStateIndividual {
    pub fn new(required: CustomerStateIndividualRequired) -> Self {
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
            active_subscriptions: required.active_subscriptions,
            granted_benefits: required.granted_benefits,
            active_meters: required.active_meters,
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
        value: crate::v2026_04::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerStateMeterRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::CustomerStateMeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the meter.
    pub meter_id: crate::v2026_04::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
}
/// An active meter for a customer, with latest consumed and credited units.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerStateMeter {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::CustomerStateMeterId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the meter.
    pub meter_id: crate::v2026_04::models::common::ids::MeterId,
    /// The number of consumed units.
    pub consumed_units: f64,
    /// The number of credited units.
    pub credited_units: i64,
    /// The balance of the meter, i.e. the difference between credited and
    /// consumed units.
    pub balance: f64,
}
#[allow(clippy::type_complexity)]
impl CustomerStateMeter {
    pub fn new(required: CustomerStateMeterRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            meter_id: required.meter_id,
            consumed_units: required.consumed_units,
            credited_units: required.credited_units,
            balance: required.balance,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerStateSubscriptionRequired {
    /// The ID of the subscription.
    pub id: crate::v2026_04::models::common::ids::SubscriptionId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    pub status: crate::v2026_04::models::customers::enums::CustomerStateSubscriptionStatus,
    /// The amount of the subscription.
    pub amount: i64,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: crate::v2026_04::models::common::enums::RecurringInterval,
    /// The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    /// The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
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
    pub started_at: Option<jiff::Timestamp>,
    /// The timestamp when the subscription will end.
    pub ends_at: Option<jiff::Timestamp>,
    /// The ID of the subscribed product.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    /// The ID of the applied discount, if any.
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    /// The number of seats for seat-based subscriptions. None for non-seat
    /// subscriptions.
    pub seats: Option<i64>,
    /// The number of units for unit-based subscriptions. None for non-unit
    /// subscriptions.
    pub units: Option<i64>,
    /// List of meters associated with the subscription.
    pub meters: Vec<crate::v2026_04::models::customers::types::CustomerStateSubscriptionMeter>,
}
/// An active customer subscription.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerStateSubscription {
    ///The ID of the subscription.
    pub id: crate::v2026_04::models::common::ids::SubscriptionId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_04::models::customers::unions::CustomerStateSubscriptionCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    pub status: crate::v2026_04::models::customers::enums::CustomerStateSubscriptionStatus,
    ///The amount of the subscription.
    pub amount: i64,
    ///The currency of the subscription.
    pub currency: String,
    ///The interval at which the subscription recurs.
    pub recurring_interval: crate::v2026_04::models::common::enums::RecurringInterval,
    ///The start timestamp of the current billing period.
    pub current_period_start: jiff::Timestamp,
    ///The end timestamp of the current billing period.
    pub current_period_end: jiff::Timestamp,
    ///The start timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_start: Option<jiff::Timestamp>,
    ///The end timestamp of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    ///Whether the subscription will be canceled at the end of the current period.
    pub cancel_at_period_end: bool,
    ///The timestamp when the subscription was canceled. The subscription might still be active if `cancel_at_period_end` is `true`.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub canceled_at: Option<jiff::Timestamp>,
    ///The timestamp when the subscription started.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub started_at: Option<jiff::Timestamp>,
    ///The timestamp when the subscription will end.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    ///The ID of the subscribed product.
    pub product_id: crate::v2026_04::models::common::ids::ProductId,
    ///The ID of the applied discount, if any.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_04::models::common::ids::DiscountId>,
    ///The number of seats for seat-based subscriptions. None for non-seat subscriptions.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub seats: Option<i64>,
    ///The number of units for unit-based subscriptions. None for non-unit subscriptions.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    ///List of meters associated with the subscription.
    pub meters: Vec<
        crate::v2026_04::models::customers::types::CustomerStateSubscriptionMeter,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomerStateSubscription {
    pub fn new(required: CustomerStateSubscriptionRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            custom_field_data: None,
            metadata: required.metadata,
            status: required.status,
            amount: required.amount,
            currency: required.currency,
            recurring_interval: required.recurring_interval,
            current_period_start: required.current_period_start,
            current_period_end: required.current_period_end,
            trial_start: required.trial_start,
            trial_end: required.trial_end,
            cancel_at_period_end: required.cancel_at_period_end,
            canceled_at: required.canceled_at,
            started_at: required.started_at,
            ends_at: required.ends_at,
            product_id: required.product_id,
            discount_id: required.discount_id,
            seats: required.seats,
            units: required.units,
            meters: required.meters,
        }
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_04::models::customers::unions::CustomerStateSubscriptionCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CustomerStateSubscriptionMeterRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::MeterId,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: f64,
    /// The number of credited units so far in this billing period.
    pub credited_units: i64,
    /// The amount due in cents so far in this billing period.
    pub amount: i64,
    /// The ID of the meter.
    pub meter_id: crate::v2026_04::models::common::ids::MeterId,
}
/// Current consumption and spending for a subscription meter.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerStateSubscriptionMeter {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::MeterId,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: f64,
    /// The number of credited units so far in this billing period.
    pub credited_units: i64,
    /// The amount due in cents so far in this billing period.
    pub amount: i64,
    /// The ID of the meter.
    pub meter_id: crate::v2026_04::models::common::ids::MeterId,
}
#[allow(clippy::type_complexity)]
impl CustomerStateSubscriptionMeter {
    pub fn new(required: CustomerStateSubscriptionMeterRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            consumed_units: required.consumed_units,
            credited_units: required.credited_units,
            amount: required.amount,
            meter_id: required.meter_id,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerStateTeamRequired {
    /// The ID of the customer.
    pub id: crate::v2026_04::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
    /// Whether the customer email address is verified. The address is
    /// automatically verified when the customer accesses the customer portal
    /// using their email address.
    pub email_verified: bool,
    /// The name of the customer.
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    pub billing_name: Option<String>,
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    pub tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    pub first_user_event_at: Option<jiff::Timestamp>,
    pub avatar_url: Option<String>,
    /// The customer's active subscriptions.
    pub active_subscriptions:
        Vec<crate::v2026_04::models::customers::types::CustomerStateSubscription>,
    /// The customer's active benefit grants.
    pub granted_benefits: Vec<crate::v2026_04::models::customers::types::CustomerStateBenefitGrant>,
    /// The customer's active meters.
    pub active_meters: Vec<crate::v2026_04::models::customers::types::CustomerStateMeter>,
}
/// A team customer along with additional state information:
///
/// * Active subscriptions
/// * Granted benefits
/// * Active meters
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerStateTeam {
    /// The ID of the customer.
    pub id: crate::v2026_04::models::common::ids::CustomerId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
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
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub name: Option<String>,
    /// The name that should appear on the customer's invoices. Falls back to
    /// the customer name when not explicitly set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub billing_address: Option<crate::v2026_04::models::common::types::Address>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    /// The ID of the organization owning the customer.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// The ID of the customer's default payment method, if any. Use the payment
    /// methods endpoint to retrieve its details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_payment_method_id:
        presence_rs::Presence<crate::v2026_04::models::common::ids::DefaultPaymentMethodId>,
    /// Timestamp for when the customer was soft deleted.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub deleted_at: Option<jiff::Timestamp>,
    /// Timestamp of the first event ingested for this customer. Can predate
    /// `created_at`, and is null if no event was ever ingested.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub first_user_event_at: Option<jiff::Timestamp>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
    /// The customer's active subscriptions.
    pub active_subscriptions:
        Vec<crate::v2026_04::models::customers::types::CustomerStateSubscription>,
    /// The customer's active benefit grants.
    pub granted_benefits: Vec<crate::v2026_04::models::customers::types::CustomerStateBenefitGrant>,
    /// The customer's active meters.
    pub active_meters: Vec<crate::v2026_04::models::customers::types::CustomerStateMeter>,
}
#[allow(clippy::type_complexity)]
impl CustomerStateTeam {
    pub fn new(required: CustomerStateTeamRequired) -> Self {
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
            active_subscriptions: required.active_subscriptions,
            granted_benefits: required.granted_benefits,
            active_meters: required.active_meters,
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
        value: crate::v2026_04::models::common::ids::DefaultPaymentMethodId,
    ) -> Self {
        self.default_payment_method_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerTeamCreate {
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
            crate::v2026_04::models::customers::newtypes::CustomerTeamMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerTeamCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The ID of the customer in your system. This must be unique within the organization. Once set, it can't be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::customers::newtypes::CustomerTeamName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address: presence_rs::Presence<
        crate::v2026_04::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///The ID of the organization owning the customer. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2026_04::models::common::ids::OrganizationId,
    >,
    ///Optional owner member to create with the customer. If not provided, an owner member will be automatically created using the customer's email and name.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub owner: presence_rs::Presence<
        crate::v2026_04::models::customers::types::MemberOwnerCreate,
    >,
    pub r#type: monostate::MustBe!("team"),
    ///The email address of the team customer. Optional for team customers — if omitted, an owner with an email must be provided.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerTeamCreate {
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
            crate::v2026_04::models::customers::newtypes::CustomerTeamMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerTeamCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::customers::newtypes::CustomerTeamName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the organization owning the customer. **Required unless you
    /// use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// Optional owner member to create with the customer. If not provided, an
    /// owner member will be automatically created using the customer's email
    /// and name.
    #[allow(clippy::should_implement_trait)]
    pub fn owner(
        mut self,
        value: crate::v2026_04::models::customers::types::MemberOwnerCreate,
    ) -> Self {
        self.owner = presence_rs::Presence::Some(value);
        self
    }

    /// The email address of the team customer. Optional for team customers — if
    /// omitted, an owner with an email must be provided.
    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerUpdate {
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
            crate::v2026_04::models::customers::newtypes::CustomerMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The email address of the customer. This must be unique within the organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::customers::newtypes::CustomerName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address: presence_rs::Presence<
        crate::v2026_04::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///The ID of the customer in your system. This must be unique within the organization. Once set, it can't be updated.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    ///The customer type. Can only be upgraded from 'individual' to 'team', never downgraded.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub r#type: presence_rs::Presence<
        crate::v2026_04::models::common::enums::CustomerType,
    >,
}
#[allow(clippy::type_complexity)]
impl CustomerUpdate {
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
            crate::v2026_04::models::customers::newtypes::CustomerMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
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
    pub fn name(
        mut self,
        value: crate::v2026_04::models::customers::newtypes::CustomerName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The ID of the customer in your system. This must be unique within the
    /// organization. Once set, it can't be updated.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// The customer type. Can only be upgraded from 'individual' to 'team',
    /// never downgraded.
    #[allow(clippy::should_implement_trait)]
    pub fn r#type(mut self, value: crate::v2026_04::models::common::enums::CustomerType) -> Self {
        self.r#type = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CustomerUpdateExternalId {
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
            crate::v2026_04::models::customers::newtypes::CustomerUpdateExternalIdMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerUpdateExternalIdMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The email address of the customer. This must be unique within the organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::customers::newtypes::CustomerUpdateExternalIdName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub billing_address: presence_rs::Presence<
        crate::v2026_04::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl CustomerUpdateExternalId {
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
            crate::v2026_04::models::customers::newtypes::CustomerUpdateExternalIdMetadataKey,
            crate::v2026_04::models::customers::unions::CustomerUpdateExternalIdMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
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
    pub fn name(
        mut self,
        value: crate::v2026_04::models::customers::newtypes::CustomerUpdateExternalIdName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn billing_address(
        mut self,
        value: crate::v2026_04::models::common::types::AddressInput,
    ) -> Self {
        self.billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn tax_id(mut self, value: impl Into<String>) -> Self {
        self.tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct MemberCreateFromCustomerRequired {
    /// The email address of the member.
    pub email: String,
}
/// Schema for creating a new member nested under a customer.
///
/// The customer is taken from the URL path, so it's not part of the body.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MemberCreateFromCustomer {
    /// The email address of the member.
    pub email: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2026_04::models::customers::newtypes::MemberCreateFromCustomerName,
    >,
    /// The ID of the member in your system. This must be unique within the
    /// customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
    /// The role of the member within the customer. To assign or transfer
    /// ownership, use the member update endpoint.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub role: Option<crate::v2026_04::models::customers::enums::MemberCreateFromCustomerRole>,
}
#[allow(clippy::type_complexity)]
impl MemberCreateFromCustomer {
    pub fn new(required: MemberCreateFromCustomerRequired) -> Self {
        Self {
            email: required.email,
            name: presence_rs::Presence::Absent,
            external_id: presence_rs::Presence::Absent,
            role: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::customers::newtypes::MemberCreateFromCustomerName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member in your system. This must be unique within the
    /// customer.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// The role of the member within the customer. To assign or transfer
    /// ownership, use the member update endpoint.
    #[allow(clippy::should_implement_trait)]
    pub fn role(
        mut self,
        value: crate::v2026_04::models::customers::enums::MemberCreateFromCustomerRole,
    ) -> Self {
        self.role = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct MemberOwnerCreateRequired {
    /// The email address of the member.
    pub email: String,
}
/// Schema for creating an owner member during customer creation.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct MemberOwnerCreate {
    /// The email address of the member.
    pub email: String,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name:
        presence_rs::Presence<crate::v2026_04::models::customers::newtypes::MemberOwnerCreateName>,
    /// The ID of the member in your system. This must be unique within the
    /// customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl MemberOwnerCreate {
    pub fn new(required: MemberOwnerCreateRequired) -> Self {
        Self {
            email: required.email,
            name: presence_rs::Presence::Absent,
            external_id: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::customers::newtypes::MemberOwnerCreateName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the member in your system. This must be unique within the
    /// customer.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
        self
    }
}
/// Schema for updating a member.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct MemberUpdate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<crate::v2026_04::models::customers::newtypes::MemberName>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    /// The role of the member within the customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub role: presence_rs::Presence<crate::v2026_04::models::common::enums::MemberRole>,
}
#[allow(clippy::type_complexity)]
impl MemberUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: crate::v2026_04::models::customers::newtypes::MemberName) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    /// The role of the member within the customer.
    #[allow(clippy::should_implement_trait)]
    pub fn role(mut self, value: crate::v2026_04::models::common::enums::MemberRole) -> Self {
        self.role = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodCardRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodCardMetadata,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethodCard {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: monostate::MustBe!("card"),
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodCardMetadata,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodCard {
    pub fn new(required: PaymentMethodCardRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            processor: required.processor,
            customer_id: required.customer_id,
            r#type: Default::default(),
            method_metadata: required.method_metadata,
        }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodCardMetadataRequired {
    pub brand: String,
    pub last4: String,
    pub exp_month: i64,
    pub exp_year: i64,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethodCardMetadata {
    pub brand: String,
    pub last4: String,
    pub exp_month: i64,
    pub exp_year: i64,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub wallet: presence_rs::Presence<String>,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodCardMetadata {
    pub fn new(required: PaymentMethodCardMetadataRequired) -> Self {
        Self {
            brand: required.brand,
            last4: required.last4,
            exp_month: required.exp_month,
            exp_year: required.exp_year,
            wallet: presence_rs::Presence::Absent,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn wallet(mut self, value: impl Into<String>) -> Self {
        self.wallet = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodGenericRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethodGeneric {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: String,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodGeneric {
    pub fn new(required: PaymentMethodGenericRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            processor: required.processor,
            customer_id: required.customer_id,
            r#type: required.r#type,
        }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodInUseByActiveSubscriptionRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentMethodInUseByActiveSubscription {
    pub error: monostate::MustBe!("PaymentMethodInUseByActiveSubscription"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodInUseByActiveSubscription {
    pub fn new(required: PaymentMethodInUseByActiveSubscriptionRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodKrCardRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodKrCardMetadata,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethodKrCard {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::PaymentMethodId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    pub processor: crate::v2026_04::models::common::enums::PaymentProcessor,
    pub customer_id: crate::v2026_04::models::common::ids::CustomerId,
    pub r#type: monostate::MustBe!("kr_card"),
    pub method_metadata: crate::v2026_04::models::customers::types::PaymentMethodKrCardMetadata,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodKrCard {
    pub fn new(required: PaymentMethodKrCardRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            processor: required.processor,
            customer_id: required.customer_id,
            r#type: Default::default(),
            method_metadata: required.method_metadata,
        }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodKrCardMetadataRequired {
    pub brand: Option<String>,
    pub last4: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct PaymentMethodKrCardMetadata {
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub brand: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub last4: Option<String>,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodKrCardMetadata {
    pub fn new(required: PaymentMethodKrCardMetadataRequired) -> Self {
        Self { brand: required.brand, last4: required.last4 }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentMethodSetupFailedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentMethodSetupFailed {
    pub error: monostate::MustBe!("PaymentMethodSetupFailed"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentMethodSetupFailed {
    pub fn new(required: PaymentMethodSetupFailedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
