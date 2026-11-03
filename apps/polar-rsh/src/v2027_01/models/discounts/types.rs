#[derive(Debug, Clone)]
pub struct DiscountFixedCreateRequired {
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateName,
    /// For subscriptions, determines if the discount should be applied once on
    /// the first invoice, forever, or for a certain number of months determined
    /// by `duration_in_months`.
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
}
/// Schema to create a fixed amount discount.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountFixedCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMetadataKey,
            crate::v2027_01::models::discounts::unions::DiscountFixedCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///Name of the discount. Will be displayed to the customer when the discount is applied.
    pub name: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateName,
    ///Code customers can use to apply the discount during checkout. Must be between 3 and 256 characters long and contain only alphanumeric characters.If not provided, the discount can only be applied via the API.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub code: presence_rs::Presence<String>,
    ///Optional timestamp after which the discount is redeemable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub starts_at: presence_rs::Presence<jiff::Timestamp>,
    ///Optional timestamp after which the discount is no longer redeemable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub ends_at: presence_rs::Presence<jiff::Timestamp>,
    ///Optional maximum number of times the discount can be redeemed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_redemptions: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMaxRedemptions,
    >,
    ///Optional maximum number of times the discount can be redeemed by a single customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_redemptions_per_customer: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMaxRedemptionsPerCustomer,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub products: presence_rs::Presence<
        Vec<crate::v2027_01::models::common::ids::ProductId>,
    >,
    ///The ID of the organization owning the discount. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub r#type: Option<monostate::MustBe!("fixed")>,
    ///For subscriptions, determines if the discount should be applied once on the first invoice, forever, or for a certain number of months determined by `duration_in_months`.
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    ///Number of months the discount should be applied.
    ///
    ///Required when `duration` is `repeating`. Must be omitted otherwise.
    ///
    ///For this to work on yearly pricing, you should multiply this by 12.
    ///For example, to apply the discount for 2 years, set this to 24.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub duration_in_months: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateDurationInMonths,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateAmount,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<
        crate::v2027_01::models::common::enums::PresentmentCurrency,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amounts: presence_rs::Presence<
        std::collections::BTreeMap<
            crate::v2027_01::models::common::enums::PresentmentCurrency,
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateAmountsValue,
        >,
    >,
}
#[allow(clippy::type_complexity)]
impl DiscountFixedCreate {
    pub fn new(required: DiscountFixedCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            name: required.name,
            code: presence_rs::Presence::Absent,
            starts_at: presence_rs::Presence::Absent,
            ends_at: presence_rs::Presence::Absent,
            max_redemptions: presence_rs::Presence::Absent,
            max_redemptions_per_customer: presence_rs::Presence::Absent,
            products: presence_rs::Presence::Absent,
            organization_id: presence_rs::Presence::Absent,
            r#type: None,
            duration: required.duration,
            duration_in_months: presence_rs::Presence::Absent,
            amount: presence_rs::Presence::Absent,
            currency: presence_rs::Presence::Absent,
            amounts: presence_rs::Presence::Absent,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
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
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMetadataKey,
            crate::v2027_01::models::discounts::unions::DiscountFixedCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Code customers can use to apply the discount during checkout. Must be
    /// between 3 and 256 characters long and contain only alphanumeric
    /// characters.If not provided, the discount can only be applied via the
    /// API.
    #[allow(clippy::should_implement_trait)]
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = presence_rs::Presence::Some(value.into());
        self
    }

    /// Optional timestamp after which the discount is redeemable.
    #[allow(clippy::should_implement_trait)]
    pub fn starts_at(mut self, value: jiff::Timestamp) -> Self {
        self.starts_at = presence_rs::Presence::Some(value);
        self
    }

    /// Optional timestamp after which the discount is no longer redeemable.
    #[allow(clippy::should_implement_trait)]
    pub fn ends_at(mut self, value: jiff::Timestamp) -> Self {
        self.ends_at = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum number of times the discount can be redeemed.
    #[allow(clippy::should_implement_trait)]
    pub fn max_redemptions(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMaxRedemptions,
    ) -> Self {
        self.max_redemptions = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum number of times the discount can be redeemed by a
    /// single customer.
    #[allow(clippy::should_implement_trait)]
    pub fn max_redemptions_per_customer(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMaxRedemptionsPerCustomer,
    ) -> Self {
        self.max_redemptions_per_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn products(mut self, value: Vec<crate::v2027_01::models::common::ids::ProductId>) -> Self {
        self.products = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the organization owning the discount. **Required unless you
    /// use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn r#type(mut self, value: monostate::MustBe!("fixed")) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Number of months the discount should be applied.
    ///
    /// Required when `duration` is `repeating`. Must be omitted otherwise.
    ///
    /// For this to work on yearly pricing, you should multiply this by 12.
    /// For example, to apply the discount for 2 years, set this to 24.
    #[allow(clippy::should_implement_trait)]
    pub fn duration_in_months(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateDurationInMonths,
    ) -> Self {
        self.duration_in_months = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateAmount,
    ) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.currency = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amounts(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::common::enums::PresentmentCurrency,
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateAmountsValue,
        >,
    ) -> Self {
        self.amounts = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct DiscountFixedOnceForeverDurationRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
/// Schema for a fixed amount discount that is applied once or forever.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountFixedOnceForeverDuration {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
#[allow(clippy::type_complexity)]
impl DiscountFixedOnceForeverDuration {
    pub fn new(required: DiscountFixedOnceForeverDurationRequired) -> Self {
        Self {
            duration: required.duration,
            r#type: required.r#type,
            amount: required.amount,
            currency: required.currency,
            amounts: required.amounts,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
            products: required.products,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountFixedRepeatDurationRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
/// Schema for a fixed amount discount that is applied on every invoice
/// for a certain number of months.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountFixedRepeatDuration {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    pub amount: i64,
    pub currency: String,
    /// Map of currency to fixed amount to discount from the total.
    pub amounts: std::collections::BTreeMap<String, i64>,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
#[allow(clippy::type_complexity)]
impl DiscountFixedRepeatDuration {
    pub fn new(required: DiscountFixedRepeatDurationRequired) -> Self {
        Self {
            duration: required.duration,
            duration_in_months: required.duration_in_months,
            r#type: required.r#type,
            amount: required.amount,
            currency: required.currency,
            amounts: required.amounts,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
            products: required.products,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountPercentageCreateRequired {
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateName,
    /// For subscriptions, determines if the discount should be applied once on
    /// the first invoice, forever, or for a certain number of months determined
    /// by `duration_in_months`.
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    /// Discount percentage in basis points.
    ///
    /// A basis point is 1/100th of a percent.
    /// For example, to create a 25.5% discount, set this to 2550.
    pub basis_points:
        crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateBasisPoints,
}
/// Schema to create a percentage discount.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountPercentageCreate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMetadataKey,
            crate::v2027_01::models::discounts::unions::DiscountPercentageCreateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///Name of the discount. Will be displayed to the customer when the discount is applied.
    pub name: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateName,
    ///Code customers can use to apply the discount during checkout. Must be between 3 and 256 characters long and contain only alphanumeric characters.If not provided, the discount can only be applied via the API.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub code: presence_rs::Presence<String>,
    ///Optional timestamp after which the discount is redeemable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub starts_at: presence_rs::Presence<jiff::Timestamp>,
    ///Optional timestamp after which the discount is no longer redeemable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub ends_at: presence_rs::Presence<jiff::Timestamp>,
    ///Optional maximum number of times the discount can be redeemed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_redemptions: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMaxRedemptions,
    >,
    ///Optional maximum number of times the discount can be redeemed by a single customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_redemptions_per_customer: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMaxRedemptionsPerCustomer,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub products: presence_rs::Presence<
        Vec<crate::v2027_01::models::common::ids::ProductId>,
    >,
    ///The ID of the organization owning the discount. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub r#type: Option<monostate::MustBe!("percentage")>,
    ///For subscriptions, determines if the discount should be applied once on the first invoice, forever, or for a certain number of months determined by `duration_in_months`.
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    ///Number of months the discount should be applied.
    ///
    ///Required when `duration` is `repeating`. Must be omitted otherwise.
    ///
    ///For this to work on yearly pricing, you should multiply this by 12.
    ///For example, to apply the discount for 2 years, set this to 24.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub duration_in_months: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateDurationInMonths,
    >,
    ///Discount percentage in basis points.
    ///
    ///A basis point is 1/100th of a percent.
    ///For example, to create a 25.5% discount, set this to 2550.
    pub basis_points: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateBasisPoints,
}
#[allow(clippy::type_complexity)]
impl DiscountPercentageCreate {
    pub fn new(required: DiscountPercentageCreateRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            name: required.name,
            code: presence_rs::Presence::Absent,
            starts_at: presence_rs::Presence::Absent,
            ends_at: presence_rs::Presence::Absent,
            max_redemptions: presence_rs::Presence::Absent,
            max_redemptions_per_customer: presence_rs::Presence::Absent,
            products: presence_rs::Presence::Absent,
            organization_id: presence_rs::Presence::Absent,
            r#type: None,
            duration: required.duration,
            duration_in_months: presence_rs::Presence::Absent,
            basis_points: required.basis_points,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
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
            crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMetadataKey,
            crate::v2027_01::models::discounts::unions::DiscountPercentageCreateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Code customers can use to apply the discount during checkout. Must be
    /// between 3 and 256 characters long and contain only alphanumeric
    /// characters.If not provided, the discount can only be applied via the
    /// API.
    #[allow(clippy::should_implement_trait)]
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = presence_rs::Presence::Some(value.into());
        self
    }

    /// Optional timestamp after which the discount is redeemable.
    #[allow(clippy::should_implement_trait)]
    pub fn starts_at(mut self, value: jiff::Timestamp) -> Self {
        self.starts_at = presence_rs::Presence::Some(value);
        self
    }

    /// Optional timestamp after which the discount is no longer redeemable.
    #[allow(clippy::should_implement_trait)]
    pub fn ends_at(mut self, value: jiff::Timestamp) -> Self {
        self.ends_at = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum number of times the discount can be redeemed.
    #[allow(clippy::should_implement_trait)]
    pub fn max_redemptions(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMaxRedemptions,
    ) -> Self {
        self.max_redemptions = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum number of times the discount can be redeemed by a
    /// single customer.
    #[allow(clippy::should_implement_trait)]
    pub fn max_redemptions_per_customer(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMaxRedemptionsPerCustomer,
    ) -> Self {
        self.max_redemptions_per_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn products(mut self, value: Vec<crate::v2027_01::models::common::ids::ProductId>) -> Self {
        self.products = presence_rs::Presence::Some(value);
        self
    }

    /// The ID of the organization owning the discount. **Required unless you
    /// use an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn r#type(mut self, value: monostate::MustBe!("percentage")) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Number of months the discount should be applied.
    ///
    /// Required when `duration` is `repeating`. Must be omitted otherwise.
    ///
    /// For this to work on yearly pricing, you should multiply this by 12.
    /// For example, to apply the discount for 2 years, set this to 24.
    #[allow(clippy::should_implement_trait)]
    pub fn duration_in_months(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateDurationInMonths,
    ) -> Self {
        self.duration_in_months = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct DiscountPercentageOnceForeverDurationRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
/// Schema for a percentage discount that is applied once or forever.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountPercentageOnceForeverDuration {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
#[allow(clippy::type_complexity)]
impl DiscountPercentageOnceForeverDuration {
    pub fn new(required: DiscountPercentageOnceForeverDurationRequired) -> Self {
        Self {
            duration: required.duration,
            r#type: required.r#type,
            basis_points: required.basis_points,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
            products: required.products,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountPercentageRepeatDurationRequired {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
/// Schema for a percentage discount that is applied on every invoice
/// for a certain number of months.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountPercentageRepeatDuration {
    pub duration: crate::v2027_01::models::common::enums::DiscountDuration,
    pub duration_in_months: i64,
    pub r#type: crate::v2027_01::models::common::enums::DiscountType,
    /// Discount percentage in basis points. A basis point is 1/100th of a
    /// percent. For example, 1000 basis points equals a 10% discount.
    pub basis_points: i64,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::DiscountId,
    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub external_id: Option<String>,
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// Name of the discount. Will be displayed to the customer when the
    /// discount is applied.
    pub name: String,
    /// Code customers can use to apply the discount during checkout.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub code: Option<String>,
    /// Timestamp after which the discount is redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub starts_at: Option<jiff::Timestamp>,
    /// Timestamp after which the discount is no longer redeemable.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub ends_at: Option<jiff::Timestamp>,
    /// Maximum number of times the discount can be redeemed.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions: Option<i64>,
    /// Maximum number of times the discount can be redeemed by a single
    /// customer.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub max_redemptions_per_customer: Option<i64>,
    /// Number of times the discount has been redeemed.
    pub redemptions_count: i64,
    /// The organization ID.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
    pub products: Vec<crate::v2027_01::models::discounts::types::DiscountProduct>,
}
#[allow(clippy::type_complexity)]
impl DiscountPercentageRepeatDuration {
    pub fn new(required: DiscountPercentageRepeatDurationRequired) -> Self {
        Self {
            duration: required.duration,
            duration_in_months: required.duration_in_months,
            r#type: required.r#type,
            basis_points: required.basis_points,
            created_at: required.created_at,
            modified_at: required.modified_at,
            id: required.id,
            external_id: required.external_id,
            metadata: required.metadata,
            name: required.name,
            code: required.code,
            starts_at: required.starts_at,
            ends_at: required.ends_at,
            max_redemptions: required.max_redemptions,
            max_redemptions_per_customer: required.max_redemptions_per_customer,
            redemptions_count: required.redemptions_count,
            organization_id: required.organization_id,
            products: required.products,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DiscountProductRequired {
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    pub trial_interval: Option<crate::v2027_01::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2027_01::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    pub recurring_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    pub meter_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
/// A product that a discount can be applied to.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct DiscountProduct {
    pub metadata: crate::v2027_01::models::common::aliases::MetadataOutputType,
    /// The ID of the object.
    pub id: crate::v2027_01::models::common::ids::ProductId,
    /// Creation timestamp of the object.
    pub created_at: jiff::Timestamp,
    /// Last modification timestamp of the object.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub modified_at: Option<jiff::Timestamp>,
    /// The interval unit for the trial period.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval: Option<crate::v2027_01::models::common::enums::TrialInterval>,
    /// The number of interval units for the trial period.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub trial_interval_count: Option<i64>,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub description: Option<String>,
    /// The visibility of the product.
    pub visibility: crate::v2027_01::models::common::enums::ProductVisibility,
    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. None for one-time products.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub recurring_interval_count: Option<i64>,
    /// The meter cycle of the product, independent of the billing interval. If
    /// `None`, metered concerns follow the billing interval.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval: Option<crate::v2027_01::models::common::enums::RecurringInterval>,
    /// Number of meter interval units. None when no meter cycle is set.
    #[serde(
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_required_nullable"
    )]
    pub meter_interval_count: Option<i64>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: crate::v2027_01::models::common::ids::OrganizationId,
}
#[allow(clippy::type_complexity)]
impl DiscountProduct {
    pub fn new(required: DiscountProductRequired) -> Self {
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
        }
    }
}
/// Schema to update a discount.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct DiscountUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub external_id: presence_rs::Presence<String>,
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
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub metadata: Option<
        std::collections::BTreeMap<
            crate::v2027_01::models::discounts::newtypes::DiscountMetadataKey,
            crate::v2027_01::models::discounts::unions::DiscountUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountName,
    >,
    ///Code customers can use to apply the discount during checkout. Must be between 3 and 256 characters long and contain only alphanumeric characters.If not provided, the discount can only be applied via the API.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub code: presence_rs::Presence<String>,
    ///Optional timestamp after which the discount is redeemable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub starts_at: presence_rs::Presence<jiff::Timestamp>,
    ///Optional timestamp after which the discount is no longer redeemable.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub ends_at: presence_rs::Presence<jiff::Timestamp>,
    ///Optional maximum number of times the discount can be redeemed.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_redemptions: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountMaxRedemptions,
    >,
    ///Optional maximum number of times the discount can be redeemed by a single customer.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub max_redemptions_per_customer: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountMaxRedemptionsPerCustomer,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub duration: presence_rs::Presence<
        crate::v2027_01::models::common::enums::DiscountDuration,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub duration_in_months: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountDurationInMonths,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub r#type: presence_rs::Presence<
        crate::v2027_01::models::common::enums::DiscountType,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amount: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountAmount,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub currency: presence_rs::Presence<
        crate::v2027_01::models::common::enums::PresentmentCurrency,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub amounts: presence_rs::Presence<
        std::collections::BTreeMap<
            crate::v2027_01::models::common::enums::PresentmentCurrency,
            crate::v2027_01::models::discounts::newtypes::DiscountAmountsValue,
        >,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub basis_points: presence_rs::Presence<
        crate::v2027_01::models::discounts::newtypes::DiscountBasisPoints,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub products: presence_rs::Presence<
        Vec<crate::v2027_01::models::common::ids::ProductId>,
    >,
}
#[allow(clippy::type_complexity)]
impl DiscountUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource.
    #[allow(clippy::should_implement_trait)]
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = presence_rs::Presence::Some(value.into());
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
            crate::v2027_01::models::discounts::newtypes::DiscountMetadataKey,
            crate::v2027_01::models::discounts::unions::DiscountUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountName,
    ) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// Code customers can use to apply the discount during checkout. Must be
    /// between 3 and 256 characters long and contain only alphanumeric
    /// characters.If not provided, the discount can only be applied via the
    /// API.
    #[allow(clippy::should_implement_trait)]
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = presence_rs::Presence::Some(value.into());
        self
    }

    /// Optional timestamp after which the discount is redeemable.
    #[allow(clippy::should_implement_trait)]
    pub fn starts_at(mut self, value: jiff::Timestamp) -> Self {
        self.starts_at = presence_rs::Presence::Some(value);
        self
    }

    /// Optional timestamp after which the discount is no longer redeemable.
    #[allow(clippy::should_implement_trait)]
    pub fn ends_at(mut self, value: jiff::Timestamp) -> Self {
        self.ends_at = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum number of times the discount can be redeemed.
    #[allow(clippy::should_implement_trait)]
    pub fn max_redemptions(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountMaxRedemptions,
    ) -> Self {
        self.max_redemptions = presence_rs::Presence::Some(value);
        self
    }

    /// Optional maximum number of times the discount can be redeemed by a
    /// single customer.
    #[allow(clippy::should_implement_trait)]
    pub fn max_redemptions_per_customer(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountMaxRedemptionsPerCustomer,
    ) -> Self {
        self.max_redemptions_per_customer = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn duration(
        mut self,
        value: crate::v2027_01::models::common::enums::DiscountDuration,
    ) -> Self {
        self.duration = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn duration_in_months(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountDurationInMonths,
    ) -> Self {
        self.duration_in_months = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn r#type(mut self, value: crate::v2027_01::models::common::enums::DiscountType) -> Self {
        self.r#type = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amount(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountAmount,
    ) -> Self {
        self.amount = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn currency(
        mut self,
        value: crate::v2027_01::models::common::enums::PresentmentCurrency,
    ) -> Self {
        self.currency = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn amounts(
        mut self,
        value: std::collections::BTreeMap<
            crate::v2027_01::models::common::enums::PresentmentCurrency,
            crate::v2027_01::models::discounts::newtypes::DiscountAmountsValue,
        >,
    ) -> Self {
        self.amounts = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn basis_points(
        mut self,
        value: crate::v2027_01::models::discounts::newtypes::DiscountBasisPoints,
    ) -> Self {
        self.basis_points = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn products(mut self, value: Vec<crate::v2027_01::models::common::ids::ProductId>) -> Self {
        self.products = presence_rs::Presence::Some(value);
        self
    }
}
