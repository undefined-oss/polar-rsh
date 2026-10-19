use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, ensure};
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::{Parser, ValueEnum};
use hmac::{Hmac, Mac};
use sdkgen::tracking::{Tracking, Version, decode_tracking};
use sha2::{Digest, Sha256};
use toml_edit::{ArrayOfTables, DocumentMut, Table, value};

pub const TRACKING: &str = "sdkgen.lock.json";
pub const SOURCE: &str = "vendor/polar/docs/openapi";

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq, Eq)]
pub enum Stage {
    Discover,
    Generate,
}

#[derive(Parser)]
#[command(about = "Discover, generate, and validate versioned Polar SDKs")]
pub struct Cli {
    #[arg(long, value_enum, default_value = "generate")]
    pub stage: Stage,
    #[arg(long)]
    pub api_version: Option<String>,
    /// Verify source tracking, output freshness, and prior validation without
    /// writes.
    #[arg(long)]
    pub check: bool,
}

pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn file_hash(path: &Path) -> Result<String> {
    Ok(hash(&fs::read(path).with_context(|| format!("Read {}", path.display()))?))
}

fn git(root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").current_dir(root).args(args).output()?;
    ensure!(
        output.status.success(),
        "git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().into())
}

fn module_path(version: &str) -> String {
    format!("apps/polar-rsh/src/v{}", version.replace('-', "_"))
}

fn load(root: &Path) -> Result<Tracking> {
    let path = root.join(TRACKING);
    let tracking: Tracking =
        if path.exists() { decode_tracking(&fs::read(path)?)? } else { Tracking::default() };
    for (version, record) in &tracking.versions {
        ensure!(super::valid_version(version), "Invalid tracked API version");
        ensure!(
            record.module_path == module_path(version)
                && record.source == format!("{SOURCE}/{version}.openapi.json"),
            "Invalid tracked version paths"
        );
        for path in &record.files {
            ensure!(
                safe_path(path) && path.starts_with(&format!("{}/", record.module_path)),
                "Unsafe managed path: {path}"
            );
        }
    }
    for path in &tracking.shared {
        ensure!(SHARED.contains(&path.as_str()), "Unknown shared path: {path}");
    }
    Ok(tracking)
}

fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && Path::new(path).components().all(|part| matches!(part, std::path::Component::Normal(_)))
}

fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    if fs::read(path).ok().as_deref() == Some(bytes) {
        return Ok(());
    }
    fs::create_dir_all(path.parent().context("Missing parent directory")?)?;
    let mut temporary = tempfile::NamedTempFile::new_in(path.parent().unwrap())?;
    use std::io::Write;
    temporary.write_all(bytes)?;
    temporary.persist(path)?;
    Ok(())
}

fn save(root: &Path, tracking: &Tracking) -> Result<()> {
    write(&root.join(TRACKING), format!("{}\n", serde_json::to_string_pretty(tracking)?).as_bytes())
}

fn files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    if !root.exists() {
        return Ok(result);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        ensure!(
            !entry.file_type()?.is_symlink(),
            "Symlink in managed directory: {}",
            entry.path().display()
        );
        if entry.file_type()?.is_dir() {
            result.extend(files(&entry.path())?);
        } else {
            result.push(entry.path());
        }
    }
    result.sort();
    Ok(result)
}

fn specs(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(root.join(SOURCE))
        .context("Initialize the Polar submodule with just submodules")?
    {
        let path = entry?.path();
        let name = path.file_name().unwrap().to_str().context("Non-UTF8 spec filename")?;
        let Some(version) = name.strip_suffix(".openapi.json") else {
            continue;
        };
        ensure!(super::valid_version(version), "Invalid API spec filename: {name}");
        let bytes = fs::read(&path)?;
        let spec: crate::openapi::Document =
            serde_json::from_slice(&bytes).with_context(|| format!("Invalid spec {name}"))?;
        ensure!(spec.openapi.starts_with("3."), "Invalid OpenAPI document: {name}");
        result.insert(version.to_owned(), hash(&bytes));
    }
    ensure!(!result.is_empty(), "No versioned OpenAPI specifications found");
    Ok(result)
}

fn upstream(root: &Path) -> Result<String> {
    let source = root.join("vendor/polar");
    ensure!(source.join(".git").exists(), "Initialize the Polar submodule with just submodules");
    ensure!(
        git(&source, &["status", "--porcelain", "--untracked-files=all"])?.is_empty(),
        "Polar reference checkout has local changes"
    );
    git(&source, &["rev-parse", "HEAD"])
}

fn fingerprint(root: &Path, validation: bool) -> Result<String> {
    let mut paths = files(&root.join("tasks/sdkgen/src"))?;
    paths.extend(files(&root.join("tasks/sdkgen/templates"))?);
    for file in ["tasks/sdkgen/Cargo.toml", "rust-toolchain.toml", "rustfmt.toml"] {
        paths.push(root.join(file));
    }
    if validation {
        paths.extend(files(&root.join("tasks/sdk-contracts/src"))?);
        for file in [
            "Cargo.toml",
            "tasks/sdk-contracts/Cargo.toml",
            "tasks/sdk-contracts/typescript.mjs",
            "tasks/sdk-contracts/scenarios.json",
            "Cargo.lock",
            "apps/polar-rsh/src/lib.rs",
            "apps/polar-rsh/src/versions.rs",
            "apps/polar-rsh/Cargo.toml",
            "justfile",
        ] {
            paths.push(root.join(file));
        }
        paths.extend(files(&root.join("tasks/sdk-contracts/scenarios"))?);
    }
    paths.sort();
    let mut digest = Sha256::new();
    digest.update(super::GENERATOR_VERSION);
    for path in paths {
        digest.update(path.strip_prefix(root)?.to_string_lossy().as_bytes());
        digest.update([0]);
        digest.update(file_hash(&path)?);
    }
    if validation {
        digest.update(upstream(root)?);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn render_templates(root: &Path, version: &str) -> Result<BTreeMap<String, Vec<u8>>> {
    let templates = root.join("tasks/sdkgen/templates/version");
    let mut output = BTreeMap::new();
    for path in files(&templates)? {
        let relative = path
            .strip_prefix(&templates)?
            .to_str()
            .context("Invalid template path")?
            .strip_suffix(".hbs")
            .context("Version templates must end in .hbs")?;
        let content = fs::read_to_string(&path)?
            .replace("__API_VERSION__", version)
            .replace("__API_CRATE__", &version.replace('-', "_"));
        output.insert(relative.to_owned(), content.into_bytes());
    }
    let body = output
        .get("tests/fixtures/customer-deleted.json")
        .context("Missing webhook fixture template")?;
    let secret = "whsec_Zml4dHVyZS1zaWduaW5nLWtleS1kby1ub3QtdXNl";
    let sign = |secret: &[u8]| -> Result<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret)?;
        mac.update(b"msg_fixture.1767225600.");
        mac.update(body);
        Ok(STANDARD.encode(mac.finalize().into_bytes()))
    };
    let webhook = String::from_utf8(output["webhooks.rs"].clone())?
        .replace("__LEGACY_SIGNATURE__", &sign(secret.as_bytes())?)
        .replace(
            "__STANDARD_SIGNATURE__",
            &sign(&STANDARD.decode(secret.strip_prefix("whsec_").unwrap())?)?,
        );
    output.insert("webhooks.rs".into(), webhook.into_bytes());
    Ok(output)
}

const SHARED: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    "apps/polar-rsh/Cargo.toml",
    "apps/polar-rsh/src/versions.rs",
    "tasks/sdk-contracts/Cargo.toml",
    "tasks/sdk-contracts/src/adapters.rs",
];

fn integrations(
    root: &Path,
    versions: &BTreeMap<String, Version>,
) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut output = BTreeMap::new();
    let mut facade =
        fs::read_to_string(root.join("apps/polar-rsh/Cargo.toml"))?.parse::<DocumentMut>()?;
    let mut contracts =
        fs::read_to_string(root.join("tasks/sdk-contracts/Cargo.toml"))?.parse::<DocumentMut>()?;
    let mut exports =
        "// Generated by sdkgen. Enable an API version feature to use its namespace.\n".to_owned();
    let mut adapters =
        "// Generated by sdkgen. Each adapter uses the actual versioned Rust client.\n".to_owned();
    let mut available = "pub fn available(version: &str) -> bool { match version {\n".to_owned();
    let mut dispatch = "pub async fn run(version: &str, scenario: &crate::Scenario, base_url: \
                        &str) -> anyhow::Result<Vec<crate::model::Outcome>> { match version {\n"
        .to_owned();
    // Every version uses the SDK's shared optional runtime dependencies.
    // Presence remains separately selectable without enabling an API version.
    let sdk_features: toml_edit::Array = std::iter::once("presence".to_owned())
        .chain(
            facade["dependencies"]
                .as_table()
                .context("Missing SDK dependencies")?
                .iter()
                .filter(|(name, dep)| {
                    *name != "presence-rs"
                        && dep.get("optional").and_then(|item| item.as_bool()) == Some(true)
                })
                .map(|(name, _)| format!("dep:{name}")),
        )
        .collect();
    let tests = facade
        .entry("test")
        .or_insert_with(|| ArrayOfTables::new().into())
        .as_array_of_tables_mut()
        .context("Invalid SDK test targets")?;
    for version in versions.keys() {
        let module = format!("v{}", version.replace('-', "_"));
        for kind in ["http", "presence"] {
            let name = format!("{kind}_{}", version.replace('-', "_"));
            let path = format!("src/{module}/tests/{kind}.rs");
            let required = value([version.as_str()].into_iter().collect::<toml_edit::Array>());
            if let Some(test) = tests
                .iter_mut()
                .find(|test| test.get("name").and_then(|name| name.as_str()) == Some(&name))
            {
                test["path"] = value(path);
                test["required-features"] = required;
            } else {
                let mut test = Table::new();
                test["name"] = value(name);
                test["path"] = value(path);
                test["required-features"] = required;
                tests.push(test);
            }
        }
        contracts["features"][version] =
            value([format!("polar-rsh/{version}")].into_iter().collect::<toml_edit::Array>());
        exports.push_str(&format!(
            "#[cfg(feature = \"{version}\")]\n#[path = \"{module}/mod.rs\"]\npub mod {module};\n"
        ));
        adapters.push_str(&format!(
            "#[cfg(feature = \"{version}\")]\nmod {module} {{ use polar_rsh::{module} as sdk; \
             include!(\"adapter.rs\"); }}\n"
        ));
        available.push_str(&format!("#[cfg(feature = \"{version}\")] \"{version}\" => true,\n"));
        dispatch.push_str(&format!(
            "#[cfg(feature = \"{version}\")] \"{version}\" => {module}::run(scenario, \
             base_url).await,\n"
        ));
    }
    for version in versions.keys() {
        facade["features"][version] = value(sdk_features.clone());
    }
    available.push_str("_ => false, } }\n");
    dispatch.push_str("_ => anyhow::bail!(\"No compiled adapter for API {version}\"), } }\n");
    adapters.push_str(&available);
    adapters.push_str(&dispatch);
    output.insert("apps/polar-rsh/Cargo.toml".into(), facade.to_string().into_bytes());
    output.insert("tasks/sdk-contracts/Cargo.toml".into(), contracts.to_string().into_bytes());
    output.insert("apps/polar-rsh/src/versions.rs".into(), format_rust(root, &exports)?);
    output.insert("tasks/sdk-contracts/src/adapters.rs".into(), format_rust(root, &adapters)?);
    Ok(output)
}

fn format_rust(root: &Path, source: &str) -> Result<Vec<u8>> {
    use std::io::Write;
    let mut child = Command::new("rustfmt")
        .current_dir(root)
        .args(["--edition", "2024", "--config", "skip_children=true"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(source.as_bytes())?;
    let output = child.wait_with_output()?;
    ensure!(output.status.success(), "Could not format generated integration");
    Ok(output.stdout)
}

fn apply(
    root: &Path,
    output: &BTreeMap<String, Vec<u8>>,
    previous: &BTreeSet<String>,
    check: bool,
) -> Result<BTreeSet<String>> {
    let mut paths = BTreeSet::new();
    for (path, bytes) in output {
        let file = root.join(path);
        if check {
            ensure!(
                fs::read(&file).ok().as_deref() == Some(bytes.as_slice()),
                "Stale or missing generated output: {path}"
            );
        } else {
            write(&file, bytes)?;
        }
        paths.insert(path.clone());
    }
    for obsolete in previous.iter().filter(|path| !output.contains_key(*path)) {
        ensure!(!check, "Obsolete generated output: {obsolete}");
        if root.join(obsolete).exists() {
            fs::remove_file(root.join(obsolete))?;
        }
    }
    Ok(paths)
}

fn matches_output(
    root: &Path,
    output: &BTreeMap<String, Vec<u8>>,
    inventory: &BTreeSet<String>,
) -> bool {
    output.keys().eq(inventory.iter())
        && output.iter().all(|(path, bytes)| {
            fs::read(root.join(path)).ok().as_deref() == Some(bytes.as_slice())
        })
}

fn snapshot(root: &Path, tracking: &Tracking) -> Result<BTreeMap<String, Vec<u8>>> {
    tracking
        .versions
        .values()
        .flat_map(|entry| entry.files.iter())
        .chain(tracking.shared.iter())
        .map(|path| Ok((path.clone(), fs::read(root.join(path))?)))
        .collect()
}

fn discover(root: &Path, tracking: &mut Tracking, check: bool) -> Result<()> {
    let sources = specs(root)?;
    for version in tracking.versions.keys() {
        ensure!(
            sources.contains_key(version),
            "Previously tracked specification disappeared: {version}; no version modules were \
             deleted"
        );
    }
    let revision = upstream(root)?;
    if check {
        ensure!(
            tracking.upstream_revision == revision,
            "Upstream reference changed since discovery"
        );
    }
    for (version, digest) in sources {
        if let Some(entry) = tracking.versions.get_mut(&version) {
            if check {
                ensure!(entry.observed_hash == digest, "Specification changed: {version}");
            }
            if entry.observed_hash != digest {
                entry.observed_hash = digest;
                entry.status = "pending".into();
            }
            continue;
        }
        ensure!(!check, "New API version requires discovery: {version}");
        let module_path = module_path(&version);
        let mut entry = Version {
            source: format!("{SOURCE}/{version}.openapi.json"),
            observed_hash: digest,
            module_path: module_path.clone(),
            status: "pending".into(),
            ..Version::default()
        };
        let templates = render_templates(root, &version)?;
        // Existing module source is adopted once. Unknown files are never
        // claimed.
        for relative in templates.keys().cloned().chain([
            format!("{version}.openapi.json"),
            "query-parameters.overlay.json".into(),
            "resources.rs".into(),
            "webhook_events.rs".into(),
        ]) {
            let path = format!("{module_path}/{relative}");
            if root.join(&path).is_file() {
                entry.files.insert(path);
            }
        }
        for path in files(&root.join(&module_path).join("generated"))? {
            entry.files.insert(path.strip_prefix(root)?.to_str().unwrap().into());
        }
        let path = format!("{module_path}/mod.rs");
        if !root.join(&path).exists() {
            write(
                &root.join(&path),
                format!("//! Polar API {version} scaffold; run just sdk-generate.\n").as_bytes(),
            )?;
            entry.files.insert(path);
        }
        tracking.versions.insert(version, entry);
    }
    let output = integrations(root, &tracking.versions)?;
    // Shared TOML files have structural ownership, so preserve unrelated keys.
    let owned: BTreeSet<_> =
        tracking.shared.iter().filter(|path| path.ends_with(".rs")).cloned().collect();
    let paths = apply(root, &output, &owned, check)?;
    tracking.shared.extend(paths);
    tracking.upstream_revision = revision;
    Ok(())
}

fn render(root: &Path, version: &str) -> Result<BTreeMap<String, Vec<u8>>> {
    let temporary = tempfile::tempdir()?;
    let module_path = module_path(version);
    let staging = temporary.path().join(&module_path);
    for (path, bytes) in render_templates(root, version)? {
        write(&staging.join(path), &bytes)?;
    }
    for name in ["rustfmt.toml", "rust-toolchain.toml"] {
        fs::copy(root.join(name), temporary.path().join(name))?;
    }
    fs::copy(
        root.join(SOURCE).join(format!("{version}.openapi.json")),
        staging.join(format!("{version}.openapi.json")),
    )?;
    super::run(&staging, &super::Args { api_version: version.into(), check: false })?;
    let rust_files: Vec<_> = files(&staging)?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .collect();
    super::checked_command(
        Command::new("rustfmt")
            .current_dir(temporary.path())
            .args(["--edition", "2024"])
            .args(rust_files),
    )?;
    files(&staging)?
        .into_iter()
        .map(|path| {
            Ok((
                format!("{module_path}/{}", path.strip_prefix(&staging)?.display()),
                fs::read(path)?,
            ))
        })
        .collect()
}

fn current(root: &Path, tracking: &Tracking) -> Result<()> {
    let source = specs(root)?;
    ensure!(source.keys().eq(tracking.versions.keys()), "Discovery is stale");
    ensure!(upstream(root)? == tracking.upstream_revision, "Upstream pin changed after validation");
    let generator = fingerprint(root, false)?;
    let validation = fingerprint(root, true)?;
    for (version, entry) in &tracking.versions {
        ensure!(
            entry.status == "validated"
                && entry.generated_hash.as_ref() == source.get(version)
                && entry.validated_hash.as_ref() == source.get(version)
                && entry.generator_fingerprint.as_ref() == Some(&generator)
                && entry.validation_fingerprint.as_ref() == Some(&validation),
            "API {version} needs generation/conformance; run just sdk-generate"
        );
        for path in &entry.files {
            ensure!(root.join(path).is_file(), "Missing managed output: {path}");
        }
    }
    for path in &tracking.shared {
        ensure!(root.join(path).is_file(), "Missing shared output: {path}");
    }
    Ok(())
}

pub fn run(root: &Path, args: &Cli) -> Result<()> {
    if let Some(version) = &args.api_version {
        ensure!(super::valid_version(version), "API version must be YYYY-MM");
    }
    let mut tracking = load(root)?;
    if args.check && args.stage == Stage::Generate {
        current(root, &tracking)?;
    }
    discover(root, &mut tracking, args.check)?;
    if let Some(version) = &args.api_version {
        ensure!(tracking.versions.contains_key(version), "No source specification for {version}");
    }
    if !args.check {
        save(root, &tracking)?;
    }
    if args.stage == Stage::Discover {
        println!("Discovered {} API versions", tracking.versions.len());
        return Ok(());
    }
    let generator = fingerprint(root, false)?;
    let selected: Vec<_> = tracking
        .versions
        .keys()
        .filter(|v| args.api_version.as_ref().is_none_or(|filter| *v == filter))
        .cloned()
        .collect();
    let mut staged = Vec::new();
    for version in &selected {
        let entry = &tracking.versions[version];
        // Compare rendered bytes directly; the lock stores no output hashes.
        let output = render(root, version)?;
        let changed = !matches_output(root, &output, &entry.files);
        ensure!(!args.check || !changed, "Stale generated output or inventory for API {version}");
        if args.check
            || entry.generated_hash.as_ref() != Some(&entry.observed_hash)
            || entry.generator_fingerprint.as_ref() != Some(&generator)
            || changed
        {
            staged.push((version.clone(), output));
        }
    }
    // All generators must succeed before replacing any version output.
    for (version, output) in staged {
        let entry = tracking.versions.get_mut(&version).unwrap();
        entry.files = apply(root, &output, &entry.files, args.check)?;
        if !args.check {
            entry.generated_hash = Some(entry.observed_hash.clone());
            entry.generator_fingerprint = Some(generator.clone());
            entry.status = "generated".into();
            entry.validated_hash = None;
            entry.validation_fingerprint = None;
        }
    }
    if args.check {
        current(root, &tracking)?;
        println!("SDK sources, output, and validation are current");
        return Ok(());
    }
    save(root, &tracking)?;
    super::checked_command(Command::new("cargo").current_dir(root).args([
        "check",
        "--workspace",
        "--all-targets",
    ]))?;
    let validation = fingerprint(root, true)?;
    let needs_validation: Vec<_> = selected
        .into_iter()
        .filter(|v| {
            let entry = &tracking.versions[v];
            entry.status != "validated"
                || entry.validation_fingerprint.as_ref() != Some(&validation)
        })
        .collect();
    if !needs_validation.is_empty() {
        for version in &needs_validation {
            let entry = tracking.versions.get_mut(version).unwrap();
            entry.status = "generated".into();
            entry.validated_hash = None;
            entry.validation_fingerprint = None;
        }
        save(root, &tracking)?;
        let validated_outputs = snapshot(root, &tracking)?;
        super::checked_command(Command::new("cargo").current_dir(root).args([
            "test",
            "--locked",
            "-p",
            "sdkgen",
            "-p",
            "sdk-contracts",
        ]))?;
        super::checked_command(
            Command::new("cargo").current_dir(root).args(["fmt", "--all", "--check"]),
        )?;
        super::checked_command(Command::new("cargo").current_dir(root).args([
            "clippy",
            "--locked",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ]))?;
        for version in &needs_validation {
            super::checked_command(Command::new("cargo").current_dir(root).args([
                "test",
                "--locked",
                "-p",
                "polar-rsh",
                "--no-default-features",
                "--features",
                version,
            ]))?;
            super::checked_command(Command::new("cargo").current_dir(root).args([
                "check",
                "--locked",
                "-p",
                "polar-rsh",
                "--no-default-features",
                "--features",
                version,
            ]))?;
            super::checked_command(Command::new("cargo").current_dir(root).args([
                "run",
                "--locked",
                "-p",
                "sdk-contracts",
                "--no-default-features",
                "--features",
                version,
                "--",
                "--api-version",
                version,
            ]))?;
        }
        for features in [None, Some("presence"), Some("__all__")] {
            let mut command = Command::new("cargo");
            command.current_dir(root).args([
                "check",
                "--locked",
                "-p",
                "polar-rsh",
                "--no-default-features",
            ]);
            if let Some(feature) = features {
                if feature == "__all__" {
                    command.arg("--all-features");
                } else {
                    command.args(["--features", feature]);
                }
            }
            super::checked_command(&mut command)?;
        }
        ensure!(
            fingerprint(root, false)? == generator,
            "Generator inputs changed during validation"
        );
        ensure!(
            fingerprint(root, true)? == validation,
            "Validation inputs changed during validation"
        );
        ensure!(
            upstream(root)? == tracking.upstream_revision,
            "Upstream pin changed during validation"
        );
        let sources = specs(root)?;
        ensure!(sources.keys().eq(tracking.versions.keys()), "Sources changed during validation");
        for (version, entry) in &tracking.versions {
            ensure!(
                sources.get(version) == Some(&entry.observed_hash),
                "Source changed during validation: {version}"
            );
        }
        for (path, expected) in validated_outputs {
            ensure!(
                fs::read(root.join(&path))? == expected,
                "Output changed during validation: {path}"
            );
        }
        for version in needs_validation {
            let entry = tracking.versions.get_mut(&version).unwrap();
            entry.status = "validated".into();
            entry.validated_hash = Some(entry.observed_hash.clone());
            entry.validation_fingerprint = Some(validation.clone());
        }
    }
    for path in SHARED {
        tracking.shared.insert((*path).into());
    }
    save(root, &tracking)?;
    println!("Generation and conformance complete for {} API versions", tracking.versions.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_exact_content_including_whitespace() {
        assert_eq!(
            hash(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_ne!(hash(b"{}"), hash(b"{}\n"));
    }

    #[test]
    fn direct_comparison_detects_edits_and_generation_repairs_managed_files() {
        let root = tempfile::tempdir().unwrap();
        let output = BTreeMap::from([("src/lib.rs".into(), b"original".to_vec())]);
        let recorded = apply(root.path(), &output, &BTreeSet::new(), false).unwrap();
        assert!(matches_output(root.path(), &output, &recorded));
        assert_eq!(apply(root.path(), &output, &recorded, false).unwrap(), recorded);
        fs::remove_file(root.path().join("src/lib.rs")).unwrap();
        assert!(apply(root.path(), &output, &recorded, true).is_err());
        assert!(!root.path().join("src/lib.rs").exists());
        apply(root.path(), &output, &recorded, false).unwrap();
        fs::write(root.path().join("src/lib.rs"), "user change").unwrap();
        assert!(!matches_output(root.path(), &output, &recorded));
        assert!(apply(root.path(), &output, &recorded, true).is_err());
        assert_eq!(fs::read_to_string(root.path().join("src/lib.rs")).unwrap(), "user change");
        apply(root.path(), &output, &recorded, false).unwrap();
        assert!(matches_output(root.path(), &output, &recorded));
        assert!(!matches_output(root.path(), &output, &BTreeSet::new()));
    }

    #[test]
    fn obsolete_managed_files_are_removed_and_unowned_files_are_preserved() {
        let root = tempfile::tempdir().unwrap();
        let recorded = apply(
            root.path(),
            &BTreeMap::from([("old.rs".into(), b"old".to_vec())]),
            &BTreeSet::new(),
            false,
        )
        .unwrap();
        fs::write(root.path().join("unowned.rs"), "mine").unwrap();
        assert!(apply(root.path(), &BTreeMap::new(), &recorded, true).is_err());
        apply(root.path(), &BTreeMap::new(), &recorded, false).unwrap();
        assert!(!root.path().join("old.rs").exists());
        assert!(root.path().join("unowned.rs").exists());
    }

    #[test]
    fn discovery_validates_every_document_and_filename() {
        let root = tempfile::tempdir().unwrap();
        let spec = br#"{"openapi":"3.1.0","paths":{},"components":{"schemas":{}}}"#;
        write(&root.path().join(SOURCE).join("2026-04.openapi.json"), spec).unwrap();
        assert_eq!(specs(root.path()).unwrap()["2026-04"], hash(spec));
        write(&root.path().join(SOURCE).join("2026-13.openapi.json"), spec).unwrap();
        assert!(specs(root.path()).is_err());
        fs::remove_file(root.path().join(SOURCE).join("2026-13.openapi.json")).unwrap();
        write(&root.path().join(SOURCE).join("2026-10.openapi.json"), b"{}").unwrap();
        assert!(specs(root.path()).is_err());
    }

    #[test]
    fn missing_tracked_spec_does_not_delete_modules() {
        let root = tempfile::tempdir().unwrap();
        write(
            &root.path().join(SOURCE).join("2026-04.openapi.json"),
            br#"{"openapi":"3.1.0","paths":{},"components":{"schemas":{}}}"#,
        )
        .unwrap();
        write(&root.path().join("apps/polar-rsh/src/v2026_10/mod.rs"), b"preserve").unwrap();
        let mut tracking = Tracking::default();
        tracking.versions.insert("2026-10".into(), Version::default());
        assert!(
            discover(root.path(), &mut tracking, false)
                .unwrap_err()
                .to_string()
                .contains("disappeared")
        );
        assert_eq!(
            fs::read(root.path().join("apps/polar-rsh/src/v2026_10/mod.rs")).unwrap(),
            b"preserve"
        );
    }

    #[test]
    fn integrations_preserve_unrelated_manifest_entries() {
        let root = tempfile::tempdir().unwrap();
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for file in ["rustfmt.toml", "rust-toolchain.toml"] {
            fs::copy(workspace.join(file), root.path().join(file)).unwrap();
        }
        write(
            &root.path().join("Cargo.toml"),
            b"[workspace]\nmembers = [\"custom\"]\n[workspace.package]\nversion = \"1.2.3\"\n",
        )
        .unwrap();
        write(
            &root.path().join("apps/polar-rsh/Cargo.toml"),
            b"[features]\ncustom = []\n[dependencies]\nother = \"1\"\nreqwest = { version = \"0.13\", optional = true }\n",
        )
        .unwrap();
        write(
            &root.path().join("tasks/sdk-contracts/Cargo.toml"),
            b"[dependencies]\nother = \"2\"\n",
        )
        .unwrap();
        let versions = BTreeMap::from([(
            "2026-10".into(),
            Version { module_path: "apps/polar-rsh/src/v2026_10".into(), ..Version::default() },
        )]);
        let output = integrations(root.path(), &versions).unwrap();
        let facade = String::from_utf8(output["apps/polar-rsh/Cargo.toml"].clone())
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        assert!(facade["features"]["custom"].is_array());
        assert_eq!(facade["dependencies"]["other"].as_str(), Some("1"));
        assert!(facade["dependencies"].get("polar-rsh-2026-10").is_none());
        assert!(!output.contains_key("Cargo.toml"));
        assert_eq!(
            facade["features"]["2026-10"]
                .as_array()
                .unwrap()
                .iter()
                .map(|feature| feature.as_str().unwrap())
                .collect::<Vec<_>>(),
            ["presence", "dep:reqwest"]
        );
        let tests = facade["test"].as_array_of_tables().unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(tests.get(0).unwrap()["path"].as_str(), Some("src/v2026_10/tests/http.rs"));
        assert_eq!(tests.get(0).unwrap()["required-features"][0].as_str(), Some("2026-10"));
        let contracts = String::from_utf8(output["tasks/sdk-contracts/Cargo.toml"].clone())
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        assert_eq!(contracts["features"]["2026-10"][0].as_str(), Some("polar-rsh/2026-10"));
        assert_eq!(contracts["dependencies"]["other"].as_str(), Some("2"));
        assert!(
            String::from_utf8(output["apps/polar-rsh/src/versions.rs"].clone())
                .unwrap()
                .contains("pub mod v2026_10")
        );

        // Registration is idempotent and does not interfere with releases.
        for file in ["apps/polar-rsh/Cargo.toml", "tasks/sdk-contracts/Cargo.toml"] {
            write(&root.path().join(file), &output[file]).unwrap();
        }
        write(
            &root.path().join("Cargo.toml"),
            b"[workspace]\nmembers = [\"custom\"]\n[workspace.package]\nversion = \"1.2.4\"\n",
        )
        .unwrap();
        let after_release = integrations(root.path(), &versions).unwrap();
        for file in ["apps/polar-rsh/Cargo.toml", "tasks/sdk-contracts/Cargo.toml"] {
            assert_eq!(after_release[file], output[file]);
        }

        let mut versions = versions;
        versions.insert(
            "2027-01".into(),
            Version { module_path: module_path("2027-01"), ..Version::default() },
        );
        let expanded = integrations(root.path(), &versions).unwrap();
        let facade = String::from_utf8(expanded["apps/polar-rsh/Cargo.toml"].clone())
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        assert_eq!(facade["test"].as_array_of_tables().unwrap().len(), 4);
        assert_eq!(
            facade["features"]["2027-01"].to_string(),
            facade["features"]["2026-10"].to_string()
        );
        assert!(!expanded.contains_key("Cargo.toml"));
    }

    #[test]
    fn rejects_tracking_paths_outside_the_owned_module() {
        let root = tempfile::tempdir().unwrap();
        let mut tracking = Tracking::default();
        tracking.versions.insert(
            "2026-04".into(),
            Version {
                source: format!("{SOURCE}/2026-04.openapi.json"),
                module_path: "apps/polar-rsh/src/v2026_04".into(),
                files: BTreeSet::from(["apps/polar-rsh/src/v2026_04/../../../outside".into()]),
                ..Version::default()
            },
        );
        save(root.path(), &tracking).unwrap();
        assert!(load(root.path()).is_err());
    }

    /// Minimal real Git fixture for validation tests.
    fn fixture() -> (tempfile::TempDir, Tracking) {
        let root = tempfile::tempdir().unwrap();
        let upstream = root.path().join("vendor/polar");
        fs::create_dir_all(upstream.join("docs/openapi")).unwrap();
        git(&upstream, &["init", "--initial-branch=main"]).unwrap();
        fs::write(
            upstream.join("docs/openapi/2026-04.openapi.json"),
            r#"{"openapi":"3.1.0","paths":{},"components":{"schemas":{}}}"#,
        )
        .unwrap();
        git(&upstream, &["add", "."]).unwrap();
        git(
            &upstream,
            &[
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-m",
                "reference",
            ],
        )
        .unwrap();
        for name in [
            "tasks/sdkgen/Cargo.toml",
            "rust-toolchain.toml",
            "rustfmt.toml",
            "tasks/sdk-contracts/Cargo.toml",
            "tasks/sdk-contracts/typescript.mjs",
            "tasks/sdk-contracts/scenarios.json",
            "Cargo.lock",
            "apps/polar-rsh/src/lib.rs",
            "apps/polar-rsh/src/versions.rs",
            "apps/polar-rsh/Cargo.toml",
            "justfile",
            "Cargo.toml",
            "tasks/sdk-contracts/src/adapters.rs",
        ] {
            write(&root.path().join(name), b"fixture\n").unwrap();
        }
        let module_path = "apps/polar-rsh/src/v2026_04";
        let path = format!("{module_path}/mod.rs");
        write(&root.path().join(&path), b"// generated\n").unwrap();
        let digest = file_hash(&upstream.join("docs/openapi/2026-04.openapi.json")).unwrap();
        let entry = Version {
            source: format!("{SOURCE}/2026-04.openapi.json"),
            observed_hash: digest.clone(),
            module_path: module_path.into(),
            status: "validated".into(),
            generated_hash: Some(digest.clone()),
            validated_hash: Some(digest),
            generator_fingerprint: Some(fingerprint(root.path(), false).unwrap()),
            validation_fingerprint: Some(fingerprint(root.path(), true).unwrap()),
            files: BTreeSet::from([path]),
        };
        let tracking = Tracking {
            upstream_revision: git(&upstream, &["rev-parse", "HEAD"]).unwrap(),
            versions: BTreeMap::from([("2026-04".into(), entry)]),
            shared: BTreeSet::new(),
            ..Tracking::default()
        };
        save(root.path(), &tracking).unwrap();
        (root, tracking)
    }

    #[test]
    fn validation_is_bound_to_sources_generator_and_contract_inputs() {
        let (root, mut tracking) = fixture();
        current(root.path(), &tracking).unwrap();
        tracking.versions.get_mut("2026-04").unwrap().status = "generated".into();
        assert!(current(root.path(), &tracking).is_err());
        tracking.versions.get_mut("2026-04").unwrap().status = "validated".into();
        fs::write(root.path().join("tasks/sdk-contracts/scenarios.json"), "changed").unwrap();
        assert!(current(root.path(), &tracking).is_err());
        fs::write(root.path().join("tasks/sdk-contracts/scenarios.json"), "fixture\n").unwrap();
        current(root.path(), &tracking).unwrap();
        fs::remove_file(root.path().join("apps/polar-rsh/src/v2026_04/mod.rs")).unwrap();
        assert!(current(root.path(), &tracking).is_err());
    }

    #[test]
    fn legacy_lock_migrates_hash_maps_to_path_inventories() {
        let (_, tracking) = fixture();
        let mut legacy = serde_json::to_value(&tracking).unwrap();
        legacy["format"] = 1.into();
        let path = tracking.versions["2026-04"].files.first().unwrap();
        legacy["versions"]["2026-04"]["files"] = serde_json::json!({path: "old-output-hash"});
        legacy["shared"] = serde_json::json!({"Cargo.lock": "old-lock-hash"});
        let migrated = decode_tracking(&serde_json::to_vec(&legacy).unwrap()).unwrap();
        assert_eq!(migrated.format, 2);
        assert_eq!(migrated.versions, tracking.versions);
        assert_eq!(migrated.shared, BTreeSet::from(["Cargo.lock".into()]));
        let serialized = serde_json::to_value(migrated).unwrap();
        assert!(serialized["versions"]["2026-04"]["files"].is_array());
        assert_eq!(serialized["shared"], serde_json::json!(["Cargo.lock"]));
        assert!(!serialized.to_string().contains("old-output-hash"));
        assert!(!serialized.to_string().contains("old-lock-hash"));
    }

    #[test]
    fn validation_snapshot_compares_bytes_without_persisting_output_hashes() {
        let (root, tracking) = fixture();
        let before = snapshot(root.path(), &tracking).unwrap();
        let path = tracking.versions["2026-04"].files.first().unwrap();
        fs::write(root.path().join(path), "edited during validation").unwrap();
        assert_ne!(snapshot(root.path(), &tracking).unwrap(), before);
        assert!(
            serde_json::to_value(&tracking).unwrap()["versions"]["2026-04"]["files"].is_array()
        );
    }

    #[test]
    fn tracking_serialization_is_deterministic() {
        let (root, tracking) = fixture();
        let bytes = fs::read(root.path().join(TRACKING)).unwrap();
        save(root.path(), &load(root.path()).unwrap()).unwrap();
        assert_eq!(bytes, fs::read(root.path().join(TRACKING)).unwrap());
        assert_eq!(tracking, load(root.path()).unwrap());
    }
}
