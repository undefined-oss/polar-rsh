use std::{convert::Infallible, time::Duration};

use http::StatusCode;

use crate::{
    client::{AccessToken, PolarClient, RequestBody, RequestOptions},
    models::{
        common::{OrganizationId, errors::ValidationLocation},
        organization::{GetOrganizationError, Organization},
        pagination::ListResource,
    },
};

mod client;
mod error;
mod models;
mod query;
mod templates;

fn mock_transport(
    status: StatusCode,
    body: &'static [u8],
) -> impl tower::Service<
    http::Request<RequestBody>,
    Response = http::Response<RequestBody>,
    Error = Infallible,
> + Clone {
    tower::service_fn(move |request: http::Request<RequestBody>| async move {
        assert_eq!(request.method(), &http::Method::GET);
        assert_eq!(
            request.uri().to_string(),
            "https://sandbox-api.polar.sh/v1/organizations/1dbfc517-0bbf-4301-9ba8-555ca42b9737"
        );
        assert_eq!(
            request.headers().get("Authorization").unwrap().to_str().unwrap(),
            "Bearer example-token"
        );
        assert!(request.headers().get("Authorization").unwrap().is_sensitive());
        assert_eq!(request.headers().get("Accept").unwrap().to_str().unwrap(), "application/json");
        assert_eq!(request.headers().get("Polar-Version").unwrap().to_str().unwrap(), "2026-10");
        let mut response = http::Response::new(RequestBody::new(bytes::Bytes::from_static(body)));
        *response.status_mut() = status;
        Ok::<_, Infallible>(response)
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let page: ListResource<Organization> = serde_json::from_str(
        r#"{
            "items": [],
            "pagination": {"total_count": 0, "max_page": 0}
        }"#,
    )?;
    assert!(page.items.is_empty());
    assert_eq!(page.pagination.total_count, 0);
    assert_eq!(page.pagination.max_page, 0);

    for body in [
        r#"{"items":[]}"#,
        r#"{"pagination":{"total_count":0,"max_page":0}}"#,
        r#"{"items":[],"pagination":{"total_count":0}}"#,
        r#"{"items":[],"pagination":{"max_page":0}}"#,
    ] {
        let result = serde_json::from_str::<ListResource<Organization>>(body);
        assert!(
            matches!(&result, Err(error) if error.is_data()),
            "expected a missing required field error for {body}, got {result:?}"
        );
    }

    let organization_id =
        OrganizationId::from(uuid::Uuid::parse_str("1dbfc517-0bbf-4301-9ba8-555ca42b9737")?);

    let configuration_client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(client::PolarEnvironment::Sandbox)
            .build_with(())?;
    assert_eq!(configuration_client.default_timeout(), Duration::from_secs(5));

    let overridden = configuration_client
        .organizations()
        .get(organization_id)
        .options(RequestOptions {
            access_token: Some(
                AccessToken::try_from("override-token").expect("nonempty test token"),
            ),
            ..Default::default()
        })
        .build_request()?;
    let authorization = overridden.headers().get("Authorization").unwrap();
    assert_eq!(authorization.to_str()?, "Bearer override-token");
    assert!(authorization.is_sensitive());

    let ordinary = configuration_client.organizations().get(organization_id).build_request()?;
    let authorization = ordinary.headers().get("Authorization").unwrap();
    assert_eq!(authorization.to_str()?, "Bearer example-token");
    assert!(authorization.is_sensitive());

    let invalid = configuration_client
        .organizations()
        .get(organization_id)
        .options(RequestOptions {
            access_token: Some(
                AccessToken::try_from("invalid\ntoken").expect("nonempty test token"),
            ),
            ..Default::default()
        })
        .build_request();
    assert!(matches!(invalid, Err(_)), "a token containing a newline must be rejected");

    for body in [b"not-json".as_slice(), b"{}".as_slice()] {
        let polar_client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(client::PolarEnvironment::Sandbox)
                .build_with(mock_transport(StatusCode::OK, body))?;
        let result = polar_client.organizations().get(organization_id).await;

        assert!(
            matches!(&result, Err(GetOrganizationError::DecodeResponse(_))),
            "expected DecodeResponse for {body:?}, got {result:?}"
        );
    }

    let error_body = br#"{"error":"ResourceNotFound","detail":"Organization not found."}"#;
    let polar_client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(client::PolarEnvironment::Sandbox)
            .build_with(mock_transport(StatusCode::NOT_FOUND, error_body))?;
    let result = polar_client.organizations().get(organization_id).await;

    assert!(
        matches!(
            &result,
            Err(GetOrganizationError::NotFound(error))
                if error.detail == "Organization not found."
                    && matches!(
                        &error.error,
                        models::common::errors::ResourceNotFoundKind::ResourceNotFound
                    )
        ),
        "expected typed NotFound for 404, got {result:?}"
    );

    let error_body = b"server failure";
    let polar_client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(client::PolarEnvironment::Sandbox)
            .build_with(mock_transport(StatusCode::INTERNAL_SERVER_ERROR, error_body))?;
    let result = polar_client.organizations().get(organization_id).await;
    assert!(
        matches!(
            &result,
            Err(GetOrganizationError::UnexpectedStatus { status, body })
                if *status == StatusCode::INTERNAL_SERVER_ERROR
                    && body.as_ref() == error_body
        ),
        "expected UnexpectedStatus preserving the 500 response body, got {result:?}"
    );

    for body in [
        b"not-json".as_slice(),
        br#"{"error":"OtherError","detail":"Organization not found."}"#.as_slice(),
        br#"{"error":"ResourceNotFound"}"#.as_slice(),
        br#"{"detail":"Organization not found."}"#.as_slice(),
    ] {
        let polar_client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(client::PolarEnvironment::Sandbox)
                .build_with(mock_transport(StatusCode::NOT_FOUND, body))?;
        let result = polar_client.organizations().get(organization_id).await;
        assert!(
            matches!(&result, Err(GetOrganizationError::DecodeResponse(_))),
            "expected DecodeResponse for invalid 404 body {body:?}, got {result:?}"
        );
    }

    let validation_body = br#"{
        "detail": [
            {
                "loc": ["path", "id"],
                "msg": "Input should be a valid UUID",
                "type": "uuid_parsing",
                "input": "invalid-id"
            },
            {
                "loc": ["body", "items", 0],
                "msg": "Field required",
                "type": "missing",
                "ctx": {"field": "name"}
            }
        ]
    }"#;
    let polar_client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .environment(client::PolarEnvironment::Sandbox)
            .build_with(mock_transport(StatusCode::UNPROCESSABLE_ENTITY, validation_body))?;
    let result = polar_client.organizations().get(organization_id).await;
    match &result {
        Err(GetOrganizationError::Validation(error)) => {
            let details = error.detail.as_ref().expect("expected validation details");
            assert_eq!(details.len(), 2);
            let first = &details[0];
            assert!(matches!(
                first.loc.as_slice(),
                [ValidationLocation::Field(scope), ValidationLocation::Field(field)]
                    if scope == "path" && field == "id"
            ));
            assert_eq!(first.msg, "Input should be a valid UUID");
            assert_eq!(first.kind, "uuid_parsing");
            assert_eq!(first.input, Some(serde_json::json!("invalid-id")));
            assert!(first.ctx.is_none());

            let second = &details[1];
            assert!(matches!(
                second.loc.as_slice(),
                [ValidationLocation::Field(scope), ValidationLocation::Field(field), ValidationLocation::Index(0)]
                    if scope == "body" && field == "items"
            ));
            assert_eq!(second.msg, "Field required");
            assert_eq!(second.kind, "missing");
            assert!(second.input.is_none());
            assert_eq!(
                second.ctx.as_ref().and_then(|ctx| ctx.get("field")),
                Some(&serde_json::json!("name"))
            );
        }
        other => panic!("expected typed Validation for 422, got {other:?}"),
    }

    for (body, has_detail) in [(br#"{"detail":[]}"#.as_slice(), true), (b"{}".as_slice(), false)] {
        let polar_client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(client::PolarEnvironment::Sandbox)
                .build_with(mock_transport(StatusCode::UNPROCESSABLE_ENTITY, body))?;
        let result = polar_client.organizations().get(organization_id).await;
        assert!(
            matches!(
                &result,
                Err(GetOrganizationError::Validation(error))
                    if matches!((&error.detail, has_detail), (Some(details), true) if details.is_empty())
                        || matches!((&error.detail, has_detail), (None, false))
            ),
            "expected Validation with empty or absent details for {body:?}, got {result:?}"
        );
    }

    for body in [br#"{"detail":[{"loc":["path","id"]}]}"#.as_slice(), b"not-json".as_slice()] {
        let polar_client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .environment(client::PolarEnvironment::Sandbox)
                .build_with(mock_transport(StatusCode::UNPROCESSABLE_ENTITY, body))?;
        let result = polar_client.organizations().get(organization_id).await;
        assert!(
            matches!(&result, Err(GetOrganizationError::DecodeResponse(_))),
            "expected DecodeResponse for invalid 422 body {body:?}, got {result:?}"
        );
    }

    let pending_transport = tower::service_fn(|_: http::Request<RequestBody>| {
        std::future::pending::<Result<http::Response<RequestBody>, Infallible>>()
    });
    let timeout_client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .timeout(Duration::from_millis(10))
            .build_with(pending_transport.clone())?;
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        timeout_client.organizations().get(organization_id).send(),
    )
    .await
    .expect("the request must inherit the builder's 10 ms timeout");
    assert!(
        matches!(&result, Err(GetOrganizationError::Timeout(_))),
        "expected Timeout from the builder setting, got {result:?}"
    );

    let timeout_client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .timeout(Duration::from_secs(60))
            .build_with(pending_transport)?;
    let result = tokio::time::timeout(Duration::from_secs(1), async {
        timeout_client
            .organizations()
            .get(organization_id)
            .options(RequestOptions {
                timeout: Some(Duration::from_millis(10)),
                ..Default::default()
            })
            .await
    })
    .await
    .expect("the request's 10 ms timeout must override the builder's 60 s timeout");
    assert!(
        matches!(&result, Err(GetOrganizationError::Timeout(_))),
        "expected Timeout from the request override, got {result:?}"
    );
    assert_eq!(timeout_client.default_timeout(), Duration::from_secs(60));

    Ok(())
}
