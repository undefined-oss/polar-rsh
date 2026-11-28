#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomerOrderPay {
    CustomerOrderPayPaymentMethod(
        crate::v2027_01::models::orders::types::CustomerOrderPayPaymentMethod,
    ),
    CustomerOrderPayConfirmationToken(
        crate::v2027_01::models::orders::types::CustomerOrderPayConfirmationToken,
    ),
    CustomerOrderPayWithoutCredentials(
        crate::v2027_01::models::orders::types::CustomerOrderPayWithoutCredentials,
    ),
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
pub enum OrderCreateExternalCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateExternalCustomerCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateExternalCustomerMetadataAdditionalPropertiesUnion {
    String(
        crate::v2027_01::models::orders::newtypes::OrderCreateExternalCustomerMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateExternalMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::orders::newtypes::OrderCreateExternalMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateExternalProductCustomFieldDataAdditionalPropertiesUnion {
    String(String),
    Integer(i64),
    Boolean(bool),
    String4(jiff::Timestamp),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateExternalProductMetadataAdditionalPropertiesUnion {
    String(
        crate::v2027_01::models::orders::newtypes::OrderCreateExternalProductMetadataValueString,
    ),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OrderCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::orders::newtypes::OrderMetadataValueString),
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
impl<'de> serde::Deserialize<'de> for OrderDiscountAnyOf0Union {
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
                    "OrderDiscountAnyOf0Union"
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
                    "OrderDiscountAnyOf0Union"
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
                    "OrderDiscountAnyOf0Union"
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
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum OrdersCreateRequestUnion {
    OrderCreate(crate::v2027_01::models::orders::types::OrderCreate),
    OrderCreateExternalCustomer(
        crate::v2027_01::models::orders::types::OrderCreateExternalCustomer,
    ),
    OrderCreateExternalProduct(crate::v2027_01::models::orders::types::OrderCreateExternalProduct),
    OrderCreateExternal(crate::v2027_01::models::orders::types::OrderCreateExternal),
}
impl<'de> serde::Deserialize<'de> for OrdersCreateRequestUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::orders::types::OrderCreate,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrdersCreateRequestUnion"
                )));
            }
            matched = Some(Self::OrderCreate(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::orders::types::OrderCreateExternalCustomer,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrdersCreateRequestUnion"
                )));
            }
            matched = Some(Self::OrderCreateExternalCustomer(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::orders::types::OrderCreateExternalProduct,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrdersCreateRequestUnion"
                )));
            }
            matched = Some(Self::OrderCreateExternalProduct(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::orders::types::OrderCreateExternal,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "OrdersCreateRequestUnion"
                )));
            }
            matched = Some(Self::OrderCreateExternal(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "OrdersCreateRequestUnion"
            ))
        })
    }
}
