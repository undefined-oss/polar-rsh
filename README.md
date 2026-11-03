# polar-rsh

An async Rust SDK for [Polar](https://polar.sh), generated from the pinned
versioned OpenAPI specifications. The SDK is under active development; the
native generator introduces a new Tower-based client API.

## API versions

No API version is enabled by default. Enable one or more version features:

```toml
polar-rsh = { version = "0.2", features = ["2026-10"] }
```

The feature `2026-10` exposes `polar_rsh::v2026_10`. Multiple versions can coexist,
and each client sends its corresponding `Polar-Version` header. Use models and
operations from the same version module as the client.

## Client and transport

The generated client accepts a caller-supplied Tower service. The service receives
`http::Request<RequestBody>` and returns an `http::Response` whose body implements
`http_body::Body<Data = bytes::Bytes>`. The client keeps the service's concrete
error type in operation errors.

```rust
use polar_rsh::v2026_10::client::{AccessToken, PolarClient, PolarEnvironment};

// Supply your Tower HTTP transport in place of `()` to send requests.
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

Models are organized by resource under `models`. UUID identifiers use distinct
newtypes; timestamps use `jiff`, and URLs use `url::Url`. Validated scalar types
reject values outside the schema's constraints.

Array filters use vectors. Sorting uses `(property, query::Sorting)` pairs.
List queries use positive page numbers and a validated `PageLimit`.
List operations support `.pages()` and `.items()` streams as well as fetching
one page directly. Query encoding omits absent values and empty collections while
preserving false, zero metadata values, and empty strings.

Optional nullable body fields use `presence_rs::Presence<T>`:

| Value | JSON behavior |
| --- | --- |
| `Presence::Absent` | Omit the field |
| `Presence::Null` | Send explicit null |
| `Presence::Some(value)` | Send the value |

The `presence` feature re-exports `polar_rsh::Presence` and
`polar_rsh::presence_rs`. Every API-version feature enables it automatically;
it can also be enabled independently.

## Webhooks

Webhook types and verification live under `models::webhooks`.
`verification::validate_event(body, headers, secret)` accepts the original body
bytes and an `http::HeaderMap`, verifies the signature, and returns a typed
`events::WebhookPayload`. Keep the raw body intact until verification completes.
Track processed webhook IDs in your application to handle duplicate deliveries.

## Development

See the [SDK generator guide](tasks/sdkgen/README.md) for generation and validation
commands. API conformance testing is deferred; current validation covers
generation, Rust tests, formatting, linting, and feature compilation. The workspace toolchain is pinned in
[`rust-toolchain.toml`](rust-toolchain.toml).

Install [cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/)
before running Rust tests. Use `just test --all-features` for unit and integration
tests and `just test-docs --all-features` for documentation tests. GitHub Actions
installs nextest automatically.
