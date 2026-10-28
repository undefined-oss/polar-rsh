use std::collections::BTreeSet;

use http_body_util::BodyExt;
use serde_json::{Value, json};

use super::{
    OrganizationCreate, OrganizationCreateRequired, OrganizationDetails,
    OrganizationFeatureSettingsUpdate, OrganizationLegalEntity, OrganizationSocialLink,
    OrganizationSocialPlatforms, OrganizationSwitchingFrom,
};
use crate::{
    client::PolarClient,
    models::common::enums::{CountryAlpha2Input, PresentmentCurrency, TaxBehaviorOption},
};

fn payload() -> OrganizationCreate {
    OrganizationCreate::new(OrganizationCreateRequired {
        name: "Acme".into(),
        slug: "acme".to_owned().into(),
    })
}

fn spec() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../vendor/polar/docs/openapi/2026-10.openapi.json"
    )))
    .unwrap()
}

#[tokio::test]
async fn complete_payload_covers_every_create_property_and_encodes_the_expected_body() {
    let fixture = super::item_stream_tests::organization(1);
    let details = OrganizationDetails::new()
        .about("About Acme")
        .product_description("Software")
        .selling_categories(vec!["software".into()])
        .pricing_models(vec!["subscription".into()])
        .intended_use("Subscriptions")
        .customer_acquisition(vec!["website".into()])
        .future_annual_revenue(1000)
        .switching(false)
        .switching_from(OrganizationSwitchingFrom::LemonSqueezy)
        .previous_annual_revenue(0);
    let features = OrganizationFeatureSettingsUpdate::new()
        .member_model_enabled(false)
        .checkout_localization_enabled(true)
        .overview_metrics(vec!["revenue".into()]);
    let payload = payload()
        .avatar_url("https://example.com/avatar.png")
        .email("support@example.com")
        .website("https://example.com")
        .legal_entity(OrganizationLegalEntity::Company { registered_name: "Acme Ltd".into() })
        .socials(vec![OrganizationSocialLink {
            platform: OrganizationSocialPlatforms::Github,
            url: "https://github.com/acme".into(),
        }])
        .details(details)
        .country(CountryAlpha2Input::ES)
        .feature_settings(features)
        .subscription_settings(
            serde_json::from_value(fixture["subscription_settings"].clone()).unwrap(),
        )
        .customer_email_settings(
            serde_json::from_value(fixture["customer_email_settings"].clone()).unwrap(),
        )
        .customer_portal_settings(
            serde_json::from_value(fixture["customer_portal_settings"].clone()).unwrap(),
        )
        .default_presentment_currency(PresentmentCurrency::Eur)
        .default_tax_behavior(TaxBehaviorOption::Inclusive);
    let expected = json!({
        "name":"Acme", "slug":"acme",
        "avatar_url":"https://example.com/avatar.png",
        "email":"support@example.com", "website":"https://example.com",
        "legal_entity":{"type":"company", "registered_name":"Acme Ltd"},
        "socials":[{"platform":"github", "url":"https://github.com/acme"}],
        "details":{
            "about":"About Acme", "product_description":"Software",
            "selling_categories":["software"], "pricing_models":["subscription"],
            "intended_use":"Subscriptions", "customer_acquisition":["website"],
            "future_annual_revenue":1000, "switching":false,
            "switching_from":"lemon_squeezy", "previous_annual_revenue":0
        },
        "country":"ES",
        "feature_settings":{
            "member_model_enabled":false, "checkout_localization_enabled":true,
            "overview_metrics":["revenue"]
        },
        "subscription_settings":fixture["subscription_settings"],
        "customer_email_settings":fixture["customer_email_settings"],
        "customer_portal_settings":fixture["customer_portal_settings"],
        "default_presentment_currency":"eur", "default_tax_behavior":"inclusive"
    });
    let client = PolarClient::builder("example-token").build_with(()).unwrap();
    let request = client.organizations().create(payload).build_request().unwrap();
    let body = request.into_body().collect().await.unwrap().to_bytes();
    let encoded: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(encoded, expected);
    let schemas = spec();
    for (value, schema_name) in [
        (&encoded, "OrganizationCreate"),
        (&encoded["details"], "OrganizationDetails"),
        (&encoded["feature_settings"], "OrganizationFeatureSettingsUpdate"),
    ] {
        let actual: BTreeSet<_> = value.as_object().unwrap().keys().collect();
        let declared: BTreeSet<_> = schemas["components"]["schemas"][schema_name]["properties"]
            .as_object()
            .unwrap()
            .keys()
            .collect();
        assert_eq!(actual, declared, "missing or extra fields in {schema_name}");
    }
    let decoded: OrganizationCreate = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), encoded);
}

#[test]
fn defaults_remain_omitted_and_explicit_empty_objects_are_preserved() {
    assert_eq!(serde_json::to_value(payload()).unwrap(), json!({"name":"Acme", "slug":"acme"}));
    assert_eq!(serde_json::to_value(OrganizationDetails::new()).unwrap(), json!({}));
    assert_eq!(serde_json::to_value(OrganizationFeatureSettingsUpdate::new()).unwrap(), json!({}));
    let value = serde_json::to_value(
        payload()
            .details(OrganizationDetails::new())
            .feature_settings(OrganizationFeatureSettingsUpdate::new())
            .default_presentment_currency(PresentmentCurrency::Usd)
            .default_tax_behavior(TaxBehaviorOption::Location),
    )
    .unwrap();
    assert_eq!(
        value,
        json!({
            "name":"Acme", "slug":"acme", "details":{}, "feature_settings":{},
            "default_presentment_currency":"usd", "default_tax_behavior":"location"
        })
    );
}

#[test]
fn preserves_false_empty_lists_and_last_setter_values() {
    let value = serde_json::to_value(
        payload()
            .country(CountryAlpha2Input::US)
            .country(CountryAlpha2Input::ES)
            .default_presentment_currency(PresentmentCurrency::Usd)
            .default_presentment_currency(PresentmentCurrency::Eur)
            .default_tax_behavior(TaxBehaviorOption::Location)
            .default_tax_behavior(TaxBehaviorOption::Exclusive)
            .details(OrganizationDetails::new().about("old"))
            .details(
                OrganizationDetails::new()
                    .switching(true)
                    .switching(false)
                    .selling_categories(vec![])
                    .pricing_models(vec![])
                    .customer_acquisition(vec![]),
            )
            .feature_settings(OrganizationFeatureSettingsUpdate::new().member_model_enabled(true))
            .feature_settings(
                OrganizationFeatureSettingsUpdate::new()
                    .member_model_enabled(false)
                    .checkout_localization_enabled(false)
                    .overview_metrics(vec![]),
            ),
    )
    .unwrap();
    assert_eq!(
        value,
        json!({
            "name":"Acme", "slug":"acme", "country":"ES",
            "default_presentment_currency":"eur", "default_tax_behavior":"exclusive",
            "details":{"switching":false, "selling_categories":[], "pricing_models":[], "customer_acquisition":[]},
            "feature_settings":{"member_model_enabled":false, "checkout_localization_enabled":false, "overview_metrics":[]}
        })
    );
}

#[test]
fn enums_accept_all_schema_values_and_reject_invalid_inputs() {
    let spec = spec();
    for country in spec["components"]["schemas"]["OrganizationCreate"]["properties"]["country"]
        ["anyOf"][0]["enum"]
        .as_array()
        .unwrap()
    {
        let value: CountryAlpha2Input = serde_json::from_value(country.clone()).unwrap();
        assert_eq!(value.as_str(), country.as_str().unwrap());
        assert_eq!(serde_json::to_value(value).unwrap(), *country);
    }
    for country in ["CU", "IR", "KP", "RU", "SY"] {
        assert!(serde_json::from_value::<CountryAlpha2Input>(json!(country)).is_err());
        assert!(
            serde_json::from_value::<super::enums::OrganizationCountry>(json!(country)).is_ok()
        );
    }
    assert!(serde_json::from_value::<CountryAlpha2Input>(json!("ZZ")).is_err());
    for currency in spec["components"]["schemas"]["PresentmentCurrency"]["enum"].as_array().unwrap()
    {
        let value: PresentmentCurrency = serde_json::from_value(currency.clone()).unwrap();
        assert_eq!(value.as_str(), currency.as_str().unwrap());
        assert_eq!(serde_json::to_value(value).unwrap(), *currency);
    }
    for currency in ["USD", "unknown"] {
        assert!(serde_json::from_value::<PresentmentCurrency>(json!(currency)).is_err());
    }
    for platform in spec["components"]["schemas"]["OrganizationDetails"]["properties"]
        ["switching_from"]["anyOf"][0]["enum"]
        .as_array()
        .unwrap()
    {
        let value: OrganizationSwitchingFrom = serde_json::from_value(platform.clone()).unwrap();
        assert_eq!(value.as_str(), platform.as_str().unwrap());
        assert_eq!(serde_json::to_value(value).unwrap(), *platform);
    }
}

#[test]
fn rejects_null_non_nullable_fields_and_negative_revenue() {
    for (field, value) in [
        ("default_presentment_currency", Value::Null),
        ("default_tax_behavior", Value::Null),
        ("feature_settings", json!({"member_model_enabled":null})),
        ("feature_settings", json!({"checkout_localization_enabled":null})),
        ("details", json!({"selling_categories":null})),
        ("details", json!({"pricing_models":null})),
        ("details", json!({"customer_acquisition":null})),
        ("details", json!({"switching":null})),
        ("details", json!({"future_annual_revenue":-1})),
        ("details", json!({"previous_annual_revenue":-1})),
    ] {
        let mut body = json!({"name":"Acme", "slug":"acme"});
        body[field] = value;
        assert!(
            serde_json::from_value::<OrganizationCreate>(body.clone()).is_err(),
            "invalid input accepted: {body}"
        );
    }
}
