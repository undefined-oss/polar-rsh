# polar-rsh

An async Rust SDK for [Polar](https://polar.sh). It aims to stay as close as
possible to Polar's official [TypeScript](https://polar.sh/docs/integrate/sdk/typescript)
and [Python](https://polar.sh/docs/integrate/sdk/python) SDKs in resource
organization, API behavior, and version selection, while using Rust's types and
async request patterns.

> [!IMPORTANT]
> **Beta:** the SDK is under active development. Its Rust API may change between
> releases. Compatibility checks compare selected REST scenarios with the official
> TypeScript SDK; they do not establish complete parity with either official SDK.

## Installation

> [!IMPORTANT]
> No API version is enabled by default.

Choose an API version explicitly. For a published `0.1` release, add:

```toml
[dependencies]
polar-rsh = { version = "0.1", features = ["2026-10"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
```

The examples below use API `2026-10` and a Tokio runtime with time enabled.
Store your Polar access token in the `POLAR_ACCESS_TOKEN` environment variable.

## Choose sandbox or production

Select the environment when building the client with `.environment(...)`:

| Setting | API URL |
| --- | --- |
| `Environment::Sandbox` | `https://sandbox-api.polar.sh` |
| `Environment::Production` | `https://api.polar.sh` |

**Production is the default**, including when you use `Polar::new(token)`.
Use a token issued for the selected environment; sandbox and production tokens
are separate. The API-version Cargo feature selects the API schema, while
`.environment(...)` selects the server.

The quick start below uses sandbox. Change `Environment::Sandbox` to
`Environment::Production` and supply your production token to use production.

## Quick start

```rust
use polar_rsh::v2026_10::{Environment, Polar, resources::ProductsListParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let polar = Polar::builder(std::env::var("POLAR_ACCESS_TOKEN")?)
        .environment(Environment::Sandbox)
        .build()?;

    let page = polar
        .products()
        .list(ProductsListParams {
            limit: Some(20),
            is_archived: Some(false),
            ..Default::default()
        })
        .await?;

    for product in page.items {
        println!("{}: {}", product.id, product.name);
    }

    Ok(())
}
```

Resource methods expose typed models and parameters through the selected
version's `models` and `resources` modules. Optional method arguments are grouped
in `*Params` structs; required arguments are passed directly.

## API version features

All supported API versions are bundled as modules in the `polar-rsh` crate.
Only the versions enabled through features are compiled.

The Cargo feature name is the API version. For example, `2026-10` enables
`polar_rsh::v2026_10`.

Each client sends the corresponding `Polar-Version` header. The crate release
version, such as `0.1`, is separate from the Polar API version.

You can enable multiple API versions in one application:

```toml
polar-rsh = { version = "0.1", features = ["2026-04", "2026-10"] }
```

```rust
use polar_rsh::{v2026_04, v2026_10};

fn versioned_clients(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _april = v2026_04::Polar::new(token)?;
    let _october = v2026_10::Polar::new(token)?;
    Ok(())
}
```

Use models and parameter types from the same version module as the client.

## Request options

Set a client-wide timeout on the builder and override it for individual requests:

```rust
use std::time::Duration;
use polar_rsh::v2026_10::{Environment, Polar, resources::ProductsListParams};

async fn sandbox_products() -> Result<(), Box<dyn std::error::Error>> {
    let polar = Polar::builder(std::env::var("POLAR_ACCESS_TOKEN")?)
        .environment(Environment::Sandbox)
        .timeout(Duration::from_secs(30))
        .build()?;

    let page = polar
        .products()
        .list(ProductsListParams::default())
        .timeout(Duration::from_secs(60))
        .await?;

    println!("{} products", page.items.len());
    Ok(())
}
```

Requests are lazy: an HTTP call starts when you `.await` the request or call
`.send().await`. The default deadline is five seconds, including reading the
response body. Per-request `.timeout(...)`, `.access_token(...)`, and
`.options(RequestOptions { ... })` apply to that request only.

The builder also supports an organization ID, a custom base URL, a custom
`reqwest::Client`, and tracing configuration. A custom transport's timeout
remains an independent limit. `polar.raw()` exposes the generated low-level
client and bypasses the resource wrapper's deadline and per-request options.

Timeouts return `RequestError::Timeout`; API and transport failures are exposed
through `RequestError::Api`, preserving the generated operation error type.
Automatic retries are not configured.

## Pagination

List methods return one page. Request subsequent pages explicitly using the
response's pagination information:

```rust
use polar_rsh::v2026_10::{Polar, resources::ProductsListParams};

async fn list_all_products(polar: &Polar) -> Result<(), Box<dyn std::error::Error>> {
    let mut page_number = 1;
    loop {
        let page = polar
            .products()
            .list(ProductsListParams {
                page: Some(page_number),
                limit: Some(20),
                ..Default::default()
            })
            .await?;

        for product in page.items {
            println!("{}: {}", product.id, product.name);
        }
        if page_number >= page.pagination.max_page {
            break;
        }
        page_number += 1;
    }
    Ok(())
}
```

Pagination fields depend on the endpoint. Array filters use vectors, including
for a single value. UUIDs, timestamps, and URLs use `uuid::Uuid`,
`chrono::DateTime<chrono::Utc>`, and `url::Url` respectively.

## Optional and nullable fields

The `presence` feature re-exports the `presence-rs` crate as
`polar_rsh::presence_rs` and its `Presence` type as `polar_rsh::Presence`.
Every API-version feature enables `presence` automatically. It can also be
enabled independently:

```toml
polar-rsh = { version = "0.1", features = ["presence"] }
```

Optional nullable model fields use `Presence<T>` to distinguish three JSON
states:

| Rust value | JSON body |
| --- | --- |
| `Presence::Absent` | Omit the field; this is the default |
| `Presence::Null` | Send explicit `null` |
| `Presence::Some(value)` | Send the supplied value |

For example, clear a customer's name while leaving other fields unspecified:

```rust
use polar_rsh::{Presence, v2026_10::{Polar, models::CustomerUpdate}};

async fn clear_customer_name(
    polar: &Polar,
    customer_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let customer = polar
        .customers()
        .update(
            customer_id.parse()?,
            CustomerUpdate {
                name: Presence::Null,
                ..Default::default()
            },
        )
        .await?;

    println!("Updated customer: {customer:?}");
    Ok(())
}
```

Use `Presence::Some("Ada".to_owned())` to set the name. Ordinary optional fields
and required nullable fields use `Option<T>`. Query encoding omits nulls and
empty collections while preserving `false`, `0`, and empty strings.

## Webhooks

The selected API module exposes `webhooks::validate_event(body, headers, secret)`.
Pass the original request bytes and a `reqwest::header::HeaderMap`; the verifier
checks the signature before returning a typed `WebhookEvent`.

If your application imports `reqwest` headers directly, add
`reqwest = { version = "0.13", default-features = false }` to its dependencies.

```rust
use polar_rsh::v2026_10::webhooks::{self, WebhookError, WebhookEvent};
use reqwest::header::HeaderMap;

fn verify_webhook(
    body: &[u8],
    headers: &HeaderMap,
    secret: &str,
) -> Result<WebhookEvent, WebhookError> {
    webhooks::validate_event(body, headers, secret)
}
```

Use the webhook signing secret for verification. Keep the raw body intact until
verification completes, and track processed `webhook-id` values in your
application to handle duplicate deliveries.

## Roadmap

This list is non-exhaustive.

- [x] Generate the Rust SDK automatically.
- [ ] Improve the conformance test suite.

## Development

See the [SDK generator guide](tasks/sdkgen/README.md) for generation and validation
commands, and the [REST contract guide](tasks/sdk-contracts/README.md) for the
cross-language compatibility checks. The workspace toolchain is pinned in
[`rust-toolchain.toml`](rust-toolchain.toml).
