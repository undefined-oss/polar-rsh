#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerSubscriptionUpdate {
    CustomerSubscriptionUpdateProduct(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionUpdateProduct,
    ),
    CustomerSubscriptionUpdateSeats(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionUpdateSeats,
    ),
    CustomerSubscriptionUpdateUnits(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionUpdateUnits,
    ),
    CustomerSubscriptionCancel(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionCancel,
    ),
    CustomerSubscriptionPause(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionPause,
    ),
    CustomerSubscriptionResume(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionResume,
    ),
    CustomerSubscriptionUpdateClear(
        crate::v2026_10::models::subscriptions::types::CustomerSubscriptionUpdateClear,
    ),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionCreateCustomerMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::subscriptions::newtypes::SubscriptionCreateCustomerMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionCreateExternalCustomerMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::subscriptions::newtypes::SubscriptionCreateExternalCustomerMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SubscriptionDiscountAnyOf0Union {
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
impl<'de> serde::Deserialize<'de> for SubscriptionDiscountAnyOf0Union {
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
                    "SubscriptionDiscountAnyOf0Union"
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
                    "SubscriptionDiscountAnyOf0Union"
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
                    "SubscriptionDiscountAnyOf0Union"
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
                    "SubscriptionDiscountAnyOf0Union"
                )));
            }
            matched = Some(Self::DiscountPercentageRepeatDurationBase(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "SubscriptionDiscountAnyOf0Union"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SubscriptionPricesItemsUnion {
    LegacyRecurringProductPrice(
        crate::v2026_10::models::common::unions::LegacyRecurringProductPrice,
    ),
    ProductPrice(crate::v2026_10::models::common::unions::ProductPrice),
}
impl<'de> serde::Deserialize<'de> for SubscriptionPricesItemsUnion {
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
                    "SubscriptionPricesItemsUnion"
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
                    "SubscriptionPricesItemsUnion"
                )));
            }
            matched = Some(Self::ProductPrice(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "SubscriptionPricesItemsUnion"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionUpdate {
    SubscriptionUpdateBase(crate::v2026_10::models::subscriptions::types::SubscriptionUpdateBase),
    SubscriptionUpdateSeats(crate::v2026_10::models::subscriptions::types::SubscriptionUpdateSeats),
    SubscriptionUpdateUnits(crate::v2026_10::models::subscriptions::types::SubscriptionUpdateUnits),
    SubscriptionUpdateBillingPeriod(
        crate::v2026_10::models::subscriptions::types::SubscriptionUpdateBillingPeriod,
    ),
    SubscriptionCancel(crate::v2026_10::models::subscriptions::types::SubscriptionCancel),
    SubscriptionRevoke(crate::v2026_10::models::subscriptions::types::SubscriptionRevoke),
    SubscriptionPause(crate::v2026_10::models::subscriptions::types::SubscriptionPause),
    SubscriptionResume(crate::v2026_10::models::subscriptions::types::SubscriptionResume),
    SubscriptionUpdateClear(crate::v2026_10::models::subscriptions::types::SubscriptionUpdateClear),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionUpdateBaseMetadataAdditionalPropertiesUnion {
    String(
        crate::v2026_10::models::subscriptions::newtypes::SubscriptionUpdateBaseMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
/// Set or extend the trial period of the subscription. If set to `now`, the
/// trial will end immediately and the first billing cycle will be charged
/// synchronously. The subscription remains trialing if the payment fails.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionUpdateBaseTrialEndUnion {
    String(jiff::Timestamp),
    Now(monostate::MustBe!("now")),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubscriptionsCreateRequestUnion {
    SubscriptionCreateCustomer(
        crate::v2026_10::models::subscriptions::types::SubscriptionCreateCustomer,
    ),
    SubscriptionCreateExternalCustomer(
        crate::v2026_10::models::subscriptions::types::SubscriptionCreateExternalCustomer,
    ),
}
