#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CustomerCreate {
    CustomerIndividualCreate(crate::v2026_04::models::customers::types::CustomerIndividualCreate),
    CustomerTeamCreate(crate::v2026_04::models::customers::types::CustomerTeamCreate),
}
impl<'de> serde::Deserialize<'de> for CustomerCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_04::models::customers::types::CustomerIndividualCreate,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CustomerCreate"
                )));
            }
            matched = Some(Self::CustomerIndividualCreate(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_04::models::customers::types::CustomerTeamCreate,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CustomerCreate"
                )));
            }
            matched = Some(Self::CustomerTeamCreate(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!("no matching oneOf variant for ", "CustomerCreate"))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerIndividualCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_04::models::customers::newtypes::CustomerIndividualMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerPaymentMethod {
    PaymentMethodCard(crate::v2026_04::models::customers::types::PaymentMethodCard),
    PaymentMethodKrCard(crate::v2026_04::models::customers::types::PaymentMethodKrCard),
    PaymentMethodGeneric(crate::v2026_04::models::customers::types::PaymentMethodGeneric),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", deny_unknown_fields)]
pub enum CustomerPaymentMethodCreateResponse {
    #[serde(rename = "succeeded")]
    Succeeded { payment_method: crate::v2026_04::models::customers::unions::CustomerPaymentMethod },
    #[serde(rename = "requires_action")]
    RequiresAction { client_secret: String },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum CustomerState {
    #[serde(rename = "individual")]
    Individual {
        /// The ID of the customer.
        id: crate::v2026_04::models::common::ids::CustomerId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
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
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        name: Option<String>,
        /// The name that should appear on the customer's invoices. Falls back
        /// to the customer name when not explicitly set.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_name: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_address: Option<crate::v2026_04::models::common::types::Address>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        locale: presence_rs::Presence<String>,
        /// The ID of the organization owning the customer.
        organization_id: crate::v2026_04::models::common::ids::OrganizationId,
        /// The ID of the customer's default payment method, if any. Use the
        /// payment methods endpoint to retrieve its details.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        default_payment_method_id:
            presence_rs::Presence<crate::v2026_04::models::common::ids::DefaultPaymentMethodId>,
        /// Timestamp for when the customer was soft deleted.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        deleted_at: Option<jiff::Timestamp>,
        /// Timestamp of the first event ingested for this customer. Can predate
        /// `created_at`, and is null if no event was ever ingested.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        first_user_event_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        avatar_url: Option<String>,
        /// The customer's active subscriptions.
        active_subscriptions:
            Vec<crate::v2026_04::models::customers::types::CustomerStateSubscription>,
        /// The customer's active benefit grants.
        granted_benefits: Vec<crate::v2026_04::models::customers::types::CustomerStateBenefitGrant>,
        /// The customer's active meters.
        active_meters: Vec<crate::v2026_04::models::customers::types::CustomerStateMeter>,
    },
    #[serde(rename = "team")]
    Team {
        /// The ID of the customer.
        id: crate::v2026_04::models::common::ids::CustomerId,
        /// Creation timestamp of the object.
        created_at: jiff::Timestamp,
        /// Last modification timestamp of the object.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        modified_at: Option<jiff::Timestamp>,
        metadata: crate::v2026_04::models::common::aliases::MetadataOutputType,
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
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        name: Option<String>,
        /// The name that should appear on the customer's invoices. Falls back
        /// to the customer name when not explicitly set.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_name: Option<String>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        billing_address: Option<crate::v2026_04::models::common::types::Address>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        tax_id: Option<(String, crate::v2026_04::models::common::enums::TaxIDFormat)>,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        locale: presence_rs::Presence<String>,
        /// The ID of the organization owning the customer.
        organization_id: crate::v2026_04::models::common::ids::OrganizationId,
        /// The ID of the customer's default payment method, if any. Use the
        /// payment methods endpoint to retrieve its details.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        default_payment_method_id:
            presence_rs::Presence<crate::v2026_04::models::common::ids::DefaultPaymentMethodId>,
        /// Timestamp for when the customer was soft deleted.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        deleted_at: Option<jiff::Timestamp>,
        /// Timestamp of the first event ingested for this customer. Can predate
        /// `created_at`, and is null if no event was ever ingested.
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        first_user_event_at: Option<jiff::Timestamp>,
        #[serde(
            deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
        )]
        avatar_url: Option<String>,
        /// The customer's active subscriptions.
        active_subscriptions:
            Vec<crate::v2026_04::models::customers::types::CustomerStateSubscription>,
        /// The customer's active benefit grants.
        granted_benefits: Vec<crate::v2026_04::models::customers::types::CustomerStateBenefitGrant>,
        /// The customer's active meters.
        active_meters: Vec<crate::v2026_04::models::customers::types::CustomerStateMeter>,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerStateBenefitGrantPropertiesUnion {
    BenefitGrantDiscordProperties(
        crate::v2026_04::models::common::types::BenefitGrantDiscordProperties,
    ),
    BenefitGrantGitHubRepositoryProperties(
        crate::v2026_04::models::common::types::BenefitGrantGitHubRepositoryProperties,
    ),
    BenefitGrantDownloadablesProperties(
        crate::v2026_04::models::common::types::BenefitGrantDownloadablesProperties,
    ),
    BenefitGrantLicenseKeysProperties(
        crate::v2026_04::models::common::types::BenefitGrantLicenseKeysProperties,
    ),
    BenefitGrantCustomProperties(
        crate::v2026_04::models::common::types::BenefitGrantCustomProperties,
    ),
    BenefitGrantFeatureFlagProperties(
        crate::v2026_04::models::common::types::BenefitGrantFeatureFlagProperties,
    ),
    BenefitGrantSlackSharedChannelProperties(
        crate::v2026_04::models::common::types::BenefitGrantSlackSharedChannelProperties,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerStateSubscriptionCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerTeamCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_04::models::customers::newtypes::CustomerTeamMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerUpdateExternalIdMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_04::models::customers::newtypes::CustomerUpdateExternalIdMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_04::models::customers::newtypes::CustomerMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PaymentMethod {
    CustomerPaymentMethodCard(crate::v2026_04::models::customers::types::CustomerPaymentMethodCard),
    CustomerPaymentMethodKrCard(
        crate::v2026_04::models::customers::types::CustomerPaymentMethodKrCard,
    ),
    CustomerPaymentMethodGeneric(
        crate::v2026_04::models::customers::types::CustomerPaymentMethodGeneric,
    ),
}
