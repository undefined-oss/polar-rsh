# SDK generator

`sdkgen` discovers Polar's versioned OpenAPI specifications, generates Rust SDK
modules inside `polar-rsh`, and validates them against the pinned Polar reference. It is an
unpublished workspace task.

## Setup

Run these commands from the repository root:

```sh
rustup show
git submodule update --init --depth 1 -- vendor/polar
cargo install --locked openapi-to-rust --version 0.22.0
cargo install --locked cargo-nextest
cargo sdkgen
```

Use the Rust toolchain in [`rust-toolchain.toml`](../../rust-toolchain.toml),
including its `rustfmt` and Clippy components. Generation requires exactly
`openapi-to-rust 0.22.0`. REST contract validation also requires Node.js 24 or
newer and permission to open local HTTP sockets.

The `vendor/polar` checkout must be clean and match the submodule commit in the
parent Git index. After switching branches, run the submodule update command
again to restore that branch's reference.

## Commands

The `cargo sdkgen` alias runs `cargo run -p sdkgen --`. Arguments follow the
alias directly.

| Command                                     | Purpose                                                                                                 |
| ------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `cargo sdkgen`                              | Discover all specifications, generate SDKs, and run required validation                                 |
| `cargo sdkgen --stage discover`             | Record specifications, scaffold version modules, and register workspace integrations                    |
| `cargo sdkgen --api-version 2026-10`        | Generate and validate a selected API version                                                            |
| `cargo sdkgen --check`                      | Check source tracking, freshly rendered output, and recorded validation without modifying the workspace |
| `cargo nextest run --locked -p sdkgen`      | Run generator unit tests                                                                                |
| `cargo sdk-contracts --api-version 2026-10` | Run REST contract comparisons for a selected version                                                    |

Discovery always considers every specification, including when
`--api-version` limits generation and validation. `--check` requires current
validation records for all tracked versions; run `cargo sdkgen` when those
records are stale.

The root [`justfile`](../../justfile) provides corresponding recipes:
`just submodules`, `just install-generator`, `just sdk-discover`,
`just sdk-generate`, `just sdkgen-check`, and `just contracts --api-version 2026-10`.

## Inputs and generated files

Specifications come from `vendor/polar/docs/openapi/*.openapi.json`. Each
filename supplies a `YYYY-MM` API version, which maps to a feature-gated module
inside `polar-rsh`.

| Path                                                                 | Role                                                                             |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `vendor/polar/docs/openapi/`                                         | Pinned source specifications                                                     |
| [`templates/version/`](templates/version/)                           | Client wrappers, webhook verification, and tests                                 |
| [`templates/resources.rs.hbs`](templates/resources.rs.hbs)           | Resource accessors and request methods                                           |
| [`templates/webhook_events.rs.hbs`](templates/webhook_events.rs.hbs) | Typed webhook event dispatch                                                     |
| [`templates/query.rs`](templates/query.rs)                           | Shared query encoder                                                             |
| `apps/polar-rsh/src/vYYYY_MM/`                                       | Generated module, source specification copy, query overlay, and binding metadata |
| [`sdkgen.lock.json`](../../sdkgen.lock.json)                         | Source inventory and generation/validation state                                 |

Discovery exposes API-version features and modules in `polar-rsh`, and adds
contract-test adapters. For example, API `2026-10` is available through the `2026-10` feature and
`polar_rsh::v2026_10` module.

`polar-rsh` is the only published package. All version
modules ship inside it; only enabled versions are compiled. Runtime dependencies
are maintained centrally in `apps/polar-rsh/Cargo.toml`. Discovery wires the
optional dependencies and version-specific test targets into each version feature.

Edit templates or generator code to change generated SDK behavior. Generation
renders selected modules in temporary directories, formats the output, and
compares it with managed files before applying changes. It restores modified or
missing managed files and removes obsolete managed output. Unmanaged files are
preserved. A missing tracked source specification fails discovery.

## Generation and tracking

OpenAPI specifications, binding metadata, and tracking records use typed Serde
models. Generation normalizes query schemas, runs `openapi-to-rust`, and applies
Rust syntax transformations for resource methods, query encoding, and optional
nullable fields.

The query encoder omits nulls and empty collections while preserving false,
zero, and empty strings. Optional nullable JSON body fields use
`presence_rs::Presence<T>` to distinguish absent, null, and supplied values.
Webhook dispatch uses typed event payloads.

`sdkgen.lock.json` records the upstream revision, source paths, API versions,
managed file paths, and these source-content hashes:

| Field            | Meaning                                               |
| ---------------- | ----------------------------------------------------- |
| `observed_hash`  | SHA-256 of the discovered specification's exact bytes |
| `generated_hash` | Source hash used for successful generation            |
| `validated_hash` | Source hash that passed validation                    |

Each version has a `pending`, `generated`, or `validated` status. Generator and
validation fingerprints record the relevant tooling inputs. Changes to source
specifications, templates, dependencies, contract fixtures, or the upstream pin
can require generation or validation again. Successful runs with unchanged
inputs and output leave the tracking file unchanged.

## Validation

The generation pipeline checks the workspace and, for versions requiring
validation, runs generator and contract-harness tests, formatting, Clippy, SDK
version tests, feature checks, and Rust/TypeScript REST comparisons. Rust unit
and integration tests use nextest; documentation tests use `cargo test --doc`.
Validation also verifies that its inputs and managed output remain unchanged
during the run. Failed validation does not mark a version as validated.

The REST suite runs 18 scenarios per API version against local HTTP servers.
It compares the generated Rust SDK with Polar's TypeScript services from the
pinned submodule. Reports are written to
`target/sdk-contracts/<version>/report.json`. See the
[contract task documentation](../sdk-contracts/README.md) for coverage and limits.
SDK tests also exercise webhook signatures locally. These checks need no live
Polar API credentials.

After changing generator code or templates, run:

```sh
cargo nextest run --locked -p sdkgen
cargo sdkgen
cargo sdkgen --check
```

## Automated SDK updates

The [SDK update workflow](../../.github/workflows/sdk-update.yml) runs daily at
06:00 UTC and supports manual dispatch. It checks out the default branch,
saves the tracking inventory, advances `vendor/polar` to upstream `main`, stages
the submodule pin, and runs generation and validation.

The separate [classification task](../sdk-classify/README.md) compares API
versions and their `observed_hash` values before and after the update. The
workflow uses its result to prepare the commit message and PR body:

| Classification                                                       | Message and PR body                                                                   | Merge behavior                              |
| -------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------- |
| `chore`: API versions and content hashes match                       | Fixed `chore(sdk): update Polar submodule` message and standard submodule-update body | Automatically squash-merge after validation |
| `feat`: a content hash changes or an API version is added or removed | AI-generated subject and SDK-update body                                              | Leave open for review                       |

For API content changes, the [commit message task](../commit-message/README.md)
uses AI to generate a commit subject from a bounded summary of file statistics,
tracking changes, and new filenames. It accepts any Conventional Commit type
except `chore`; the classification controls merge behavior independently of the
generated subject.
The subject becomes both the commit message and PR title.

`create-pull-request` creates or updates `automation/sdk-update`. A completely
unchanged checkout creates no PR, and obsolete automation PRs are cleaned up.
Automatic merges require the expected PR head SHA and remain subject to GitHub
branch protection. A blocked merge leaves the PR open and fails the merge step.
Contract reports are uploaded even when the workflow fails.

Repository configuration:

- Allow GitHub Actions to create pull requests.
- Configure credentials and model settings as described in the
  [commit message task documentation](../commit-message/README.md).

The update workflow uses `GITHUB_TOKEN`, so ordinary PR workflows are not
triggered by its PR creation. Validation runs before the PR step. The
[REST contract workflow](../../.github/workflows/sdk-contracts.yml) also runs
on ordinary pull requests and pushes to `main`.

## Releases

Publishing is handled by the [release-plz workflow](../../.github/workflows/release-plz.yml)
and [`release-plz.toml`](../../release-plz.toml). The workflow runs on pushes to
`main` and manual dispatch in `undefined-oss/polar-rsh`. It prepares release PRs
with version and changelog updates, tests the workspace before publication,
and uses crates.io trusted publishing through the release job's OIDC permission.
`release_always = false` configures publication after release PRs are merged.

`polar-rsh` inherits the workspace version. Development tasks are unpublished.
The generator leaves the package version and changelog to release-plz. Release
changes to workspace manifests or the lockfile invalidate validation fingerprints;
the next SDK generation run refreshes them.
