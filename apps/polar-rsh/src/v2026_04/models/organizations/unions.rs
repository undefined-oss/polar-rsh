#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CustomerProductPricesItemsUnion {
    LegacyRecurringProductPrice(
        crate::v2026_04::models::common::unions::LegacyRecurringProductPrice,
    ),
    ProductPrice(crate::v2026_04::models::common::unions::ProductPrice),
}
impl<'de> serde::Deserialize<'de> for CustomerProductPricesItemsUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_04::models::common::unions::LegacyRecurringProductPrice,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CustomerProductPricesItemsUnion"
                )));
            }
            matched = Some(Self::LegacyRecurringProductPrice(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2026_04::models::common::unions::ProductPrice,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "CustomerProductPricesItemsUnion"
                )));
            }
            matched = Some(Self::ProductPrice(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!(
                "no matching oneOf variant for ",
                "CustomerProductPricesItemsUnion"
            ))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum OrganizationCreateLegalEntityAnyOf0Union {
    #[serde(rename = "individual")]
    Individual {},
    #[serde(rename = "company")]
    Company { registered_name: String },
}
