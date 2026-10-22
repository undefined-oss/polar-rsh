set positional-arguments

# List available commands.
default:
    @just --list

# Install the exact generator version required by sdkgen.
install-generator:
    cargo install --locked openapi-to-rust --version 0.22.0

# Initialize or restore Polar's pinned submodule commit.
submodules:
    git submodule update --init --depth 1 -- vendor/polar

# Build all workspace packages; accepts additional Cargo arguments.
build *args:
    cargo build --workspace --locked "$@"

# Check all workspace targets without producing binaries.
check *args:
    cargo check --workspace --all-targets --locked "$@"

# Format the workspace.
fmt:
    cargo fmt --all

# Check formatting without modifying files.
fmt-check:
    cargo fmt --all --check

# Run Clippy with warnings treated as errors.
lint *args:
    cargo clippy --workspace --all-targets --locked "$@" -- -D warnings

# Run workspace tests with nextest; accepts nextest arguments and test filters.
test *args:
    cargo nextest run --workspace --locked "$@"

# Run documentation tests; accepts Cargo arguments and test filters.
test-docs *args:
    cargo test --doc --workspace --locked "$@"

# Regenerate the SDK; e.g. just sdkgen --api-version 2026-04.
sdkgen *args:
    cargo sdkgen "$@"

# Verify generated files are current without modifying them.
sdkgen-check *args:
    cargo sdkgen --check "$@"

# Discover pinned specifications and create version modules/features.
sdk-discover *args:
    cargo sdkgen --stage discover "$@"

# Generate changed SDKs and run conformance.
sdk-generate *args:
    cargo sdkgen --stage generate "$@"

# Compare REST contracts; requires --api-version, Node.js 24+, and the submodule.
contracts *args:
    cargo sdk-contracts "$@"

# Run formatting, lint, tests, generation freshness, and REST contracts.
verify api_version: fmt-check lint test test-docs sdkgen-check (contracts "--api-version" api_version)
