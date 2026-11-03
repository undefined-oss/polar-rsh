#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum ProductCreate {
    ProductCreateRecurring(crate::v2027_01::models::products::types::ProductCreateRecurring),
    ProductCreateOneTime(crate::v2027_01::models::products::types::ProductCreateOneTime),
}
impl<'de> serde::Deserialize<'de> for ProductCreate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        let mut matched = None;
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::products::types::ProductCreateRecurring,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "ProductCreate"
                )));
            }
            matched = Some(Self::ProductCreateRecurring(payload));
        }
        if let Ok(payload) = serde_json::from_value::<
            crate::v2027_01::models::products::types::ProductCreateOneTime,
        >(value.clone())
        {
            if matched.is_some() {
                return Err(serde::de::Error::custom(concat!(
                    "ambiguous oneOf value for ",
                    "ProductCreate"
                )));
            }
            matched = Some(Self::ProductCreateOneTime(payload));
        }
        matched.ok_or_else(|| {
            serde::de::Error::custom(concat!("no matching oneOf variant for ", "ProductCreate"))
        })
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProductCreateOneTimeMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::products::newtypes::ProductCreateOneTimeMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "amount_type", deny_unknown_fields)]
pub enum ProductCreateOneTimePricesItemsUnion {
    #[serde(rename = "fixed")]
    Fixed {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The price in cents. Set to `0` for a free price.
        /// Minimum amounts per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        price_amount: crate::v2027_01::models::common::newtypes::ProductPriceFixedPriceAmount,
    },
    #[serde(rename = "custom")]
    Custom {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The minimum amount the customer can pay. If set to 0, the price is
        /// 'free or pay what you want' and $0 is accepted. If set to a value
        /// below the minimum price amount for the currency, it will be
        /// rejected. Defaults to the minimum price amount for the currency.
        /// Minimum per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        minimum_amount:
            Option<crate::v2027_01::models::common::newtypes::ProductPriceCustomMinimumAmount>,
        /// The maximum amount the customer can pay. Maximum per currency:
        /// - USD: 999,999.99
        /// - EUR: 999,999.99
        /// - GBP: 999,999.99
        /// - ARS: 1,400,000
        /// - CDF: 2,800,000
        /// - COP: 4,000,000
        /// - IDR: 16,000,000
        /// - KHR: 4,000,000
        /// - LAK: 21,000,000
        /// - MNT: 3,500,000
        /// - MWK: 1,750,000
        /// - NGN: 1,550,000
        /// - TZS: 2,500,000
        /// - UGX: 3,700,000
        /// - UZS: 12,500,000
        /// - Other currencies: 99,999,999 minor units
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        maximum_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceCustomMaximumAmount,
        >,
        /// The initial amount shown to the customer. If 0, the customer will
        /// see $0 as the default. If set to a value below the minimum price
        /// amount for the currency, it will be rejected.Minimum per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        preset_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceCustomPresetAmount,
        >,
    },
    #[serde(rename = "seat_based")]
    SeatBased {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Tiered pricing based on the purchased seat quantity.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// The minimum purchasable seat quantity (inclusive). Defaults to 1
        /// when not set.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        minimum_units: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceSeatBasedMinimumUnits,
        >,
    },
    #[serde(rename = "unit_based")]
    UnitBased {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Tiered pricing based on the purchased unit quantity.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// The minimum purchasable quantity (inclusive). Defaults to 1 when not
        /// set.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        minimum_units: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceUnitBasedMinimumUnits,
        >,
        /// Per-locale unit nouns shown at checkout and on invoices. `{"en":
        /// {"=1": "device", "other": "devices"}}`. Defaults to "unit"/"units"
        /// when unset.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        unit_label: presence_rs::Presence<
            std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
        >,
    },
    #[serde(rename = "metered_unit")]
    MeteredUnit {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// The price per unit in cents. Supports up to 12 decimal places.
        unit_amount:
            crate::v2027_01::models::common::unions::ProductPriceMeteredUnitCreateUnitAmountUnion,
        /// Optional maximum amount in cents that can be charged, regardless of
        /// the number of units consumed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        cap_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceMeteredUnitCapAmount,
        >,
    },
    #[serde(rename = "metered_tiers")]
    MeteredTiers {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// Tiered pricing based on consumed units.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// Optional maximum amount in cents that can be charged, regardless of
        /// the number of units consumed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        cap_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceMeteredTiersCapAmount,
        >,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProductCreateRecurringMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::products::newtypes::ProductCreateRecurringMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "amount_type", deny_unknown_fields)]
pub enum ProductCreateRecurringPricesItemsUnion {
    #[serde(rename = "fixed")]
    Fixed {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The price in cents. Set to `0` for a free price.
        /// Minimum amounts per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        price_amount: crate::v2027_01::models::common::newtypes::ProductPriceFixedPriceAmount,
    },
    #[serde(rename = "custom")]
    Custom {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The minimum amount the customer can pay. If set to 0, the price is
        /// 'free or pay what you want' and $0 is accepted. If set to a value
        /// below the minimum price amount for the currency, it will be
        /// rejected. Defaults to the minimum price amount for the currency.
        /// Minimum per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        minimum_amount:
            Option<crate::v2027_01::models::common::newtypes::ProductPriceCustomMinimumAmount>,
        /// The maximum amount the customer can pay. Maximum per currency:
        /// - USD: 999,999.99
        /// - EUR: 999,999.99
        /// - GBP: 999,999.99
        /// - ARS: 1,400,000
        /// - CDF: 2,800,000
        /// - COP: 4,000,000
        /// - IDR: 16,000,000
        /// - KHR: 4,000,000
        /// - LAK: 21,000,000
        /// - MNT: 3,500,000
        /// - MWK: 1,750,000
        /// - NGN: 1,550,000
        /// - TZS: 2,500,000
        /// - UGX: 3,700,000
        /// - UZS: 12,500,000
        /// - Other currencies: 99,999,999 minor units
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        maximum_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceCustomMaximumAmount,
        >,
        /// The initial amount shown to the customer. If 0, the customer will
        /// see $0 as the default. If set to a value below the minimum price
        /// amount for the currency, it will be rejected.Minimum per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        preset_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceCustomPresetAmount,
        >,
    },
    #[serde(rename = "seat_based")]
    SeatBased {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Tiered pricing based on the purchased seat quantity.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// The minimum purchasable seat quantity (inclusive). Defaults to 1
        /// when not set.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        minimum_units: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceSeatBasedMinimumUnits,
        >,
    },
    #[serde(rename = "unit_based")]
    UnitBased {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Tiered pricing based on the purchased unit quantity.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// The minimum purchasable quantity (inclusive). Defaults to 1 when not
        /// set.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        minimum_units: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceUnitBasedMinimumUnits,
        >,
        /// Per-locale unit nouns shown at checkout and on invoices. `{"en":
        /// {"=1": "device", "other": "devices"}}`. Defaults to "unit"/"units"
        /// when unset.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        unit_label: presence_rs::Presence<
            std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
        >,
    },
    #[serde(rename = "metered_unit")]
    MeteredUnit {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// The price per unit in cents. Supports up to 12 decimal places.
        unit_amount:
            crate::v2027_01::models::common::unions::ProductPriceMeteredUnitCreateUnitAmountUnion,
        /// Optional maximum amount in cents that can be charged, regardless of
        /// the number of units consumed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        cap_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceMeteredUnitCapAmount,
        >,
    },
    #[serde(rename = "metered_tiers")]
    MeteredTiers {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// Tiered pricing based on consumed units.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// Optional maximum amount in cents that can be charged, regardless of
        /// the number of units consumed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        cap_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceMeteredTiersCapAmount,
        >,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProductUpdateMetadataAdditionalPropertiesUnion {
    String(crate::v2027_01::models::products::newtypes::ProductMetadataValueString),
    Integer(i64),
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "amount_type", deny_unknown_fields)]
pub enum ProductUpdatePricesAnyOf0ItemsAnyOf1Union {
    #[serde(rename = "fixed")]
    Fixed {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The price in cents. Set to `0` for a free price.
        /// Minimum amounts per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        price_amount: crate::v2027_01::models::common::newtypes::ProductPriceFixedPriceAmount,
    },
    #[serde(rename = "custom")]
    Custom {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The minimum amount the customer can pay. If set to 0, the price is
        /// 'free or pay what you want' and $0 is accepted. If set to a value
        /// below the minimum price amount for the currency, it will be
        /// rejected. Defaults to the minimum price amount for the currency.
        /// Minimum per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        minimum_amount:
            Option<crate::v2027_01::models::common::newtypes::ProductPriceCustomMinimumAmount>,
        /// The maximum amount the customer can pay. Maximum per currency:
        /// - USD: 999,999.99
        /// - EUR: 999,999.99
        /// - GBP: 999,999.99
        /// - ARS: 1,400,000
        /// - CDF: 2,800,000
        /// - COP: 4,000,000
        /// - IDR: 16,000,000
        /// - KHR: 4,000,000
        /// - LAK: 21,000,000
        /// - MNT: 3,500,000
        /// - MWK: 1,750,000
        /// - NGN: 1,550,000
        /// - TZS: 2,500,000
        /// - UGX: 3,700,000
        /// - UZS: 12,500,000
        /// - Other currencies: 99,999,999 minor units
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        maximum_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceCustomMaximumAmount,
        >,
        /// The initial amount shown to the customer. If 0, the customer will
        /// see $0 as the default. If set to a value below the minimum price
        /// amount for the currency, it will be rejected.Minimum per currency:
        /// - USD: 0.5
        /// - AED: 2
        /// - ALL: 50
        /// - AMD: 200
        /// - AOA: 500
        /// - ARS: 750
        /// - AUD: 0.7
        /// - AWG: 1
        /// - AZN: 1
        /// - BAM: 1
        /// - BBD: 2
        /// - BDT: 70
        /// - BIF: 2,000
        /// - BMD: 1
        /// - BND: 1
        /// - BOB: 5
        /// - BRL: 2.5
        /// - BSD: 1
        /// - BWP: 10
        /// - BZD: 2
        /// - CAD: 0.7
        /// - CDF: 2,000
        /// - CHF: 0.5
        /// - CLP: 500
        /// - CNY: 5
        /// - COP: 2,000
        /// - CRC: 300
        /// - CVE: 50
        /// - CZK: 15
        /// - DJF: 100
        /// - DKK: 3.2
        /// - DOP: 40
        /// - DZD: 70
        /// - EGP: 30
        /// - ETB: 80
        /// - EUR: 0.5
        /// - FJD: 2
        /// - FKP: 1
        /// - GBP: 0.4
        /// - GEL: 2
        /// - GNF: 5,000
        /// - GIP: 1
        /// - GMD: 40
        /// - GTQ: 5
        /// - GYD: 200
        /// - HKD: 4
        /// - HNL: 20
        /// - HTG: 70
        /// - HUF: 175
        /// - IDR: 9,000
        /// - ILS: 1.5
        /// - INR: 60
        /// - ISK: 70
        /// - JMD: 80
        /// - JPY: 80
        /// - KES: 70
        /// - KGS: 50
        /// - KHR: 3,000
        /// - KMF: 500
        /// - KRW: 800
        /// - KYD: 1
        /// - KZT: 300
        /// - LAK: 20,000
        /// - LKR: 200
        /// - LRD: 100
        /// - LSL: 10
        /// - MAD: 5
        /// - MDL: 10
        /// - MGA: 3,000
        /// - MKD: 50
        /// - MNT: 2,000
        /// - MOP: 5
        /// - MUR: 50
        /// - MVR: 8
        /// - MXN: 9
        /// - MWK: 1,000
        /// - MYR: 2
        /// - MZN: 50
        /// - NAD: 10
        /// - NGN: 700
        /// - NIO: 20
        /// - NOK: 5
        /// - NPR: 80
        /// - NZD: 0.9
        /// - PAB: 1
        /// - PEN: 2
        /// - PGK: 3
        /// - PHP: 35
        /// - PKR: 200
        /// - PLN: 2
        /// - PYG: 4,000
        /// - QAR: 2
        /// - RON: 2.5
        /// - RSD: 60
        /// - RWF: 1,000
        /// - SAR: 2
        /// - SBD: 4
        /// - SCR: 8
        /// - SEK: 5
        /// - SGD: 0.7
        /// - SHP: 1
        /// - SOS: 500
        /// - SRD: 20
        /// - SZL: 10
        /// - THB: 20
        /// - TJS: 5
        /// - TOP: 2
        /// - TRY: 30
        /// - TTD: 4
        /// - TWD: 20
        /// - TZS: 2,000
        /// - UAH: 30
        /// - UGX: 2,000
        /// - UYU: 20
        /// - UZS: 7,000
        /// - VND: 20,000
        /// - VUV: 100
        /// - WST: 2
        /// - XAF: 500
        /// - XCD: 2
        /// - XCG: 1
        /// - XOF: 500
        /// - XPF: 100
        /// - YER: 200
        /// - ZAR: 9
        /// - ZMW: 10
        /// - Other currencies: 50 minor units
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        preset_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceCustomPresetAmount,
        >,
    },
    #[serde(rename = "seat_based")]
    SeatBased {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Tiered pricing based on the purchased seat quantity.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// The minimum purchasable seat quantity (inclusive). Defaults to 1
        /// when not set.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        minimum_units: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceSeatBasedMinimumUnits,
        >,
    },
    #[serde(rename = "unit_based")]
    UnitBased {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// Tiered pricing based on the purchased unit quantity.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// The minimum purchasable quantity (inclusive). Defaults to 1 when not
        /// set.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        minimum_units: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceUnitBasedMinimumUnits,
        >,
        /// Per-locale unit nouns shown at checkout and on invoices. `{"en":
        /// {"=1": "device", "other": "devices"}}`. Defaults to "unit"/"units"
        /// when unset.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        unit_label: presence_rs::Presence<
            std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
        >,
    },
    #[serde(rename = "metered_unit")]
    MeteredUnit {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// The price per unit in cents. Supports up to 12 decimal places.
        unit_amount:
            crate::v2027_01::models::common::unions::ProductPriceMeteredUnitCreateUnitAmountUnion,
        /// Optional maximum amount in cents that can be charged, regardless of
        /// the number of units consumed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        cap_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceMeteredUnitCapAmount,
        >,
    },
    #[serde(rename = "metered_tiers")]
    MeteredTiers {
        /// The currency in which the customer will be charged.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "crate::v2027_01::models::common::serde_helpers::deserialize_optional_value"
        )]
        price_currency: Option<crate::v2027_01::models::common::enums::PresentmentCurrency>,
        /// The tax behavior of the price. If not set, it will default to the
        /// organization's default tax behavior.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        tax_behavior:
            presence_rs::Presence<crate::v2027_01::models::common::enums::TaxBehaviorOption>,
        /// The ID of the meter associated to the price.
        meter_id: crate::v2027_01::models::common::ids::MeterId,
        /// Tiered pricing based on consumed units.
        tiers: crate::v2027_01::models::common::types::TiersInput,
        /// Optional maximum amount in cents that can be charged, regardless of
        /// the number of units consumed.
        #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
        cap_amount: presence_rs::Presence<
            crate::v2027_01::models::common::newtypes::ProductPriceMeteredTiersCapAmount,
        >,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProductUpdatePricesAnyOf0ItemsUnion {
    ExistingProductPrice(crate::v2027_01::models::products::types::ExistingProductPrice),
    Variant(crate::v2027_01::models::products::unions::ProductUpdatePricesAnyOf0ItemsAnyOf1Union),
}
