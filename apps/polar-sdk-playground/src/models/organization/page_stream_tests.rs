use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    convert::Infallible,
    num::NonZeroU64,
    rc::Rc,
    time::Duration,
};

use futures_util::StreamExt;
use http::{Request, Response, StatusCode};

use super::{ListOrganizationsParams, OrganizationSortField};
use crate::{
    client::{PolarClient, PolarEnvironment, RequestBody, RequestOptions},
    error::RequestError,
    models::common::enums::OrganizationSlug,
    query::Sorting,
};

const TWO_PAGES: &[u8] = br#"{"items":[],"pagination":{"total_count":0,"max_page":2}}"#;
const THREE_PAGES: &[u8] = br#"{"items":[],"pagination":{"total_count":0,"max_page":3}}"#;

type Requests = Rc<RefCell<Vec<Request<RequestBody>>>>;

fn mock_pages(
    responses: Vec<(StatusCode, &'static [u8])>,
    requests: Requests,
) -> impl tower::Service<Request<RequestBody>, Response = Response<RequestBody>, Error = Infallible>
+ Clone {
    let responses = Rc::new(RefCell::new(VecDeque::from(responses)));
    tower::service_fn(move |request: Request<RequestBody>| {
        assert_eq!(request.method(), http::Method::GET);
        assert_eq!(request.uri().path(), "/v1/organizations/");
        requests.borrow_mut().push(request);
        let (status, body) = responses.borrow_mut().pop_front().expect("unexpected extra request");
        std::future::ready(Ok::<_, Infallible>(
            Response::builder()
                .status(status)
                .body(RequestBody::new(bytes::Bytes::from_static(body)))
                .unwrap(),
        ))
    })
}

#[tokio::test]
async fn fetches_empty_pages_lazily_and_stops_at_the_last_page() {
    let requests = Requests::default();
    let client = PolarClient::builder("example-token")
        .environment(PolarEnvironment::Sandbox)
        .build_with(mock_pages(
            vec![(StatusCode::OK, TWO_PAGES), (StatusCode::OK, TWO_PAGES)],
            requests.clone(),
        ))
        .unwrap();
    let pages = client.organizations().list().pages();
    futures_util::pin_mut!(pages);

    assert!(requests.borrow().is_empty(), "constructing a stream must not send requests");
    for page in 1..=2 {
        let response = pages.next().await.unwrap().unwrap();
        assert!(response.items.is_empty());
        assert_eq!(response.pagination.max_page, 2);
        assert_eq!(requests.borrow().len(), page, "must fetch only the requested page");
        assert_eq!(
            requests.borrow()[page - 1].uri().query(),
            Some(format!("page={page}").as_str())
        );
        tokio::task::yield_now().await;
        assert_eq!(requests.borrow().len(), page, "must not prefetch the next page");
    }
    assert!(pages.next().await.is_none());
    assert!(pages.next().await.is_none());
    assert_eq!(requests.borrow().len(), 2);
}

#[tokio::test]
async fn preserves_starting_page_filters_sorting_and_token_override() {
    let requests = Requests::default();
    let client = PolarClient::builder("example-token")
        .environment(PolarEnvironment::Sandbox)
        .build_with(mock_pages(
            vec![(StatusCode::OK, THREE_PAGES), (StatusCode::OK, THREE_PAGES)],
            requests.clone(),
        ))
        .unwrap();
    let pages = client
        .organizations()
        .list()
        .params(ListOrganizationsParams {
            slug: Some(OrganizationSlug::from("acme & sons".to_owned())),
            page: NonZeroU64::new(2),
            limit: NonZeroU64::new(20),
            sorting: vec![
                (OrganizationSortField::Name, Sorting::Desc),
                (OrganizationSortField::CreatedAt, Sorting::Asc),
            ],
        })
        .options(RequestOptions {
            access_token: Some("override-token".into()),
            ..Default::default()
        })
        .pages();
    futures_util::pin_mut!(pages);

    for (index, expected_page) in [2, 3].into_iter().enumerate() {
        pages.next().await.unwrap().unwrap();
        let requests = requests.borrow();
        assert_eq!(requests.len(), index + 1);
        let request = &requests[index];
        assert_eq!(
            request.uri().query(),
            Some(
                format!(
                    "slug=acme+%26+sons&page={expected_page}&limit=20&sorting=-name&\
                     sorting=created_at"
                )
                .as_str()
            )
        );
        assert_eq!(request.headers()["Authorization"], "Bearer override-token");
        assert!(request.headers()["Authorization"].is_sensitive());
    }
    assert!(pages.next().await.is_none());
    assert_eq!(requests.borrow().len(), 2);
}

#[tokio::test]
async fn yields_a_page_error_once_and_stops_requesting() {
    for status in [StatusCode::INTERNAL_SERVER_ERROR, StatusCode::OK] {
        let requests = Requests::default();
        let client = PolarClient::builder("example-token")
            .build_with(mock_pages(
                vec![(StatusCode::OK, THREE_PAGES), (status, b"not-json")],
                requests.clone(),
            ))
            .unwrap();
        let pages = client.organizations().list().pages();
        futures_util::pin_mut!(pages);

        pages.next().await.unwrap().unwrap();
        let error = pages.next().await.unwrap().unwrap_err();
        if status == StatusCode::OK {
            assert!(matches!(error, RequestError::DecodeResponse(_)));
        } else {
            assert!(matches!(
                error,
                RequestError::UnexpectedStatus { status: actual, body }
                    if actual == status && body.as_ref() == b"not-json"
            ));
        }
        assert!(pages.next().await.is_none());
        assert!(pages.next().await.is_none());
        assert_eq!(requests.borrow().len(), 2);
    }
}

#[tokio::test]
async fn retains_the_timeout_override_for_later_pages() {
    let calls = Rc::new(Cell::new(0));
    let transport_calls = calls.clone();
    let transport = tower::service_fn(move |request: Request<RequestBody>| {
        let call = transport_calls.get() + 1;
        transport_calls.set(call);
        assert_eq!(request.uri().query(), Some(format!("page={call}").as_str()));
        async move {
            if call == 1 {
                Ok::<_, Infallible>(Response::new(RequestBody::new(bytes::Bytes::from_static(
                    TWO_PAGES,
                ))))
            } else {
                std::future::pending::<Result<Response<RequestBody>, Infallible>>().await
            }
        }
    });
    let client = PolarClient::builder("example-token")
        .timeout(Duration::from_secs(60))
        .build_with(transport)
        .unwrap();
    let pages = client
        .organizations()
        .list()
        .options(RequestOptions { timeout: Some(Duration::from_millis(10)), ..Default::default() })
        .pages();
    futures_util::pin_mut!(pages);

    pages.next().await.unwrap().unwrap();
    let result = tokio::time::timeout(Duration::from_secs(1), pages.next())
        .await
        .expect("the second page must retain the short request timeout");
    assert!(matches!(result, Some(Err(RequestError::Timeout(_)))));
    assert!(pages.next().await.is_none());
    assert_eq!(calls.get(), 2);
    assert_eq!(client.default_timeout(), Duration::from_secs(60));
}

#[tokio::test]
async fn zero_max_page_yields_the_initial_page_then_stops() {
    let requests = Requests::default();
    let client = PolarClient::builder("example-token")
        .build_with(mock_pages(
            vec![(StatusCode::OK, br#"{"items":[],"pagination":{"total_count":0,"max_page":0}}"#)],
            requests.clone(),
        ))
        .unwrap();
    let pages = client.organizations().list().pages();
    futures_util::pin_mut!(pages);

    assert_eq!(pages.next().await.unwrap().unwrap().pagination.max_page, 0);
    assert!(pages.next().await.is_none());
    assert_eq!(requests.borrow().len(), 1);
}
