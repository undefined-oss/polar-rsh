use polar_rsh::v2026_10::{
    Presence,
    models::{
        CustomerUpdate, ProductUpdate, SubscriptionPause, WebhookEndpointCreate,
        WebhookEndpointUpdate, WebhookFormat,
    },
};
use serde_json::{Value, json};

#[test]
fn model_fields_distinguish_absent_null_and_present_values() {
    for (wire, expected) in [
        (json!({}), Presence::Absent),
        (json!({"name": null}), Presence::Null),
        (json!({"name": ""}), Presence::Some(String::new())),
        (json!({"name": "Ada"}), Presence::Some("Ada".to_owned())),
    ] {
        let model: CustomerUpdate = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(model.name, expected);
        assert_eq!(serde_json::to_value(model).unwrap(), wire);
    }
    assert!(CustomerUpdate::default().name.is_absent());
    // Optional non-nullable fields still accept ordinary Option values.
    let model = CustomerUpdate { metadata: None, ..Default::default() };
    assert_eq!(serde_json::to_value(model).unwrap(), json!({}));
}

#[test]
fn presence_preserves_native_values_and_empty_collections() {
    let url = url::Url::parse("https://example.com/webhooks").unwrap();
    let model = WebhookEndpointUpdate {
        url: Presence::Some(url.clone()),
        enabled: Presence::Some(false),
        events: Presence::Some(vec![]),
        ..Default::default()
    };
    let wire = serde_json::to_value(&model).unwrap();
    assert_eq!(wire, json!({"url":url, "enabled":false, "events":[]}));
    let parsed: WebhookEndpointUpdate = serde_json::from_value(wire).unwrap();
    assert_eq!(parsed.url, Presence::Some(url));
    assert_eq!(parsed.enabled, Presence::Some(false));
    assert!(matches!(parsed.events, Presence::Some(events) if events.is_empty()));

    let timestamp = "2026-10-19T12:30:00Z".parse::<chrono::DateTime<chrono::Utc>>().unwrap();
    for resumes_at in [Presence::Absent, Presence::Null, Presence::Some(timestamp)] {
        let model = SubscriptionPause { pause_at_period_end: true, resumes_at };
        let wire = serde_json::to_value(&model).unwrap();
        let parsed: SubscriptionPause = serde_json::from_value(wire).unwrap();
        assert_eq!(parsed.resumes_at, resumes_at);
    }

    for medias in [Presence::Absent, Presence::Null, Presence::Some(vec![])] {
        let model = ProductUpdate { medias: medias.clone(), ..Default::default() };
        let wire = serde_json::to_value(&model).unwrap();
        match &medias {
            Presence::Absent => assert!(wire.get("medias").is_none()),
            Presence::Null => assert_eq!(wire["medias"], Value::Null),
            Presence::Some(_) => assert_eq!(wire["medias"], json!([])),
        }
        let parsed: ProductUpdate = serde_json::from_value(wire).unwrap();
        assert_eq!(parsed.medias, medias);
    }
}

#[test]
fn constructors_and_builders_use_presence_states() {
    let url = url::Url::parse("https://example.com/webhooks").unwrap();
    let model = WebhookEndpointCreate::new(vec![], WebhookFormat::Raw, url.clone());
    assert_eq!(model.name, Presence::Absent);
    assert_eq!(model.organization_id, Presence::Absent);
    assert!(serde_json::to_value(model).unwrap().get("name").is_none());

    let id = uuid::Uuid::nil();
    let builder = WebhookEndpointCreate::builder(vec![], WebhookFormat::Raw, url)
        .name("Billing".to_owned())
        .organization_id(id);
    let model = builder.clone().build();
    assert_eq!(model.name, Presence::Some("Billing".to_owned()));
    assert_eq!(model.organization_id, Presence::Some(id));
    let wire = serde_json::to_value(model).unwrap();
    let parsed: WebhookEndpointCreate = serde_json::from_value(wire).unwrap();
    assert_eq!(parsed.organization_id, Presence::Some(id));

    let null = builder.clone().name_null().organization_id_null().build();
    assert_eq!(null.name, Presence::Null);
    assert_eq!(null.organization_id, Presence::Null);
    assert_eq!(serde_json::to_value(null).unwrap()["name"], Value::Null);

    let absent = builder.name_null().name_absent().organization_id_absent().build();
    assert_eq!(absent.name, Presence::Absent);
    assert_eq!(absent.organization_id, Presence::Absent);
    assert!(serde_json::to_value(absent).unwrap().get("name").is_none());
}
