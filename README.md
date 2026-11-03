# polar-rsh

An async Rust SDK for [Polar](https://polar.sh), generated from the pinned
versioned OpenAPI specifications. The SDK is under active development; the
native generator introduces a new Tower-based client API.

> [!IMPORTANT]
> **Beta:** the SDK is under active development. Its Rust API may change between
> releases. API conformance testing is currently deferred; complete parity with
> Polar's official TypeScript and Python SDKs has not been established.

## Goals

- Use Rust's type system to make invalid data unrepresentable wherever possible,
  with branded IDs, constrained value types, and validation at construction.
- Stay client agnostic: bring your own HTTP client compatible with `tower` and
  `http`, and control its transport configuration and middleware.

## Installation and API versions

> [!IMPORTANT]
> No API version is enabled by default.

Add the SDK to your `Cargo.toml` and explicitly enable one or more API-version
features:

```toml
[dependencies]
polar-rsh = { version = "0.3", features = ["2026-10"] }
```

The feature `2026-10` exposes `polar_rsh::v2026_10`. Multiple versions can coexist,
and each client sends its corresponding `Polar-Version` header. Use models and
operations from the same version module as the client.

## Client and transport

The client builder is transport agnostic: bring your own HTTP client compatible
with the `tower` and `http` crates. Supply a cloneable `tower::Service` that receives
`http::Request<RequestBody>` and returns an `http::Response` whose body implements
`http_body::Body<Data = bytes::Bytes>`, with body errors convertible to
`tower::BoxError`. `RequestBody` is `http_body_util::Full<bytes::Bytes>`.
The client keeps the service's concrete error type in operation errors.

```rust
use polar_rsh::v2026_10::client::{AccessToken, PolarClient, PolarEnvironment};

// `()` is only a placeholder and cannot send requests; replace it with your transport.
let client = PolarClient::builder(AccessToken::try_new("your-token")?)
    .environment(PolarEnvironment::Sandbox)
    .build_with(())?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Production is the default. Configure `.base_url(...)` for a custom endpoint and
`.timeout(...)` for the default operation deadline. The default deadline is five
seconds, including response-body collection. Authentication can be omitted by
passing `None` to the builder.

Resource accessors are available on the client. For example, with a compatible
transport, list products using `client.products().list().params(parameters).await`.
Parameter and error types live under `models::products`, including
`ProductsListParams` and `ProductsListError`. Requests can also be executed with
`.send().await`.

Use `.options(RequestOptions { access_token, timeout })` to override authentication
or the timeout for a single operation. Transport configuration and any automatic
retry policy belong to the supplied Tower service.

## Models, queries, and pagination

Models are organized by resource under `models`. UUID identifiers use the `uuid`
crate internally: resource-specific newtypes wrap `uuid::Uuid` to act as branded
IDs, preventing unrelated identifier types from being mixed. Event variants are
the exception: they currently all share the same `EventId` brand.

To construct an ID from a UUID string, add `uuid = "1"` to your application's
dependencies, parse the UUID, and convert it into the appropriate ID type:

```rust
use polar_rsh::v2026_10::models::common::ids::{CustomerId, ProductId};
use uuid::Uuid;

let customer_id = CustomerId::from(Uuid::parse_str(
    "550e8400-e29b-41d4-a716-446655440000",
)?);
let product_id = ProductId::from(Uuid::parse_str(
    "550e8400-e29b-41d4-a716-446655440001",
)?);

// Both wrap UUIDs, but their types are distinct:
// let wrong_id: CustomerId = product_id; // Does not compile.
let id_text = customer_id.to_string();
```

For example, `client.customers().get(customer_id)` accepts a `CustomerId`, while
passing `product_id` to that operation is a compile-time error. The brand checks
the ID's type; it does not verify that the resource exists on the server.

Date/time properties use `jiff` instead of `chrono`. A follow-up PR will introduce
feature flags to choose between `jiff` and `chrono`; that choice is not available
yet. URL properties use the `url` crate's `url::Url` type, with validated wrappers
where the schema adds constraints.

Constrained scalar values use dedicated newtypes that wrap the underlying Rust
type. For example, strings with minimum or maximum lengths wrap `String`, and
numbers with minimum or maximum values wrap their numeric type. These wrappers
validate values during construction and deserialization, rejecting values outside
the schema's constraints. Fixed-length tuples encode their length in the Rust
tuple type; constrained elements can themselves use validated wrappers.

```rust
use polar_rsh::v2026_10::models::{
    customers::CustomerIndividualName,
    pagination::PageLimit,
};

// Customer names allow at most 256 characters.
let name = CustomerIndividualName::try_new("Ada Lovelace")?;
assert!(CustomerIndividualName::try_new("a".repeat(257)).is_err());

// Page limits must be between 1 and 100 inclusive.
let limit = PageLimit::try_from(25_u8)?;
assert!(PageLimit::try_from(0_u8).is_err());
assert!(PageLimit::try_from(101_u8).is_err());

// Extract the underlying values when needed.
let name_text: String = name.into_inner();
let page_size: u8 = limit.into();
```

Pass these wrappers directly to typed fields and setters, such as `.name(name)`
or `ProductsListParams { limit: Some(limit), ..Default::default() }`, before
consuming them to extract their underlying values.

Pass query parameters through the operation's typed parameter struct using
`.params(...)`; the SDK encodes them into the request URL automatically. For
example, inside an async function with a client configured with a compatible
transport:

```rust
use polar_rsh::v2026_10::models::products::ProductsListParams;

let parameters = ProductsListParams {
    query: Some("subscription".to_owned()),
    is_archived: Some(false),
    ..Default::default()
};

let products = client.products().list().params(parameters).await?;
```

Array filters use vectors, even when filtering on a single value.
Pagination query parameters are typed. For example, `ProductsListParams::page`
is an `Option<std::num::NonZeroU64>`: the unsigned integer cannot represent
negative values, and the nonzero type also excludes zero. Use
`page: std::num::NonZeroU64::new(1)` for the first page; `NonZeroU64::new(0)`
returns `None`, which would omit the query parameter. The `limit` field uses
`Option<PageLimit>` to enforce the inclusive range 1–100.

Unsigned integer types represent values that cannot be below zero; nonzero
unsigned types additionally require a strictly positive value. These types
express numeric constraints directly instead of accepting arbitrary signed
integers.

List operations support `.pages()` and `.items()` streams as well as fetching
one page directly. Query encoding omits absent values and empty collections while
preserving false, zero metadata values, and empty strings.

### Sorting

The `sorting` field is a vector of `(property, Sorting)` pairs. Use the endpoint's
property enum, such as `ProductSortProperty`, with `Sorting::Asc` or
`Sorting::Desc`. Multiple criteria are applied in vector order: the first is the
primary sort, and subsequent criteria break ties.

```rust
use polar_rsh::v2026_10::{
    models::products::{ProductSortProperty, ProductsListParams},
    query::Sorting,
};

let parameters = ProductsListParams {
    sorting: vec![
        (ProductSortProperty::Name, Sorting::Asc),
        (ProductSortProperty::CreatedAt, Sorting::Desc),
    ],
    ..Default::default()
};

let products = client.products().list().params(parameters).await?;
```

This sorts by name ascending, then by creation time descending for matching
names. The SDK encodes the criteria as
`sorting=name&sorting=-created_at`; do not add the `-` prefix yourself.
Product sorting also supports `PriceAmountType` and `PriceAmount`. Other
endpoints expose their own sort-property enums. An empty `sorting` vector omits
the parameter and leaves ordering to the API's default.

## Creating request bodies

Pass a typed body directly to operations such as `.create(body)` and
`.update(id, body)`. The SDK serializes it as JSON; you do not need to construct
JSON strings or HTTP request bodies yourself.

For models with required fields, call `Model::new(ModelRequired { ... })`, then
chain setters for optional fields. The constructor initializes omitted optional
fields to `None` or `Presence::Absent`. Validated scalar wrappers expose
`try_new(...)`, which returns an error when the value violates schema constraints.

For example, create an individual customer using an organization access token:

```rust
use polar_rsh::v2026_10::models::customers::{
    CustomerCreate, CustomerIndividualCreate, CustomerIndividualCreateRequired,
    CustomerIndividualName,
};

let body = CustomerIndividualCreate::new(CustomerIndividualCreateRequired {
    email: "ada@example.com".to_owned(),
})
.name(CustomerIndividualName::try_new("Ada Lovelace")?)
.external_id("customer-123");

let customer = client
    .customers()
    .create(CustomerCreate::CustomerIndividualCreate(body))
    .await?;
```

`CustomerCreate` selects the individual or team payload variant. Without an
organization access token, also set `.organization_id(...)` on the body with an
`OrganizationId`. As with the query examples, these calls belong inside an async
function using a client configured with a compatible transport.

Models with no required fields, such as `CustomerUpdate`, can start from
`Default::default()` or `new()`. Their public fields also support struct literals.
Optional nullable body fields use `presence_rs::Presence<T>`:

| Value                   | JSON behavior      |
| ----------------------- | ------------------ |
| `Presence::Absent`      | Omit the field     |
| `Presence::Null`        | Send explicit null |
| `Presence::Some(value)` | Send the value     |

The `presence` feature re-exports `polar_rsh::Presence` and
`polar_rsh::presence_rs`. Every API-version feature enables it automatically;
it can also be enabled independently.

For example, given an existing customer's `CustomerId` in `customer_id`, clear
their name while leaving other fields unspecified:

```rust
use polar_rsh::{Presence, v2026_10::models::customers::CustomerUpdate};

let body = CustomerUpdate {
    name: Presence::Null,
    ..Default::default()
};

let updated = client.customers().update(customer_id, body).await?;
```

Optional-field setters supply a value; to send explicit JSON `null`, assign `Presence::Null`
to the public field. Leaving a field as `Presence::Absent` omits it entirely.

## Webhooks

Webhook types and verification live under `models::webhooks`.
`verification::validate_event(body, headers, secret)` accepts the original body
bytes and an `http::HeaderMap`, verifies the signature, and returns a typed
`events::WebhookPayload`. Keep the raw body intact until verification completes.
Track processed webhook IDs in your application to handle duplicate deliveries.

## Roadmap

This list is non-exhaustive.

- [x] Generate the Rust SDK automatically using generators and patches.
- [x] Generate a better Polar-sh SDK.
- [ ] Add chrono conditional compilation.
- [ ] Improve serde deserialization for complex union types.
- [ ] Add test generation for testing.
- [ ] Improve the conformance test suite.

## Development

See the [SDK generator guide](tasks/sdkgen/README.md) for generation and validation
commands. API conformance testing is deferred; current validation covers
generation, Rust tests, formatting, linting, and feature compilation. The workspace toolchain is pinned in
[`rust-toolchain.toml`](rust-toolchain.toml).

Install [cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/)
before running Rust tests. Use `just test --all-features` for unit and integration
tests and `just test-docs --all-features` for documentation tests. GitHub Actions
installs nextest automatically.
