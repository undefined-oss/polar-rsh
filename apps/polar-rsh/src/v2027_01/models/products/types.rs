#[derive(Debug, Clone)]
pub struct AttachedCustomFieldCreateRequired {
    /// ID of the custom field to attach.
    pub custom_field_id: crate::v2027_01::models::common::ids::CustomFieldId,
    /// Whether the value is required for this custom field.
    pub required: bool,
}
/// Schema to attach a custom field to a resource.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct AttachedCustomFieldCreate {
    /// ID of the custom field to attach.
    pub custom_field_id: crate::v2027_01::models::common::ids::CustomFieldId,
    /// Whether the value is required for this custom field.
    pub required: bool,
}
#[allow(clippy::type_complexity)]
impl AttachedCustomFieldCreate {
    pub fn new(required: AttachedCustomFieldCreateRequired) -> Self {
        Self { custom_field_id: required.custom_field_id, required: required.required }
    }
}
#[derive(Debug, Clone)]
pub struct ExistingProductPriceRequired {
    pub id: crate::v2027_01::models::common::ids::ExistingProductPriceId,
}
/// A price that already exists for this product.
///
/// Useful when updating a product if you want to keep an existing price.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ExistingProductPrice {
    pub id: crate::v2027_01::models::common::ids::ExistingProductPriceId,
}
#[allow(clippy::type_complexity)]
impl ExistingProductPrice {
    pub fn new(required: ExistingProductPriceRequired) -> Self {
        Self { id: required.id }
    }
}
#[derive(Debug, Clone)]
pub struct ProductBenefitsUpdateRequired {
    /// List of benefit IDs. Each one must be on the same organization as the
    /// product.
    pub benefits: Vec<crate::v2027_01::models::common::ids::BenefitId>,
}
/// Schema to update the benefits granted by a product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductBenefitsUpdate {
    /// List of benefit IDs. Each one must be on the same organization as the
    /// product.
    pub benefits: Vec<crate::v2027_01::models::common::ids::BenefitId>,
}
#[allow(clippy::type_complexity)]
impl ProductBenefitsUpdate {
    pub fn new(required: ProductBenefitsUpdateRequired) -> Self {
        Self { benefits: required.benefits }
    }
}
#[derive(Debug, Clone)]
pub struct ProductCreateOneTimeRequired {
    /// The name of the product.
    pub name: crate::v2027_01::models::products::newtypes::ProductCreateOneTimeName,
    /// List of available prices for this product. It may combine at most one
    /// fixed price with one seat-based price (billed as `fixed + seat_charge`),
    /// or contain a single custom or free price, plus any number of metered
    /// prices. A free price cannot be combined with other prices, and a custom
    /// price cannot be combined with a fixed or seat-based price. Metered
    /// prices are not supported on one-time purchase products.
    pub prices:
        Vec<crate::v2027_01::models::products::unions::ProductCreateOneTimePricesItemsUnion>,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductCreateOneTime {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource. It must start with a letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
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
            crate::v2027_01::models::products::newtypes::ProductCreateOneTimeMetadataKey,
            crate::v2027_01::models::products::unions::ProductCreateOneTimeMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The name of the product.
    pub name: crate::v2027_01::models::products::newtypes::ProductCreateOneTimeName,
    ///The description of the product.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<String>,
    ///The visibility of the product.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub visibility: Option<crate::v2027_01::models::common::enums::ProductVisibility>,
    ///List of available prices for this product. It may combine at most one fixed price with one seat-based price (billed as `fixed + seat_charge`), or contain a single custom or free price, plus any number of metered prices. A free price cannot be combined with other prices, and a custom price cannot be combined with a fixed or seat-based price. Metered prices are not supported on one-time purchase products.
    pub prices: Vec<
        crate::v2027_01::models::products::unions::ProductCreateOneTimePricesItemsUnion,
    >,
    ///List of file IDs. Each one must be on the same organization as the product, of type `product_media` and correctly uploaded.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub medias: presence_rs::Presence<
        Vec<crate::v2027_01::models::common::ids::ProductCreateMediasId>,
    >,
    ///List of custom fields to attach.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub attached_custom_fields: Option<
        Vec<crate::v2027_01::models::products::types::AttachedCustomFieldCreate>,
    >,
    ///The ID of the organization owning the product. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///States that the product is a one-time purchase.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval: Option<()>,
    ///One-time products don't have a recurring interval count.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<()>,
}
#[allow(clippy::type_complexity)]
impl ProductCreateOneTime {
    pub fn new(required: ProductCreateOneTimeRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            name: required.name,
            description: presence_rs::Presence::Absent,
            visibility: None,
            prices: required.prices,
            medias: presence_rs::Presence::Absent,
            attached_custom_fields: None,
            organization_id: presence_rs::Presence::Absent,
            recurring_interval: None,
            recurring_interval_count: None,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
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
            crate::v2027_01::models::products::newtypes::ProductCreateOneTimeMetadataKey,
            crate::v2027_01::models::products::unions::ProductCreateOneTimeMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the product.
    #[allow(clippy::should_implement_trait)]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = presence_rs::Presence::Some(value.into());
        self
    }

    /// The visibility of the product.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::ProductVisibility,
    ) -> Self {
        self.visibility = Some(value);
        self
    }

    /// List of file IDs. Each one must be on the same organization as the
    /// product, of type `product_media` and correctly uploaded.
    #[allow(clippy::should_implement_trait)]
    pub fn medias(
        mut self,
        value: Vec<crate::v2027_01::models::common::ids::ProductCreateMediasId>,
    ) -> Self {
        self.medias = presence_rs::Presence::Some(value);
        self
    }

    /// List of custom fields to attach.
    #[allow(clippy::should_implement_trait)]
    pub fn attached_custom_fields(
        mut self,
        value: Vec<crate::v2027_01::models::products::types::AttachedCustomFieldCreate>,
    ) -> Self {
        self.attached_custom_fields = Some(value);
        self
    }

    /// The ID of the organization owning the product. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// States that the product is a one-time purchase.
    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(mut self, value: ()) -> Self {
        self.recurring_interval = Some(value);
        self
    }

    /// One-time products don't have a recurring interval count.
    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(mut self, value: ()) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductCreateRecurringRequired {
    /// The name of the product.
    pub name: crate::v2027_01::models::products::newtypes::ProductCreateRecurringName,
    /// List of available prices for this product. It may combine at most one
    /// fixed price with one seat-based price (billed as `fixed + seat_charge`),
    /// or contain a single custom or free price, plus any number of metered
    /// prices. A free price cannot be combined with other prices, and a custom
    /// price cannot be combined with a fixed or seat-based price. Metered
    /// prices are not supported on one-time purchase products.
    pub prices:
        Vec<crate::v2027_01::models::products::unions::ProductCreateRecurringPricesItemsUnion>,
    /// The recurring interval of the product.
    pub recurring_interval: crate::v2027_01::models::common::enums::RecurringInterval,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
pub struct ProductCreateRecurring {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource. It must start with a letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
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
            crate::v2027_01::models::products::newtypes::ProductCreateRecurringMetadataKey,
            crate::v2027_01::models::products::unions::ProductCreateRecurringMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The name of the product.
    pub name: crate::v2027_01::models::products::newtypes::ProductCreateRecurringName,
    ///The description of the product.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<String>,
    ///The visibility of the product.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub visibility: Option<crate::v2027_01::models::common::enums::ProductVisibility>,
    ///List of available prices for this product. It may combine at most one fixed price with one seat-based price (billed as `fixed + seat_charge`), or contain a single custom or free price, plus any number of metered prices. A free price cannot be combined with other prices, and a custom price cannot be combined with a fixed or seat-based price. Metered prices are not supported on one-time purchase products.
    pub prices: Vec<
        crate::v2027_01::models::products::unions::ProductCreateRecurringPricesItemsUnion,
    >,
    ///List of file IDs. Each one must be on the same organization as the product, of type `product_media` and correctly uploaded.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub medias: presence_rs::Presence<
        Vec<crate::v2027_01::models::common::ids::ProductCreateMediasId>,
    >,
    ///List of custom fields to attach.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub attached_custom_fields: Option<
        Vec<crate::v2027_01::models::products::types::AttachedCustomFieldCreate>,
    >,
    ///The ID of the organization owning the product. **Required unless you use an organization token.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub organization_id: presence_rs::Presence<
        crate::v2027_01::models::common::ids::OrganizationId,
    >,
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2027_01::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2027_01::models::products::newtypes::ProductCreateRecurringTrialIntervalCount,
    >,
    ///Whether customers must provide a payment method to start a trial. If `false`, customers can start a trial without entering payment details.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub trial_payment_setup_required: Option<bool>,
    ///The recurring interval of the product.
    pub recurring_interval: crate::v2027_01::models::common::enums::RecurringInterval,
    ///Number of interval units of the subscription. If this is set to 1 the charge will happen every interval (e.g. every month), if set to 2 it will be every other month, and so on.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
    )]
    pub recurring_interval_count: Option<
        crate::v2027_01::models::products::newtypes::ProductCreateRecurringRecurringIntervalCount,
    >,
    ///Optional meter cycle, independent of the billing interval. When set, overage settlement, meter resets and meter-credit grants run on this cadence rather than the billing interval — e.g. yearly billing with monthly credits. It must evenly divide the billing interval. If `None`, metered concerns follow the billing interval. **Once set, it can't be changed.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub meter_interval: presence_rs::Presence<
        crate::v2027_01::models::common::enums::RecurringInterval,
    >,
    ///Number of meter interval units. Defaults to 1 when `meter_interval` is set. Ignored when `meter_interval` is `None`.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub meter_interval_count: presence_rs::Presence<
        crate::v2027_01::models::products::newtypes::ProductCreateRecurringMeterIntervalCount,
    >,
}
#[allow(clippy::type_complexity)]
impl ProductCreateRecurring {
    pub fn new(required: ProductCreateRecurringRequired) -> Self {
        Self {
            external_id: presence_rs::Presence::Absent,
            metadata: None,
            name: required.name,
            description: presence_rs::Presence::Absent,
            visibility: None,
            prices: required.prices,
            medias: presence_rs::Presence::Absent,
            attached_custom_fields: None,
            organization_id: presence_rs::Presence::Absent,
            trial_interval: presence_rs::Presence::Absent,
            trial_interval_count: presence_rs::Presence::Absent,
            trial_payment_setup_required: None,
            recurring_interval: required.recurring_interval,
            recurring_interval_count: None,
            meter_interval: presence_rs::Presence::Absent,
            meter_interval_count: presence_rs::Presence::Absent,
        }
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
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
            crate::v2027_01::models::products::newtypes::ProductCreateRecurringMetadataKey,
            crate::v2027_01::models::products::unions::ProductCreateRecurringMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The description of the product.
    #[allow(clippy::should_implement_trait)]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = presence_rs::Presence::Some(value.into());
        self
    }

    /// The visibility of the product.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::ProductVisibility,
    ) -> Self {
        self.visibility = Some(value);
        self
    }

    /// List of file IDs. Each one must be on the same organization as the
    /// product, of type `product_media` and correctly uploaded.
    #[allow(clippy::should_implement_trait)]
    pub fn medias(
        mut self,
        value: Vec<crate::v2027_01::models::common::ids::ProductCreateMediasId>,
    ) -> Self {
        self.medias = presence_rs::Presence::Some(value);
        self
    }

    /// List of custom fields to attach.
    #[allow(clippy::should_implement_trait)]
    pub fn attached_custom_fields(
        mut self,
        value: Vec<crate::v2027_01::models::products::types::AttachedCustomFieldCreate>,
    ) -> Self {
        self.attached_custom_fields = Some(value);
        self
    }

    /// The ID of the organization owning the product. **Required unless you use
    /// an organization token.**
    #[allow(clippy::should_implement_trait)]
    pub fn organization_id(
        mut self,
        value: crate::v2027_01::models::common::ids::OrganizationId,
    ) -> Self {
        self.organization_id = presence_rs::Presence::Some(value);
        self
    }

    /// The interval unit for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval(
        mut self,
        value: crate::v2027_01::models::common::enums::TrialInterval,
    ) -> Self {
        self.trial_interval = presence_rs::Presence::Some(value);
        self
    }

    /// The number of interval units for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval_count(
        mut self,
        value: crate::v2027_01::models::products::newtypes::ProductCreateRecurringTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
        self
    }

    /// Whether customers must provide a payment method to start a trial. If
    /// `false`, customers can start a trial without entering payment details.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_payment_setup_required(mut self, value: bool) -> Self {
        self.trial_payment_setup_required = Some(value);
        self
    }

    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on.
    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(
        mut self,
        value: crate::v2027_01::models::products::newtypes::ProductCreateRecurringRecurringIntervalCount,
    ) -> Self {
        self.recurring_interval_count = Some(value);
        self
    }

    /// Optional meter cycle, independent of the billing interval. When set,
    /// overage settlement, meter resets and meter-credit grants run on this
    /// cadence rather than the billing interval — e.g. yearly billing with
    /// monthly credits. It must evenly divide the billing interval. If `None`,
    /// metered concerns follow the billing interval. **Once set, it can't be
    /// changed.**
    #[allow(clippy::should_implement_trait)]
    pub fn meter_interval(
        mut self,
        value: crate::v2027_01::models::common::enums::RecurringInterval,
    ) -> Self {
        self.meter_interval = presence_rs::Presence::Some(value);
        self
    }

    /// Number of meter interval units. Defaults to 1 when `meter_interval` is
    /// set. Ignored when `meter_interval` is `None`.
    #[allow(clippy::should_implement_trait)]
    pub fn meter_interval_count(
        mut self,
        value: crate::v2027_01::models::products::newtypes::ProductCreateRecurringMeterIntervalCount,
    ) -> Self {
        self.meter_interval_count = presence_rs::Presence::Some(value);
        self
    }
}
#[derive(Debug, Clone)]
pub struct ProductNotDeletableRequired {
    pub detail: String,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(thiserror::Error)]
#[error("{self:?}")]
pub struct ProductNotDeletable {
    pub error: monostate::MustBe!("ProductNotDeletable"),
    pub detail: String,
}
#[allow(clippy::type_complexity)]
impl ProductNotDeletable {
    pub fn new(required: ProductNotDeletableRequired) -> Self {
        Self { error: Default::default(), detail: required.detail }
    }
}
/// Schema to update a product.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[allow(clippy::type_complexity)]
#[serde(deny_unknown_fields)]
#[derive(Default)]
pub struct ProductUpdate {
    ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource. It must start with a letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
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
            crate::v2027_01::models::products::newtypes::ProductMetadataKey,
            crate::v2027_01::models::products::unions::ProductUpdateMetadataAdditionalPropertiesUnion,
        >,
    >,
    ///The interval unit for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval: presence_rs::Presence<
        crate::v2027_01::models::common::enums::TrialInterval,
    >,
    ///The number of interval units for the trial period.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_interval_count: presence_rs::Presence<
        crate::v2027_01::models::products::newtypes::ProductTrialIntervalCount,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub name: presence_rs::Presence<
        crate::v2027_01::models::products::newtypes::ProductName,
    >,
    ///The description of the product.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub description: presence_rs::Presence<String>,
    ///The recurring interval of the product. If `None`, the product is a one-time purchase. **Can only be set on legacy recurring products. Once set, it can't be changed.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub recurring_interval: presence_rs::Presence<
        crate::v2027_01::models::common::enums::RecurringInterval,
    >,
    ///Number of interval units of the subscription. If this is set to 1 the charge will happen every interval (e.g. every month), if set to 2 it will be every other month, and so on. Once set, it can't be changed.**
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub recurring_interval_count: presence_rs::Presence<
        crate::v2027_01::models::products::newtypes::ProductRecurringIntervalCount,
    >,
    ///Whether the product is archived. If `true`, the product won't be available for purchase anymore. Existing customers will still have access to their benefits, and subscriptions will continue normally.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub is_archived: presence_rs::Presence<bool>,
    ///The visibility of the product.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub visibility: presence_rs::Presence<
        crate::v2027_01::models::common::enums::ProductVisibility,
    >,
    ///List of available prices for this product. If you want to keep existing prices, include them in the list as an `ExistingProductPrice` object.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub prices: presence_rs::Presence<
        Vec<
            crate::v2027_01::models::products::unions::ProductUpdatePricesAnyOf0ItemsUnion,
        >,
    >,
    ///List of file IDs. Each one must be on the same organization as the product, of type `product_media` and correctly uploaded.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub medias: presence_rs::Presence<
        Vec<crate::v2027_01::models::common::ids::ProductUpdateMediasId>,
    >,
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub attached_custom_fields: presence_rs::Presence<
        Vec<crate::v2027_01::models::products::types::AttachedCustomFieldCreate>,
    >,
    ///Whether customers must provide a payment method to start a trial. If `false`, customers can start a trial without entering payment details.
    #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
    pub trial_payment_setup_required: presence_rs::Presence<bool>,
}
#[allow(clippy::type_complexity)]
impl ProductUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    /// An ID from your own system to reference this resource. It must be unique
    /// within the organization for this type of resource. It must start with a
    /// letter or digit and contain only letters, digits, `_`, `-`, `.` and `:`.
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
            crate::v2027_01::models::products::newtypes::ProductMetadataKey,
            crate::v2027_01::models::products::unions::ProductUpdateMetadataAdditionalPropertiesUnion,
        >,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    /// The interval unit for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval(
        mut self,
        value: crate::v2027_01::models::common::enums::TrialInterval,
    ) -> Self {
        self.trial_interval = presence_rs::Presence::Some(value);
        self
    }

    /// The number of interval units for the trial period.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_interval_count(
        mut self,
        value: crate::v2027_01::models::products::newtypes::ProductTrialIntervalCount,
    ) -> Self {
        self.trial_interval_count = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn name(mut self, value: crate::v2027_01::models::products::newtypes::ProductName) -> Self {
        self.name = presence_rs::Presence::Some(value);
        self
    }

    /// The description of the product.
    #[allow(clippy::should_implement_trait)]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = presence_rs::Presence::Some(value.into());
        self
    }

    /// The recurring interval of the product. If `None`, the product is a
    /// one-time purchase. **Can only be set on legacy recurring products. Once
    /// set, it can't be changed.**
    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval(
        mut self,
        value: crate::v2027_01::models::common::enums::RecurringInterval,
    ) -> Self {
        self.recurring_interval = presence_rs::Presence::Some(value);
        self
    }

    /// Number of interval units of the subscription. If this is set to 1 the
    /// charge will happen every interval (e.g. every month), if set to 2 it
    /// will be every other month, and so on. Once set, it can't be changed.**
    #[allow(clippy::should_implement_trait)]
    pub fn recurring_interval_count(
        mut self,
        value: crate::v2027_01::models::products::newtypes::ProductRecurringIntervalCount,
    ) -> Self {
        self.recurring_interval_count = presence_rs::Presence::Some(value);
        self
    }

    /// Whether the product is archived. If `true`, the product won't be
    /// available for purchase anymore. Existing customers will still have
    /// access to their benefits, and subscriptions will continue normally.
    #[allow(clippy::should_implement_trait)]
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = presence_rs::Presence::Some(value);
        self
    }

    /// The visibility of the product.
    #[allow(clippy::should_implement_trait)]
    pub fn visibility(
        mut self,
        value: crate::v2027_01::models::common::enums::ProductVisibility,
    ) -> Self {
        self.visibility = presence_rs::Presence::Some(value);
        self
    }

    /// List of available prices for this product. If you want to keep existing
    /// prices, include them in the list as an `ExistingProductPrice` object.
    #[allow(clippy::should_implement_trait)]
    pub fn prices(
        mut self,
        value: Vec<crate::v2027_01::models::products::unions::ProductUpdatePricesAnyOf0ItemsUnion>,
    ) -> Self {
        self.prices = presence_rs::Presence::Some(value);
        self
    }

    /// List of file IDs. Each one must be on the same organization as the
    /// product, of type `product_media` and correctly uploaded.
    #[allow(clippy::should_implement_trait)]
    pub fn medias(
        mut self,
        value: Vec<crate::v2027_01::models::common::ids::ProductUpdateMediasId>,
    ) -> Self {
        self.medias = presence_rs::Presence::Some(value);
        self
    }

    #[allow(clippy::should_implement_trait)]
    pub fn attached_custom_fields(
        mut self,
        value: Vec<crate::v2027_01::models::products::types::AttachedCustomFieldCreate>,
    ) -> Self {
        self.attached_custom_fields = presence_rs::Presence::Some(value);
        self
    }

    /// Whether customers must provide a payment method to start a trial. If
    /// `false`, customers can start a trial without entering payment details.
    #[allow(clippy::should_implement_trait)]
    pub fn trial_payment_setup_required(mut self, value: bool) -> Self {
        self.trial_payment_setup_required = presence_rs::Presence::Some(value);
        self
    }
}
