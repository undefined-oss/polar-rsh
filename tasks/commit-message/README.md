# Commit message task

Generate a Conventional Commit subject from a change summary using
[Google Gemini's native HTTP API](https://ai.google.dev/api/generate-content).
This unpublished Rust task uses the existing workspace dependency versions of
`reqwest` and Serde, with typed request/response structs and no LLM SDK.

```sh
# Set GEMINI_API_KEY in the environment first.
cargo run --locked -p commit-message < change-summary.txt
```

The workflow uses the separate [SDK classification task](../sdk-classify/README.md)
to compare API content hashes. Chore updates use a fixed commit message and
standard submodule-update PR body without calling Gemini. For content changes,
this task generates a subject with any Conventional Commit type except `chore`.
It does not read `COMMIT_TYPE` or enforce a fixed type or scope.

`GEMINI_MODEL` defaults to `gemini-flash-latest`. The task calls `generateContent`
with the API key in the `x-goog-api-key` header. Input is capped at
32 KiB; oversized summaries are truncated with an explicit notice. The request
has a 60-second timeout and a 2,048-token output budget, including model thinking.
The final subject is limited to 100 characters. Empty, malformed,
incomplete, multiline, oversized, chore, and non-Conventional Commit responses fail
the task. Successful subjects go to stdout and, when `GITHUB_OUTPUT` is set,
the `message` step output.

The SDK update workflow supplies file statistics, tracking changes (API
versions, source hashes, and upstream revision), and untracked filenames.
It skips the API call for chore updates and still runs `create-pull-request`
so obsolete automation PRs can be cleaned up. It does not send generated Rust
or complete OpenAPI documents. The summary is sent directly to Google.

Configure the `GEMINI_API_KEY` GitHub Actions secret and optionally the
`GEMINI_MODEL` repository variable. The task only generates text;
`create-pull-request` owns commits and PRs. API or response-validation failures
stop the workflow before PR creation.
