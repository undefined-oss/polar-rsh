use std::{convert::Infallible, time::Duration};

use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use url::Url;

use super::{
    CreateOrganization, OrganizationCreate, OrganizationCreateRequired, OrganizationSocialLink,
    OrganizationSocialPlatforms,
};
use crate::{
    client::{AccessToken, PolarClient, PolarEnvironment, RequestBody, RequestOptions},
    error::BuildRequestError,
    models::{
        common::{OrganizationName, OrganizationSlug},
        organization::CreateOrganizationError,
    },
};

fn payload() -> OrganizationCreate {
    OrganizationCreate::new(OrganizationCreateRequired {
        name: OrganizationName::try_from("Acme").expect("valid test name"),
        slug: OrganizationSlug::try_from("acme").expect("valid test slug"),
    })
}

fn create<S>(client: &PolarClient<S>, payload: OrganizationCreate) -> CreateOrganization<'_, S> {
    client.organizations().create(payload)
}

fn mock_transport(
    status: StatusCode,
    body: Bytes,
    expected_payload: Value,
) -> impl tower::Service<Request<RequestBody>, Response = Response<RequestBody>, Error = Infallible>
+ Clone {
    tower::service_fn(move |request: Request<RequestBody>| {
        let body = body.clone();
        let expected_payload = expected_payload.clone();
        async move {
            assert_eq!(request.method(), Method::POST);
            assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/");
            assert_eq!(request.headers()["Content-Type"], "application/json");
            assert_eq!(request.headers()["Accept"], "application/json");
            assert_eq!(request.headers()["Polar-Version"], "2026-10");
            assert_eq!(request.headers()["Authorization"], "Bearer example-token");
            assert!(request.headers()["Authorization"].is_sensitive());
            let request_body = request.into_body().collect().await.unwrap().to_bytes();
            assert_eq!(serde_json::from_slice::<Value>(&request_body).unwrap(), expected_payload);

            Ok::<_, Infallible>(
                Response::builder().status(status).body(RequestBody::new(body)).unwrap(),
            )
        }
    })
}

#[tokio::test]
async fn builds_minimal_and_fluent_json_payloads() {
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(())
            .unwrap();
    for (payload, expected) in [
        (payload(), json!({"name":"Acme","slug":"acme"})),
        (
            payload().legal_entity(super::OrganizationLegalEntity::Individual),
            json!({"name":"Acme", "slug":"acme", "legal_entity":{"type":"individual"}}),
        ),
        (
            payload().legal_entity(super::OrganizationLegalEntity::Company {
                registered_name: "Acme Ltd".into(),
            }),
            json!({
                "name":"Acme", "slug":"acme",
                "legal_entity":{"type":"company", "registered_name":"Acme Ltd"}
            }),
        ),
        (
            payload().socials(vec![OrganizationSocialLink {
                platform: OrganizationSocialPlatforms::Github,
                url: Url::parse("https://github.com/acme").expect("valid test URL"),
            }]),
            json!({
                "name":"Acme", "slug":"acme",
                "socials":[{"platform":"github", "url":"https://github.com/acme"}]
            }),
        ),
        (
            payload()
                .email("support@example.com")
                .website(Url::parse("https://example.com/").expect("valid test URL"))
                .avatar_url(Url::parse("https://example.com/avatar.png").expect("valid test URL")),
            json!({
                "name":"Acme", "slug":"acme", "email":"support@example.com",
                "website":"https://example.com/", "avatar_url":"https://example.com/avatar.png"
            }),
        ),
    ] {
        let request = create(&client, payload).build_request().unwrap();
        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/");
        assert_eq!(request.headers()["Content-Type"], "application/json");
        assert_eq!(request.headers()["Accept"], "application/json");
        assert_eq!(request.headers()["Polar-Version"], "2026-10");
        let body = request.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), expected);
    }
}

#[test]
fn token_override_is_sensitive_and_does_not_change_the_client_default() {
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(())
            .unwrap();
    let request = create(&client, payload())
        .options(RequestOptions {
            access_token: Some(
                AccessToken::try_from("override-token").expect("nonempty test token"),
            ),
            ..Default::default()
        })
        .build_request()
        .unwrap();
    assert_eq!(request.headers()["Authorization"], "Bearer override-token");
    assert!(request.headers()["Authorization"].is_sensitive());
    let ordinary = create(&client, payload()).build_request().unwrap();
    assert_eq!(ordinary.headers()["Authorization"], "Bearer example-token");
    assert!(ordinary.headers()["Authorization"].is_sensitive());
}

#[tokio::test]
async fn nested_settings_are_serialized_and_repeated_setters_replace_them() {
    use super::{
        CustomerPortalCustomerSettings, CustomerPortalSubscriptionSettings,
        CustomerPortalUsageSettings, OrganizationCustomerEmailSettings,
        OrganizationCustomerPortalSettings, OrganizationSubscriptionSettings,
        OrganizationSubscriptionSettingsProrationBehavior,
    };

    let subscriptions = OrganizationSubscriptionSettings {
        allow_customer_updates: true,
        allow_multiple_subscriptions: false,
        benefit_revocation_grace_period: 7,
        prevent_trial_abuse: true,
        proration_behavior: OrganizationSubscriptionSettingsProrationBehavior::Prorate,
    };
    let emails = OrganizationCustomerEmailSettings {
        order_confirmation: true,
        payment_method_expiration_reminder: false,
        subscription_cancellation: false,
        subscription_confirmation: false,
        subscription_cycled: false,
        subscription_cycled_after_trial: false,
        subscription_past_due: false,
        subscription_paused: false,
        subscription_renewal_reminder: false,
        subscription_resumed: false,
        subscription_revoked: false,
        subscription_trial_conversion_reminder: false,
        subscription_uncanceled: false,
        subscription_updated: false,
    };
    let portal = OrganizationCustomerPortalSettings {
        customer: Some(CustomerPortalCustomerSettings { allow_email_change: Some(true) }),
        subscription: CustomerPortalSubscriptionSettings {
            pause: None,
            update_plan: true,
            update_seats: false,
            update_units: None,
        },
        usage: CustomerPortalUsageSettings { show: true },
    };
    let mut previous_subscriptions = subscriptions.clone();
    previous_subscriptions.benefit_revocation_grace_period = 30;
    let mut previous_emails = emails.clone();
    previous_emails.order_confirmation = false;
    let mut previous_portal = portal.clone();
    previous_portal.usage.show = false;

    let payload = payload()
        .subscription_settings(previous_subscriptions)
        .customer_email_settings(previous_emails)
        .customer_portal_settings(previous_portal)
        .subscription_settings(subscriptions)
        .customer_email_settings(emails)
        .customer_portal_settings(portal);
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(())
            .unwrap();
    let request = create(&client, payload).build_request().unwrap();
    let body = request.into_body().collect().await.unwrap().to_bytes();
    let encoded: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        encoded["subscription_settings"],
        json!({
            "allow_customer_updates": true,
            "allow_multiple_subscriptions": false,
            "benefit_revocation_grace_period": 7,
            "prevent_trial_abuse": true,
            "proration_behavior": "prorate"
        })
    );
    let email_settings = encoded["customer_email_settings"].as_object().unwrap();
    assert_eq!(email_settings.len(), 14);
    assert_eq!(email_settings["order_confirmation"], true);
    for (field, value) in email_settings {
        if field != "order_confirmation" {
            assert_eq!(value, false, "unexpected email setting: {field}");
        }
    }
    assert_eq!(
        encoded["customer_portal_settings"],
        json!({
            "customer": {"allow_email_change": true},
            "subscription": {"update_plan": true, "update_seats": false},
            "usage": {"show": true}
        })
    );

    let decoded: OrganizationCreate = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), encoded);
}

#[tokio::test]
async fn send_decodes_one_organization_from_201() {
    let expected = super::item_stream_tests::organization(1);
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(mock_transport(
                StatusCode::CREATED,
                Bytes::from(serde_json::to_vec(&expected).unwrap()),
                json!({"name":"Acme","slug":"acme"}),
            ))
            .unwrap();

    let organization =
        create(&client, payload()).send().await.expect("201 must return one Organization");
    let value = serde_json::to_value(organization).unwrap();
    assert_eq!(value["id"], expected["id"]);
    assert_eq!(value["name"], expected["name"]);
    assert_eq!(value["slug"], expected["slug"]);
    assert!(value.get("items").is_none(), "create must not return a list wrapper");
}

#[tokio::test]
async fn direct_await_decodes_one_organization_from_201() {
    let expected = super::item_stream_tests::organization(2);
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(mock_transport(
                StatusCode::CREATED,
                Bytes::from(serde_json::to_vec(&expected).unwrap()),
                json!({"name":"Acme","slug":"acme","email":"support@example.com"}),
            ))
            .unwrap();

    let organization = create(&client, payload().email("support@example.com"))
        .await
        .expect("direct await must return one Organization for 201");
    let value = serde_json::to_value(organization).unwrap();
    assert_eq!(value["id"], expected["id"]);
    assert_eq!(value["name"], expected["name"]);
    assert!(value.get("items").is_none());
}

#[tokio::test]
async fn invalid_created_bodies_return_decode_errors() {
    for body in [b"not-json".as_slice(), b"{}".as_slice()] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    StatusCode::CREATED,
                    Bytes::from_static(body),
                    json!({"name":"Acme","slug":"acme"}),
                ))
                .unwrap();
        let result = create(&client, payload()).await;
        assert!(
            matches!(&result, Err(CreateOrganizationError::DecodeResponse(_))),
            "expected DecodeResponse, got {result:?}"
        );
    }
}

#[tokio::test]
async fn forbidden_response_is_a_typed_create_error_with_a_source() {
    let client = PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
        .environment(PolarEnvironment::Sandbox)
        .build_with(mock_transport(
            StatusCode::FORBIDDEN,
            Bytes::from_static(br#"{"error":"CannotCreateOrganizationError","detail":"Organization creation is unavailable."}"#),
            json!({"name":"Acme","slug":"acme"}),
        ))
        .unwrap();
    let error = create(&client, payload()).await.unwrap_err();
    match &error {
        CreateOrganizationError::CannotCreateOrganization(error) => {
            assert!(matches!(
                error.error,
                super::CannotCreateOrganizationErrorKind::CannotCreateOrganizationError
            ));
            assert_eq!(error.detail, "Organization creation is unavailable.");
        }
        other => panic!("expected CannotCreateOrganization, got {other:?}"),
    }
    assert_eq!(
        std::error::Error::source(&error).unwrap().to_string(),
        "Organization creation is unavailable."
    );
}

#[tokio::test]
async fn invalid_forbidden_bodies_return_decode_errors() {
    for body in [
        b"not-json".as_slice(),
        b"{}".as_slice(),
        br#"{"error":"OtherError","detail":"Unavailable"}"#.as_slice(),
        br#"{"error":"CannotCreateOrganizationError"}"#.as_slice(),
        br#"{"detail":"Unavailable"}"#.as_slice(),
        br#"{"error":"CannotCreateOrganizationError","detail":null}"#.as_slice(),
    ] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    StatusCode::FORBIDDEN,
                    Bytes::from_static(body),
                    json!({"name":"Acme","slug":"acme"}),
                ))
                .unwrap();
        let result = create(&client, payload()).await;
        assert!(
            matches!(&result, Err(CreateOrganizationError::DecodeResponse(_))),
            "expected DecodeResponse for {body:?}, got {result:?}"
        );
    }
}

#[tokio::test]
async fn validation_errors_are_typed() {
    let client = PolarClient::builder(
        AccessToken::try_from("example-token").expect("valid test token"),
    )
    .environment(PolarEnvironment::Sandbox)
    .build_with(mock_transport(
        StatusCode::UNPROCESSABLE_ENTITY,
        Bytes::from_static(
            br#"{"detail":[{"loc":["body","slug"],"msg":"Invalid slug","type":"value_error"}]}"#,
        ),
        json!({"name":"Acme","slug":"acme"}),
    ))
    .unwrap();
    let result = create(&client, payload()).await;
    match result {
        Err(CreateOrganizationError::Validation(error)) => {
            let details = error.detail.unwrap();
            assert_eq!(details.len(), 1);
            assert_eq!(details[0].msg, "Invalid slug");
        }
        other => panic!("expected Validation, got {other:?}"),
    }
}

#[tokio::test]
async fn unexpected_statuses_preserve_the_response_including_200() {
    for (status, body) in [
        (StatusCode::OK, br#"{"items":[],"pagination":{"total_count":0,"max_page":0}}"#.as_slice()),
        (StatusCode::NOT_FOUND, b"missing route".as_slice()),
        (StatusCode::INTERNAL_SERVER_ERROR, b"server failure".as_slice()),
    ] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    status,
                    Bytes::from_static(body),
                    json!({"name":"Acme","slug":"acme"}),
                ))
                .unwrap();
        let result = create(&client, payload()).await;
        assert!(
            matches!(
                &result, Err(CreateOrganizationError::UnexpectedStatus { status: actual, body: actual_body })
                    if *actual == status && actual_body.as_ref() == body
            ),
            "expected UnexpectedStatus for {status}, got {result:?}"
        );
    }
}

#[tokio::test]
async fn invalid_authorization_fails_before_calling_transport() {
    let transport = tower::service_fn(|_: Request<RequestBody>| {
        panic!("invalid request must not reach transport");
        #[allow(unreachable_code)]
        std::future::ready(Ok::<_, Infallible>(Response::new(RequestBody::default())))
    });
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(transport)
            .unwrap();
    let result = create(&client, payload())
        .options(RequestOptions {
            access_token: Some(
                AccessToken::try_from("invalid\ntoken").expect("nonempty test token"),
            ),
            ..Default::default()
        })
        .await;
    assert!(matches!(
        result,
        Err(CreateOrganizationError::BuildRequest(BuildRequestError::Http(_)))
    ));
}

#[tokio::test]
async fn inherits_timeout_and_honors_request_override() {
    for overridden in [false, true] {
        let transport = tower::service_fn(|_: Request<RequestBody>| {
            std::future::pending::<Result<Response<RequestBody>, Infallible>>()
        });
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .timeout(if overridden {
                    Duration::from_secs(60)
                } else {
                    Duration::from_millis(10)
                })
                .build_with(transport)
                .unwrap();
        let request = create(&client, payload());
        let result = tokio::time::timeout(Duration::from_secs(1), async {
            if overridden {
                request
                    .options(RequestOptions {
                        timeout: Some(Duration::from_millis(10)),
                        ..Default::default()
                    })
                    .await
            } else {
                request.send().await
            }
        })
        .await
        .expect("request timeout must expire before the test deadline");
        assert!(matches!(result, Err(CreateOrganizationError::Timeout(_))));
    }
}
