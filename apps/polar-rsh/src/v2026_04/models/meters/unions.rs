/// The aggregation to apply on the filtered events to calculate the meter.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MeterCreateAggregationUnion {
    Count(crate::v2026_04::models::common::types::CountAggregation),
    Avg(crate::v2026_04::models::common::types::PropertyAggregation),
    Unique(crate::v2026_04::models::common::types::UniqueAggregation),
}
impl<'de> serde::Deserialize<'de> for MeterCreateAggregationUnion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let tag = value
            .get("func")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                serde::de::Error::custom(concat!(
                    "missing or non-string discriminator ",
                    "func",
                    " for ",
                    "MeterCreateAggregationUnion"
                ))
            })?
            .to_owned();
        match tag.as_str() {
            "count" => serde_json::from_value::<
                crate::v2026_04::models::common::types::CountAggregation,
            >(value)
            .map(Self::Count)
            .map_err(serde::de::Error::custom),
            "avg" | "max" | "min" | "sum" => serde_json::from_value::<
                crate::v2026_04::models::common::types::PropertyAggregation,
            >(value)
            .map(Self::Avg)
            .map_err(serde::de::Error::custom),
            "unique" => serde_json::from_value::<
                crate::v2026_04::models::common::types::UniqueAggregation,
            >(value)
            .map(Self::Unique)
            .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "unknown {} discriminator {:?} for {}",
                "func", tag, "MeterCreateAggregationUnion"
            ))),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MeterCreateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_04::models::meters::newtypes::MeterMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MeterUpdateAggregationAnyOf0Union {
    Count(crate::v2026_04::models::common::types::CountAggregation),
    Avg(crate::v2026_04::models::common::types::PropertyAggregation),
    Unique(crate::v2026_04::models::common::types::UniqueAggregation),
}
impl<'de> serde::Deserialize<'de> for MeterUpdateAggregationAnyOf0Union {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let tag = value
            .get("func")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                serde::de::Error::custom(concat!(
                    "missing or non-string discriminator ",
                    "func",
                    " for ",
                    "MeterUpdateAggregationAnyOf0Union"
                ))
            })?
            .to_owned();
        match tag.as_str() {
            "count" => serde_json::from_value::<
                crate::v2026_04::models::common::types::CountAggregation,
            >(value)
            .map(Self::Count)
            .map_err(serde::de::Error::custom),
            "avg" | "max" | "min" | "sum" => serde_json::from_value::<
                crate::v2026_04::models::common::types::PropertyAggregation,
            >(value)
            .map(Self::Avg)
            .map_err(serde::de::Error::custom),
            "unique" => serde_json::from_value::<
                crate::v2026_04::models::common::types::UniqueAggregation,
            >(value)
            .map(Self::Unique)
            .map_err(serde::de::Error::custom),
            _ => Err(serde::de::Error::custom(format!(
                "unknown {} discriminator {:?} for {}",
                "func", tag, "MeterUpdateAggregationAnyOf0Union"
            ))),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MeterUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2026_04::models::meters::newtypes::MeterMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
