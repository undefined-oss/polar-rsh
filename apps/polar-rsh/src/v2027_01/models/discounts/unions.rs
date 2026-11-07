#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Discount {
    DiscountFixedOnceForeverDuration(
        crate::v2027_01::models::discounts::types::DiscountFixedOnceForeverDuration,
    ),
    DiscountFixedRepeatDuration(
        crate::v2027_01::models::discounts::types::DiscountFixedRepeatDuration,
    ),
    DiscountPercentageOnceForeverDuration(
        crate::v2027_01::models::discounts::types::DiscountPercentageOnceForeverDuration,
    ),
    DiscountPercentageRepeatDuration(
        crate::v2027_01::models::discounts::types::DiscountPercentageRepeatDuration,
    ),
}
impl<'de> serde::Deserialize<'de> for Discount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::discounts::types::DiscountFixedOnceForeverDuration,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Discount"
                )));
            }
            matched = Some(Self::DiscountFixedOnceForeverDuration(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::discounts::types::DiscountFixedRepeatDuration,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Discount"
                )));
            }
            matched = Some(Self::DiscountFixedRepeatDuration(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::discounts::types::DiscountPercentageOnceForeverDuration,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Discount"
                )));
            }
            matched = Some(Self::DiscountPercentageOnceForeverDuration(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::discounts::types::DiscountPercentageRepeatDuration,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "Discount"
                )));
            }
            matched = Some(Self::DiscountPercentageRepeatDuration(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!("no matching oneOf variant for ", "Discount"))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum DiscountCreate {
    #[serde(rename = "fixed")]
    Fixed {
        ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_id: presence_rs::Presence<String>,
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
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMetadataKey,
                crate::v2027_01::models::discounts::unions::DiscountFixedCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Name of the discount. Will be displayed to the customer when the discount is applied.
        name: crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateName,
        ///Code customers can use to apply the discount during checkout. Must be between 3 and 256 characters long and contain only alphanumeric characters.If not provided, the discount can only be applied via the API.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        code: presence_rs::Presence<String>,
        ///Optional timestamp after which the discount is redeemable.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        starts_at: presence_rs::Presence<jiff::Timestamp>,
        ///Optional timestamp after which the discount is no longer redeemable.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        ends_at: presence_rs::Presence<jiff::Timestamp>,
        ///Optional maximum number of times the discount can be redeemed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        max_redemptions: presence_rs::Presence<
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMaxRedemptions,
        >,
        ///Optional maximum number of times the discount can be redeemed by a single customer.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        max_redemptions_per_customer: presence_rs::Presence<
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMaxRedemptionsPerCustomer,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        products: presence_rs::Presence<
            Vec<crate::v2027_01::models::common::ids::ProductId>,
        >,
        ///The ID of the organization owning the discount. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2027_01::models::common::ids::OrganizationId,
        >,
        ///For subscriptions, determines if the discount should be applied once on the first invoice, forever, or for a certain number of months determined by `duration_in_months`.
        duration: crate::v2027_01::models::common::enums::DiscountDuration,
        ///Number of months the discount should be applied.
        ///
        ///Required when `duration` is `repeating`. Must be omitted otherwise.
        ///
        ///For this to work on yearly pricing, you should multiply this by 12.
        ///For example, to apply the discount for 2 years, set this to 24.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        duration_in_months: presence_rs::Presence<
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateDurationInMonths,
        >,
        ///Map of currency to fixed amount to discount from the total. This allows specifying different discount amounts for different currencies.
        amounts: std::collections::BTreeMap<
            crate::v2027_01::models::common::enums::PresentmentCurrency,
            crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateAmountsValue,
        >,
    },
    #[serde(rename = "percentage")]
    Percentage {
        ///An ID from your own system to reference this resource. It must be unique within the organization for this type of resource.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        external_id: presence_rs::Presence<String>,
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
        metadata: Option<
            std::collections::BTreeMap<
                crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMetadataKey,
                crate::v2027_01::models::discounts::unions::DiscountPercentageCreateMetadataAdditionalPropertiesUnion,
            >,
        >,
        ///Name of the discount. Will be displayed to the customer when the discount is applied.
        name: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateName,
        ///Code customers can use to apply the discount during checkout. Must be between 3 and 256 characters long and contain only alphanumeric characters.If not provided, the discount can only be applied via the API.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        code: presence_rs::Presence<String>,
        ///Optional timestamp after which the discount is redeemable.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        starts_at: presence_rs::Presence<jiff::Timestamp>,
        ///Optional timestamp after which the discount is no longer redeemable.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        ends_at: presence_rs::Presence<jiff::Timestamp>,
        ///Optional maximum number of times the discount can be redeemed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        max_redemptions: presence_rs::Presence<
            crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMaxRedemptions,
        >,
        ///Optional maximum number of times the discount can be redeemed by a single customer.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        max_redemptions_per_customer: presence_rs::Presence<
            crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMaxRedemptionsPerCustomer,
        >,
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        products: presence_rs::Presence<
            Vec<crate::v2027_01::models::common::ids::ProductId>,
        >,
        ///The ID of the organization owning the discount. **Required unless you use an organization token.**
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        organization_id: presence_rs::Presence<
            crate::v2027_01::models::common::ids::OrganizationId,
        >,
        ///For subscriptions, determines if the discount should be applied once on the first invoice, forever, or for a certain number of months determined by `duration_in_months`.
        duration: crate::v2027_01::models::common::enums::DiscountDuration,
        ///Number of months the discount should be applied.
        ///
        ///Required when `duration` is `repeating`. Must be omitted otherwise.
        ///
        ///For this to work on yearly pricing, you should multiply this by 12.
        ///For example, to apply the discount for 2 years, set this to 24.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        duration_in_months: presence_rs::Presence<
            crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateDurationInMonths,
        >,
        ///Discount percentage in basis points.
        ///
        ///A basis point is 1/100th of a percent.
        ///For example, to create a 25.5% discount, set this to 2550.
        basis_points: crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateBasisPoints,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum DiscountFixedCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::discounts::newtypes::DiscountFixedCreateMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum DiscountPercentageCreateMetadataAdditionalPropertiesUnion {
    String(
        crate::v2027_01::models::discounts::newtypes::DiscountPercentageCreateMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum DiscountUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::discounts::newtypes::DiscountMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
