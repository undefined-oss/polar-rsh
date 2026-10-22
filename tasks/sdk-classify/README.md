# SDK classification task

Compare API-version/content-hash maps from two `sdkgen.lock.json` inventories
using typed Serde structs. This unpublished Rust task requires no API key.

```sh
cargo run --locked -p sdk-classify -- before.json sdkgen.lock.json
```

The task returns `chore` when all API versions and their `observed_hash` values
match, or `feat` when a hash changes or a version is added or removed. Changes to
the upstream revision or validation fingerprints alone are chores. Missing or
malformed inventories fail classification.

The result goes to stdout and, when `GITHUB_OUTPUT` is set, the `kind` step
output. The SDK update workflow uses this result as `COMMIT_TYPE` to choose
between `chore(sdk): update Polar submodule` with a standard submodule-update
PR body, or `feat(sdk): update generated Polar SDKs` with an SDK-update PR body.
Both messages are fixed and require no external AI service.

After successful SDK validation, the workflow automatically squash-merges
chore PRs based on this classification. It uses GitHub's
merge API with the PR head SHA, so a changed head or unmet branch protection
requirements prevent the merge. PRs with changed API content remain open for
review. A completely unchanged checkout creates no PR.
