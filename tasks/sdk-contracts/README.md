# Rust / TypeScript REST contracts

Run the same HTTP scenarios through our Rust SDK and Polar's official
TypeScript REST services for each discovered API version:

```sh
git submodule update --init --depth 1 -- vendor/polar
cargo sdk-contracts --api-version 2026-04
cargo sdk-contracts --api-version 2026-10 --case presence
cargo nextest run --locked -p sdk-contracts
```

The equivalent root recipes are `just submodules` and `just contracts --api-version 2026-04`.
Use `just contracts --api-version 2026-04 --case presence` to select scenarios, or `just verify 2026-04`
for the full workspace checks, including generator freshness and contracts.

Requires the workspace Rust toolchain and Node.js 24+. No npm installation,
Polar account, tokens, Docker, or live API access is needed. Initialize the
submodule once from the workspace root. Contract execution performs no downloads
and only makes local loopback HTTP connections. `--node /path/to/node` selects
a Node executable.

The reference is Polar commit
[`49d4c8c`](https://github.com/polarsource/polar/tree/49d4c8cbbe5436ee50a3d8dc04dec7182b9886c9),
the revision from the linked SDK workflow. The `vendor/polar` Git submodule
pins this commit through the parent repository's gitlink; `.gitmodules` records
the repository URL. `sdkgen.lock.json` records that revision alongside the
versioned source hashes and validation state; the gitlink remains authoritative.
The harness checks that the submodule is initialized, its HEAD matches the
gitlink in the parent index, and it has no tracked or untracked local changes.
An unstaged revision change fails instead of silently testing a different SDK.

Node's built-in TypeScript transformer loads the base client, product/customer
services, and error classes directly from the submodule; type-only imports are
erased. The driver invokes the real service functions and HTTP implementation,
without copying their serialization logic. Upstream's license remains in the
submodule.

`--api-version` is required and has no default. If its adapter is not compiled into the
current executable, the task rebuilds itself with that version's Cargo feature.
`sdkgen` manages version features and adapter modules. Each adapter includes the
same Rust request driver against its generated module exposed by `polar-rsh`; the TypeScript
driver loads the matching upstream service directory and version header.

`scenarios/<version>.json`, when present, supplies a complete version-specific
fixture set that must retain every shared scenario name without duplicates.
Otherwise the shared `scenarios.json` is used. Unsupported methods,
missing upstream services, and schema incompatibilities fail instead of skipping
coverage. Both clients use identical inputs and expected outcomes.

`scenarios.json` provides identical inputs, scripted HTTP responses, and expected
outcomes for both clients. Each client gets a fresh local server per scenario;
multiple actions within one scenario share a client to check override isolation.
The Rust adapter maps a scalar TypeScript filter to a one-element Rust vector
and deserializes update bodies into the real `Presence<T>` model fields.

Fixtures, action variants, query fields, outcomes, captured requests, and reports
use shared Serde models in `src/model.rs`. Malformed known fields fail while
loading fixtures. Query fields use `Presence<T>` to retain absent, null, scalar,
and array inputs when forwarding them to TypeScript. Version-specific API bodies
and arbitrary metadata values retain JSON values until the versioned SDK decodes
them. Tracking files use the same typed loader and legacy migration as `sdkgen`.

Coverage includes repeated UUID filters and sorting, scalar filters, empty and
null query values, bracketed metadata, escaping, false/zero/empty strings,
customer response decoding, external ID paths, omitted/null/value update bodies,
204 responses, organization headers, token and timeout overrides, typed 422
errors, 429 retry metadata, and 500 errors.

The comparison checks:

- HTTP method and path segments, preserving the distinction between encoded
  slashes and path separators.
- Decoded query names and values. Parameter ordering is ignored; the order of
  values for a repeated parameter is preserved, including sorting precedence.
- Authorization, API version, organization headers, and JSON body content type.
- JSON bodies, including missing fields versus explicit nulls.
- Expected list responses and selected customer fields, or normalized HTTP
  status/error details and timeouts. Rust's 422 error must also decode into its
  generated typed error model.

Transport headers such as User-Agent, Accept, Content-Length, and Connection
are excluded. TypeScript sends Content-Type even on bodyless GET requests;
the comparison only checks that header on requests with bodies. Customer
outcomes project `id`, `email`, `name`, `created_at`, and `organization_id` after
Rust deserializes the complete model. This does not establish parity for every
response field, invalid response bodies, retry policies, or all API operations.

There is one explicit allowed request difference: with an empty product-list
query, TypeScript sends `page=1`, `limit=10`, and `sorting=-created_at`; Rust sends
none of these and relies on server defaults. That case asserts the exact request
from each client. Changes to either side fail; other differences are not ignored.

Every run writes `target/sdk-contracts/<version>/report.json`, including canonical requests,
outcomes, failures, and upstream revision. Any unexpected mismatch exits with a
failure. `.github/workflows/sdk-contracts.yml` runs the suite and uploads the
report, including on failure. CI initializes the pinned submodule with
`actions/checkout`'s `submodules: true` option.

These are local HTTP contract tests, not backend E2E tests. Polar's linked
[`sdk/e2e`](https://github.com/polarsource/polar/tree/49d4c8cbbe5436ee50a3d8dc04dec7182b9886c9/sdk/e2e)
suite exercises Outpost metering, Redis, and reconciliation, which this REST
client does not implement. These tests do not validate actual server defaults,
authentication, database effects, payments, or webhooks against a running Polar
backend. The existing SDK tests continue to cover webhook verification locally.

To update the reference, choose and review an upstream commit with the matching
API version, then run from the workspace root:

```sh
git -C vendor/polar fetch --depth 1 origin <commit>
git -C vendor/polar checkout --detach <commit>
git add vendor/polar
cargo sdk-contracts --api-version 2026-04
```

Review failures and the report before changing fixtures or allowing new
differences. Commit the updated gitlink with any corresponding contract changes.
After switching parent branches, run `git submodule update --init --depth 1 --
vendor/polar` to restore that branch's pinned revision.
