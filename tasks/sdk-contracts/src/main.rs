mod adapters;
mod model;
mod server;
mod upstream;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    process::Stdio,
    time::Duration,
};

use anyhow::{Context, Result, bail, ensure};
use clap::Parser;
use model::{CapturedRequest, CaseReport, Outcome, Report, Response, Scenario, TypescriptInput};
use tokio::{io::AsyncWriteExt, process::Command};

#[derive(Parser)]
#[command(about = "Compare Rust REST contracts with pinned official Polar TypeScript sources")]
struct Args {
    #[arg(long)]
    api_version: String,
    #[arg(long)]
    case: Option<String>,
    #[arg(long, default_value = "node")]
    node: String,
}

async fn typescript(
    node: &str,
    api_version: &str,
    source: &Path,
    scenario: &Scenario,
    base_url: &str,
) -> Result<Vec<Outcome>> {
    let mut child = Command::new(node)
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/typescript.mjs"))
        .arg(source)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("Start Node.js 24+")?;
    let mut stdin = child.stdin.take().unwrap();
    stdin
        .write_all(&serde_json::to_vec(&TypescriptInput { scenario, base_url, api_version })?)
        .await?;
    drop(stdin);
    let output = tokio::time::timeout(Duration::from_secs(30), child.wait_with_output()).await??;
    ensure!(
        output.status.success(),
        "TypeScript failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).context("Read TypeScript outcomes")
}

fn compare(
    scenario: &Scenario,
    rust_requests: &[CapturedRequest],
    ts_requests: &[CapturedRequest],
    rust: &[Outcome],
    ts: &[Outcome],
) -> Result<()> {
    ensure!(
        rust == scenario.expected,
        "Rust outcome differs from fixture: expected {:?}, got {:?}",
        scenario.expected,
        rust
    );
    ensure!(
        ts == scenario.expected,
        "TypeScript outcome differs from fixture: expected {:?}, got {:?}",
        scenario.expected,
        ts
    );
    let mut adjusted = ts_requests.to_vec();
    if scenario.known_default_query {
        let rust_first = rust_requests.first().context("Missing Rust default-query request")?;
        let ts_first = adjusted.first_mut().context("Missing TypeScript default-query request")?;
        ensure!(rust_first.query.is_empty(), "Rust default query changed");
        let defaults = BTreeMap::from([
            ("page".into(), vec!["1".into()]),
            ("limit".into(), vec!["10".into()]),
            ("sorting".into(), vec!["-created_at".into()]),
        ]);
        ensure!(ts_first.query == defaults, "TypeScript defaults changed");
        ts_first.query.clear();
    }
    ensure!(
        rust_requests == adjusted,
        "Request mismatch:\nRust: {rust_requests:?}\nTypeScript: {ts_requests:?}"
    );
    Ok(())
}

fn validate_coverage(scenarios: &[Scenario]) -> Result<()> {
    let baseline: Vec<Scenario> = serde_json::from_str(include_str!("../scenarios.json"))?;
    let required: BTreeSet<_> = baseline.iter().map(|scenario| scenario.name.as_str()).collect();
    let actual: BTreeSet<_> = scenarios.iter().map(|scenario| scenario.name.as_str()).collect();
    ensure!(actual.len() == scenarios.len(), "Duplicate contract scenario names");
    ensure!(
        required.is_subset(&actual),
        "Version fixtures omit required scenarios: {:?}",
        required.difference(&actual).collect::<Vec<_>>()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize()?;
    let tracked =
        sdkgen::tracking::decode_tracking(&std::fs::read(root.join("sdkgen.lock.json"))?)?;
    ensure!(
        tracked.versions.contains_key(&args.api_version),
        "API {} has not been discovered",
        args.api_version
    );
    if !adapters::available(&args.api_version) {
        let status = std::process::Command::new("cargo")
            .current_dir(&root)
            .args([
                "run",
                "--locked",
                "-p",
                "sdk-contracts",
                "--no-default-features",
                "--features",
                &args.api_version,
                "--",
            ])
            .args(std::env::args().skip(1))
            .status()?;
        ensure!(status.success(), "Conformance failed for {}", args.api_version);
        return Ok(());
    }
    let (source, revision) = upstream::load(&root)?;
    let override_path =
        root.join(format!("tasks/sdk-contracts/scenarios/{}.json", args.api_version));
    let fixtures = if override_path.exists() {
        std::fs::read_to_string(override_path)?
    } else {
        include_str!("../scenarios.json").to_owned()
    };
    let scenarios: Vec<Scenario> = serde_json::from_str(&fixtures)?;
    validate_coverage(&scenarios)?;
    let mut reports = Vec::new();
    let mut failures = 0;
    for scenario in
        scenarios.iter().filter(|s| args.case.as_ref().is_none_or(|q| s.name.contains(q)))
    {
        ensure!(
            scenario.actions.len() == scenario.responses.len(),
            "One fixture response required per action"
        );
        let mut report = CaseReport {
            name: scenario.name.clone(),
            known_default_query: scenario.known_default_query,
            ..Default::default()
        };
        let result: Result<()> = async {
            let rust_server = server::Server::start(scenario.responses.clone()).await?;
            let rust_result = adapters::run(&args.api_version, scenario, &rust_server.url).await;
            let rust_requests = rust_server.finish().await;
            let rust = rust_result?;
            report.rust_outcomes = Some(rust.clone());
            let rust_requests = rust_requests?;
            report.rust_requests = Some(rust_requests.clone());
            let ts_server = server::Server::start(scenario.responses.clone()).await?;
            let ts_result =
                typescript(&args.node, &args.api_version, &source, scenario, &ts_server.url).await;
            let ts_requests = ts_server.finish().await;
            let ts = ts_result?;
            report.typescript_outcomes = Some(ts.clone());
            let ts_requests = ts_requests?;
            report.typescript_requests = Some(ts_requests.clone());
            compare(scenario, &rust_requests, &ts_requests, &rust, &ts)
        }
        .await;
        match result {
            Ok(()) => {
                report.passed = true;
                println!(
                    "PASS {}{}",
                    scenario.name,
                    if scenario.known_default_query {
                        " (documented default-query difference)"
                    } else {
                        ""
                    }
                );
            }
            Err(error) => {
                failures += 1;
                report.passed = false;
                report.error = Some(format!("{error:#}"));
                eprintln!("FAIL {}: {error:#}", scenario.name);
            }
        }
        reports.push(report);
    }
    ensure!(!reports.is_empty(), "No matching scenarios");
    let path = root.join(format!("target/sdk-contracts/{}/report.json", args.api_version));
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(&Report {
            upstream_revision: &revision,
            api_version: &args.api_version,
            cases: &reports,
        })?,
    )?;
    println!("{} cases, {failures} failures; {}", reports.len(), path.display());
    if failures > 0 {
        bail!("REST contracts failed");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn version_fixtures_cannot_silently_reduce_coverage() {
        let mut scenarios: Vec<Scenario> =
            serde_json::from_str(include_str!("../scenarios.json")).unwrap();
        validate_coverage(&scenarios).unwrap();
        let last = scenarios.pop().unwrap();
        assert!(validate_coverage(&scenarios).is_err());
        scenarios.push(last.clone());
        scenarios.push(last);
        assert!(validate_coverage(&scenarios).is_err());
    }

    fn defaults_case() -> Scenario {
        serde_json::from_str::<Vec<Scenario>>(include_str!("../scenarios.json"))
            .unwrap()
            .into_iter()
            .find(|case| case.known_default_query)
            .unwrap()
    }

    #[test]
    fn known_defaults_allow_only_the_recorded_query_difference() {
        let scenario = defaults_case();
        let rust: Vec<CapturedRequest> = serde_json::from_value(
            json!([{"query":{},"method":"GET","path":[],"headers":{},"body":null}]),
        )
        .unwrap();
        let mut ts: Vec<CapturedRequest> = serde_json::from_value(json!([{"query":{"page":["1"],"limit":["10"],"sorting":["-created_at"]},"method":"GET","path":[],"headers":{},"body":null}])).unwrap();
        assert!(compare(&scenario, &rust, &ts, &scenario.expected, &scenario.expected).is_ok());
        ts[0].query.insert("limit".into(), vec!["20".into()]);
        assert!(compare(&scenario, &rust, &ts, &scenario.expected, &scenario.expected).is_err());
        ts[0].query.insert("limit".into(), vec!["10".into()]);
        ts[0].method = "POST".into();
        assert!(compare(&scenario, &rust, &ts, &scenario.expected, &scenario.expected).is_err());
    }

    #[test]
    fn matching_but_incorrect_outcomes_fail() {
        let mut scenario = defaults_case();
        scenario.known_default_query = false;
        assert!(compare(&scenario, &[], &[], &[], &[]).is_err());
    }
}
