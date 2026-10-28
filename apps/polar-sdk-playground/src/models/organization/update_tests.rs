use std::{collections::BTreeSet, convert::Infallible, time::Duration};

use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use http_body_util::BodyExt;
use presence_rs::Presence;
use serde_json::{Value, json};
use url::Url;

use super::{
    OrganizationDetails, OrganizationDisputeSettingsUpdate, OrganizationFeatureSettingsUpdate,
    OrganizationSocialLink, OrganizationSocialPlatforms, OrganizationUpdate,
};
use crate::{
    client::{AccessToken, PolarClient, PolarEnvironment, RequestBody, RequestOptions},
    error::{BuildRequestError, RequestError},
    models::common::{
        OrganizationId, OrganizationName,
        enums::{CountryAlpha2Input, PresentmentCurrency, TaxBehaviorOption},
    },
};

fn id() -> OrganizationId {
    uuid::Uuid::from_u128(1).into()
}

fn mock_transport(
    status: StatusCode,
    body: Bytes,
    payload: Value,
    token: &'static str,
) -> impl tower::Service<Request<RequestBody>, Response = Response<RequestBody>, Error = Infallible>
+ Clone {
    tower::service_fn(move |request: Request<RequestBody>| {
        let body = body.clone();
        let payload = payload.clone();
        async move {
            assert_eq!(request.method(), Method::PATCH);
            assert_eq!(request.uri().to_string(), "https://sandbox-api.polar.sh/v1/organizations/00000000-0000-0000-0000-000000000001");
            assert_eq!(request.headers()["Content-Type"], "application/json");
            assert_eq!(request.headers()["Accept"], "application/json");
            assert_eq!(request.headers()["Polar-Version"], "2026-10");
            assert_eq!(request.headers()["Authorization"], token);
            assert!(request.headers()["Authorization"].is_sensitive());
            let data = request.into_body().collect().await.unwrap().to_bytes();
            assert_eq!(serde_json::from_slice::<Value>(&data).unwrap(), payload);
            Ok::<_, Infallible>(
                Response::builder().status(status).body(RequestBody::new(body)).unwrap(),
            )
        }
    })
}

#[tokio::test]
async fn complete_update_covers_the_schema_and_builds_a_patch_request() {
    let fixture = super::item_stream_tests::organization(1);
    let payload = OrganizationUpdate::new()
        .name(OrganizationName::try_from("Acme Updated").expect("valid test name"))
        .avatar_url(Url::parse("https://example.com/avatar.png").expect("valid test URL"))
        .email("support@example.com")
        .website(Url::parse("https://example.com/").expect("valid test URL"))
        .socials(vec![OrganizationSocialLink {
            platform: OrganizationSocialPlatforms::Github,
            url: Url::parse("https://github.com/acme").expect("valid test URL"),
        }])
        .details(OrganizationDetails::new().product_description("Software"))
        .country(CountryAlpha2Input::ES)
        .feature_settings(OrganizationFeatureSettingsUpdate::new().member_model_enabled(false))
        .subscription_settings(
            serde_json::from_value(fixture["subscription_settings"].clone()).unwrap(),
        )
        .customer_email_settings(
            serde_json::from_value(fixture["customer_email_settings"].clone()).unwrap(),
        )
        .customer_portal_settings(
            serde_json::from_value(fixture["customer_portal_settings"].clone()).unwrap(),
        )
        .dispute_settings(OrganizationDisputeSettingsUpdate::new().auto_accept_below_amount(1000))
        .embed_hosts(vec!["example.com".into()])
        .default_presentment_currency(PresentmentCurrency::Eur)
        .default_tax_behavior(TaxBehaviorOption::Inclusive)
        .sso_enforced(false);
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(())
            .unwrap();
    let request = client.organizations().update(id(), payload).build_request().unwrap();
    assert_eq!(request.method(), Method::PATCH);
    let data = request.into_body().collect().await.unwrap().to_bytes();
    let value: Value = serde_json::from_slice(&data).unwrap();
    assert_eq!(
        value,
        json!({
            "name":"Acme Updated", "avatar_url":"https://example.com/avatar.png",
            "email":"support@example.com", "website":"https://example.com/",
            "socials":[{"platform":"github", "url":"https://github.com/acme"}],
            "details":{"product_description":"Software"}, "country":"ES",
            "feature_settings":{"member_model_enabled":false},
            "subscription_settings":fixture["subscription_settings"],
            "customer_email_settings":fixture["customer_email_settings"],
            "customer_portal_settings":fixture["customer_portal_settings"],
            "dispute_settings":{"auto_accept_below_amount":1000}, "embed_hosts":["example.com"],
            "default_presentment_currency":"eur", "default_tax_behavior":"inclusive", "sso_enforced":false
        })
    );
    let spec: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../vendor/polar/docs/openapi/2026-10.openapi.json"
    )))
    .unwrap();
    let actual: BTreeSet<_> = value.as_object().unwrap().keys().collect();
    let expected: BTreeSet<_> = spec["components"]["schemas"]["OrganizationUpdate"]["properties"]
        .as_object()
        .unwrap()
        .keys()
        .collect();
    assert_eq!(actual, expected);
    let decoded: OrganizationUpdate = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), value);
}

#[test]
fn partial_payloads_preserve_omission_false_empty_arrays_and_null() {
    assert_eq!(serde_json::to_value(OrganizationUpdate::new()).unwrap(), json!({}));
    assert_eq!(serde_json::to_value(OrganizationDisputeSettingsUpdate::new()).unwrap(), json!({}));
    let mut payload = OrganizationUpdate::new()
        .name(OrganizationName::try_from("Old").expect("valid test name"))
        .name(OrganizationName::try_from("New").expect("valid test name"))
        .sso_enforced(true)
        .sso_enforced(false)
        .socials(vec![])
        .embed_hosts(vec![]);
    payload.email = Presence::Null;
    payload.default_presentment_currency = Presence::Null;
    payload.default_tax_behavior = Presence::Null;
    let value = serde_json::to_value(payload).unwrap();
    assert_eq!(
        value,
        json!({
            "name":"New", "sso_enforced":false, "socials":[], "embed_hosts":[], "email":null,
            "default_presentment_currency":null, "default_tax_behavior":null
        })
    );
    let decoded: OrganizationUpdate = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), value);
}

#[tokio::test]
async fn both_await_styles_return_one_organization_and_forward_partial_payloads() {
    let response = super::item_stream_tests::organization(1);
    for direct in [false, true] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    StatusCode::OK,
                    Bytes::from(serde_json::to_vec(&response).unwrap()),
                    json!({"name":"Updated"}),
                    "Bearer example-token",
                ))
                .unwrap();
        let request = client.organizations().update(
            id(),
            OrganizationUpdate::new()
                .name(OrganizationName::try_from("Updated").expect("valid test name")),
        );
        let organization = if direct { request.await } else { request.send().await }.unwrap();
        assert_eq!(organization.id, id());
        assert_eq!(organization.name.as_ref(), "Organization 1");
    }
}

#[tokio::test]
async fn token_override_is_forwarded_and_client_default_is_preserved() {
    let response = super::item_stream_tests::organization(1);
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(PolarEnvironment::Sandbox)
            .build_with(mock_transport(
                StatusCode::OK,
                Bytes::from(serde_json::to_vec(&response).unwrap()),
                json!({}),
                "Bearer override-token",
            ))
            .unwrap();
    client
        .organizations()
        .update(id(), OrganizationUpdate::new())
        .options(RequestOptions {
            access_token: Some(
                AccessToken::try_from("override-token").expect("nonempty test token"),
            ),
            ..Default::default()
        })
        .await
        .unwrap();
    let ordinary =
        client.organizations().update(id(), OrganizationUpdate::new()).build_request().unwrap();
    assert_eq!(ordinary.headers()["Authorization"], "Bearer example-token");
    assert!(ordinary.headers()["Authorization"].is_sensitive());
}

#[tokio::test]
async fn maps_all_declared_error_responses_and_preserves_sources() {
    for (status, kind) in [
        (StatusCode::FORBIDDEN, "NotPermitted"),
        (StatusCode::FORBIDDEN, "DisputeAutoAcceptNotEnabled"),
        (StatusCode::CONFLICT, "SSOEnforcementRequiresConnection"),
        (StatusCode::NOT_FOUND, "ResourceNotFound"),
    ] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    status,
                    Bytes::from(
                        serde_json::to_vec(&json!({"error":kind,"detail":"Request rejected"}))
                            .unwrap(),
                    ),
                    json!({}),
                    "Bearer example-token",
                ))
                .unwrap();
        let error =
            client.organizations().update(id(), OrganizationUpdate::new()).await.unwrap_err();
        assert!(
            matches!(
                (kind, &error),
                ("NotPermitted", RequestError::NotPermitted(_))
                    | ("DisputeAutoAcceptNotEnabled", RequestError::DisputeAutoAcceptNotEnabled(_))
                    | (
                        "SSOEnforcementRequiresConnection",
                        RequestError::SsoEnforcementRequiresConnection(_)
                    )
                    | ("ResourceNotFound", RequestError::NotFound(_))
            ),
            "wrong mapping for {kind}: {error:?}"
        );
        assert_eq!(std::error::Error::source(&error).unwrap().to_string(), "Request rejected");
    }
    let client = PolarClient::builder(
        AccessToken::try_from("example-token").expect("valid test token"),
    )
    .environment(PolarEnvironment::Sandbox)
    .build_with(mock_transport(
        StatusCode::UNPROCESSABLE_ENTITY,
        Bytes::from_static(
            br#"{"detail":[{"loc":["body","name"],"msg":"Invalid name","type":"value_error"}]}"#,
        ),
        json!({}),
        "Bearer example-token",
    ))
    .unwrap();
    let error = client.organizations().update(id(), OrganizationUpdate::new()).await.unwrap_err();
    match error {
        RequestError::Validation(error) => assert_eq!(error.detail.unwrap()[0].msg, "Invalid name"),
        other => panic!("expected Validation, got {other:?}"),
    }
}

#[tokio::test]
async fn malformed_or_unknown_declared_responses_fail_decoding() {
    for (status, body) in [
        (StatusCode::OK, "not-json"),
        (StatusCode::OK, "{}"),
        (StatusCode::FORBIDDEN, "not-json"),
        (StatusCode::FORBIDDEN, r#"{"error":"Other","detail":"Rejected"}"#),
        (StatusCode::FORBIDDEN, r#"{"error":"NotPermitted"}"#),
        (StatusCode::FORBIDDEN, r#"{"error":"DisputeAutoAcceptNotEnabled","detail":null}"#),
        (StatusCode::CONFLICT, r#"{"error":"NotPermitted","detail":"Rejected"}"#),
        (StatusCode::CONFLICT, r#"{"error":"SSOEnforcementRequiresConnection"}"#),
        (StatusCode::NOT_FOUND, "not-json"),
        (StatusCode::UNPROCESSABLE_ENTITY, "not-json"),
    ] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    status,
                    Bytes::from_static(body.as_bytes()),
                    json!({}),
                    "Bearer example-token",
                ))
                .unwrap();
        let result = client.organizations().update(id(), OrganizationUpdate::new()).await;
        assert!(
            matches!(result, Err(RequestError::DecodeResponse(_))),
            "expected decode error for {status}: {body}"
        );
    }
}

#[tokio::test]
async fn unexpected_status_preserves_raw_body() {
    for status in [StatusCode::CREATED, StatusCode::INTERNAL_SERVER_ERROR] {
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(PolarEnvironment::Sandbox)
                .build_with(mock_transport(
                    status,
                    Bytes::from_static(b"unexpected response"),
                    json!({}),
                    "Bearer example-token",
                ))
                .unwrap();
        let result = client.organizations().update(id(), OrganizationUpdate::new()).await;
        assert!(
            matches!(result, Err(RequestError::UnexpectedStatus { status: actual, body }) if actual == status && body.as_ref() == b"unexpected response")
        );
    }
}

#[tokio::test]
async fn construction_transport_and_body_failures_are_distinct() {
    let transport = tower::service_fn(|_: Request<RequestBody>| {
        std::future::ready(Err::<Response<RequestBody>, _>(std::io::Error::other(
            "transport failed",
        )))
    });
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(transport)
            .unwrap();
    let invalid = client
        .organizations()
        .update(id(), OrganizationUpdate::new())
        .options(RequestOptions {
            access_token: Some(
                AccessToken::try_from("invalid\ntoken").expect("nonempty test token"),
            ),
            ..Default::default()
        })
        .await;
    assert!(matches!(invalid, Err(RequestError::BuildRequest(BuildRequestError::Http(_)))));
    let failed = client.organizations().update(id(), OrganizationUpdate::new()).await;
    assert!(
        matches!(failed, Err(RequestError::Transport(error)) if error.to_string() == "transport failed")
    );

    let transport = tower::service_fn(|_: Request<RequestBody>| async {
        let body = http_body_util::StreamBody::new(futures_util::stream::once(async {
            Err::<http_body::Frame<Bytes>, _>(std::io::Error::other("body failed"))
        }));
        Ok::<_, Infallible>(Response::new(body))
    });
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(transport)
            .unwrap();
    let failed = client.organizations().update(id(), OrganizationUpdate::new()).await;
    assert!(
        matches!(failed, Err(RequestError::ReadBody(error)) if error.to_string() == "body failed")
    );
}

#[tokio::test]
async fn inherits_timeout_and_honors_request_override() {
    for overridden in [false, true] {
        let transport = tower::service_fn(|_: Request<RequestBody>| {
            std::future::pending::<Result<Response<RequestBody>, Infallible>>()
        });
        let duration = if overridden { Duration::from_secs(60) } else { Duration::from_millis(10) };
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .timeout(duration)
                .build_with(transport)
                .unwrap();
        let request = client.organizations().update(id(), OrganizationUpdate::new());
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
        assert!(matches!(result, Err(RequestError::Timeout(_))));
        assert_eq!(client.default_timeout(), duration);
    }
}
