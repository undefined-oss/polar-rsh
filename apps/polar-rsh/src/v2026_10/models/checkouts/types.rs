#[derive(Debug, Clone)]
pub struct AlreadyActiveSubscriptionErrorRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct AlreadyActiveSubscriptionError {
    pub error: monostate::MustBe!("AlreadyActiveSubscriptionError"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl AlreadyActiveSubscriptionError {
    pub fn new(required: AlreadyActiveSubscriptionErrorRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutRequired {
    ///The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CheckoutId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    ///Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    ///
    ///        Status of the checkout session.
    ///
    ///        - Open: the checkout session was opened.
    ///        - Expired: the checkout session was expired and is no more accessible.
    ///        - Confirmed: the user on the checkout session clicked Pay. This is not indicative of the payment's success status.
    ///        - Failed: the checkout definitely failed for technical reasons and cannot be retried. In most cases, this state is never reached.
    ///        - Succeeded: the payment on the checkout was performed successfully.
    ///
    pub status: crate::v2026_10::models::checkouts::enums::CheckoutStatus,
    ///Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    ///URL where the customer can access the checkout session.
    pub url: String,
    ///Expiration date and time of the checkout session.
    pub expires_at: jiff::Timestamp,
    ///URL where the customer will be redirected after a successful payment.
    pub success_url: String,
    ///When set, a back button will be shown in the checkout to return to this URL.
    pub return_url: Option<String>,
    ///When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    pub embed_origin: Option<String>,
    ///Amount in cents, before discounts and taxes.
    pub amount: i64,
    ///Predefined number of units (works with unit-based pricing only)
    pub units: Option<i64>,
    ///Minimum number of units (works with unit-based pricing only)
    pub min_units: Option<i64>,
    ///Maximum number of units (works with unit-based pricing only)
    pub max_units: Option<i64>,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    pub tax_amount: Option<i64>,
    ///Tax behavior of the checkout. `inclusive` means the price includes tax, `exclusive` means tax is added on top. If `null`, tax is not yet calculated.
    pub tax_behavior: Option<crate::v2026_10::models::checkouts::enums::TaxBehavior>,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Currency code of the checkout session.
    pub currency: String,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    pub allow_trial: Option<bool>,
    ///Interval unit of the trial period, if any. This value is either set from the checkout, if `trial_interval` is set, or from the selected product.
    pub active_trial_interval: Option<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///Number of interval units of the trial period, if any. This value is either set from the checkout, if `trial_interval_count` is set, or from the selected product.
    pub active_trial_interval_count: Option<i64>,
    ///End date and time of the trial period, if any.
    pub trial_end: Option<jiff::Timestamp>,
    ///ID of the organization owning the checkout session.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    ///ID of the product to checkout.
    pub product_id: Option<crate::v2026_10::models::common::ids::ProductId>,
    ///ID of the product price to checkout.
    pub product_price_id: Option<crate::v2026_10::models::common::ids::ProductPriceId>,
    ///ID of the discount applied to the checkout.
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    ///Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    ///Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    ///Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    ///Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    ///Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    pub customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    ///Name of the customer.
    pub customer_name: Option<String>,
    ///Email address of the customer.
    pub customer_email: Option<String>,
    pub customer_ip_address: Option<String>,
    pub customer_billing_name: Option<String>,
    pub customer_billing_address: Option<
        crate::v2026_10::models::common::types::Address,
    >,
    pub customer_tax_id: Option<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    pub payment_method_type: Option<String>,
    pub payment_processor_metadata: std::collections::BTreeMap<String, String>,
    ///Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: crate::v2026_10::models::checkouts::types::CheckoutBillingAddressFields,
    ///The interval unit for the trial period.
    pub trial_interval: Option<crate::v2026_10::models::common::enums::TrialInterval>,
    ///The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
    ///ID of the customer in your system. If a matching customer exists on Polar, the resulting order will be linked to this customer. Otherwise, a new customer will be created with this external ID set.
    pub external_customer_id: Option<String>,
    ///List of products available to select.
    pub products: Vec<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Product selected to checkout.
    pub product: Option<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Price of the selected product.
    pub product_price: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutProductPriceAnyOf0Union,
    >,
    ///Mapping of product IDs to their list of prices.
    pub prices: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::CheckoutPricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutPricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
    pub discount: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutDiscountAnyOf0Union,
    >,
    pub subscription_id: Option<crate::v2026_10::models::common::ids::SubscriptionId>,
    pub attached_custom_fields: Option<
        Vec<crate::v2026_10::models::common::types::AttachedCustomField>,
    >,
    pub customer_metadata: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::checkouts::unions::CheckoutCustomerMetadataAdditionalPropertiesUnion,
    >,
}
/// Checkout session data retrieved using an access token.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct Checkout {
    ///The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CheckoutId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    ///
    ///        Status of the checkout session.
    ///
    ///        - Open: the checkout session was opened.
    ///        - Expired: the checkout session was expired and is no more accessible.
    ///        - Confirmed: the user on the checkout session clicked Pay. This is not indicative of the payment's success status.
    ///        - Failed: the checkout definitely failed for technical reasons and cannot be retried. In most cases, this state is never reached.
    ///        - Succeeded: the payment on the checkout was performed successfully.
    ///
    pub status: crate::v2026_10::models::checkouts::enums::CheckoutStatus,
    ///Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    ///URL where the customer can access the checkout session.
    pub url: String,
    ///Expiration date and time of the checkout session.
    pub expires_at: jiff::Timestamp,
    ///URL where the customer will be redirected after a successful payment.
    pub success_url: String,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub return_url: Option<String>,
    ///When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub embed_origin: Option<String>,
    ///Amount in cents, before discounts and taxes.
    pub amount: i64,
    ///Predefined number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    ///Minimum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub min_seats: presence_rs::Presence<i64>,
    ///Maximum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_seats: presence_rs::Presence<i64>,
    ///Predefined number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    ///Minimum number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub min_units: Option<i64>,
    ///Maximum number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_units: Option<i64>,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_amount: Option<i64>,
    ///Tax behavior of the checkout. `inclusive` means the price includes tax, `exclusive` means tax is added on top. If `null`, tax is not yet calculated.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2026_10::models::checkouts::enums::TaxBehavior>,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Currency code of the checkout session.
    pub currency: String,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub allow_trial: Option<bool>,
    ///Interval unit of the trial period, if any. This value is either set from the checkout, if `trial_interval` is set, or from the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub active_trial_interval: Option<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///Number of interval units of the trial period, if any. This value is either set from the checkout, if `trial_interval_count` is set, or from the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub active_trial_interval_count: Option<i64>,
    ///End date and time of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    ///ID of the organization owning the checkout session.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    ///ID of the product to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_id: Option<crate::v2026_10::models::common::ids::ProductId>,
    ///ID of the product price to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price_id: Option<crate::v2026_10::models::common::ids::ProductPriceId>,
    ///ID of the discount applied to the checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    ///Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    ///Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    ///Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    ///Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    ///Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    ///Name of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_name: Option<String>,
    ///Email address of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_ip_address: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_billing_address: Option<
        crate::v2026_10::models::common::types::Address,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_tax_id: Option<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub payment_method_type: Option<String>,
    pub payment_processor_metadata: std::collections::BTreeMap<String, String>,
    ///Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: crate::v2026_10::models::checkouts::types::CheckoutBillingAddressFields,
    ///The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2026_10::models::common::enums::TrialInterval>,
    ///The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
    ///ID of the customer in your system. If a matching customer exists on Polar, the resulting order will be linked to this customer. Otherwise, a new customer will be created with this external ID set.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_customer_id: Option<String>,
    ///List of products available to select.
    pub products: Vec<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Product selected to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product: Option<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Price of the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutProductPriceAnyOf0Union,
    >,
    ///Mapping of product IDs to their list of prices.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub prices: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::CheckoutPricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutPricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutDiscountAnyOf0Union,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub subscription_id: Option<crate::v2026_10::models::common::ids::SubscriptionId>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub attached_custom_fields: Option<
        Vec<crate::v2026_10::models::common::types::AttachedCustomField>,
    >,
    pub customer_metadata: std::collections::BTreeMap<
        String,
        crate::v2026_10::models::checkouts::unions::CheckoutCustomerMetadataAdditionalPropertiesUnion,
    >,
}
#[allow(clippy::type_complexity)]
impl Checkout {
    pub fn new(required: CheckoutRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            custom_field_data: None,
            payment_processor: required.payment_processor,
            status: required.status,
            client_secret: required.client_secret,
            url: required.url,
            expires_at: required.expires_at,
            success_url: required.success_url,
            return_url: required.return_url,
            embed_origin: required.embed_origin,
            amount: required.amount,
            seats: presence_rs::Presence::Absent,
            min_seats: presence_rs::Presence::Absent,
            max_seats: presence_rs::Presence::Absent,
            units: required.units,
            min_units: required.min_units,
            max_units: required.max_units,
            discount_amount: required.discount_amount,
            net_amount: required.net_amount,
            tax_amount: required.tax_amount,
            tax_behavior: required.tax_behavior,
            total_amount: required.total_amount,
            currency: required.currency,
            allow_trial: required.allow_trial,
            active_trial_interval: required.active_trial_interval,
            active_trial_interval_count: required.active_trial_interval_count,
            trial_end: required.trial_end,
            organization_id: required.organization_id,
            product_id: required.product_id,
            product_price_id: required.product_price_id,
            discount_id: required.discount_id,
            allow_discount_codes: required.allow_discount_codes,
            require_billing_address: required.require_billing_address,
            is_discount_applicable: required.is_discount_applicable,
            is_free_product_price: required.is_free_product_price,
            is_payment_required: required.is_payment_required,
            is_payment_setup_required: required.is_payment_setup_required,
            is_payment_form_required: required.is_payment_form_required,
            customer_id: required.customer_id,
            is_business_customer: required.is_business_customer,
            customer_name: required.customer_name,
            customer_email: required.customer_email,
            customer_ip_address: required.customer_ip_address,
            customer_billing_name: required.customer_billing_name,
            customer_billing_address: required.customer_billing_address,
            customer_tax_id: required.customer_tax_id,
            locale: presence_rs::Presence::Absent,
            payment_method_type: required.payment_method_type,
            payment_processor_metadata: required.payment_processor_metadata,
            billing_address_fields: required.billing_address_fields,
            trial_interval: required.trial_interval,
            trial_interval_count: required.trial_interval_count,
            metadata: required.metadata,
            external_customer_id: required.external_customer_id,
            products: required.products,
            product: required.product,
            product_price: required.product_price,
            prices: required.prices,
            discount: required.discount,
            subscription_id: required.subscription_id,
            attached_custom_fields: required.attached_custom_fields,
            customer_metadata: required.customer_metadata,
        }
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// Predefined number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Minimum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn min_seats(mut self, value: i64) -> Self {
        self.min_seats = presence_rs::Presence::Some(value);
        self
    }

    /// Maximum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn max_seats(mut self, value: i64) -> Self {
        self.max_seats = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutBillingAddressFieldsRequired {
    pub country: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub state: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub city: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub postal_code: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub line1: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub line2: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutBillingAddressFields {
    pub country: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub state: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub city: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub postal_code: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub line1: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
    pub line2: crate::v2026_10::models::checkouts::enums::BillingAddressFieldMode,
}
#[allow(clippy::type_complexity)]
impl CheckoutBillingAddressFields {
    pub fn new(required: CheckoutBillingAddressFieldsRequired) -> Self {
        Self {
            country: required.country,
            state: required.state,
            city: required.city,
            postal_code: required.postal_code,
            line1: required.line1,
            line2: required.line2,
        }
    }
}
/// Confirm a checkout session using a Stripe confirmation token.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CheckoutConfirmStripe {
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutConfirmStripeCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///ID of the product to checkout. Must be present in the checkout's product list.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ProductId,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeAmount,
    >,
    ///Number of seats for seat-based pricing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeSeats,
    >,
    ///Number of units for unit-based pricing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeUnits,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub is_business_customer: presence_rs::Presence<bool>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeCustomerName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeCustomerBillingName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_address: presence_rs::Presence<
        crate::v2026_10::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub payment_method_type: presence_rs::Presence<String>,
    ///Discount code to apply to the checkout.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_code: presence_rs::Presence<String>,
    ///Disable the trial period for the checkout session. It's mainly useful when the trial is blocked because the customer already redeemed one.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub allow_trial: presence_rs::Presence<monostate::MustBe!(false)>,
    ///ID of the Stripe confirmation token. Required for fixed prices and custom prices.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub confirmation_token_id: presence_rs::Presence<String>,
    ///ID of the product price to checkout. Must correspond to a price present in the checkout's product list.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_price_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ProductPriceId,
    >,
}
#[allow(clippy::type_complexity)]
impl CheckoutConfirmStripe {
    pub fn new() -> Self {
        Self::default()
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutConfirmStripeCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// ID of the product to checkout. Must be present in the checkout's product
    /// list.
    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: crate::v2026_10::models::common::ids::ProductId) -> Self {
        self.product_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeAmount,
    ) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    /// Number of seats for seat-based pricing.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Number of units for unit-based pricing.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn is_business_customer(mut self, value: bool) -> Self {
        self.is_business_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeCustomerName,
    ) -> Self {
        self.customer_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_email(mut self, value: impl Into<String>) -> Self {
        self.customer_email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutConfirmStripeCustomerBillingName,
    ) -> Self {
        self.customer_billing_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_address(
        mut self,
        value: crate::v2026_10::models::common::types::AddressInput,
    ) -> Self {
        self.customer_billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_tax_id(mut self, value: impl Into<String>) -> Self {
        self.customer_tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// Payment method type selected by the customer in the checkout form, e.g.
    /// `card`, `apple_pay` or `upi`.
    #[allow(clippy::should_implement_trait)]
    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = presence_rs::Presence::Some(value.into());
        self
    }

    /// Discount code to apply to the checkout.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_code(mut self, value: impl Into<String>) -> Self {
        self.discount_code = presence_rs::Presence::Some(value.into());
        self
    }

    /// Disable the trial period for the checkout session. It's mainly useful
    /// when the trial is blocked because the customer already redeemed one.
    #[allow(clippy::should_implement_trait)]
    pub fn allow_trial(mut self, value: monostate::MustBe!(false)) -> Self {
        self.allow_trial = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the Stripe confirmation token. Required for fixed prices and
    /// custom prices.
    #[allow(clippy::should_implement_trait)]
    pub fn confirmation_token_id(mut self, value: impl Into<String>) -> Self {
        self.confirmation_token_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// ID of the product price to checkout. Must correspond to a price present
    /// in the checkout's product list.
    #[allow(clippy::should_implement_trait)]
    pub fn product_price_id(
        mut self,
        value: crate::v2026_10::models::common::ids::ProductPriceId,
    ) -> Self {
        self.product_price_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutDiscountFixedOnceForeverDurationRequired {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    pub code: Option<String>,
}
/// Schema for a fixed amount discount that is applied once or forever.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutDiscountFixedOnceForeverDuration {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CheckoutDiscountFixedOnceForeverDuration {
    pub fn new(required: CheckoutDiscountFixedOnceForeverDurationRequired) -> Self {
        Self {
            duration: required.duration,
            r#type: required.r#type,
            amount: required.amount,
            currency: required.currency,
            amounts: required.amounts,
            id: required.id,
            name: required.name,
            code: required.code,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutDiscountFixedRepeatDurationRequired {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    pub code: Option<String>,
}
/// Schema for a fixed amount discount that is applied on every invoice
/// for a certain number of months.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutDiscountFixedRepeatDuration {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CheckoutDiscountFixedRepeatDuration {
    pub fn new(required: CheckoutDiscountFixedRepeatDurationRequired) -> Self {
        Self {
            duration: required.duration,
            duration_in_months: required.duration_in_months,
            r#type: required.r#type,
            amount: required.amount,
            currency: required.currency,
            amounts: required.amounts,
            id: required.id,
            name: required.name,
            code: required.code,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutDiscountPercentageOnceForeverDurationRequired {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    pub code: Option<String>,
}
/// Schema for a percentage discount that is applied once or forever.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutDiscountPercentageOnceForeverDuration {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CheckoutDiscountPercentageOnceForeverDuration {
    pub fn new(required: CheckoutDiscountPercentageOnceForeverDurationRequired) -> Self {
        Self {
            duration: required.duration,
            r#type: required.r#type,
            basis_points: required.basis_points,
            id: required.id,
            name: required.name,
            code: required.code,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutDiscountPercentageRepeatDurationRequired {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    pub code: Option<String>,
}
/// Schema for a percentage discount that is applied on every invoice
/// for a certain number of months.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutDiscountPercentageRepeatDuration {
    pub duration: crate::v2026_10::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2026_10::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::DiscountId,
    pub name: String,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CheckoutDiscountPercentageRepeatDuration {
    pub fn new(required: CheckoutDiscountPercentageRepeatDurationRequired) -> Self {
        Self {
            duration: required.duration,
            duration_in_months: required.duration_in_months,
            r#type: required.r#type,
            basis_points: required.basis_points,
            id: required.id,
            name: required.name,
            code: required.code,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutLockedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct CheckoutLocked {
    pub error: monostate::MustBe!("CheckoutLocked"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl CheckoutLocked {
    pub fn new(required: CheckoutLockedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutOrganizationRequired {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::OrganizationId,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: String,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    pub avatar_url: Option<String>,
    /// Proration behavior applied when customer updates their subscription from
    /// the portal.
    pub proration_behavior: crate::v2026_10::models::common::enums::SubscriptionProrationBehavior,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutOrganization {
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::OrganizationId,
    /// Organization name shown in checkout, customer portal, emails etc.
    pub name: String,
    /// Unique organization slug in checkout, customer portal and credit card
    /// statements.
    pub slug: String,
    /// Avatar URL shown in checkout, customer portal, emails etc.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub avatar_url: Option<String>,
    /// Proration behavior applied when customer updates their subscription from
    /// the portal.
    pub proration_behavior: crate::v2026_10::models::common::enums::SubscriptionProrationBehavior,
    /// Whether customers can update their subscriptions from the customer
    /// portal.
    pub allow_customer_updates: bool,
}
#[allow(clippy::type_complexity)]
impl CheckoutOrganization {
    pub fn new(required: CheckoutOrganizationRequired) -> Self {
        Self {
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            name: required.name,
            slug: required.slug,
            avatar_url: required.avatar_url,
            proration_behavior: required.proration_behavior,
            allow_customer_updates: required.allow_customer_updates,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutProductRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2026_10::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_10::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2026_10::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2026_10::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// List of prices for this product.
    pub prices: Vec<crate::v2026_10::models::checkouts::unions::CheckoutProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_10::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2026_10::models::common::types::ProductMediaFileRead>,
}
/// Product data for a checkout session.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutProduct {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2026_10::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2026_10::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2026_10::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2026_10::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    /// List of prices for this product.
    pub prices: Vec<crate::v2026_10::models::checkouts::unions::CheckoutProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_10::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2026_10::models::common::types::ProductMediaFileRead>,
}
#[allow(clippy::type_complexity)]
impl CheckoutProduct {
    pub fn new(required: CheckoutProductRequired) -> Self {
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
pub struct CheckoutProductsCreateRequired {
    /// List of product IDs available to select at that checkout. The first one
    /// will be selected by default.
    pub products: Vec<crate::v2026_10::models::common::ids::ProductId>,
}
/// Create a new checkout session from a list of products.
/// Customers will be able to switch between those products.
///
/// Metadata set on the checkout will be copied
/// to the resulting order and/or subscription.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutProductsCreate {
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateTrialIntervalCount,
    >,
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
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutProductsCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutProductsCreateCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///ID of the discount to apply to the checkout.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::DiscountId,
    >,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub allow_discount_codes: Option<bool>,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub require_billing_address: Option<bool>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateAmount,
    >,
    ///Predefined number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateSeats,
    >,
    ///Minimum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub min_seats: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMinSeats,
    >,
    ///Maximum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_seats: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMaxSeats,
    >,
    ///Predefined number of units (works with unit-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateUnits,
    >,
    ///Minimum number of units (works with unit-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub min_units: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMinUnits,
    >,
    ///Maximum number of units (works with unit-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_units: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMaxUnits,
    >,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub allow_trial: Option<bool>,
    ///ID of an existing customer in the organization. The customer data will be pre-filled in the checkout form. The resulting order will be linked to this customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::CustomerId,
    >,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub is_business_customer: Option<bool>,
    ///ID of the customer in your system. If a matching customer exists on Polar, the resulting order will be linked to this customer. Otherwise, a new customer will be created with this external ID set.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_customer_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateCustomerName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_ip_address: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateCustomerBillingName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_address: presence_rs::Presence<
        crate::v2026_10::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_tax_id: presence_rs::Presence<String>,
    ///Key-value object allowing you to store additional information that'll be copied to the created customer.
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
    pub customer_metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateCustomerMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutProductsCreateCustomerMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///ID of a subscription to upgrade. It must be on a free pricing. If checkout is successful, metadata set on this checkout will be copied to the subscription, and existing keys will be overwritten.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub subscription_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::SubscriptionId,
    >,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub success_url: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateSuccessUrl,
    >,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateReturnUrl,
    >,
    ///If you plan to embed the checkout session, set this to the Origin of the embedding page. It'll allow the Polar iframe to communicate with the parent page.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub embed_origin: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<
        crate::v2026_10::models::common::enums::PresentmentCurrency,
    >,
    ///List of product IDs available to select at that checkout. The first one will be selected by default.
    pub products: Vec<crate::v2026_10::models::common::ids::ProductId>,
    ///Optional mapping of product IDs to a list of ad-hoc prices to create for that product. If not set, catalog prices of the product will be used.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub prices: presence_rs::Presence<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::CheckoutProductsCreatePricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutProductsCreatePricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
}
#[allow(clippy::type_complexity)]
impl CheckoutProductsCreate {
    pub fn new(required: CheckoutProductsCreateRequired) -> Self {
        Self {
            trial_interval: presence_rs::Presence::Absent,
            trial_interval_count: presence_rs::Presence::Absent,
            metadata: None,
            custom_field_data: None,
            discount_id: presence_rs::Presence::Absent,
            allow_discount_codes: None,
            require_billing_address: None,
            amount: presence_rs::Presence::Absent,
            seats: presence_rs::Presence::Absent,
            min_seats: presence_rs::Presence::Absent,
            max_seats: presence_rs::Presence::Absent,
            units: presence_rs::Presence::Absent,
            min_units: presence_rs::Presence::Absent,
            max_units: presence_rs::Presence::Absent,
            allow_trial: None,
            customer_id: presence_rs::Presence::Absent,
            is_business_customer: None,
            external_customer_id: presence_rs::Presence::Absent,
            customer_name: presence_rs::Presence::Absent,
            customer_email: presence_rs::Presence::Absent,
            customer_ip_address: presence_rs::Presence::Absent,
            customer_billing_name: presence_rs::Presence::Absent,
            customer_billing_address: presence_rs::Presence::Absent,
            customer_tax_id: presence_rs::Presence::Absent,
            customer_metadata: None,
            subscription_id: presence_rs::Presence::Absent,
            success_url: presence_rs::Presence::Absent,
            return_url: presence_rs::Presence::Absent,
            embed_origin: presence_rs::Presence::Absent,
            locale: presence_rs::Presence::Absent,
            currency: presence_rs::Presence::Absent,
            products: required.products,
            prices: presence_rs::Presence::Absent,
        }
    }

    /// The interval unit for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval(
        mut self,
        value: crate::v2026_10::models::common::enums::TrialInterval,
    ) -> Self {
        self.trial_interval = presence_rs::Presence::Some(value);
        self
    }

    /// The number of interval units for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval_count(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
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
            crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutProductsCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutProductsCreateCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// ID of the discount to apply to the checkout.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2026_10::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Whether to allow the customer to apply discount codes. If you apply a
    /// discount through `discount_id`, it'll still be applied, but the customer
    /// won't be able to change it.
    #[allow(clippy::should_implement_trait)]
    pub fn allow_discount_codes(mut self, value: bool) -> Self {
        self.allow_discount_codes = Some(value);
        self
    }

    /// Whether to require the customer to fill their full billing address,
    /// instead of just the country. Customers in the US will always be required
    /// to fill their full address, regardless of this setting. If you preset
    /// the billing address, this setting will be automatically set to `true`.
    #[allow(clippy::should_implement_trait)]
    pub fn require_billing_address(mut self, value: bool) -> Self {
        self.require_billing_address = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateAmount,
    ) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    /// Predefined number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Minimum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn min_seats(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMinSeats,
    ) -> Self {
        self.min_seats = presence_rs::Presence::Some(value);
        self
    }

    /// Maximum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn max_seats(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMaxSeats,
    ) -> Self {
        self.max_seats = presence_rs::Presence::Some(value);
        self
    }

    /// Predefined number of units (works with unit-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    /// Minimum number of units (works with unit-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn min_units(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMinUnits,
    ) -> Self {
        self.min_units = presence_rs::Presence::Some(value);
        self
    }

    /// Maximum number of units (works with unit-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn max_units(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateMaxUnits,
    ) -> Self {
        self.max_units = presence_rs::Presence::Some(value);
        self
    }

    /// Whether to enable the trial period for the checkout session. If `false`,
    /// the trial period will be disabled, even if the selected product has a
    /// trial configured.
    #[allow(clippy::should_implement_trait)]
    pub fn allow_trial(mut self, value: bool) -> Self {
        self.allow_trial = Some(value);
        self
    }

    /// ID of an existing customer in the organization. The customer data will
    /// be pre-filled in the checkout form. The resulting order will be linked
    /// to this customer.
    #[allow(clippy::should_implement_trait)]
    pub fn customer_id(mut self, value: crate::v2026_10::models::common::ids::CustomerId) -> Self {
        self.customer_id = presence_rs::Presence::Some(value);
        self
    }

    /// Whether the customer is a business or an individual. If `true`, the
    /// customer will be required to fill their full billing address and billing
    /// name.
    #[allow(clippy::should_implement_trait)]
    pub fn is_business_customer(mut self, value: bool) -> Self {
        self.is_business_customer = Some(value);
        self
    }

    /// ID of the customer in your system. If a matching customer exists on
    /// Polar, the resulting order will be linked to this customer. Otherwise, a
    /// new customer will be created with this external ID set.
    #[allow(clippy::should_implement_trait)]
    pub fn external_customer_id(mut self, value: impl Into<String>) -> Self {
        self.external_customer_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateCustomerName,
    ) -> Self {
        self.customer_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_email(mut self, value: impl Into<String>) -> Self {
        self.customer_email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_ip_address(mut self, value: impl Into<String>) -> Self {
        self.customer_ip_address = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateCustomerBillingName,
    ) -> Self {
        self.customer_billing_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_address(
        mut self,
        value: crate::v2026_10::models::common::types::AddressInput,
    ) -> Self {
        self.customer_billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_tax_id(mut self, value: impl Into<String>) -> Self {
        self.customer_tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information that'll be
    /// copied to the created customer.
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
    pub fn customer_metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateCustomerMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutProductsCreateCustomerMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.customer_metadata = Some(value);
        self
    }

    /// ID of a subscription to upgrade. It must be on a free pricing. If
    /// checkout is successful, metadata set on this checkout will be copied to
    /// the subscription, and existing keys will be overwritten.
    #[allow(clippy::should_implement_trait)]
    pub fn subscription_id(
        mut self,
        value: crate::v2026_10::models::common::ids::SubscriptionId,
    ) -> Self {
        self.subscription_id = presence_rs::Presence::Some(value);
        self
    }

    /// URL where the customer will be redirected after a successful payment.You
    /// can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the
    /// checkout session id.
    #[allow(clippy::should_implement_trait)]
    pub fn success_url(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateSuccessUrl,
    ) -> Self {
        self.success_url = presence_rs::Presence::Some(value);
        self
    }

    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutProductsCreateReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }

    /// If you plan to embed the checkout session, set this to the Origin of the
    /// embedding page. It'll allow the Polar iframe to communicate with the
    /// parent page.
    #[allow(clippy::should_implement_trait)]
    pub fn embed_origin(mut self, value: impl Into<String>) -> Self {
        self.embed_origin = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(
        mut self,
        value: crate::v2026_10::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.currency = presence_rs::Presence::Some(value);
        self
    }

    /// Optional mapping of product IDs to a list of ad-hoc prices to create for
    /// that product. If not set, catalog prices of the product will be used.
    #[allow(clippy::should_implement_trait)]
    pub fn prices(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::CheckoutProductsCreatePricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutProductsCreatePricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    ) -> Self {
        self.prices = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutPublicRequired {
    ///The ID of the object.
    pub id: crate::v2026_10::models::common::ids::ClientSecretId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    ///Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    ///
    ///        Status of the checkout session.
    ///
    ///        - Open: the checkout session was opened.
    ///        - Expired: the checkout session was expired and is no more accessible.
    ///        - Confirmed: the user on the checkout session clicked Pay. This is not indicative of the payment's success status.
    ///        - Failed: the checkout definitely failed for technical reasons and cannot be retried. In most cases, this state is never reached.
    ///        - Succeeded: the payment on the checkout was performed successfully.
    ///
    pub status: crate::v2026_10::models::checkouts::enums::CheckoutStatus,
    ///Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    ///URL where the customer can access the checkout session.
    pub url: String,
    ///Expiration date and time of the checkout session.
    pub expires_at: jiff::Timestamp,
    ///URL where the customer will be redirected after a successful payment.
    pub success_url: String,
    ///When set, a back button will be shown in the checkout to return to this URL.
    pub return_url: Option<String>,
    ///When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    pub embed_origin: Option<String>,
    ///Amount in cents, before discounts and taxes.
    pub amount: i64,
    ///Predefined number of units (works with unit-based pricing only)
    pub units: Option<i64>,
    ///Minimum number of units (works with unit-based pricing only)
    pub min_units: Option<i64>,
    ///Maximum number of units (works with unit-based pricing only)
    pub max_units: Option<i64>,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    pub tax_amount: Option<i64>,
    ///Tax behavior of the checkout. `inclusive` means the price includes tax, `exclusive` means tax is added on top. If `null`, tax is not yet calculated.
    pub tax_behavior: Option<crate::v2026_10::models::checkouts::enums::TaxBehavior>,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Currency code of the checkout session.
    pub currency: String,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    pub allow_trial: Option<bool>,
    ///Interval unit of the trial period, if any. This value is either set from the checkout, if `trial_interval` is set, or from the selected product.
    pub active_trial_interval: Option<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///Number of interval units of the trial period, if any. This value is either set from the checkout, if `trial_interval_count` is set, or from the selected product.
    pub active_trial_interval_count: Option<i64>,
    ///End date and time of the trial period, if any.
    pub trial_end: Option<jiff::Timestamp>,
    ///ID of the organization owning the checkout session.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    ///ID of the product to checkout.
    pub product_id: Option<crate::v2026_10::models::common::ids::ProductId>,
    ///ID of the product price to checkout.
    pub product_price_id: Option<crate::v2026_10::models::common::ids::ProductPriceId>,
    ///ID of the discount applied to the checkout.
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    ///Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    ///Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    ///Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    ///Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    ///Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    pub customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    ///Name of the customer.
    pub customer_name: Option<String>,
    ///Email address of the customer.
    pub customer_email: Option<String>,
    pub customer_ip_address: Option<String>,
    pub customer_billing_name: Option<String>,
    pub customer_billing_address: Option<
        crate::v2026_10::models::common::types::Address,
    >,
    pub customer_tax_id: Option<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    pub payment_method_type: Option<String>,
    pub payment_processor_metadata: std::collections::BTreeMap<String, String>,
    ///Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: crate::v2026_10::models::checkouts::types::CheckoutBillingAddressFields,
    ///List of products available to select.
    pub products: Vec<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Product selected to checkout.
    pub product: Option<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Price of the selected product.
    pub product_price: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicProductPriceAnyOf0Union,
    >,
    ///Mapping of product IDs to their list of prices.
    pub prices: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::ClientSecretPricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicPricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
    pub discount: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicDiscountAnyOf0Union,
    >,
    pub organization: crate::v2026_10::models::checkouts::types::CheckoutOrganization,
    pub attached_custom_fields: Option<
        Vec<crate::v2026_10::models::common::types::AttachedCustomField>,
    >,
}
/// Checkout session data retrieved using the client secret.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutPublic {
    ///The ID of the object.
    pub id: crate::v2026_10::models::common::ids::ClientSecretId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    ///
    ///        Status of the checkout session.
    ///
    ///        - Open: the checkout session was opened.
    ///        - Expired: the checkout session was expired and is no more accessible.
    ///        - Confirmed: the user on the checkout session clicked Pay. This is not indicative of the payment's success status.
    ///        - Failed: the checkout definitely failed for technical reasons and cannot be retried. In most cases, this state is never reached.
    ///        - Succeeded: the payment on the checkout was performed successfully.
    ///
    pub status: crate::v2026_10::models::checkouts::enums::CheckoutStatus,
    ///Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    ///URL where the customer can access the checkout session.
    pub url: String,
    ///Expiration date and time of the checkout session.
    pub expires_at: jiff::Timestamp,
    ///URL where the customer will be redirected after a successful payment.
    pub success_url: String,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub return_url: Option<String>,
    ///When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub embed_origin: Option<String>,
    ///Amount in cents, before discounts and taxes.
    pub amount: i64,
    ///Predefined number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    ///Minimum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub min_seats: presence_rs::Presence<i64>,
    ///Maximum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_seats: presence_rs::Presence<i64>,
    ///Predefined number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    ///Minimum number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub min_units: Option<i64>,
    ///Maximum number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_units: Option<i64>,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_amount: Option<i64>,
    ///Tax behavior of the checkout. `inclusive` means the price includes tax, `exclusive` means tax is added on top. If `null`, tax is not yet calculated.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2026_10::models::checkouts::enums::TaxBehavior>,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Currency code of the checkout session.
    pub currency: String,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub allow_trial: Option<bool>,
    ///Interval unit of the trial period, if any. This value is either set from the checkout, if `trial_interval` is set, or from the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub active_trial_interval: Option<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///Number of interval units of the trial period, if any. This value is either set from the checkout, if `trial_interval_count` is set, or from the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub active_trial_interval_count: Option<i64>,
    ///End date and time of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    ///ID of the organization owning the checkout session.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    ///ID of the product to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_id: Option<crate::v2026_10::models::common::ids::ProductId>,
    ///ID of the product price to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price_id: Option<crate::v2026_10::models::common::ids::ProductPriceId>,
    ///ID of the discount applied to the checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    ///Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    ///Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    ///Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    ///Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    ///Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    ///Name of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_name: Option<String>,
    ///Email address of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_ip_address: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_billing_address: Option<
        crate::v2026_10::models::common::types::Address,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_tax_id: Option<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub payment_method_type: Option<String>,
    pub payment_processor_metadata: std::collections::BTreeMap<String, String>,
    ///Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: crate::v2026_10::models::checkouts::types::CheckoutBillingAddressFields,
    ///List of products available to select.
    pub products: Vec<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Product selected to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product: Option<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Price of the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicProductPriceAnyOf0Union,
    >,
    ///Mapping of product IDs to their list of prices.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub prices: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::ClientSecretPricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicPricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicDiscountAnyOf0Union,
    >,
    pub organization: crate::v2026_10::models::checkouts::types::CheckoutOrganization,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub attached_custom_fields: Option<
        Vec<crate::v2026_10::models::common::types::AttachedCustomField>,
    >,
}
#[allow(clippy::type_complexity)]
impl CheckoutPublic {
    pub fn new(required: CheckoutPublicRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            custom_field_data: None,
            payment_processor: required.payment_processor,
            status: required.status,
            client_secret: required.client_secret,
            url: required.url,
            expires_at: required.expires_at,
            success_url: required.success_url,
            return_url: required.return_url,
            embed_origin: required.embed_origin,
            amount: required.amount,
            seats: presence_rs::Presence::Absent,
            min_seats: presence_rs::Presence::Absent,
            max_seats: presence_rs::Presence::Absent,
            units: required.units,
            min_units: required.min_units,
            max_units: required.max_units,
            discount_amount: required.discount_amount,
            net_amount: required.net_amount,
            tax_amount: required.tax_amount,
            tax_behavior: required.tax_behavior,
            total_amount: required.total_amount,
            currency: required.currency,
            allow_trial: required.allow_trial,
            active_trial_interval: required.active_trial_interval,
            active_trial_interval_count: required.active_trial_interval_count,
            trial_end: required.trial_end,
            organization_id: required.organization_id,
            product_id: required.product_id,
            product_price_id: required.product_price_id,
            discount_id: required.discount_id,
            allow_discount_codes: required.allow_discount_codes,
            require_billing_address: required.require_billing_address,
            is_discount_applicable: required.is_discount_applicable,
            is_free_product_price: required.is_free_product_price,
            is_payment_required: required.is_payment_required,
            is_payment_setup_required: required.is_payment_setup_required,
            is_payment_form_required: required.is_payment_form_required,
            customer_id: required.customer_id,
            is_business_customer: required.is_business_customer,
            customer_name: required.customer_name,
            customer_email: required.customer_email,
            customer_ip_address: required.customer_ip_address,
            customer_billing_name: required.customer_billing_name,
            customer_billing_address: required.customer_billing_address,
            customer_tax_id: required.customer_tax_id,
            locale: presence_rs::Presence::Absent,
            payment_method_type: required.payment_method_type,
            payment_processor_metadata: required.payment_processor_metadata,
            billing_address_fields: required.billing_address_fields,
            products: required.products,
            product: required.product,
            product_price: required.product_price,
            prices: required.prices,
            discount: required.discount,
            organization: required.organization,
            attached_custom_fields: required.attached_custom_fields,
        }
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// Predefined number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Minimum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn min_seats(mut self, value: i64) -> Self {
        self.min_seats = presence_rs::Presence::Some(value);
        self
    }

    /// Maximum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn max_seats(mut self, value: i64) -> Self {
        self.max_seats = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutPublicConfirmedRequired {
    ///The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CheckoutPublicConfirmedId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    ///Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    ///Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    ///URL where the customer can access the checkout session.
    pub url: String,
    ///Expiration date and time of the checkout session.
    pub expires_at: jiff::Timestamp,
    ///URL where the customer will be redirected after a successful payment.
    pub success_url: String,
    ///When set, a back button will be shown in the checkout to return to this URL.
    pub return_url: Option<String>,
    ///When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    pub embed_origin: Option<String>,
    ///Amount in cents, before discounts and taxes.
    pub amount: i64,
    ///Predefined number of units (works with unit-based pricing only)
    pub units: Option<i64>,
    ///Minimum number of units (works with unit-based pricing only)
    pub min_units: Option<i64>,
    ///Maximum number of units (works with unit-based pricing only)
    pub max_units: Option<i64>,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    pub tax_amount: Option<i64>,
    ///Tax behavior of the checkout. `inclusive` means the price includes tax, `exclusive` means tax is added on top. If `null`, tax is not yet calculated.
    pub tax_behavior: Option<crate::v2026_10::models::checkouts::enums::TaxBehavior>,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Currency code of the checkout session.
    pub currency: String,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    pub allow_trial: Option<bool>,
    ///Interval unit of the trial period, if any. This value is either set from the checkout, if `trial_interval` is set, or from the selected product.
    pub active_trial_interval: Option<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///Number of interval units of the trial period, if any. This value is either set from the checkout, if `trial_interval_count` is set, or from the selected product.
    pub active_trial_interval_count: Option<i64>,
    ///End date and time of the trial period, if any.
    pub trial_end: Option<jiff::Timestamp>,
    ///ID of the organization owning the checkout session.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    ///ID of the product to checkout.
    pub product_id: Option<crate::v2026_10::models::common::ids::ProductId>,
    ///ID of the product price to checkout.
    pub product_price_id: Option<crate::v2026_10::models::common::ids::ProductPriceId>,
    ///ID of the discount applied to the checkout.
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    ///Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    ///Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    ///Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    ///Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    ///Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    pub customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    ///Name of the customer.
    pub customer_name: Option<String>,
    ///Email address of the customer.
    pub customer_email: Option<String>,
    pub customer_ip_address: Option<String>,
    pub customer_billing_name: Option<String>,
    pub customer_billing_address: Option<
        crate::v2026_10::models::common::types::Address,
    >,
    pub customer_tax_id: Option<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    pub payment_method_type: Option<String>,
    pub payment_processor_metadata: std::collections::BTreeMap<String, String>,
    ///Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: crate::v2026_10::models::checkouts::types::CheckoutBillingAddressFields,
    ///List of products available to select.
    pub products: Vec<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Product selected to checkout.
    pub product: Option<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Price of the selected product.
    pub product_price: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedProductPriceAnyOf0Union,
    >,
    ///Mapping of product IDs to their list of prices.
    pub prices: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::CheckoutPublicConfirmedPricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedPricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
    pub discount: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedDiscountAnyOf0Union,
    >,
    pub organization: crate::v2026_10::models::checkouts::types::CheckoutOrganization,
    pub attached_custom_fields: Option<
        Vec<crate::v2026_10::models::common::types::AttachedCustomField>,
    >,
    pub customer_session_token: Option<String>,
}
/// Checkout session data retrieved using the client secret after confirmation.
///
/// It contains a customer session token to retrieve order information
/// right after the checkout.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutPublicConfirmed {
    ///The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CheckoutPublicConfirmedId,
    ///Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    ///Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    pub status: monostate::MustBe!("confirmed"),
    ///Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    ///URL where the customer can access the checkout session.
    pub url: String,
    ///Expiration date and time of the checkout session.
    pub expires_at: jiff::Timestamp,
    ///URL where the customer will be redirected after a successful payment.
    pub success_url: String,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub return_url: Option<String>,
    ///When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub embed_origin: Option<String>,
    ///Amount in cents, before discounts and taxes.
    pub amount: i64,
    ///Predefined number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<i64>,
    ///Minimum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub min_seats: presence_rs::Presence<i64>,
    ///Maximum number of seats (works with seat-based pricing only)
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_seats: presence_rs::Presence<i64>,
    ///Predefined number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    ///Minimum number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub min_units: Option<i64>,
    ///Maximum number of units (works with unit-based pricing only)
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_units: Option<i64>,
    ///Discount amount in cents.
    pub discount_amount: i64,
    ///Amount in cents, after discounts but before taxes.
    pub net_amount: i64,
    ///Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_amount: Option<i64>,
    ///Tax behavior of the checkout. `inclusive` means the price includes tax, `exclusive` means tax is added on top. If `null`, tax is not yet calculated.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub tax_behavior: Option<crate::v2026_10::models::checkouts::enums::TaxBehavior>,
    ///Amount in cents, after discounts and taxes.
    pub total_amount: i64,
    ///Currency code of the checkout session.
    pub currency: String,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub allow_trial: Option<bool>,
    ///Interval unit of the trial period, if any. This value is either set from the checkout, if `trial_interval` is set, or from the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub active_trial_interval: Option<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///Number of interval units of the trial period, if any. This value is either set from the checkout, if `trial_interval_count` is set, or from the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub active_trial_interval_count: Option<i64>,
    ///End date and time of the trial period, if any.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_end: Option<jiff::Timestamp>,
    ///ID of the organization owning the checkout session.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    ///ID of the product to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_id: Option<crate::v2026_10::models::common::ids::ProductId>,
    ///ID of the product price to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price_id: Option<crate::v2026_10::models::common::ids::ProductPriceId>,
    ///ID of the discount applied to the checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    ///Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    ///Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    ///Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    ///Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    ///Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_id: Option<crate::v2026_10::models::common::ids::CustomerId>,
    ///Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    ///Name of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_name: Option<String>,
    ///Email address of the customer.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_email: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_ip_address: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_billing_name: Option<String>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_billing_address: Option<
        crate::v2026_10::models::common::types::Address,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_tax_id: Option<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub payment_method_type: Option<String>,
    pub payment_processor_metadata: std::collections::BTreeMap<String, String>,
    ///Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: crate::v2026_10::models::checkouts::types::CheckoutBillingAddressFields,
    ///List of products available to select.
    pub products: Vec<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Product selected to checkout.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product: Option<crate::v2026_10::models::checkouts::types::CheckoutProduct>,
    ///Price of the selected product.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub product_price: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedProductPriceAnyOf0Union,
    >,
    ///Mapping of product IDs to their list of prices.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub prices: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::common::ids::CheckoutPublicConfirmedPricesId,
            Vec<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedPricesAnyOf0AdditionalPropertiesItemsUnion,
            >,
        >,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount: Option<
        crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedDiscountAnyOf0Union,
    >,
    pub organization: crate::v2026_10::models::checkouts::types::CheckoutOrganization,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub attached_custom_fields: Option<
        Vec<crate::v2026_10::models::common::types::AttachedCustomField>,
    >,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub customer_session_token: Option<String>,
}
#[allow(clippy::type_complexity)]
impl CheckoutPublicConfirmed {
    pub fn new(required: CheckoutPublicConfirmedRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            custom_field_data: None,
            payment_processor: required.payment_processor,
            status: Default::default(),
            client_secret: required.client_secret,
            url: required.url,
            expires_at: required.expires_at,
            success_url: required.success_url,
            return_url: required.return_url,
            embed_origin: required.embed_origin,
            amount: required.amount,
            seats: presence_rs::Presence::Absent,
            min_seats: presence_rs::Presence::Absent,
            max_seats: presence_rs::Presence::Absent,
            units: required.units,
            min_units: required.min_units,
            max_units: required.max_units,
            discount_amount: required.discount_amount,
            net_amount: required.net_amount,
            tax_amount: required.tax_amount,
            tax_behavior: required.tax_behavior,
            total_amount: required.total_amount,
            currency: required.currency,
            allow_trial: required.allow_trial,
            active_trial_interval: required.active_trial_interval,
            active_trial_interval_count: required.active_trial_interval_count,
            trial_end: required.trial_end,
            organization_id: required.organization_id,
            product_id: required.product_id,
            product_price_id: required.product_price_id,
            discount_id: required.discount_id,
            allow_discount_codes: required.allow_discount_codes,
            require_billing_address: required.require_billing_address,
            is_discount_applicable: required.is_discount_applicable,
            is_free_product_price: required.is_free_product_price,
            is_payment_required: required.is_payment_required,
            is_payment_setup_required: required.is_payment_setup_required,
            is_payment_form_required: required.is_payment_form_required,
            customer_id: required.customer_id,
            is_business_customer: required.is_business_customer,
            customer_name: required.customer_name,
            customer_email: required.customer_email,
            customer_ip_address: required.customer_ip_address,
            customer_billing_name: required.customer_billing_name,
            customer_billing_address: required.customer_billing_address,
            customer_tax_id: required.customer_tax_id,
            locale: presence_rs::Presence::Absent,
            payment_method_type: required.payment_method_type,
            payment_processor_metadata: required.payment_processor_metadata,
            billing_address_fields: required.billing_address_fields,
            products: required.products,
            product: required.product,
            product_price: required.product_price,
            prices: required.prices,
            discount: required.discount,
            organization: required.organization,
            attached_custom_fields: required.attached_custom_fields,
            customer_session_token: required.customer_session_token,
        }
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutPublicConfirmedCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// Predefined number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn seats(mut self, value: i64) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Minimum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn min_seats(mut self, value: i64) -> Self {
        self.min_seats = presence_rs::Presence::Some(value);
        self
    }

    /// Maximum number of seats (works with seat-based pricing only)
    #[allow(clippy::should_implement_trait)]
    pub fn max_seats(mut self, value: i64) -> Self {
        self.max_seats = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }
}
/// Update an existing checkout session using an access token.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CheckoutUpdate {
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutUpdateCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///ID of the product to checkout. Must be present in the checkout's product list.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ProductId,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutAmount,
    >,
    ///Number of seats for seat-based pricing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutSeats,
    >,
    ///Number of units for unit-based pricing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutUnits,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub is_business_customer: presence_rs::Presence<bool>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutCustomerName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutCustomerBillingName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_address: presence_rs::Presence<
        crate::v2026_10::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutTrialIntervalCount,
    >,
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
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2026_10::models::checkouts::newtypes::CheckoutMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<
        crate::v2026_10::models::common::enums::PresentmentCurrency,
    >,
    ///ID of the discount to apply to the checkout.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::DiscountId,
    >,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub allow_discount_codes: presence_rs::Presence<bool>,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub require_billing_address: presence_rs::Presence<bool>,
    ///Whether to enable the trial period for the checkout session. If `false`, the trial period will be disabled, even if the selected product has a trial configured.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub allow_trial: presence_rs::Presence<bool>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_ip_address: presence_rs::Presence<String>,
    ///Key-value object allowing you to store additional information that'll be copied to the created customer.
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
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_metadata: presence_rs::Presence<
        std::collections::BTreeMap<
            crate::v2026_10::models::checkouts::newtypes::CheckoutCustomerMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutUpdateCustomerMetadataAnyOf0AdditionalPropertiesUnion,
        >,
    >,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub success_url: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutSuccessUrl,
    >,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutReturnUrl,
    >,
    ///If you plan to embed the checkout session, set this to the Origin of the embedding page. It'll allow the Polar iframe to communicate with the parent page.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub embed_origin: presence_rs::Presence<String>,
    ///ID of the product price to checkout. Must correspond to a price present in the checkout's product list.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_price_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ProductPriceId,
    >,
}
#[allow(clippy::type_complexity)]
impl CheckoutUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutUpdateCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// ID of the product to checkout. Must be present in the checkout's product
    /// list.
    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: crate::v2026_10::models::common::ids::ProductId) -> Self {
        self.product_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutAmount,
    ) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    /// Number of seats for seat-based pricing.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Number of units for unit-based pricing.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn is_business_customer(mut self, value: bool) -> Self {
        self.is_business_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutCustomerName,
    ) -> Self {
        self.customer_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_email(mut self, value: impl Into<String>) -> Self {
        self.customer_email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutCustomerBillingName,
    ) -> Self {
        self.customer_billing_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_address(
        mut self,
        value: crate::v2026_10::models::common::types::AddressInput,
    ) -> Self {
        self.customer_billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_tax_id(mut self, value: impl Into<String>) -> Self {
        self.customer_tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// The interval unit for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval(
        mut self,
        value: crate::v2026_10::models::common::enums::TrialInterval,
    ) -> Self {
        self.trial_interval = presence_rs::Presence::Some(value);
        self
    }

    /// The number of interval units for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval_count(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
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
            crate::v2026_10::models::checkouts::newtypes::CheckoutMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(
        mut self,
        value: crate::v2026_10::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.currency = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the discount to apply to the checkout.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2026_10::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Whether to allow the customer to apply discount codes. If you apply a
    /// discount through `discount_id`, it'll still be applied, but the customer
    /// won't be able to change it.
    #[allow(clippy::should_implement_trait)]
    pub fn allow_discount_codes(mut self, value: bool) -> Self {
        self.allow_discount_codes = presence_rs::Presence::Some(value);
        self
    }

    /// Whether to require the customer to fill their full billing address,
    /// instead of just the country. Customers in the US will always be required
    /// to fill their full address, regardless of this setting. If you preset
    /// the billing address, this setting will be automatically set to `true`.
    #[allow(clippy::should_implement_trait)]
    pub fn require_billing_address(mut self, value: bool) -> Self {
        self.require_billing_address = presence_rs::Presence::Some(value);
        self
    }

    /// Whether to enable the trial period for the checkout session. If `false`,
    /// the trial period will be disabled, even if the selected product has a
    /// trial configured.
    #[allow(clippy::should_implement_trait)]
    pub fn allow_trial(mut self, value: bool) -> Self {
        self.allow_trial = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_ip_address(mut self, value: impl Into<String>) -> Self {
        self.customer_ip_address = presence_rs::Presence::Some(value.into());
        self
    }

    /// Key-value object allowing you to store additional information that'll be
    /// copied to the created customer.
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
    pub fn customer_metadata(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2026_10::models::checkouts::newtypes::CheckoutCustomerMetadataKey,
            crate::v2026_10::models::checkouts::unions::CheckoutUpdateCustomerMetadataAnyOf0AdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.customer_metadata = presence_rs::Presence::Some(value);
        self
    }

    /// URL where the customer will be redirected after a successful payment.You
    /// can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the
    /// checkout session id.
    #[allow(clippy::should_implement_trait)]
    pub fn success_url(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutSuccessUrl,
    ) -> Self {
        self.success_url = presence_rs::Presence::Some(value);
        self
    }

    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }

    /// If you plan to embed the checkout session, set this to the Origin of the
    /// embedding page. It'll allow the Polar iframe to communicate with the
    /// parent page.
    #[allow(clippy::should_implement_trait)]
    pub fn embed_origin(mut self, value: impl Into<String>) -> Self {
        self.embed_origin = presence_rs::Presence::Some(value.into());
        self
    }

    /// ID of the product price to checkout. Must correspond to a price present
    /// in the checkout's product list.
    #[allow(clippy::should_implement_trait)]
    pub fn product_price_id(
        mut self,
        value: crate::v2026_10::models::common::ids::ProductPriceId,
    ) -> Self {
        self.product_price_id = presence_rs::Presence::Some(value);
        self
    }
}
/// Update an existing checkout session using the client secret.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CheckoutUpdatePublic {
    ///Key-value object storing custom field values.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub custom_field_data: Option<
        std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutUpdatePublicCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    >,
    ///ID of the product to checkout. Must be present in the checkout's product list.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ProductId,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicAmount,
    >,
    ///Number of seats for seat-based pricing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicSeats,
    >,
    ///Number of units for unit-based pricing.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicUnits,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub is_business_customer: presence_rs::Presence<bool>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicCustomerName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_email: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_name: presence_rs::Presence<
        crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicCustomerBillingName,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_billing_address: presence_rs::Presence<
        crate::v2026_10::models::common::types::AddressInput,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub customer_tax_id: presence_rs::Presence<String>,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub locale: presence_rs::Presence<String>,
    ///Payment method type selected by the customer in the checkout form, e.g. `card`, `apple_pay` or `upi`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub payment_method_type: presence_rs::Presence<String>,
    ///Discount code to apply to the checkout.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_code: presence_rs::Presence<String>,
    ///Disable the trial period for the checkout session. It's mainly useful when the trial is blocked because the customer already redeemed one.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub allow_trial: presence_rs::Presence<monostate::MustBe!(false)>,
    ///ID of the product price to checkout. Must correspond to a price present in the checkout's product list.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub product_price_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::ProductPriceId,
    >,
}
#[allow(clippy::type_complexity)]
impl CheckoutUpdatePublic {
    pub fn new() -> Self {
        Self::default()
    }

    /// Key-value object storing custom field values.
    #[allow(clippy::should_implement_trait)]
    pub fn custom_field_data(
        mut self,
        value: std::collections::BTreeMap<
            String,
            Option<
                crate::v2026_10::models::checkouts::unions::CheckoutUpdatePublicCustomFieldDataAdditionalPropertiesUnion,
            >,
        >,
    ) -> Self {
        self.custom_field_data = Some(value);
        self
    }

    /// ID of the product to checkout. Must be present in the checkout's product
    /// list.
    #[allow(clippy::should_implement_trait)]
    pub fn product_id(mut self, value: crate::v2026_10::models::common::ids::ProductId) -> Self {
        self.product_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicAmount,
    ) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    /// Number of seats for seat-based pricing.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Number of units for unit-based pricing.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn is_business_customer(mut self, value: bool) -> Self {
        self.is_business_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicCustomerName,
    ) -> Self {
        self.customer_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_email(mut self, value: impl Into<String>) -> Self {
        self.customer_email = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_name(
        mut self,
        value: crate::v2026_10::models::checkouts::newtypes::CheckoutUpdatePublicCustomerBillingName,
    ) -> Self {
        self.customer_billing_name = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_billing_address(
        mut self,
        value: crate::v2026_10::models::common::types::AddressInput,
    ) -> Self {
        self.customer_billing_address = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn customer_tax_id(mut self, value: impl Into<String>) -> Self {
        self.customer_tax_id = presence_rs::Presence::Some(value.into());
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = presence_rs::Presence::Some(value.into());
        self
    }

    /// Payment method type selected by the customer in the checkout form, e.g.
    /// `card`, `apple_pay` or `upi`.
    #[allow(clippy::should_implement_trait)]
    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = presence_rs::Presence::Some(value.into());
        self
    }

    /// Discount code to apply to the checkout.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_code(mut self, value: impl Into<String>) -> Self {
        self.discount_code = presence_rs::Presence::Some(value.into());
        self
    }

    /// Disable the trial period for the checkout session. It's mainly useful
    /// when the trial is blocked because the customer already redeemed one.
    #[allow(clippy::should_implement_trait)]
    pub fn allow_trial(mut self, value: monostate::MustBe!(false)) -> Self {
        self.allow_trial = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the product price to checkout. Must correspond to a price present
    /// in the checkout's product list.
    #[allow(clippy::should_implement_trait)]
    pub fn product_price_id(
        mut self,
        value: crate::v2026_10::models::common::ids::ProductPriceId,
    ) -> Self {
        self.product_price_id = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct DiscountRedemptionLimitReachedRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct DiscountRedemptionLimitReached {
    pub error: monostate::MustBe!("DiscountRedemptionLimitReached"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl DiscountRedemptionLimitReached {
    pub fn new(required: DiscountRedemptionLimitReachedRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct ExpiredCheckoutErrorRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct ExpiredCheckoutError {
    pub error: monostate::MustBe!("ExpiredCheckoutError"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl ExpiredCheckoutError {
    pub fn new(required: ExpiredCheckoutErrorRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct NotOpenCheckoutRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct NotOpenCheckout {
    pub error: monostate::MustBe!("NotOpenCheckout"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl NotOpenCheckout {
    pub fn new(required: NotOpenCheckoutRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
#[derive(Debug, Clone)]
pub struct PaymentErrorRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct PaymentError {
    pub error: monostate::MustBe!("PaymentError"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl PaymentError {
    pub fn new(required: PaymentErrorRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
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
