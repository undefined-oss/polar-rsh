use std::{convert::Infallible, error::Error, rc::Rc};

use futures_util::StreamExt;

use super::*;
use crate::{
    client::{AccessToken, PolarClient, RequestBody},
    error::RequestFailure,
    models::common::{OrganizationId, OrganizationName, OrganizationSlug},
};

// Deliberately borrowed, non-Send, non-Sync, and without Debug, Display, or
// Error.
struct LocalTransportError<'a>(&'a Rc<()>);

#[tokio::test(flavor = "current_thread")]
async fn preserves_unrestricted_transport_errors_across_all_operation_interfaces() {
    let marker = Rc::new(());
    let marker_ref = &marker;
    let transport = tower::service_fn(move |_: http::Request<RequestBody>| {
        std::future::ready(Err::<http::Response<RequestBody>, _>(LocalTransportError(marker_ref)))
    });
    let client = PolarClient::builder(AccessToken::try_from("test-token").unwrap())
        .build_with(transport)
        .unwrap();
    let id = OrganizationId::from(uuid::Uuid::from_u128(1));
    let payload = || {
        OrganizationCreate::new(OrganizationCreateRequired {
            name: OrganizationName::try_from("Acme").unwrap(),
            slug: OrganizationSlug::try_from("acme").unwrap(),
        })
    };

    macro_rules! check {
        ($result:expr, $error:ident) => {
            match $result {
                Err($error::Transport(error)) => assert!(std::ptr::eq(error.0, marker_ref)),
                _ => panic!("expected the original concrete transport error"),
            }
        };
    }
    check!(client.organizations().get(id).send().await, GetOrganizationError);
    check!(client.organizations().get(id).await, GetOrganizationError);
    check!(client.organizations().list().send().await, ListOrganizationsError);
    check!(client.organizations().list().await, ListOrganizationsError);
    check!(client.organizations().create(payload()).send().await, CreateOrganizationError);
    check!(client.organizations().create(payload()).await, CreateOrganizationError);
    check!(
        client.organizations().update(id, OrganizationUpdate::new()).send().await,
        UpdateOrganizationError
    );
    check!(
        client.organizations().update(id, OrganizationUpdate::new()).await,
        UpdateOrganizationError
    );

    let pages = client.organizations().list().pages();
    futures_util::pin_mut!(pages);
    check!(pages.next().await.unwrap(), ListOrganizationsError);
    assert!(pages.next().await.is_none());
    let items = client.organizations().list().items();
    futures_util::pin_mut!(items);
    check!(items.next().await.unwrap(), ListOrganizationsError);
    assert!(items.next().await.is_none());
}

#[test]
fn union_conversion_preserves_decode_error_and_source() {
    let original = serde_json::from_slice::<Organization>(b"not-json").unwrap_err();
    let message = original.to_string();
    let union: eros::ErrorUnion<get::GetOrganizationErrors> =
        eros::ErrorUnion::new(RequestFailure::DecodeResponse(original));
    let public: GetOrganizationError<Infallible> = union.into();
    assert_eq!(public.source().unwrap().to_string(), message);
    match public {
        GetOrganizationError::DecodeResponse(error) => assert_eq!(error.to_string(), message),
        _ => panic!("expected decode error"),
    }
}

#[test]
fn union_conversion_preserves_domain_error_and_source() {
    let original = CannotCreateOrganizationError {
        error: CannotCreateOrganizationErrorKind::CannotCreateOrganizationError,
        detail: "Creation is unavailable".into(),
    };
    let union: eros::ErrorUnion<create::CreateOrganizationErrors> = eros::ErrorUnion::new(original);
    let public: CreateOrganizationError<Infallible> = union.into();
    assert_eq!(public.source().unwrap().to_string(), "Creation is unavailable");
    match public {
        CreateOrganizationError::CannotCreateOrganization(error) => {
            assert_eq!(error.detail, "Creation is unavailable");
        }
        _ => panic!("expected create domain error"),
    }
}

#[test]
fn union_conversion_preserves_unexpected_response_bytes() {
    let body = bytes::Bytes::from_static(b"raw response");
    let union: eros::ErrorUnion<list::ListOrganizationsErrors> =
        eros::ErrorUnion::new(RequestFailure::UnexpectedStatus {
            status: http::StatusCode::BAD_GATEWAY,
            body: body.clone(),
        });
    let public: ListOrganizationsError<Infallible> = union.into();
    match public {
        ListOrganizationsError::UnexpectedStatus { status, body: actual } => {
            assert_eq!(status, http::StatusCode::BAD_GATEWAY);
            assert_eq!(actual, body);
        }
        _ => panic!("expected unexpected response"),
    }
}
