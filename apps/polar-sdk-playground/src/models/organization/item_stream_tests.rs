use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    convert::Infallible,
    rc::Rc,
};

use bytes::Bytes;
use futures_util::StreamExt;
use http::{Request, Response, StatusCode};

use crate::{
    client::{AccessToken, PolarClient, RequestBody},
    models::{common::OrganizationId, organization::ListOrganizationsError},
};

// Synthetic responses matching the playground's current Organization model.
pub(super) fn organization(index: u128) -> serde_json::Value {
    serde_json::json!({
        "id": uuid::Uuid::from_u128(index).to_string(),
        "name": format!("Organization {index}"),
        "slug": format!("organization-{index}"),
        "account_id": null,
        "allow_customer_updates": false,
        "capabilities": {
            "api_access": true,
            "checkout_payments": false,
            "dashboard_access": true,
            "payouts": false,
            "refunds": false,
            "subscription_renewals": false
        },
        "created_at": "2026-01-01T00:00:00Z",
        "customer_email_settings": {
            "order_confirmation": false,
            "payment_method_expiration_reminder": false,
            "subscription_cancellation": false,
            "subscription_confirmation": false,
            "subscription_cycled": false,
            "subscription_cycled_after_trial": false,
            "subscription_past_due": false,
            "subscription_paused": false,
            "subscription_renewal_reminder": false,
            "subscription_resumed": false,
            "subscription_revoked": false,
            "subscription_trial_conversion_reminder": false,
            "subscription_uncanceled": false,
            "subscription_updated": false
        },
        "customer_portal_settings": {
            "subscription": {"update_plan": false, "update_seats": false},
            "usage": {"show": false}
        },
        "default_presentment_currency": "usd",
        "default_tax_behavior": "location",
        "dispute_settings": {"auto_accept_below_amount": null},
        "embed_hosts": [],
        "proration_behavior": "invoice",
        "socials": [{"platform":"github", "url":format!("https://github.com/organization-{index}")}],
        "sso_enforced": false,
        "status": "created",
        "subscription_settings": {
            "allow_customer_updates": false,
            "allow_multiple_subscriptions": false,
            "benefit_revocation_grace_period": 0,
            "prevent_trial_abuse": false,
            "proration_behavior": "invoice"
        }
    })
}

fn page(ids: &[u128], max_page: i64, total_count: i64) -> (StatusCode, Bytes) {
    let body = serde_json::json!({
        "items": ids.iter().copied().map(organization).collect::<Vec<_>>(),
        "pagination": {"max_page": max_page, "total_count": total_count}
    });
    (StatusCode::OK, Bytes::from(serde_json::to_vec(&body).unwrap()))
}

fn mock_pages(
    responses: Vec<(StatusCode, Bytes)>,
    calls: Rc<Cell<usize>>,
) -> impl tower::Service<Request<RequestBody>, Response = Response<RequestBody>, Error = Infallible>
+ Clone {
    let responses = Rc::new(RefCell::new(VecDeque::from(responses)));
    tower::service_fn(move |request: Request<RequestBody>| {
        let current = calls.get() + 1;
        calls.set(current);
        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(request.uri().path(), "/v1/organizations/");
        assert_eq!(request.uri().query(), Some(format!("page={current}").as_str()));
        let (status, body) =
            responses.borrow_mut().pop_front().expect("unexpected extra page request");
        std::future::ready(Ok::<_, Infallible>(
            Response::builder().status(status).body(RequestBody::new(body)).unwrap(),
        ))
    })
}

#[tokio::test]
async fn yields_items_in_order_without_fetching_the_next_page_early() {
    let calls = Rc::new(Cell::new(0));
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(mock_pages(vec![page(&[1, 2], 2, 3), page(&[3], 2, 3)], calls.clone()))
            .unwrap();
    let items = client.organizations().list().items();
    futures_util::pin_mut!(items);

    assert_eq!(calls.get(), 0, "construction must not fetch a page");
    for (id, expected_calls) in [(1, 1), (2, 1), (3, 2)] {
        let item = items.next().await.unwrap().unwrap();
        assert_eq!(item.id, OrganizationId::from(uuid::Uuid::from_u128(id)));
        assert_eq!(item.name.as_ref(), format!("Organization {id}"));
        assert_eq!(item.socials.len(), 1);
        assert_eq!(item.socials[0].platform, super::OrganizationSocialPlatforms::Github);
        assert_eq!(item.socials[0].url.as_str(), format!("https://github.com/organization-{id}"));
        assert_eq!(calls.get(), expected_calls);
        tokio::task::yield_now().await;
        assert_eq!(calls.get(), expected_calls, "must not prefetch while the consumer is idle");
    }
    assert!(items.next().await.is_none());
    assert!(items.next().await.is_none());
    assert_eq!(calls.get(), 2);
}

#[tokio::test]
async fn skips_empty_intermediate_pages() {
    let calls = Rc::new(Cell::new(0));
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(mock_pages(
                vec![page(&[1], 3, 2), page(&[], 3, 2), page(&[2], 3, 2)],
                calls.clone(),
            ))
            .unwrap();
    let items = client.organizations().list().items();
    futures_util::pin_mut!(items);

    assert_eq!(items.next().await.unwrap().unwrap().name.as_ref(), "Organization 1");
    assert_eq!(calls.get(), 1);
    assert_eq!(items.next().await.unwrap().unwrap().name.as_ref(), "Organization 2");
    assert_eq!(calls.get(), 3, "one next() must skip the empty page");
    assert!(items.next().await.is_none());
    assert_eq!(calls.get(), 3);
}

#[tokio::test]
async fn entirely_empty_results_end_without_yielding_items() {
    for (responses, expected_calls) in
        [(vec![page(&[], 0, 0)], 1), (vec![page(&[], 2, 0), page(&[], 2, 0)], 2)]
    {
        let calls = Rc::new(Cell::new(0));
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .build_with(mock_pages(responses, calls.clone()))
                .unwrap();
        let items = client.organizations().list().items();
        futures_util::pin_mut!(items);

        assert!(items.next().await.is_none());
        assert!(items.next().await.is_none());
        assert_eq!(calls.get(), expected_calls);
    }
}

#[tokio::test]
async fn later_page_errors_are_yielded_once_then_the_stream_ends() {
    for status in [StatusCode::INTERNAL_SERVER_ERROR, StatusCode::OK] {
        let calls = Rc::new(Cell::new(0));
        let client =
            PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
                .build_with(mock_pages(
                    vec![page(&[1, 2], 3, 3), (status, Bytes::from_static(b"not-json"))],
                    calls.clone(),
                ))
                .unwrap();
        let items = client.organizations().list().items();
        futures_util::pin_mut!(items);

        assert_eq!(items.next().await.unwrap().unwrap().name.as_ref(), "Organization 1");
        assert_eq!(items.next().await.unwrap().unwrap().name.as_ref(), "Organization 2");
        assert_eq!(calls.get(), 1);
        let error = items.next().await.unwrap().unwrap_err();
        if status == StatusCode::OK {
            assert!(matches!(error, ListOrganizationsError::DecodeResponse(_)));
        } else {
            assert!(matches!(
                error,
                ListOrganizationsError::UnexpectedStatus { status: actual, body }
                    if actual == status && body.as_ref() == b"not-json"
            ));
        }
        assert!(items.next().await.is_none());
        assert!(items.next().await.is_none());
        assert_eq!(calls.get(), 2);
    }
}

#[tokio::test]
async fn dropping_early_does_not_fetch_more_pages() {
    let calls = Rc::new(Cell::new(0));
    let client =
        PolarClient::builder(AccessToken::try_from("example-token").expect("valid test token"))
            .build_with(mock_pages(vec![page(&[1, 2], 2, 3), page(&[3], 2, 3)], calls.clone()))
            .unwrap();

    drop(client.organizations().list().items());
    tokio::task::yield_now().await;
    assert_eq!(calls.get(), 0);

    {
        let items = client.organizations().list().items();
        futures_util::pin_mut!(items);
        assert_eq!(items.next().await.unwrap().unwrap().name.as_ref(), "Organization 1");
        assert_eq!(calls.get(), 1);
    }
    tokio::task::yield_now().await;
    assert_eq!(calls.get(), 1);
}
