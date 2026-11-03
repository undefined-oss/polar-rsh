#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CustomerOrderProductPricesItemsUnion {
    LegacyRecurringProductPrice(
        crate::v2026_10::models::common::unions::LegacyRecurringProductPrice,
    ),
    ProductPrice(crate::v2026_10::models::common::unions::ProductPrice),
}
impl<'de> serde::Deserialize<'de> for CustomerOrderProductPricesItemsUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_10::models::common::unions::LegacyRecurringProductPrice,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CustomerOrderProductPricesItemsUnion"
                )));
            }
            matched = Some(Self::LegacyRecurringProductPrice(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_10::models::common::unions::ProductPrice,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CustomerOrderProductPricesItemsUnion"
                )));
            }
            matched = Some(Self::ProductPrice(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "CustomerOrderProductPricesItemsUnion"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_10::models::orders::newtypes::OrderMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum OrderDiscountAnyOf0Union {
    DiscountFixedOnceForeverDurationBase(
        crate::v2026_10::models::common::types::DiscountFixedOnceForeverDurationBase,
    ),
    DiscountFixedRepeatDurationBase(
        crate::v2026_10::models::common::types::DiscountFixedRepeatDurationBase,
    ),
    DiscountPercentageOnceForeverDurationBase(
        crate::v2026_10::models::common::types::DiscountPercentageOnceForeverDurationBase,
    ),
    DiscountPercentageRepeatDurationBase(
        crate::v2026_10::models::common::types::DiscountPercentageRepeatDurationBase,
    ),
}
impl<'de> serde::Deserialize<'de> for OrderDiscountAnyOf0Union {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_10::models::common::types::DiscountFixedOnceForeverDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrderDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountFixedOnceForeverDurationBase(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_10::models::common::types::DiscountFixedRepeatDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrderDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountFixedRepeatDurationBase(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_10::models::common::types::DiscountPercentageOnceForeverDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrderDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountPercentageOnceForeverDurationBase(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_10::models::common::types::DiscountPercentageRepeatDurationBase,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrderDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountPercentageRepeatDurationBase(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "OrderDiscountAnyOf0Union"
            ))
        })
    }
}
