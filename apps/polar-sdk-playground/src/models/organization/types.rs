use std::num::NonZeroU64;

use presence_rs::Presence;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        common::{
            AccountId, OrganizationId, PayoutAccountId,
            enums::{
                CountryAlpha2Input, OrganizationSlug, PresentmentCurrency,
                SubscriptionProrationBehavior, TaxBehaviorOption,
            },
        },
        organization::enums::{
            OrganizationCountry, OrganizationSocialPlatforms, OrganizationSortField,
            OrganizationStatus, OrganizationSubscriptionSettingsProrationBehavior,
            OrganizationSwitchingFrom,
        },
        pagination::Pagination,
    },
    query::{EncodeQuery, Sorting},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListResourceOrganization {
    pub items: Vec<Organization>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone)]
pub struct OrganizationCreateRequired {
    pub name: String,
    pub slug: OrganizationSlug,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrganizationLegalEntity {
    Individual,
    Company { registered_name: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationSocialLink {
    pub platform: OrganizationSocialPlatforms,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationCreate {
    pub name: String,
    pub slug: OrganizationSlug,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub avatar_url: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub email: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub website: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub subscription_settings: Presence<OrganizationSubscriptionSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_email_settings: Presence<OrganizationCustomerEmailSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_portal_settings: Presence<OrganizationCustomerPortalSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub socials: Presence<Vec<OrganizationSocialLink>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub legal_entity: Presence<OrganizationLegalEntity>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub details: Presence<OrganizationDetails>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub country: Presence<CountryAlpha2Input>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub feature_settings: Presence<OrganizationFeatureSettingsUpdate>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub default_presentment_currency: Option<PresentmentCurrency>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub default_tax_behavior: Option<TaxBehaviorOption>,
}

impl OrganizationCreate {
    pub fn new(required: OrganizationCreateRequired) -> Self {
        Self {
            name: required.name,
            slug: required.slug,
            avatar_url: Presence::Absent,
            email: Presence::Absent,
            website: Presence::Absent,
            subscription_settings: Presence::Absent,
            customer_email_settings: Presence::Absent,
            customer_portal_settings: Presence::Absent,
            socials: Presence::Absent,
            legal_entity: Presence::Absent,
            details: Presence::Absent,
            country: Presence::Absent,
            feature_settings: Presence::Absent,
            default_presentment_currency: None,
            default_tax_behavior: None,
        }
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Presence::Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Presence::Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Presence::Some(value.into());
        self
    }

    pub fn subscription_settings(mut self, value: OrganizationSubscriptionSettings) -> Self {
        self.subscription_settings = Presence::Some(value);
        self
    }

    pub fn customer_email_settings(mut self, value: OrganizationCustomerEmailSettings) -> Self {
        self.customer_email_settings = Presence::Some(value);
        self
    }

    pub fn customer_portal_settings(mut self, value: OrganizationCustomerPortalSettings) -> Self {
        self.customer_portal_settings = Presence::Some(value);
        self
    }

    pub fn socials(mut self, value: Vec<OrganizationSocialLink>) -> Self {
        self.socials = Presence::Some(value);
        self
    }

    pub fn legal_entity(mut self, value: OrganizationLegalEntity) -> Self {
        self.legal_entity = Presence::Some(value);
        self
    }

    pub fn details(mut self, value: OrganizationDetails) -> Self {
        self.details = Presence::Some(value);
        self
    }

    pub fn country(mut self, value: CountryAlpha2Input) -> Self {
        self.country = Presence::Some(value);
        self
    }

    pub fn feature_settings(mut self, value: OrganizationFeatureSettingsUpdate) -> Self {
        self.feature_settings = Presence::Some(value);
        self
    }

    pub fn default_presentment_currency(mut self, value: PresentmentCurrency) -> Self {
        self.default_presentment_currency = Some(value);
        self
    }

    pub fn default_tax_behavior(mut self, value: TaxBehaviorOption) -> Self {
        self.default_tax_behavior = Some(value);
        self
    }
}

// Missing optional fields use None; explicitly supplied null must still satisfy
// T.
fn deserialize_optional_non_null<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationFeatureSettingsUpdate {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub member_model_enabled: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub checkout_localization_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub overview_metrics: Presence<Vec<String>>,
}

impl OrganizationFeatureSettingsUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn member_model_enabled(mut self, value: bool) -> Self {
        self.member_model_enabled = Some(value);
        self
    }

    pub fn checkout_localization_enabled(mut self, value: bool) -> Self {
        self.checkout_localization_enabled = Some(value);
        self
    }

    pub fn overview_metrics(mut self, value: Vec<String>) -> Self {
        self.overview_metrics = Presence::Some(value);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationDetails {
    /// Deprecated in the OpenAPI schema.
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub about: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub product_description: Presence<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub selling_categories: Option<Vec<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub pricing_models: Option<Vec<String>>,
    /// Deprecated in the OpenAPI schema.
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub intended_use: Presence<String>,
    /// Deprecated in the OpenAPI schema.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub customer_acquisition: Option<Vec<String>>,
    /// Deprecated in the OpenAPI schema. Revenue cannot be negative.
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub future_annual_revenue: Presence<u64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_optional_non_null"
    )]
    pub switching: Option<bool>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub switching_from: Presence<OrganizationSwitchingFrom>,
    /// Deprecated in the OpenAPI schema. Revenue cannot be negative.
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub previous_annual_revenue: Presence<u64>,
}

impl OrganizationDetails {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn about(mut self, value: impl Into<String>) -> Self {
        self.about = Presence::Some(value.into());
        self
    }

    pub fn product_description(mut self, value: impl Into<String>) -> Self {
        self.product_description = Presence::Some(value.into());
        self
    }

    pub fn selling_categories(mut self, value: Vec<String>) -> Self {
        self.selling_categories = Some(value);
        self
    }

    pub fn pricing_models(mut self, value: Vec<String>) -> Self {
        self.pricing_models = Some(value);
        self
    }

    pub fn intended_use(mut self, value: impl Into<String>) -> Self {
        self.intended_use = Presence::Some(value.into());
        self
    }

    pub fn customer_acquisition(mut self, value: Vec<String>) -> Self {
        self.customer_acquisition = Some(value);
        self
    }

    pub fn future_annual_revenue(mut self, value: u64) -> Self {
        self.future_annual_revenue = Presence::Some(value);
        self
    }

    pub fn switching(mut self, value: bool) -> Self {
        self.switching = Some(value);
        self
    }

    pub fn switching_from(mut self, value: OrganizationSwitchingFrom) -> Self {
        self.switching_from = Presence::Some(value);
        self
    }

    pub fn previous_annual_revenue(mut self, value: u64) -> Self {
        self.previous_annual_revenue = Presence::Some(value);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationUpdate {
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub name: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub avatar_url: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub email: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub website: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub socials: Presence<Vec<OrganizationSocialLink>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub details: Presence<OrganizationDetails>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub country: Presence<CountryAlpha2Input>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub feature_settings: Presence<OrganizationFeatureSettingsUpdate>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub subscription_settings: Presence<OrganizationSubscriptionSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_email_settings: Presence<OrganizationCustomerEmailSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub customer_portal_settings: Presence<OrganizationCustomerPortalSettings>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub dispute_settings: Presence<OrganizationDisputeSettingsUpdate>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub embed_hosts: Presence<Vec<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub default_presentment_currency: Presence<PresentmentCurrency>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub default_tax_behavior: Presence<TaxBehaviorOption>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub sso_enforced: Presence<bool>,
}

impl OrganizationUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Presence::Some(value.into());
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Presence::Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Presence::Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Presence::Some(value.into());
        self
    }

    pub fn socials(mut self, value: Vec<OrganizationSocialLink>) -> Self {
        self.socials = Presence::Some(value);
        self
    }

    pub fn details(mut self, value: OrganizationDetails) -> Self {
        self.details = Presence::Some(value);
        self
    }

    pub fn country(mut self, value: CountryAlpha2Input) -> Self {
        self.country = Presence::Some(value);
        self
    }

    pub fn feature_settings(mut self, value: OrganizationFeatureSettingsUpdate) -> Self {
        self.feature_settings = Presence::Some(value);
        self
    }

    pub fn subscription_settings(mut self, value: OrganizationSubscriptionSettings) -> Self {
        self.subscription_settings = Presence::Some(value);
        self
    }

    pub fn customer_email_settings(mut self, value: OrganizationCustomerEmailSettings) -> Self {
        self.customer_email_settings = Presence::Some(value);
        self
    }

    pub fn customer_portal_settings(mut self, value: OrganizationCustomerPortalSettings) -> Self {
        self.customer_portal_settings = Presence::Some(value);
        self
    }

    pub fn dispute_settings(mut self, value: OrganizationDisputeSettingsUpdate) -> Self {
        self.dispute_settings = Presence::Some(value);
        self
    }

    pub fn embed_hosts(mut self, value: Vec<String>) -> Self {
        self.embed_hosts = Presence::Some(value);
        self
    }

    pub fn default_presentment_currency(mut self, value: PresentmentCurrency) -> Self {
        self.default_presentment_currency = Presence::Some(value);
        self
    }

    pub fn default_tax_behavior(mut self, value: TaxBehaviorOption) -> Self {
        self.default_tax_behavior = Presence::Some(value);
        self
    }

    pub fn sso_enforced(mut self, value: bool) -> Self {
        self.sso_enforced = Presence::Some(value);
        self
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationDisputeSettingsUpdate {
    /// Auto-accept threshold in USD cents. The API permits 1..=10_000; null
    /// disables it.
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub auto_accept_below_amount: Presence<i64>,
}

impl OrganizationDisputeSettingsUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auto_accept_below_amount(mut self, value: i64) -> Self {
        self.auto_accept_below_amount = Presence::Some(value);
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Organization {
    /// ID of the transactions account.
    pub account_id: Option<AccountId>,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    pub avatar_url: Option<String>,
    pub capabilities: OrganizationCapabilities,
    /// Two-letter country code (ISO 3166-1 alpha-2).
    #[serde(skip_serializing_if = "presence_rs::Presence::is_absent", default)]
    pub country: presence_rs::Presence<OrganizationCountry>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    pub customer_email_settings: OrganizationCustomerEmailSettings,
    pub customer_portal_settings: OrganizationCustomerPortalSettings,
    /// Default presentment currency. Used as fallback in checkout and customer
    /// portal, if the customer's local currency is not available.
    pub default_presentment_currency: String,
    pub default_tax_behavior: TaxBehaviorOption,
    /// When the business details were submitted for review.
    pub details_submitted_at: Option<jiff::Timestamp>,
    pub dispute_settings: OrganizationDisputeSettings,
    /// Public support email.
    pub email: Option<String>,
    /// Hosts allowed to embed this organization's checkout. An entry is a host
    /// and an optional port, without a scheme: HTTPS is always allowed, and
    /// HTTP too for local hosts — `localhost`, any `.localhost` or `.local`
    /// name, and loopback or private addresses. `*.example.com` matches any
    /// subdomain, but not `example.com` itself. An app origin such as
    /// `chrome-extension://abcdef` carries its scheme, having no host to match
    /// on.
    pub embed_hosts: Vec<String>,
    /// Organization feature settings
    pub feature_settings: Option<OrganizationFeatureSettings>,
    /// The ID of the object.
    pub id: OrganizationId,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// When Polar requested that the organization review and resubmit its
    /// onboarding information, if applicable.
    pub onboarding_resubmission_requested_at: Option<jiff::Timestamp>,
    /// ID of the payout account.
    pub payout_account_id: Option<PayoutAccountId>,
    pub proration_behavior: SubscriptionProrationBehavior,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: OrganizationSlug,
    /// Links to social profiles.
    pub socials: Vec<OrganizationSocialLink>,
    /// Whether members must access this organization through its SSO
    /// connection.
    pub sso_enforced: bool,
    pub status: OrganizationStatus,
    pub subscription_settings: OrganizationSubscriptionSettings,
    /// Official website of the organization.
    pub website: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationCapabilities {
    /// Whether the organization can access the API.
    pub api_access: bool,
    /// Whether the organization can accept new checkout payments.
    pub checkout_payments: bool,
    /// Whether the organization can access the dashboard.
    pub dashboard_access: bool,
    /// Whether the organization can withdraw its balance.
    pub payouts: bool,
    /// Whether the organization can issue refunds.
    pub refunds: bool,
    /// Whether the organization can process subscription renewals.
    pub subscription_renewals: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationSubscriptionSettings {
    pub allow_customer_updates: bool,
    pub allow_multiple_subscriptions: bool,
    pub benefit_revocation_grace_period: i64,
    pub prevent_trial_abuse: bool,
    pub proration_behavior: OrganizationSubscriptionSettingsProrationBehavior,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationCustomerEmailSettings {
    pub order_confirmation: bool,
    pub payment_method_expiration_reminder: bool,
    pub subscription_cancellation: bool,
    pub subscription_confirmation: bool,
    pub subscription_cycled: bool,
    pub subscription_cycled_after_trial: bool,
    pub subscription_past_due: bool,
    pub subscription_paused: bool,
    pub subscription_renewal_reminder: bool,
    pub subscription_resumed: bool,
    pub subscription_revoked: bool,
    pub subscription_trial_conversion_reminder: bool,
    pub subscription_uncanceled: bool,
    pub subscription_updated: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationCustomerPortalSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerPortalCustomerSettings>,
    pub subscription: CustomerPortalSubscriptionSettings,
    pub usage: CustomerPortalUsageSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CustomerPortalCustomerSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_email_change: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerPortalUsageSettings {
    pub show: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerPortalSubscriptionSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<bool>,
    pub update_plan: bool,
    pub update_seats: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_units: Option<bool>,
}

/// `auto_accept_below_amount` is in Polar's settlement currency (USD).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrganizationDisputeSettings {
    pub auto_accept_below_amount: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct OrganizationFeatureSettings {
    /// If this organization has checkout localization enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_localization_enabled: Option<bool>,
    /// If this organization has the split product navigation (Billing / Compass
    /// / Customers) enabled in the dashboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compass_enabled: Option<bool>,
    /// If this organization can set a threshold below which Polar concedes
    /// disputes on its behalf. Requires `disputes_enabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_auto_accept_enabled: Option<bool>,
    /// If this organization has the disputes dashboard enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputes_enabled: Option<bool>,
    /// If this organization has issue funding enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_funding_enabled: Option<bool>,
    /// If this organization has the Member model enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_model_enabled: Option<bool>,
    /// If this organization can migrate its billing from another provider (e.g.
    /// Stripe) to Polar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_migration_enabled: Option<bool>,
    /// If this organization can set a separate meter cycle on recurring
    /// products (a meter interval independent of the billing interval).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_cycling_enabled: Option<bool>,
    /// If this organization can create and finalize draft orders via the API
    /// (off-session charges against a saved payment method).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session_charges_enabled: Option<bool>,
    /// Ordered list of metric slugs shown on the dashboard overview.
    #[serde(skip_serializing_if = "presence_rs::Presence::is_absent", default)]
    pub overview_metrics: presence_rs::Presence<Vec<String>>,
    /// If this organization has preview access to new features enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_access_enabled: Option<bool>,
    /// If this organization has access to reset proration behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_proration_behavior_enabled: Option<bool>,
    /// Enables the slack shared channel benefit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_benefit_enabled: Option<bool>,
    /// If this organization has single sign-on configuration enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,
    /// If this organization has Wallets enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallets_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct ListOrganizationsParams {
    pub slug: Option<OrganizationSlug>,
    pub page: Option<NonZeroU64>,
    pub limit: Option<NonZeroU64>,
    pub sorting: Vec<(OrganizationSortField, Sorting)>,
}

impl EncodeQuery for ListOrganizationsParams {
    fn scalar_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();

        if let Some(slug) = &self.slug {
            params.push(("slug", slug.to_string()));
        }

        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }

        params
    }

    fn sorting_values(&self) -> Vec<String> {
        self.sorting.iter().map(|(field, direction)| direction.encode(field.as_str())).collect()
    }
}

#[cfg(test)]
mod create_payload_tests {
    use serde_json::json;

    use super::{
        OrganizationCreate, OrganizationCreateRequired, OrganizationLegalEntity, OrganizationSlug,
        OrganizationSocialLink, OrganizationSocialPlatforms, Presence,
    };

    fn payload() -> OrganizationCreate {
        OrganizationCreate::new(OrganizationCreateRequired {
            name: "Acme".into(),
            slug: OrganizationSlug::from("acme".to_owned()),
        })
    }

    #[test]
    fn constructor_omits_optional_fields_and_deserialization_restores_absence() {
        let expected = json!({"name": "Acme", "slug": "acme"});
        assert_eq!(serde_json::to_value(payload()).unwrap(), expected);

        let decoded: OrganizationCreate = serde_json::from_value(expected).unwrap();
        assert_eq!(decoded.name, "Acme");
        assert_eq!(decoded.slug.as_ref(), "acme");
        assert!(matches!(decoded.avatar_url, Presence::Absent));
        assert!(matches!(decoded.email, Presence::Absent));
        assert!(matches!(decoded.website, Presence::Absent));
        assert!(matches!(decoded.subscription_settings, Presence::Absent));
        assert!(matches!(decoded.customer_email_settings, Presence::Absent));
        assert!(matches!(decoded.customer_portal_settings, Presence::Absent));
        assert!(matches!(decoded.socials, Presence::Absent));
        assert!(matches!(decoded.legal_entity, Presence::Absent));
    }

    #[test]
    fn preserves_explicit_null_for_each_optional_field() {
        let mut payload = payload();
        payload.avatar_url = Presence::Null;
        payload.email = Presence::Null;
        payload.website = Presence::Null;
        let expected = json!({
            "name": "Acme", "slug": "acme",
            "avatar_url": null, "email": null, "website": null
        });
        assert_eq!(serde_json::to_value(payload).unwrap(), expected);

        let decoded: OrganizationCreate = serde_json::from_value(expected).unwrap();
        assert!(matches!(decoded.avatar_url, Presence::Null));
        assert!(matches!(decoded.email, Presence::Null));
        assert!(matches!(decoded.website, Presence::Null));
    }

    #[test]
    fn preserves_values_for_each_optional_field() {
        let payload = payload()
            .avatar_url("https://example.com/avatar.png")
            .email("old@example.com")
            .email("support@example.com".to_owned())
            .website("https://example.com");
        let expected = json!({
            "name": "Acme", "slug": "acme",
            "avatar_url": "https://example.com/avatar.png",
            "email": "support@example.com",
            "website": "https://example.com"
        });
        assert_eq!(serde_json::to_value(payload).unwrap(), expected);

        let decoded: OrganizationCreate = serde_json::from_value(expected).unwrap();
        assert!(
            matches!(decoded.avatar_url, Presence::Some(value) if value == "https://example.com/avatar.png")
        );
        assert!(matches!(decoded.email, Presence::Some(value) if value == "support@example.com"));
        assert!(matches!(decoded.website, Presence::Some(value) if value == "https://example.com"));
    }

    #[test]
    fn rejects_missing_or_null_required_fields() {
        for value in [
            json!({"slug": "acme"}),
            json!({"name": "Acme"}),
            json!({"name": null, "slug": "acme"}),
            json!({"name": "Acme", "slug": null}),
        ] {
            assert!(
                serde_json::from_value::<OrganizationCreate>(value.clone()).is_err(),
                "expected invalid required fields to be rejected: {value}"
            );
        }
    }

    #[test]
    fn empty_socials_are_present_instead_of_omitted() {
        let omitted = serde_json::to_value(payload()).unwrap();
        assert!(omitted.get("socials").is_none());
        let empty = serde_json::to_value(payload().socials(vec![])).unwrap();
        assert_eq!(empty, json!({"name":"Acme", "slug":"acme", "socials":[]}));
        let decoded: OrganizationCreate = serde_json::from_value(empty).unwrap();
        assert!(matches!(decoded.socials, Presence::Some(links) if links.is_empty()));
    }

    #[test]
    fn socials_preserve_order_and_replace_the_previous_list() {
        let payload = payload()
            .socials(vec![OrganizationSocialLink {
                platform: OrganizationSocialPlatforms::Other,
                url: "https://old.example.com".into(),
            }])
            .socials(vec![
                OrganizationSocialLink {
                    platform: OrganizationSocialPlatforms::Github,
                    url: "https://github.com/acme".into(),
                },
                OrganizationSocialLink {
                    platform: OrganizationSocialPlatforms::X,
                    url: "https://x.com/acme".into(),
                },
            ]);
        let expected = json!({
            "name":"Acme", "slug":"acme",
            "socials":[
                {"platform":"github","url":"https://github.com/acme"},
                {"platform":"x","url":"https://x.com/acme"}
            ]
        });
        assert_eq!(serde_json::to_value(payload).unwrap(), expected);
        let decoded: OrganizationCreate = serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
    }

    #[test]
    fn rejects_incomplete_social_links_and_unknown_platforms() {
        for link in [
            json!({"url":"https://github.com/acme"}),
            json!({"platform":"github"}),
            json!({"platform":"unknown","url":"https://example.com"}),
        ] {
            let result = serde_json::from_value::<OrganizationCreate>(json!({
                "name":"Acme", "slug":"acme", "socials":[link]
            }));
            assert!(result.is_err(), "invalid social link must fail deserialization");
        }
    }

    #[test]
    fn legal_entities_use_the_expected_tagged_shapes() {
        for (entity, expected) in [
            (OrganizationLegalEntity::Individual, json!({"type":"individual"})),
            (
                OrganizationLegalEntity::Company { registered_name: "Acme Ltd".into() },
                json!({"type":"company", "registered_name":"Acme Ltd"}),
            ),
        ] {
            let encoded = serde_json::to_value(payload().legal_entity(entity)).unwrap();
            assert_eq!(encoded, json!({"name":"Acme", "slug":"acme", "legal_entity":expected}));
            let decoded: OrganizationCreate = serde_json::from_value(encoded).unwrap();
            match decoded.legal_entity {
                Presence::Some(OrganizationLegalEntity::Individual) => {
                    assert_eq!(expected["type"], "individual");
                }
                Presence::Some(OrganizationLegalEntity::Company { registered_name }) => {
                    assert_eq!(expected["type"], "company");
                    assert_eq!(registered_name, "Acme Ltd");
                }
                other => panic!("expected a populated legal entity, got {other:?}"),
            }
        }
    }

    #[test]
    fn rejects_missing_or_invalid_legal_entity_discriminators_and_company_names() {
        for entity in [
            json!({}),
            json!({"registered_name":"Acme Ltd"}),
            json!({"type":"unknown"}),
            json!({"type":null}),
            json!({"type":"company"}),
            json!({"type":"company", "registered_name":null}),
        ] {
            let result = serde_json::from_value::<OrganizationCreate>(json!({
                "name":"Acme", "slug":"acme", "legal_entity":entity
            }));
            assert!(result.is_err(), "invalid legal entity must fail deserialization");
        }
    }

    #[test]
    fn legal_entity_setter_replaces_the_previous_variant() {
        let company = payload()
            .legal_entity(OrganizationLegalEntity::Individual)
            .legal_entity(OrganizationLegalEntity::Company { registered_name: "Acme Ltd".into() });
        assert_eq!(
            serde_json::to_value(&company).unwrap()["legal_entity"],
            json!({"type":"company", "registered_name":"Acme Ltd"})
        );
        let individual = company.legal_entity(OrganizationLegalEntity::Individual);
        assert_eq!(
            serde_json::to_value(individual).unwrap()["legal_entity"],
            json!({"type":"individual"})
        );
    }
}

#[cfg(test)]
mod list_params_tests {
    use super::{ListOrganizationsParams, NonZeroU64, OrganizationSlug, OrganizationSortField};
    use crate::query::{EncodeQuery, Sorting};

    #[test]
    fn defaults_omit_all_query_parameters() {
        assert_eq!(ListOrganizationsParams::default().encode_query(), "");
    }

    #[test]
    fn encodes_filters_pagination_and_sorting_in_priority_order() {
        let params = ListOrganizationsParams {
            slug: Some(OrganizationSlug::from("acme".to_owned())),
            page: NonZeroU64::new(2),
            limit: NonZeroU64::new(20),
            sorting: vec![
                (OrganizationSortField::Name, Sorting::Desc),
                (OrganizationSortField::CreatedAt, Sorting::Asc),
            ],
        };

        assert_eq!(
            params.encode_query(),
            "slug=acme&page=2&limit=20&sorting=-name&sorting=created_at"
        );
    }

    #[test]
    fn escapes_slug_and_omits_unspecified_parameters() {
        let params = ListOrganizationsParams {
            slug: Some(OrganizationSlug::from("acme & sons+50%?x=1".to_owned())),
            ..Default::default()
        };

        assert_eq!(params.encode_query(), "slug=acme+%26+sons%2B50%25%3Fx%3D1");
    }

    #[test]
    fn encodes_every_sort_field_in_both_directions() {
        for (field, expected) in [
            (OrganizationSortField::CreatedAt, "sorting=created_at&sorting=-created_at"),
            (OrganizationSortField::Slug, "sorting=slug&sorting=-slug"),
            (OrganizationSortField::Name, "sorting=name&sorting=-name"),
            (
                OrganizationSortField::NextReviewThreshold,
                "sorting=next_review_threshold&sorting=-next_review_threshold",
            ),
            (OrganizationSortField::DaysInStatus, "sorting=days_in_status&sorting=-days_in_status"),
        ] {
            let params = ListOrganizationsParams {
                sorting: vec![(field, Sorting::Asc), (field, Sorting::Desc)],
                ..Default::default()
            };

            assert_eq!(params.encode_query(), expected, "incorrect encoding for {field:?}");
        }
    }
}
