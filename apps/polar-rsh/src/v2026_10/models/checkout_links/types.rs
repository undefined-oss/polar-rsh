#[derive(Debug, Clone)]
pub struct CheckoutLinkRequired {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CheckoutLinkId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2026_10::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
    /// Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// Client secret used to access the checkout link.
    pub client_secret: String,
    /// URL where the customer will be redirected after a successful payment.
    pub success_url: Option<String>,
    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    pub return_url: Option<String>,
    /// Optional label to distinguish links internally
    pub label: Option<String>,
    /// Whether to allow the customer to apply discount codes. If you apply a
    /// discount through `discount_id`, it'll still be applied, but the customer
    /// won't be able to change it.
    pub allow_discount_codes: bool,
    /// Whether to require the customer to fill their full billing address,
    /// instead of just the country. Customers in the US will always be required
    /// to fill their full address, regardless of this setting.
    pub require_billing_address: bool,
    /// ID of the discount to apply to the checkout. If the discount is not
    /// applicable anymore when opening the checkout link, it'll be ignored.
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    /// Preconfigured number of seats for seat-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of seats and
    /// the customer won't be able to change it. All products on the link must
    /// use seat-based pricing and allow this number of seats. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    pub seats: Option<i64>,
    /// Preconfigured number of units for unit-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of units and
    /// the customer won't be able to change it. All products on the link must
    /// use unit-based pricing and allow this number of units. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    pub units: Option<i64>,
    /// The organization ID.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2026_10::models::checkout_links::types::CheckoutLinkProduct>,
    pub discount:
        Option<crate::v2026_10::models::checkout_links::unions::CheckoutLinkDiscountAnyOf0Union>,
    pub url: String,
}
/// Checkout link data.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutLink {
    /// The ID of the object.
    pub id: crate::v2026_10::models::common::ids::CheckoutLinkId,
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
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
    /// Payment processor used.
    pub payment_processor: crate::v2026_10::models::common::enums::PaymentProcessor,
    /// Client secret used to access the checkout link.
    pub client_secret: String,
    /// URL where the customer will be redirected after a successful payment.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub success_url: Option<String>,
    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub return_url: Option<String>,
    /// Optional label to distinguish links internally
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub label: Option<String>,
    /// Whether to allow the customer to apply discount codes. If you apply a
    /// discount through `discount_id`, it'll still be applied, but the customer
    /// won't be able to change it.
    pub allow_discount_codes: bool,
    /// Whether to require the customer to fill their full billing address,
    /// instead of just the country. Customers in the US will always be required
    /// to fill their full address, regardless of this setting.
    pub require_billing_address: bool,
    /// ID of the discount to apply to the checkout. If the discount is not
    /// applicable anymore when opening the checkout link, it'll be ignored.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount_id: Option<crate::v2026_10::models::common::ids::DiscountId>,
    /// Preconfigured number of seats for seat-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of seats and
    /// the customer won't be able to change it. All products on the link must
    /// use seat-based pricing and allow this number of seats. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub seats: Option<i64>,
    /// Preconfigured number of units for unit-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of units and
    /// the customer won't be able to change it. All products on the link must
    /// use unit-based pricing and allow this number of units. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub units: Option<i64>,
    /// The organization ID.
    pub organization_id: crate::v2026_10::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2026_10::models::checkout_links::types::CheckoutLinkProduct>,
    #[serde(
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub discount:
        Option<crate::v2026_10::models::checkout_links::unions::CheckoutLinkDiscountAnyOf0Union>,
    pub url: String,
}
#[allow(clippy::type_complexity)]
impl CheckoutLink {
    pub fn new(required: CheckoutLinkRequired) -> Self {
        Self {
            id: required.id,
            created_at: required.created_at,
            modified_at: required.modified_at,
            trial_interval: required.trial_interval,
            trial_interval_count: required.trial_interval_count,
            metadata: required.metadata,
            payment_processor: required.payment_processor,
            client_secret: required.client_secret,
            success_url: required.success_url,
            return_url: required.return_url,
            label: required.label,
            allow_discount_codes: required.allow_discount_codes,
            require_billing_address: required.require_billing_address,
            discount_id: required.discount_id,
            seats: required.seats,
            units: required.units,
            organization_id: required.organization_id,
            products: required.products,
            discount: required.discount,
            url: required.url,
        }
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutLinkCreateProductRequired {
    pub product_id: crate::v2026_10::models::common::ids::ProductId,
}
/// Schema to create a new checkout link from a a single product.
///
/// **Deprecated**: Use `CheckoutLinkCreateProducts` instead.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutLinkCreateProduct {
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkCreateProductMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductTrialIntervalCount,
    >,
    ///Payment processor to use. Currently only Stripe is supported.
    pub payment_processor: monostate::MustBe!("stripe"),
    ///Optional label to distinguish links internally
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label: presence_rs::Presence<String>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub allow_discount_codes: Option<bool>,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub require_billing_address: Option<bool>,
    ///ID of the discount to apply to the checkout. If the discount is not applicable anymore when opening the checkout link, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::DiscountId,
    >,
    ///Preconfigured number of seats for seat-based pricing. When set, checkout sessions created from this link are locked to this number of seats and the customer won't be able to change it. All products on the link must use seat-based pricing and allow this number of seats. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductSeats,
    >,
    ///Preconfigured number of units for unit-based pricing. When set, checkout sessions created from this link are locked to this number of units and the customer won't be able to change it. All products on the link must use unit-based pricing and allow this number of units. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductUnits,
    >,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub success_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductSuccessUrl,
    >,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductReturnUrl,
    >,
    pub product_id: crate::v2026_10::models::common::ids::ProductId,
}
#[allow(clippy::type_complexity)]
impl CheckoutLinkCreateProduct {
    pub fn new(required: CheckoutLinkCreateProductRequired) -> Self {
        Self {
            metadata: None,
            trial_interval: presence_rs::Presence::Absent,
            trial_interval_count: presence_rs::Presence::Absent,
            payment_processor: Default::default(),
            label: presence_rs::Presence::Absent,
            allow_discount_codes: None,
            require_billing_address: None,
            discount_id: presence_rs::Presence::Absent,
            seats: presence_rs::Presence::Absent,
            units: presence_rs::Presence::Absent,
            success_url: presence_rs::Presence::Absent,
            return_url: presence_rs::Presence::Absent,
            product_id: required.product_id,
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkCreateProductMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
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
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
        self
    }

    /// Optional label to distinguish links internally
    #[allow(clippy::should_implement_trait)]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = presence_rs::Presence::Some(value.into());
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
    /// to fill their full address, regardless of this setting.
    #[allow(clippy::should_implement_trait)]
    pub fn require_billing_address(mut self, value: bool) -> Self {
        self.require_billing_address = Some(value);
        self
    }

    /// ID of the discount to apply to the checkout. If the discount is not
    /// applicable anymore when opening the checkout link, it'll be ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2026_10::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of seats for seat-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of seats and
    /// the customer won't be able to change it. All products on the link must
    /// use seat-based pricing and allow this number of seats. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of units for unit-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of units and
    /// the customer won't be able to change it. All products on the link must
    /// use unit-based pricing and allow this number of units. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    /// URL where the customer will be redirected after a successful payment.You
    /// can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the
    /// checkout session id.
    #[allow(clippy::should_implement_trait)]
    pub fn success_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductSuccessUrl,
    ) -> Self {
        self.success_url = presence_rs::Presence::Some(value);
        self
    }

    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutLinkCreateProductPriceRequired {
    pub product_price_id: crate::v2026_10::models::common::ids::ProductPriceId,
}
/// Schema to create a new checkout link from a a single product price.
///
/// **Deprecated**: Use `CheckoutLinkCreateProducts` instead.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutLinkCreateProductPrice {
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkCreateProductPriceMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceTrialIntervalCount,
    >,
    ///Payment processor to use. Currently only Stripe is supported.
    pub payment_processor: monostate::MustBe!("stripe"),
    ///Optional label to distinguish links internally
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label: presence_rs::Presence<String>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub allow_discount_codes: Option<bool>,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub require_billing_address: Option<bool>,
    ///ID of the discount to apply to the checkout. If the discount is not applicable anymore when opening the checkout link, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::DiscountId,
    >,
    ///Preconfigured number of seats for seat-based pricing. When set, checkout sessions created from this link are locked to this number of seats and the customer won't be able to change it. All products on the link must use seat-based pricing and allow this number of seats. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceSeats,
    >,
    ///Preconfigured number of units for unit-based pricing. When set, checkout sessions created from this link are locked to this number of units and the customer won't be able to change it. All products on the link must use unit-based pricing and allow this number of units. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceUnits,
    >,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub success_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceSuccessUrl,
    >,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceReturnUrl,
    >,
    pub product_price_id: crate::v2026_10::models::common::ids::ProductPriceId,
}
#[allow(clippy::type_complexity)]
impl CheckoutLinkCreateProductPrice {
    pub fn new(required: CheckoutLinkCreateProductPriceRequired) -> Self {
        Self {
            metadata: None,
            trial_interval: presence_rs::Presence::Absent,
            trial_interval_count: presence_rs::Presence::Absent,
            payment_processor: Default::default(),
            label: presence_rs::Presence::Absent,
            allow_discount_codes: None,
            require_billing_address: None,
            discount_id: presence_rs::Presence::Absent,
            seats: presence_rs::Presence::Absent,
            units: presence_rs::Presence::Absent,
            success_url: presence_rs::Presence::Absent,
            return_url: presence_rs::Presence::Absent,
            product_price_id: required.product_price_id,
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkCreateProductPriceMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
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
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
        self
    }

    /// Optional label to distinguish links internally
    #[allow(clippy::should_implement_trait)]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = presence_rs::Presence::Some(value.into());
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
    /// to fill their full address, regardless of this setting.
    #[allow(clippy::should_implement_trait)]
    pub fn require_billing_address(mut self, value: bool) -> Self {
        self.require_billing_address = Some(value);
        self
    }

    /// ID of the discount to apply to the checkout. If the discount is not
    /// applicable anymore when opening the checkout link, it'll be ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2026_10::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of seats for seat-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of seats and
    /// the customer won't be able to change it. All products on the link must
    /// use seat-based pricing and allow this number of seats. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of units for unit-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of units and
    /// the customer won't be able to change it. All products on the link must
    /// use unit-based pricing and allow this number of units. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    /// URL where the customer will be redirected after a successful payment.You
    /// can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the
    /// checkout session id.
    #[allow(clippy::should_implement_trait)]
    pub fn success_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceSuccessUrl,
    ) -> Self {
        self.success_url = presence_rs::Presence::Some(value);
        self
    }

    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutLinkCreateProductsRequired {
    /// List of products that will be available to select at checkout.
    pub products: Vec<crate::v2026_10::models::common::ids::ProductId>,
}
/// Schema to create a new checkout link.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutLinkCreateProducts {
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkCreateProductsMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsTrialIntervalCount,
    >,
    ///Payment processor to use. Currently only Stripe is supported.
    pub payment_processor: monostate::MustBe!("stripe"),
    ///Optional label to distinguish links internally
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label: presence_rs::Presence<String>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub allow_discount_codes: Option<bool>,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2026_10::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub require_billing_address: Option<bool>,
    ///ID of the discount to apply to the checkout. If the discount is not applicable anymore when opening the checkout link, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::DiscountId,
    >,
    ///Preconfigured number of seats for seat-based pricing. When set, checkout sessions created from this link are locked to this number of seats and the customer won't be able to change it. All products on the link must use seat-based pricing and allow this number of seats. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsSeats,
    >,
    ///Preconfigured number of units for unit-based pricing. When set, checkout sessions created from this link are locked to this number of units and the customer won't be able to change it. All products on the link must use unit-based pricing and allow this number of units. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsUnits,
    >,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub success_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsSuccessUrl,
    >,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsReturnUrl,
    >,
    ///List of products that will be available to select at checkout.
    pub products: Vec<crate::v2026_10::models::common::ids::ProductId>,
}
#[allow(clippy::type_complexity)]
impl CheckoutLinkCreateProducts {
    pub fn new(required: CheckoutLinkCreateProductsRequired) -> Self {
        Self {
            metadata: None,
            trial_interval: presence_rs::Presence::Absent,
            trial_interval_count: presence_rs::Presence::Absent,
            payment_processor: Default::default(),
            label: presence_rs::Presence::Absent,
            allow_discount_codes: None,
            require_billing_address: None,
            discount_id: presence_rs::Presence::Absent,
            seats: presence_rs::Presence::Absent,
            units: presence_rs::Presence::Absent,
            success_url: presence_rs::Presence::Absent,
            return_url: presence_rs::Presence::Absent,
            products: required.products,
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkCreateProductsMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
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
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
        self
    }

    /// Optional label to distinguish links internally
    #[allow(clippy::should_implement_trait)]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = presence_rs::Presence::Some(value.into());
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
    /// to fill their full address, regardless of this setting.
    #[allow(clippy::should_implement_trait)]
    pub fn require_billing_address(mut self, value: bool) -> Self {
        self.require_billing_address = Some(value);
        self
    }

    /// ID of the discount to apply to the checkout. If the discount is not
    /// applicable anymore when opening the checkout link, it'll be ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2026_10::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of seats for seat-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of seats and
    /// the customer won't be able to change it. All products on the link must
    /// use seat-based pricing and allow this number of seats. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of units for unit-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of units and
    /// the customer won't be able to change it. All products on the link must
    /// use unit-based pricing and allow this number of units. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    /// URL where the customer will be redirected after a successful payment.You
    /// can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the
    /// checkout session id.
    #[allow(clippy::should_implement_trait)]
    pub fn success_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsSuccessUrl,
    ) -> Self {
        self.success_url = presence_rs::Presence::Some(value);
        self
    }

    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkCreateProductsReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct CheckoutLinkProductRequired {
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
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
    pub prices:
        Vec<crate::v2026_10::models::checkout_links::unions::CheckoutLinkProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_10::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2026_10::models::common::types::ProductMediaFileRead>,
}
/// Product data for a checkout link.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct CheckoutLinkProduct {
    pub metadata: crate::v2026_10::models::common::aliases::MetadataOutputType,
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
    pub prices:
        Vec<crate::v2026_10::models::checkout_links::unions::CheckoutLinkProductPricesItemsUnion>,
    /// List of benefits granted by the product.
    pub benefits: Vec<crate::v2026_10::models::common::unions::BenefitPublic>,
    /// List of medias associated to the product.
    pub medias: Vec<crate::v2026_10::models::common::types::ProductMediaFileRead>,
}
#[allow(clippy::type_complexity)]
impl CheckoutLinkProduct {
    pub fn new(required: CheckoutLinkProductRequired) -> Self {
        Self {
            metadata: required.metadata,
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
/// Schema to update an existing checkout link.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct CheckoutLinkUpdate {
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2026_10::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkTrialIntervalCount,
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///List of products that will be available to select at checkout.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub products: presence_rs::Presence<
        Vec<crate::v2026_10::models::common::ids::ProductId>,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub label: presence_rs::Presence<String>,
    ///Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub allow_discount_codes: presence_rs::Presence<bool>,
    ///Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub require_billing_address: presence_rs::Presence<bool>,
    ///ID of the discount to apply to the checkout. If the discount is not applicable anymore when opening the checkout link, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub discount_id: presence_rs::Presence<
        crate::v2026_10::models::common::ids::DiscountId,
    >,
    ///Preconfigured number of seats for seat-based pricing. When set, checkout sessions created from this link are locked to this number of seats and the customer won't be able to change it. All products on the link must use seat-based pricing and allow this number of seats. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub seats: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkSeats,
    >,
    ///Preconfigured number of units for unit-based pricing. When set, checkout sessions created from this link are locked to this number of units and the customer won't be able to change it. All products on the link must use unit-based pricing and allow this number of units. If the products no longer accommodate this value when the link is opened, it'll be ignored.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub units: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkUnits,
    >,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub success_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkSuccessUrl,
    >,
    ///When set, a back button will be shown in the checkout to return to this URL.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub return_url: presence_rs::Presence<
        crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkReturnUrl,
    >,
}
#[allow(clippy::type_complexity)]
impl CheckoutLinkUpdate {
    pub fn new() -> Self {
        Self::default()
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
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkTrialIntervalCount,
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
            crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkMetadataKey,
            crate::v2026_10::models::checkout_links::unions::CheckoutLinkUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// List of products that will be available to select at checkout.
    #[allow(clippy::should_implement_trait)]
    pub fn products(mut self, value: Vec<crate::v2026_10::models::common::ids::ProductId>) -> Self {
        self.products = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = presence_rs::Presence::Some(value.into());
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
    /// to fill their full address, regardless of this setting.
    #[allow(clippy::should_implement_trait)]
    pub fn require_billing_address(mut self, value: bool) -> Self {
        self.require_billing_address = presence_rs::Presence::Some(value);
        self
    }

    /// ID of the discount to apply to the checkout. If the discount is not
    /// applicable anymore when opening the checkout link, it'll be ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn discount_id(mut self, value: crate::v2026_10::models::common::ids::DiscountId) -> Self {
        self.discount_id = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of seats for seat-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of seats and
    /// the customer won't be able to change it. All products on the link must
    /// use seat-based pricing and allow this number of seats. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn seats(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkSeats,
    ) -> Self {
        self.seats = presence_rs::Presence::Some(value);
        self
    }

    /// Preconfigured number of units for unit-based pricing. When set, checkout
    /// sessions created from this link are locked to this number of units and
    /// the customer won't be able to change it. All products on the link must
    /// use unit-based pricing and allow this number of units. If the products
    /// no longer accommodate this value when the link is opened, it'll be
    /// ignored.
    #[allow(clippy::should_implement_trait)]
    pub fn units(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkUnits,
    ) -> Self {
        self.units = presence_rs::Presence::Some(value);
        self
    }

    /// URL where the customer will be redirected after a successful payment.You
    /// can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the
    /// checkout session id.
    #[allow(clippy::should_implement_trait)]
    pub fn success_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkSuccessUrl,
    ) -> Self {
        self.success_url = presence_rs::Presence::Some(value);
        self
    }

    /// When set, a back button will be shown in the checkout to return to this
    /// URL.
    #[allow(clippy::should_implement_trait)]
    pub fn return_url(
        mut self,
        value: crate::v2026_10::models::checkout_links::newtypes::CheckoutLinkReturnUrl,
    ) -> Self {
        self.return_url = presence_rs::Presence::Some(value);
        self
    }
}
