#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CheckoutLinkCreate {
    CheckoutLinkCreateProductPrice(
        crate::v2027_01::models::checkout_links::types::CheckoutLinkCreateProductPrice,
    ),
    CheckoutLinkCreateProduct(
        crate::v2027_01::models::checkout_links::types::CheckoutLinkCreateProduct,
    ),
    CheckoutLinkCreateProducts(
        crate::v2027_01::models::checkout_links::types::CheckoutLinkCreateProducts,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CheckoutLinkCreateProductMetadataAdditionalPropertiesUnion {
    String(
        crate::v2027_01::models::checkout_links::newtypes::CheckoutLinkCreateProductMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CheckoutLinkCreateProductPriceMetadataAdditionalPropertiesUnion {
    String(
        crate::v2027_01::models::checkout_links::newtypes::CheckoutLinkCreateProductPriceMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CheckoutLinkCreateProductsMetadataAdditionalPropertiesUnion {
    String(
        crate::v2027_01::models::checkout_links::newtypes::CheckoutLinkCreateProductsMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CheckoutLinkDiscountAnyOf0Union {
    DiscountFixedOnceForeverDurationBase(
        crate::v2027_01::models::common::types::DiscountFixedOnceForeverDurationBase,
    ),
    DiscountFixedRepeatDurationBase(
        crate::v2027_01::models::common::types::DiscountFixedRepeatDurationBase,
    ),
    DiscountPercentageOnceForeverDurationBase(
        crate::v2027_01::models::common::types::DiscountPercentageOnceForeverDurationBase,
    ),
    DiscountPercentageRepeatDurationBase(
        crate::v2027_01::models::common::types::DiscountPercentageRepeatDurationBase,
    ),
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkDiscountAnyOf0Union {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::common::types::DiscountFixedOnceForeverDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CheckoutLinkDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountFixedOnceForeverDurationBase(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::common::types::DiscountFixedRepeatDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CheckoutLinkDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountFixedRepeatDurationBase(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::common::types::DiscountPercentageOnceForeverDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CheckoutLinkDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountPercentageOnceForeverDurationBase(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::common::types::DiscountPercentageRepeatDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CheckoutLinkDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountPercentageRepeatDurationBase(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "CheckoutLinkDiscountAnyOf0Union"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CheckoutLinkProductPricesItemsUnion {
    LegacyRecurringProductPrice(
        crate::v2027_01::models::common::unions::LegacyRecurringProductPrice,
    ),
    ProductPrice(crate::v2027_01::models::common::unions::ProductPrice),
}
impl<'de> serde::Deserialize<'de> for CheckoutLinkProductPricesItemsUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::common::unions::LegacyRecurringProductPrice,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CheckoutLinkProductPricesItemsUnion"
                )));
            }
            matched = Some(Self::LegacyRecurringProductPrice(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::common::unions::ProductPrice,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CheckoutLinkProductPricesItemsUnion"
                )));
            }
            matched = Some(Self::ProductPrice(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "CheckoutLinkProductPricesItemsUnion"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CheckoutLinkUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::checkout_links::newtypes::CheckoutLinkMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
