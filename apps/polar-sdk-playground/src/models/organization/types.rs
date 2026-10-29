use presence_rs::Presence;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{
    common::{
        AccountId, OrganizationId, OrganizationName, OrganizationSlug, PayoutAccountId,
        enums::{SubscriptionProrationBehavior, TaxBehaviorOption},
        serde_helpers::deserialize_optional_non_null,
    },
    organization::enums::{
        OrganizationCountry, OrganizationSocialPlatforms, OrganizationStatus,
        OrganizationSubscriptionSettingsProrationBehavior, OrganizationSwitchingFrom,
    },
    pagination::Pagination,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListResourceOrganization {
    pub items: Vec<Organization>,
    pub pagination: Pagination,
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
    pub url: Url,
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
    pub avatar_url: Option<Url>,
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
    pub name: OrganizationName,
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
    pub website: Option<Url>,
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
