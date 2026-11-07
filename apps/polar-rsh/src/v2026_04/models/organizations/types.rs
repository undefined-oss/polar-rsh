#[derive(Debug, Clone)]
pub struct CannotCreateOrganizationErrorRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct CannotCreateOrganizationError {
    pub error: monostate::MustBe!("CannotCreateOrganizationError"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl CannotCreateOrganizationError {
    pub fn new(required: CannotCreateOrganizationErrorRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerOrganizationDataRequired {
    pub organization: crate::v2026_04::models::common::types::CustomerOrganization,
    pub products: Vec<crate::v2026_04::models::organizations::types::CustomerProduct>,
}
/// Schema of an organization and related data for customer portal.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerOrganizationData {
    pub organization: crate::v2026_04::models::common::types::CustomerOrganization,
    pub products: Vec<crate::v2026_04::models::organizations::types::CustomerProduct>,
}
#[allow(clippy::type_complexity)]
impl CustomerOrganizationData {
    pub fn new(required: CustomerOrganizationDataRequired) -> Self {
        Self { organization: required.organization, products: required.products }
    }
}
#[derive(Debug, Clone)]
pub struct CustomerProductRequired {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2026_04::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_04::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// List of available prices for this product.
    pub prices:
        Vec<crate::v2026_04::models::organizations::unions::CustomerProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_04::models::common::unions::BenefitPublic>,
    /// The medias associated to the product.
    pub medias: Vec<crate::v2026_04::models::common::types::ProductMediaFileRead>,
}
/// Schema of a product for customer portal.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CustomerProduct {
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2026_04::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_04::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2026_04::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_04::models::common::ids::OrganizationId,
    /// List of available prices for this product.
    pub prices:
        Vec<crate::v2026_04::models::organizations::unions::CustomerProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_04::models::common::unions::BenefitPublic>,
    /// The medias associated to the product.
    pub medias: Vec<crate::v2026_04::models::common::types::ProductMediaFileRead>,
}
#[allow(clippy::type_complexity)]
impl CustomerProduct {
    pub fn new(required: CustomerProductRequired) -> Self {
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
        }
    }
}
#[derive(Debug, Clone)]
pub struct DisputeAutoAcceptNotEnabledRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct DisputeAutoAcceptNotEnabled {
    pub error: monostate::MustBe!("DisputeAutoAcceptNotEnabled"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl DisputeAutoAcceptNotEnabled {
    pub fn new(required: DisputeAutoAcceptNotEnabledRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrganizationId,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: String,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    pub avatar_url: Option<String>,
    /// Proration behavior applied when customer updates their subscription from
    /// the portal.
    pub proration_behavior: crate::v2026_04::models::common::enums::SubscriptionProrationBehavior,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
    /// Public support email.
    pub email: Option<String>,
    /// Official website of the organization.
    pub website: Option<String>,
    /// Links to social profiles.
    pub socials: Vec<crate::v2026_04::models::organizations::types::OrganizationSocialLink>,
    /// Current organization status
    pub status: crate::v2026_04::models::organizations::enums::OrganizationStatus,
    /// When the business details were submitted for review.
    pub details_submitted_at: Option<jiff::Timestamp>,
    /// When Polar requested that the organization review and resubmit its
    /// onboarding information, if applicable.
    pub onboarding_resubmission_requested_at: Option<jiff::Timestamp>,
    /// Whether members must access this organization through its SSO
    /// connection.
    pub sso_enforced: bool,
    /// Default presentment currency. Used as fallback in checkout and customer
    /// portal, if the customer's local currency is not available.
    pub default_presentment_currency: String,
    /// Default tax behavior applied on products.
    pub default_tax_behavior: crate::v2026_04::models::common::enums::TaxBehaviorOption,
    /// Organization feature settings
    pub feature_settings:
        Option<crate::v2026_04::models::organizations::types::OrganizationFeatureSettings>,
    /// Settings related to subscriptions management
    pub subscription_settings:
        crate::v2026_04::models::organizations::types::OrganizationSubscriptionSettings,
    /// Settings related to customer emails
    pub customer_email_settings:
        crate::v2026_04::models::organizations::types::OrganizationCustomerEmailSettings,
    /// Settings related to the customer portal
    pub customer_portal_settings:
        crate::v2026_04::models::common::types::OrganizationCustomerPortalSettings,
    /// Settings related to disputes
    pub dispute_settings:
        crate::v2026_04::models::organizations::types::OrganizationDisputeSettings,
    /// Hosts allowed to embed this organization's checkout. An entry is a host
    /// and an optional port, without a scheme: HTTPS is always allowed, and
    /// HTTP too for local hosts — `localhost`, any `.localhost` or `.local`
    /// name, and loopback or private addresses. `*.example.com` matches any
    /// subdomain, but not `example.com` itself. An app origin such as
    /// `chrome-extension://abcdef` carries its scheme, having no host to match
    /// on.
    pub embed_hosts: Vec<String>,
    /// ID of the transactions account.
    pub account_id: Option<crate::v2026_04::models::common::ids::AccountId>,
    /// ID of the payout account.
    pub payout_account_id: Option<crate::v2026_04::models::common::ids::PayoutAccountId>,
    /// Capabilities currently granted to the organization.
    pub capabilities: crate::v2026_04::models::organizations::types::OrganizationCapabilities,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Organization {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_04::models::common::ids::OrganizationId,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: String,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
    /// Proration behavior applied when customer updates their subscription from
    /// the portal.
    pub proration_behavior: crate::v2026_04::models::common::enums::SubscriptionProrationBehavior,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
    /// Public support email.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub email: Option<String>,
    /// Official website of the organization.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub website: Option<String>,
    /// Links to social profiles.
    pub socials: Vec<crate::v2026_04::models::organizations::types::OrganizationSocialLink>,
    /// Current organization status
    pub status: crate::v2026_04::models::organizations::enums::OrganizationStatus,
    /// When the business details were submitted for review.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub details_submitted_at: Option<jiff::Timestamp>,
    /// When Polar requested that the organization review and resubmit its
    /// onboarding information, if applicable.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub onboarding_resubmission_requested_at: Option<jiff::Timestamp>,
    /// Whether members must access this organization through its SSO
    /// connection.
    pub sso_enforced: bool,
    /// Default presentment currency. Used as fallback in checkout and customer
    /// portal, if the customer's local currency is not available.
    pub default_presentment_currency: String,
    /// Default tax behavior applied on products.
    pub default_tax_behavior: crate::v2026_04::models::common::enums::TaxBehaviorOption,
    /// Organization feature settings
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub feature_settings:
        Option<crate::v2026_04::models::organizations::types::OrganizationFeatureSettings>,
    /// Settings related to subscriptions management
    pub subscription_settings:
        crate::v2026_04::models::organizations::types::OrganizationSubscriptionSettings,
    /// Settings related to customer emails
    pub customer_email_settings:
        crate::v2026_04::models::organizations::types::OrganizationCustomerEmailSettings,
    /// Settings related to the customer portal
    pub customer_portal_settings:
        crate::v2026_04::models::common::types::OrganizationCustomerPortalSettings,
    /// Settings related to disputes
    pub dispute_settings:
        crate::v2026_04::models::organizations::types::OrganizationDisputeSettings,
    /// Hosts allowed to embed this organization's checkout. An entry is a host
    /// and an optional port, without a scheme: HTTPS is always allowed, and
    /// HTTP too for local hosts — `localhost`, any `.localhost` or `.local`
    /// name, and loopback or private addresses. `*.example.com` matches any
    /// subdomain, but not `example.com` itself. An app origin such as
    /// `chrome-extension://abcdef` carries its scheme, having no host to match
    /// on.
    pub embed_hosts: Vec<String>,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub country: presence_rs::Presence<crate::v2026_04::models::common::enums::CountryAlpha2>,
    /// ID of the transactions account.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub account_id: Option<crate::v2026_04::models::common::ids::AccountId>,
    /// ID of the payout account.
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub payout_account_id: Option<crate::v2026_04::models::common::ids::PayoutAccountId>,
    /// Capabilities currently granted to the organization.
    pub capabilities: crate::v2026_04::models::organizations::types::OrganizationCapabilities,
}
#[allow(clippy::type_complexity)]
impl Organization {
    pub fn new(required: OrganizationRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            slug: required.slug,
            avatar_url: required.avatar_url,
            proration_behavior: required.proration_behavior,
            allow_customer_updates: required.allow_customer_updates,
            email: required.email,
            website: required.website,
            socials: required.socials,
            status: required.status,
            details_submitted_at: required.details_submitted_at,
            onboarding_resubmission_requested_at: required.onboarding_resubmission_requested_at,
            sso_enforced: required.sso_enforced,
            default_presentment_currency: required.default_presentment_currency,
            default_tax_behavior: required.default_tax_behavior,
            feature_settings: required.feature_settings,
            subscription_settings: required.subscription_settings,
            customer_email_settings: required.customer_email_settings,
            customer_portal_settings: required.customer_portal_settings,
            dispute_settings: required.dispute_settings,
            embed_hosts: required.embed_hosts,
            country: presence_rs::Presence::Absent,
            account_id: required.account_id,
            payout_account_id: required.payout_account_id,
            capabilities: required.capabilities,
        }
    }

    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[allow(clippy::should_implement_trait)]
    pub fn country(mut self, value: crate::v2026_04::models::common::enums::CountryAlpha2) -> Self {
        self.country = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationCapabilitiesRequired {
    /// Whether the organization can accept new checkout payments.
    pub checkout_payments: bool,
    /// Whether the organization can process subscription renewals.
    pub subscription_renewals: bool,
    /// Whether the organization can withdraw its balance.
    pub payouts: bool,
    /// Whether the organization can issue refunds.
    pub refunds: bool,
    /// Whether the organization can access the API.
    pub api_access: bool,
    /// Whether the organization can access the dashboard.
    pub dashboard_access: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationCapabilities {
    /// Whether the organization can accept new checkout payments.
    pub checkout_payments: bool,
    /// Whether the organization can process subscription renewals.
    pub subscription_renewals: bool,
    /// Whether the organization can withdraw its balance.
    pub payouts: bool,
    /// Whether the organization can issue refunds.
    pub refunds: bool,
    /// Whether the organization can access the API.
    pub api_access: bool,
    /// Whether the organization can access the dashboard.
    pub dashboard_access: bool,
}
#[allow(clippy::type_complexity)]
impl OrganizationCapabilities {
    pub fn new(required: OrganizationCapabilitiesRequired) -> Self {
        Self {
            checkout_payments: required.checkout_payments,
            subscription_renewals: required.subscription_renewals,
            payouts: required.payouts,
            refunds: required.refunds,
            api_access: required.api_access,
            dashboard_access: required.dashboard_access,
        }
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationCompanyLegalEntitySchemaRequired {
    pub registered_name: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationCompanyLegalEntitySchema {
    pub r#type: monostate::MustBe!("company"),
    pub registered_name: String,
}
#[allow(clippy::type_complexity)]
impl OrganizationCompanyLegalEntitySchema {
    pub fn new(required: OrganizationCompanyLegalEntitySchemaRequired) -> Self {
        Self { r#type: Default::default(), registered_name: required.registered_name }
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationCreateRequired {
    pub name: crate::v2026_04::models::organizations::newtypes::OrganizationName,
    pub slug: crate::v2026_04::models::organizations::newtypes::OrganizationSlug,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationCreate {
    pub name: crate::v2026_04::models::organizations::newtypes::OrganizationName,
    pub slug: crate::v2026_04::models::organizations::newtypes::OrganizationSlug,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub avatar_url: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationAvatarUrl,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub legal_entity: presence_rs::Presence<
        crate::v2026_04::models::organizations::unions::OrganizationCreateLegalEntityAnyOf0Union,
    >,
    /// Public support email.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    /// Official website of the organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub website: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationWebsite,
    >,
    /// Link to social profiles.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub socials: presence_rs::Presence<
        Vec<crate::v2026_04::models::organizations::types::OrganizationSocialLink>,
    >,
    /// Additional, private, business details Polar needs about active
    /// organizations for compliance (KYC).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub details:
        presence_rs::Presence<crate::v2026_04::models::organizations::types::OrganizationDetails>,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub country: presence_rs::Presence<crate::v2026_04::models::common::enums::CountryAlpha2Input>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub feature_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationFeatureSettingsUpdate,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub subscription_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationSubscriptionSettings,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_email_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationCustomerEmailSettings,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_portal_settings: presence_rs::Presence<
        crate::v2026_04::models::common::types::OrganizationCustomerPortalSettings,
    >,
    /// Default presentment currency for the organization
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub default_presentment_currency:
        Option<crate::v2026_04::models::common::enums::PresentmentCurrency>,
    /// Default tax behavior applied on products.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub default_tax_behavior: Option<crate::v2026_04::models::common::enums::TaxBehaviorOption>,
}
#[allow(clippy::type_complexity)]
impl OrganizationCreate {
    pub fn new(required: OrganizationCreateRequired) -> Self {
        Self {
            name: required.name,
            slug: required.slug,
            avatar_url: presence_rs::Presence::Absent,
            legal_entity: presence_rs::Presence::Absent,
            email: presence_rs::Presence::Absent,
            website: presence_rs::Presence::Absent,
            socials: presence_rs::Presence::Absent,
            details: presence_rs::Presence::Absent,
            country: presence_rs::Presence::Absent,
            feature_settings: presence_rs::Presence::Absent,
            subscription_settings: presence_rs::Presence::Absent,
            customer_email_settings: presence_rs::Presence::Absent,
            customer_portal_settings: presence_rs::Presence::Absent,
            default_presentment_currency: None,
            default_tax_behavior: None,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn avatar_url(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationAvatarUrl,
    ) -> Self {
        self.avatar_url = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn legal_entity(
        mut self,
        value: crate::v2026_04::models::organizations::unions::OrganizationCreateLegalEntityAnyOf0Union,
    ) -> Self {
        self.legal_entity = presence_rs::Presence::Some(value);
        self
    }

    /// Public support email.
    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    /// Official website of the organization.
    #[allow(clippy::should_implement_trait)]
    pub fn website(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationWebsite,
    ) -> Self {
        self.website = presence_rs::Presence::Some(value);
        self
    }

    /// Link to social profiles.
    #[allow(clippy::should_implement_trait)]
    pub fn socials(
        mut self,
        value: Vec<crate::v2026_04::models::organizations::types::OrganizationSocialLink>,
    ) -> Self {
        self.socials = presence_rs::Presence::Some(value);
        self
    }

    /// Additional, private, business details Polar needs about active
    /// organizations for compliance (KYC).
    #[allow(clippy::should_implement_trait)]
    pub fn details(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationDetails,
    ) -> Self {
        self.details = presence_rs::Presence::Some(value);
        self
    }

    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[allow(clippy::should_implement_trait)]
    pub fn country(
        mut self,
        value: crate::v2026_04::models::common::enums::CountryAlpha2Input,
    ) -> Self {
        self.country = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn feature_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationFeatureSettingsUpdate,
    ) -> Self {
        self.feature_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationSubscriptionSettings,
    ) -> Self {
        self.subscription_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_email_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationCustomerEmailSettings,
    ) -> Self {
        self.customer_email_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_portal_settings(
        mut self,
        value: crate::v2026_04::models::common::types::OrganizationCustomerPortalSettings,
    ) -> Self {
        self.customer_portal_settings = presence_rs::Presence::Some(value);
        self
    }

    /// Default presentment currency for the organization
    #[allow(clippy::should_implement_trait)]
    pub fn default_presentment_currency(
        mut self,
        value: crate::v2026_04::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.default_presentment_currency = Some(value);
        self
    }

    /// Default tax behavior applied on products.
    #[allow(clippy::should_implement_trait)]
    pub fn default_tax_behavior(
        mut self,
        value: crate::v2026_04::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.default_tax_behavior = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationCustomerEmailSettingsRequired {
    pub order_confirmation: bool,
    pub payment_method_expiration_reminder: bool,
    pub subscription_cancellation: bool,
    pub subscription_confirmation: bool,
    pub subscription_cycled: bool,
    pub subscription_cycled_after_trial: bool,
    pub subscription_past_due: bool,
    pub subscription_paused: bool,
    pub subscription_resumed: bool,
    pub subscription_renewal_reminder: bool,
    pub subscription_revoked: bool,
    pub subscription_trial_conversion_reminder: bool,
    pub subscription_uncanceled: bool,
    pub subscription_updated: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationCustomerEmailSettings {
    pub order_confirmation: bool,
    pub payment_method_expiration_reminder: bool,
    pub subscription_cancellation: bool,
    pub subscription_confirmation: bool,
    pub subscription_cycled: bool,
    pub subscription_cycled_after_trial: bool,
    pub subscription_past_due: bool,
    pub subscription_paused: bool,
    pub subscription_resumed: bool,
    pub subscription_renewal_reminder: bool,
    pub subscription_revoked: bool,
    pub subscription_trial_conversion_reminder: bool,
    pub subscription_uncanceled: bool,
    pub subscription_updated: bool,
}
#[allow(clippy::type_complexity)]
impl OrganizationCustomerEmailSettings {
    pub fn new(required: OrganizationCustomerEmailSettingsRequired) -> Self {
        Self {
            order_confirmation: required.order_confirmation,
            payment_method_expiration_reminder: required.payment_method_expiration_reminder,
            subscription_cancellation: required.subscription_cancellation,
            subscription_confirmation: required.subscription_confirmation,
            subscription_cycled: required.subscription_cycled,
            subscription_cycled_after_trial: required.subscription_cycled_after_trial,
            subscription_past_due: required.subscription_past_due,
            subscription_paused: required.subscription_paused,
            subscription_resumed: required.subscription_resumed,
            subscription_renewal_reminder: required.subscription_renewal_reminder,
            subscription_revoked: required.subscription_revoked,
            subscription_trial_conversion_reminder: required.subscription_trial_conversion_reminder,
            subscription_uncanceled: required.subscription_uncanceled,
            subscription_updated: required.subscription_updated,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrganizationDetails {
    /// Description of digital products being sold.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_description: presence_rs::Presence<String>,
    /// Categories of products being sold.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub selling_categories: Option<Vec<String>>,
    /// Pricing models used by the organization.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub pricing_models: Option<Vec<String>>,
    /// Switching from another platform?
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub switching: Option<bool>,
    /// Which platform the organization is migrating from.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub switching_from: presence_rs::Presence<
        crate::v2026_04::models::organizations::enums::OrganizationDetailsSwitchingFrom,
    >,
    /// Brief information about you and your business.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub about: presence_rs::Presence<String>,
    /// How the organization will integrate and use Polar.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub intended_use: presence_rs::Presence<String>,
    /// Main customer acquisition channels.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub customer_acquisition: Option<Vec<String>>,
    /// Estimated revenue in the next 12 months
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub future_annual_revenue: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationDetailsFutureAnnualRevenue,
    >,
    /// Revenue from last year if applicable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub previous_annual_revenue: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationDetailsPreviousAnnualRevenue,
    >,
}
#[allow(clippy::type_complexity)]
impl OrganizationDetails {
    pub fn new() -> Self {
        Self::default()
    }

    /// Description of digital products being sold.
    #[allow(clippy::should_implement_trait)]
    pub fn product_description(mut self, value: impl Into<String>) -> Self {
        self.product_description = presence_rs::Presence::Some(value.into());
        self
    }

    /// Categories of products being sold.
    #[allow(clippy::should_implement_trait)]
    pub fn selling_categories(mut self, value: Vec<String>) -> Self {
        self.selling_categories = Some(value);
        self
    }

    /// Pricing models used by the organization.
    #[allow(clippy::should_implement_trait)]
    pub fn pricing_models(mut self, value: Vec<String>) -> Self {
        self.pricing_models = Some(value);
        self
    }

    /// Switching from another platform?
    #[allow(clippy::should_implement_trait)]
    pub fn switching(mut self, value: bool) -> Self {
        self.switching = Some(value);
        self
    }

    /// Which platform the organization is migrating from.
    #[allow(clippy::should_implement_trait)]
    pub fn switching_from(
        mut self,
        value: crate::v2026_04::models::organizations::enums::OrganizationDetailsSwitchingFrom,
    ) -> Self {
        self.switching_from = presence_rs::Presence::Some(value);
        self
    }

    /// Brief information about you and your business.
    #[allow(clippy::should_implement_trait)]
    pub fn about(mut self, value: impl Into<String>) -> Self {
        self.about = presence_rs::Presence::Some(value.into());
        self
    }

    /// How the organization will integrate and use Polar.
    #[allow(clippy::should_implement_trait)]
    pub fn intended_use(mut self, value: impl Into<String>) -> Self {
        self.intended_use = presence_rs::Presence::Some(value.into());
        self
    }

    /// Main customer acquisition channels.
    #[allow(clippy::should_implement_trait)]
    pub fn customer_acquisition(mut self, value: Vec<String>) -> Self {
        self.customer_acquisition = Some(value);
        self
    }

    /// Estimated revenue in the next 12 months
    #[allow(clippy::should_implement_trait)]
    pub fn future_annual_revenue(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationDetailsFutureAnnualRevenue,
    ) -> Self {
        self.future_annual_revenue = presence_rs::Presence::Some(value);
        self
    }

    /// Revenue from last year if applicable.
    #[allow(clippy::should_implement_trait)]
    pub fn previous_annual_revenue(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationDetailsPreviousAnnualRevenue,
    ) -> Self {
        self.previous_annual_revenue = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationDisputeSettingsRequired {
    pub auto_accept_below_amount: Option<i64>,
}
/// `auto_accept_below_amount` is in Polar's settlement currency (USD).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationDisputeSettings {
    #[serde(
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub auto_accept_below_amount: Option<i64>,
}
#[allow(clippy::type_complexity)]
impl OrganizationDisputeSettings {
    pub fn new(required: OrganizationDisputeSettingsRequired) -> Self {
        Self { auto_accept_below_amount: required.auto_accept_below_amount }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrganizationDisputeSettingsUpdate {
    ///Concede disputes below this amount, in USD cents, without asking the organization. A dispute charged in another currency converts at the rate its payment settled at. `null` turns it off. The disputed amount and the processor's dispute fee are still deducted.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub auto_accept_below_amount: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationDisputeSettingsAutoAcceptBelowAmount,
    >,
}
#[allow(clippy::type_complexity)]
impl OrganizationDisputeSettingsUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Concede disputes below this amount, in USD cents, without asking the
    /// organization. A dispute charged in another currency converts at the rate
    /// its payment settled at. `null` turns it off. The disputed amount and the
    /// processor's dispute fee are still deducted.
    #[allow(clippy::should_implement_trait)]
    pub fn auto_accept_below_amount(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationDisputeSettingsAutoAcceptBelowAmount,
    ) -> Self {
        self.auto_accept_below_amount = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrganizationFeatureSettings {
    /// If this organization has issue funding enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub issue_funding_enabled: Option<bool>,
    /// If this organization has Wallets enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub wallets_enabled: Option<bool>,
    /// If this organization has the Member model enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub member_model_enabled: Option<bool>,
    /// If this organization has checkout localization enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub checkout_localization_enabled: Option<bool>,
    /// Ordered list of metric slugs shown on the dashboard overview.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub overview_metrics: presence_rs::Presence<Vec<String>>,
    /// If this organization has access to reset proration behavior.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub reset_proration_behavior_enabled: Option<bool>,
    /// If this organization can create and finalize draft orders via the API
    /// (off-session charges against a saved payment method).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub off_session_charges_enabled: Option<bool>,
    /// If this organization can set a separate meter cycle on recurring
    /// products (a meter interval independent of the billing interval).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub meter_cycling_enabled: Option<bool>,
    /// Enables the slack shared channel benefit
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub slack_benefit_enabled: Option<bool>,
    /// If this organization has preview access to new features enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub preview_access_enabled: Option<bool>,
    /// If this organization has the disputes dashboard enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub disputes_enabled: Option<bool>,
    /// If this organization has single sign-on configuration enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub sso_enabled: Option<bool>,
    /// If this organization can set a threshold below which Polar concedes
    /// disputes on its behalf. Requires `disputes_enabled`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub dispute_auto_accept_enabled: Option<bool>,
    /// If this organization has the split product navigation (Billing / Compass
    /// / Customers) enabled in the dashboard
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub compass_enabled: Option<bool>,
    /// If this organization can migrate its billing from another provider (e.g.
    /// Stripe) to Polar.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub merchant_migration_enabled: Option<bool>,
}
#[allow(clippy::type_complexity)]
impl OrganizationFeatureSettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// If this organization has issue funding enabled
    #[allow(clippy::should_implement_trait)]
    pub fn issue_funding_enabled(mut self, value: bool) -> Self {
        self.issue_funding_enabled = Some(value);
        self
    }

    /// If this organization has Wallets enabled
    #[allow(clippy::should_implement_trait)]
    pub fn wallets_enabled(mut self, value: bool) -> Self {
        self.wallets_enabled = Some(value);
        self
    }

    /// If this organization has the Member model enabled
    #[allow(clippy::should_implement_trait)]
    pub fn member_model_enabled(mut self, value: bool) -> Self {
        self.member_model_enabled = Some(value);
        self
    }

    /// If this organization has checkout localization enabled
    #[allow(clippy::should_implement_trait)]
    pub fn checkout_localization_enabled(mut self, value: bool) -> Self {
        self.checkout_localization_enabled = Some(value);
        self
    }

    /// Ordered list of metric slugs shown on the dashboard overview.
    #[allow(clippy::should_implement_trait)]
    pub fn overview_metrics(mut self, value: Vec<String>) -> Self {
        self.overview_metrics = presence_rs::Presence::Some(value);
        self
    }

    /// If this organization has access to reset proration behavior.
    #[allow(clippy::should_implement_trait)]
    pub fn reset_proration_behavior_enabled(mut self, value: bool) -> Self {
        self.reset_proration_behavior_enabled = Some(value);
        self
    }

    /// If this organization can create and finalize draft orders via the API
    /// (off-session charges against a saved payment method).
    #[allow(clippy::should_implement_trait)]
    pub fn off_session_charges_enabled(mut self, value: bool) -> Self {
        self.off_session_charges_enabled = Some(value);
        self
    }

    /// If this organization can set a separate meter cycle on recurring
    /// products (a meter interval independent of the billing interval).
    #[allow(clippy::should_implement_trait)]
    pub fn meter_cycling_enabled(mut self, value: bool) -> Self {
        self.meter_cycling_enabled = Some(value);
        self
    }

    /// Enables the slack shared channel benefit
    #[allow(clippy::should_implement_trait)]
    pub fn slack_benefit_enabled(mut self, value: bool) -> Self {
        self.slack_benefit_enabled = Some(value);
        self
    }

    /// If this organization has preview access to new features enabled
    #[allow(clippy::should_implement_trait)]
    pub fn preview_access_enabled(mut self, value: bool) -> Self {
        self.preview_access_enabled = Some(value);
        self
    }

    /// If this organization has the disputes dashboard enabled
    #[allow(clippy::should_implement_trait)]
    pub fn disputes_enabled(mut self, value: bool) -> Self {
        self.disputes_enabled = Some(value);
        self
    }

    /// If this organization has single sign-on configuration enabled
    #[allow(clippy::should_implement_trait)]
    pub fn sso_enabled(mut self, value: bool) -> Self {
        self.sso_enabled = Some(value);
        self
    }

    /// If this organization can set a threshold below which Polar concedes
    /// disputes on its behalf. Requires `disputes_enabled`.
    #[allow(clippy::should_implement_trait)]
    pub fn dispute_auto_accept_enabled(mut self, value: bool) -> Self {
        self.dispute_auto_accept_enabled = Some(value);
        self
    }

    /// If this organization has the split product navigation (Billing / Compass
    /// / Customers) enabled in the dashboard
    #[allow(clippy::should_implement_trait)]
    pub fn compass_enabled(mut self, value: bool) -> Self {
        self.compass_enabled = Some(value);
        self
    }

    /// If this organization can migrate its billing from another provider (e.g.
    /// Stripe) to Polar.
    #[allow(clippy::should_implement_trait)]
    pub fn merchant_migration_enabled(mut self, value: bool) -> Self {
        self.merchant_migration_enabled = Some(value);
        self
    }
}
/// Feature settings that organizations can update themselves.
///
/// Other feature settings are managed by Polar staff: they're ignored if
/// provided and keep their current value.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrganizationFeatureSettingsUpdate {
    /// If this organization has the Member model enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub member_model_enabled: Option<bool>,
    /// If this organization has checkout localization enabled
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_04::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub checkout_localization_enabled: Option<bool>,
    /// Ordered list of metric slugs shown on the dashboard overview.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub overview_metrics: presence_rs::Presence<Vec<String>>,
}
#[allow(clippy::type_complexity)]
impl OrganizationFeatureSettingsUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// If this organization has the Member model enabled
    #[allow(clippy::should_implement_trait)]
    pub fn member_model_enabled(mut self, value: bool) -> Self {
        self.member_model_enabled = Some(value);
        self
    }

    /// If this organization has checkout localization enabled
    #[allow(clippy::should_implement_trait)]
    pub fn checkout_localization_enabled(mut self, value: bool) -> Self {
        self.checkout_localization_enabled = Some(value);
        self
    }

    /// Ordered list of metric slugs shown on the dashboard overview.
    #[allow(clippy::should_implement_trait)]
    pub fn overview_metrics(mut self, value: Vec<String>) -> Self {
        self.overview_metrics = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrganizationIndividualLegalEntitySchema {
    pub r#type: monostate::MustBe!("individual"),
}
#[allow(clippy::type_complexity)]
impl OrganizationIndividualLegalEntitySchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationSocialLinkRequired {
    /// The social platform of the URL
    pub platform: crate::v2026_04::models::organizations::enums::OrganizationSocialPlatforms,
    /// The URL to the organization profile
    pub url: crate::v2026_04::models::organizations::newtypes::OrganizationSocialLinkUrl,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationSocialLink {
    /// The social platform of the URL
    pub platform: crate::v2026_04::models::organizations::enums::OrganizationSocialPlatforms,
    /// The URL to the organization profile
    pub url: crate::v2026_04::models::organizations::newtypes::OrganizationSocialLinkUrl,
}
#[allow(clippy::type_complexity)]
impl OrganizationSocialLink {
    pub fn new(required: OrganizationSocialLinkRequired) -> Self {
        Self { platform: required.platform, url: required.url }
    }
}
#[derive(Debug, Clone)]
pub struct OrganizationSubscriptionSettingsRequired {
    pub allow_multiple_subscriptions: bool,
    pub proration_behavior: crate::v2026_04::models::organizations::enums::OrganizationSubscriptionSettingsProrationBehavior,
    pub benefit_revocation_grace_period: i64,
    pub prevent_trial_abuse: bool,
    pub allow_customer_updates: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct OrganizationSubscriptionSettings {
    pub allow_multiple_subscriptions: bool,
    pub proration_behavior: crate::v2026_04::models::organizations::enums::OrganizationSubscriptionSettingsProrationBehavior,
    pub benefit_revocation_grace_period: i64,
    pub prevent_trial_abuse: bool,
    pub allow_customer_updates: bool,
}
#[allow(clippy::type_complexity)]
impl OrganizationSubscriptionSettings {
    pub fn new(required: OrganizationSubscriptionSettingsRequired) -> Self {
        Self {
            allow_multiple_subscriptions: required.allow_multiple_subscriptions,
            proration_behavior: required.proration_behavior,
            benefit_revocation_grace_period: required.benefit_revocation_grace_period,
            prevent_trial_abuse: required.prevent_trial_abuse,
            allow_customer_updates: required.allow_customer_updates,
        }
    }
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct OrganizationUpdate {
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name:
        presence_rs::Presence<crate::v2026_04::models::organizations::newtypes::OrganizationName>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub avatar_url: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationAvatarUrl,
    >,
    /// Public support email.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub email: presence_rs::Presence<String>,
    /// Official website of the organization.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub website: presence_rs::Presence<
        crate::v2026_04::models::organizations::newtypes::OrganizationWebsite,
    >,
    /// Links to social profiles.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub socials: presence_rs::Presence<
        Vec<crate::v2026_04::models::organizations::types::OrganizationSocialLink>,
    >,
    /// Additional, private, business details Polar needs about active
    /// organizations for compliance (KYC).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub details:
        presence_rs::Presence<crate::v2026_04::models::organizations::types::OrganizationDetails>,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub country: presence_rs::Presence<crate::v2026_04::models::common::enums::CountryAlpha2Input>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub feature_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationFeatureSettingsUpdate,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub subscription_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationSubscriptionSettings,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_email_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationCustomerEmailSettings,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_portal_settings: presence_rs::Presence<
        crate::v2026_04::models::common::types::OrganizationCustomerPortalSettings,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub dispute_settings: presence_rs::Presence<
        crate::v2026_04::models::organizations::types::OrganizationDisputeSettingsUpdate,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub embed_hosts: presence_rs::Presence<Vec<String>>,
    /// Default presentment currency for the organization
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_presentment_currency:
        presence_rs::Presence<crate::v2026_04::models::common::enums::PresentmentCurrency>,
    /// Default tax behavior applied on products.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub default_tax_behavior:
        presence_rs::Presence<crate::v2026_04::models::common::enums::TaxBehaviorOption>,
    /// Whether members must access this organization through its SSO
    /// connection. Turning this on requires an active SSO session for this
    /// organization and at least one enabled SSO connection.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub sso_enforced: presence_rs::Presence<bool>,
}
#[allow(clippy::type_complexity)]
impl OrganizationUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn avatar_url(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationAvatarUrl,
    ) -> Self {
        self.avatar_url = presence_rs::Presence::Some(value);
        self
    }

    /// Public support email.
    #[allow(clippy::should_implement_trait)]
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = presence_rs::Presence::Some(value.into());
        self
    }

    /// Official website of the organization.
    #[allow(clippy::should_implement_trait)]
    pub fn website(
        mut self,
        value: crate::v2026_04::models::organizations::newtypes::OrganizationWebsite,
    ) -> Self {
        self.website = presence_rs::Presence::Some(value);
        self
    }

    /// Links to social profiles.
    #[allow(clippy::should_implement_trait)]
    pub fn socials(
        mut self,
        value: Vec<crate::v2026_04::models::organizations::types::OrganizationSocialLink>,
    ) -> Self {
        self.socials = presence_rs::Presence::Some(value);
        self
    }

    /// Additional, private, business details Polar needs about active
    /// organizations for compliance (KYC).
    #[allow(clippy::should_implement_trait)]
    pub fn details(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationDetails,
    ) -> Self {
        self.details = presence_rs::Presence::Some(value);
        self
    }

    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[allow(clippy::should_implement_trait)]
    pub fn country(
        mut self,
        value: crate::v2026_04::models::common::enums::CountryAlpha2Input,
    ) -> Self {
        self.country = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn feature_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationFeatureSettingsUpdate,
    ) -> Self {
        self.feature_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn subscription_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationSubscriptionSettings,
    ) -> Self {
        self.subscription_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_email_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationCustomerEmailSettings,
    ) -> Self {
        self.customer_email_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_portal_settings(
        mut self,
        value: crate::v2026_04::models::common::types::OrganizationCustomerPortalSettings,
    ) -> Self {
        self.customer_portal_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn dispute_settings(
        mut self,
        value: crate::v2026_04::models::organizations::types::OrganizationDisputeSettingsUpdate,
    ) -> Self {
        self.dispute_settings = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn embed_hosts(mut self, value: Vec<String>) -> Self {
        self.embed_hosts = presence_rs::Presence::Some(value);
        self
    }

    /// Default presentment currency for the organization
    #[allow(clippy::should_implement_trait)]
    pub fn default_presentment_currency(
        mut self,
        value: crate::v2026_04::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.default_presentment_currency = presence_rs::Presence::Some(value);
        self
    }

    /// Default tax behavior applied on products.
    #[allow(clippy::should_implement_trait)]
    pub fn default_tax_behavior(
        mut self,
        value: crate::v2026_04::models::common::enums::TaxBehaviorOption,
    ) -> Self {
        self.default_tax_behavior = presence_rs::Presence::Some(value);
        self
    }

    /// Whether members must access this organization through its SSO
    /// connection. Turning this on requires an active SSO session for this
    /// organization and at least one enabled SSO connection.
    #[allow(clippy::should_implement_trait)]
    pub fn sso_enforced(mut self, value: bool) -> Self {
        self.sso_enforced = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct SsoEnforcementRequiresConnectionRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct SsoEnforcementRequiresConnection {
    pub error: monostate::MustBe!("SSOEnforcementRequiresConnection"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl SsoEnforcementRequiresConnection {
    pub fn new(required: SsoEnforcementRequiresConnectionRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
