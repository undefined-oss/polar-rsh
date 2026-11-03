use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, ensure};
use clap::{Parser, ValueEnum};
use sdkgen::tracking::{Tracking, Version, decode_tracking};
use toml_edit::{DocumentMut, value};

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
    blake3::hash(bytes).to_hex().to_string()
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
    }
    Ok(tracking)
}

// Own the version entry points and model tree. The old generated directory
// and legacy wrappers are included only for migration; unknown siblings remain.
fn managed_paths(root: &Path, version: &str) -> Result<BTreeSet<String>> {
    let module = module_path(version);
    let mut paths = BTreeSet::from([format!("{module}/mod.rs")]);
    for relative in [
        "client.rs".into(),
        "error.rs".into(),
        "models.rs".into(),
        "path.rs".into(),
        "query.rs".into(),
        "lib.rs".into(),
        "uuid-mappings.json".into(),
        "newtype-mappings.json".into(),
        "model-report.json".into(),
        "operation-report.json".into(),
        format!("{version}.openapi.json"),
        "openapi-to-rust.toml".into(),
        "query-parameters.overlay.json".into(),
        "request.rs".into(),
        "resources.rs".into(),
        "webhook_events.rs".into(),
        "webhooks.rs".into(),
        "tests/http.rs".into(),
        "tests/presence.rs".into(),
        "tests/fixtures/customer-deleted.json".into(),
    ] {
        let path = format!("{module}/{relative}");
        if root.join(&path).is_file() {
            paths.insert(path);
        }
    }
    for directory in ["models", "generated"] {
        for path in files(&root.join(&module).join(directory))? {
            paths.insert(
                path.strip_prefix(root)?.to_str().context("Invalid generated path")?.into(),
            );
        }
    }
    Ok(paths)
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
        let spec: oas3::Spec =
            serde_json::from_slice(&bytes).with_context(|| format!("Invalid spec {name}"))?;
        ensure!(
            spec.openapi.starts_with("3.") && spec.info.version == version,
            "Invalid OpenAPI document: {name}"
        );
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
        paths.extend(files(&root.join("tasks/sdk-classify/src"))?);
        for file in [
            "tasks/sdk-classify/Cargo.toml",
            "Cargo.toml",
            "Cargo.lock",
            "apps/polar-rsh/src/lib.rs",
            "apps/polar-rsh/src/versions.rs",
            "apps/polar-rsh/Cargo.toml",
            "justfile",
        ] {
            paths.push(root.join(file));
        }
    }
    paths.sort();
    let mut digest = blake3::Hasher::new();
    digest.update(super::GENERATOR_VERSION.as_bytes());
    for path in paths {
        digest.update(path.strip_prefix(root)?.to_string_lossy().as_bytes());
        digest.update(&[0]);
        digest.update(file_hash(&path)?.as_bytes());
    }
    if validation {
        digest.update(upstream(root)?.as_bytes());
    }
    Ok(digest.finalize().to_hex().to_string())
}

const SHARED: &[&str] =
    &["Cargo.toml", "Cargo.lock", "apps/polar-rsh/Cargo.toml", "apps/polar-rsh/src/versions.rs"];

fn integrations(
    root: &Path,
    versions: &BTreeMap<String, Version>,
) -> Result<BTreeMap<String, Vec<u8>>> {
    let mut output = BTreeMap::new();
    let mut facade =
        fs::read_to_string(root.join("apps/polar-rsh/Cargo.toml"))?.parse::<DocumentMut>()?;
    let mut exports =
        "// Generated by sdkgen. Enable an API version feature to use its namespace.\n".to_owned();
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
    // Remove only the test targets owned by the retired backend.
    if let Some(tests) = facade.get_mut("test").and_then(|item| item.as_array_of_tables_mut()) {
        tests.retain(|test| {
            !versions.keys().any(|version| {
                ["http", "presence"].iter().any(|kind| {
                    test.get("name").and_then(|name| name.as_str())
                        == Some(format!("{kind}_{}", version.replace('-', "_")).as_str())
                })
            })
        });
        if tests.is_empty() {
            facade.remove("test");
        }
    }
    for version in versions.keys() {
        let module = format!("v{}", version.replace('-', "_"));
        exports.push_str(&format!(
            "#[cfg(feature = \"{version}\")]\n#[path = \"{module}/mod.rs\"]\npub mod {module};\n"
        ));
    }
    for version in versions.keys() {
        facade["features"][version] = value(sdk_features.clone());
    }
    output.insert("apps/polar-rsh/Cargo.toml".into(), facade.to_string().into_bytes());
    output.insert("apps/polar-rsh/src/versions.rs".into(), format_rust(root, &exports)?);
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
            let file = root.join(obsolete);
            let mut parent = file.parent();
            while let Some(directory) = parent {
                if directory == root || fs::remove_dir(directory).is_err() {
                    break;
                }
                parent = directory.parent();
            }
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
    let mut paths: BTreeSet<String> = SHARED.iter().map(|path| (*path).into()).collect();
    for (version, entry) in &tracking.versions {
        // An unselected pending version may have only a discovery scaffold.
        paths.extend(
            managed_paths(root, version)?
                .into_iter()
                .filter(|path| entry.status != "pending" || root.join(path).is_file()),
        );
    }
    paths.into_iter().map(|path| Ok((path.clone(), fs::read(root.join(path))?))).collect()
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
        let entry = Version {
            source: format!("{SOURCE}/{version}.openapi.json"),
            observed_hash: digest,
            module_path: module_path.clone(),
            status: "pending".into(),
            ..Version::default()
        };
        let path = format!("{module_path}/mod.rs");
        if !root.join(&path).exists() {
            write(
                &root.join(&path),
                format!("//! Polar API {version} scaffold; run just sdk-generate.\n").as_bytes(),
            )?;
        }
        tracking.versions.insert(version, entry);
    }
    let output = integrations(root, &tracking.versions)?;
    // Shared TOML files have structural ownership, so preserve unrelated keys.
    let owned =
        SHARED.iter().filter(|path| path.ends_with(".rs")).map(|path| (*path).into()).collect();
    apply(root, &output, &owned, check)?;
    tracking.upstream_revision = revision;
    Ok(())
}

// Rewrite Rust tokens, including macro arguments, without touching
// documentation or string literals. Each generated SDK lives in its own version
// namespace.
fn namespace_tokens(
    tokens: proc_macro2::TokenStream,
    module: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    use proc_macro2::{Group, TokenTree};
    let input: Vec<_> = tokens.into_iter().collect();
    let mut output = proc_macro2::TokenStream::new();
    for (index, token) in input.iter().enumerate() {
        match token {
            TokenTree::Group(group) => {
                let mut rewritten =
                    Group::new(group.delimiter(), namespace_tokens(group.stream(), module));
                rewritten.set_span(group.span());
                output.extend([TokenTree::Group(rewritten)]);
            }
            TokenTree::Ident(ident)
                if ident == "crate"
                    && input.get(index + 1).is_some_and(
                        |token| matches!(token, TokenTree::Punct(p) if p.as_char() == ':'),
                    )
                    && input.get(index + 2).is_some_and(
                        |token| matches!(token, TokenTree::Punct(p) if p.as_char() == ':'),
                    ) =>
            {
                output.extend(quote::quote!(crate::#module));
            }
            TokenTree::Literal(literal)
                if index >= 2
                    && matches!(&input[index - 2], TokenTree::Ident(key)
                    if matches!(key.to_string().as_str(), "deserialize_with" | "serialize_with" | "with" | "skip_serializing_if")) =>
            {
                let replacement = syn::parse_str::<syn::LitStr>(&literal.to_string())
                    .ok()
                    .filter(|value| value.value().starts_with("crate::"))
                    .map(|value| {
                        syn::LitStr::new(
                            &value.value().replacen("crate::", &format!("crate::{module}::"), 1),
                            value.span(),
                        )
                    });
                if let Some(literal) = replacement {
                    output.extend(quote::quote!(#literal));
                } else {
                    output.extend([token.clone()]);
                }
            }
            _ => output.extend([token.clone()]),
        }
    }
    output
}

fn render(root: &Path, version: &str) -> Result<BTreeMap<String, Vec<u8>>> {
    let temporary = tempfile::tempdir()?;
    let module = module_path(version);
    let staging = temporary.path().join(&module);
    let source = fs::read_to_string(root.join(SOURCE).join(format!("{version}.openapi.json")))?;
    sdkgen::generator::generate(&source, &staging)
        .map_err(|error| anyhow::anyhow!("Generate API {version}: {error}"))?;
    let module_ident = quote::format_ident!("v{}", version.replace('-', "_"));
    for path in files(&staging)? {
        if path.extension().is_some_and(|ext| ext == "rs") {
            let tokens = fs::read_to_string(&path)?
                .parse::<proc_macro2::TokenStream>()
                .map_err(|error| anyhow::anyhow!("Parse {}: {error}", path.display()))?;
            let mut file = syn::parse2::<syn::File>(namespace_tokens(tokens, &module_ident))?;
            if path == staging.join("lib.rs") {
                for item in &mut file.items {
                    if let syn::Item::Mod(module) = item
                        && module.ident == "path"
                    {
                        module.vis = syn::parse_quote!(pub(crate));
                    }
                }
            }
            fs::write(&path, prettyplease::unparse(&file))?;
        }
    }
    fs::rename(staging.join("lib.rs"), staging.join("mod.rs"))?;
    for name in ["rustfmt.toml", "rust-toolchain.toml"] {
        fs::copy(root.join(name), temporary.path().join(name))?;
    }
    super::checked_command(
        Command::new("rustfmt")
            .current_dir(temporary.path())
            .args(["--edition", "2024"])
            .arg(staging.join("mod.rs")),
    )?;
    files(&staging)?
        .into_iter()
        .map(|path| {
            Ok((format!("{module}/{}", path.strip_prefix(&staging)?.display()), fs::read(path)?))
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
            "API {version} needs generation/validation; run just sdk-generate"
        );
        for path in managed_paths(root, version)? {
            ensure!(root.join(&path).is_file(), "Missing managed output: {path}");
        }
    }
    for path in SHARED {
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
        let changed = !matches_output(root, &output, &managed_paths(root, version)?);
        ensure!(!args.check || !changed, "Stale generated output for API {version}");
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
        apply(root, &output, &managed_paths(root, &version)?, args.check)?;
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
        "--exclude",
        "polar-sdk-playground",
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
            "nextest",
            "run",
            "--locked",
            "-p",
            "sdkgen",
            "-p",
            "sdk-classify",
        ]))?;
        super::checked_command(
            Command::new("cargo").current_dir(root).args(["fmt", "--all", "--check"]),
        )?;
        super::checked_command(Command::new("cargo").current_dir(root).args([
            "clippy",
            "--locked",
            "--workspace",
            "--exclude",
            "polar-sdk-playground",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ]))?;
        for version in &needs_validation {
            super::checked_command(Command::new("cargo").current_dir(root).args([
                "nextest",
                "run",
                "--no-tests=pass",
                "--locked",
                "-p",
                "polar-rsh",
                "--no-default-features",
                "--features",
                version,
            ]))?;
            super::checked_command(Command::new("cargo").current_dir(root).args([
                "test",
                "--doc",
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
    save(root, &tracking)?;
    println!("Generation and validation complete for {} API versions", tracking.versions.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespaces_rust_paths_and_macro_tokens_without_rewriting_literals() {
        let source = quote::quote! {
            use crate::client::PolarClient;
            #[serde(deserialize_with = "crate::models::decode")]
            struct Model;
            pub(crate) fn example() {
                macro_call!(crate::models::Item);
                let documentation = "crate::models::Item";
            }
        };
        let module = quote::format_ident!("v2026_10");
        let output = namespace_tokens(source, &module).to_string();
        syn::parse_file(&output).unwrap();
        assert!(output.contains("crate :: v2026_10 :: client"));
        assert!(output.contains("crate :: v2026_10 :: models"));
        assert!(output.contains("pub (crate)"));
        assert!(output.contains("\"crate::v2026_10::models::decode\""));
        assert!(output.contains("\"crate::models::Item\""));
    }

    #[test]
    fn hashes_exact_content_including_whitespace() {
        assert_eq!(
            hash(b"abc"),
            "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85"
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
    fn runtime_inventory_repairs_outputs_and_preserves_unknown_wrappers() {
        let root = tempfile::tempdir().unwrap();

        let module = module_path("2026-04");
        let obsolete = format!("{module}/generated/obsolete.rs");
        let unknown = format!("{module}/custom.rs");
        write(&root.path().join(&obsolete), b"obsolete").unwrap();
        write(&root.path().join(&unknown), b"custom").unwrap();
        let inventory = managed_paths(root.path(), "2026-04").unwrap();
        assert!(inventory.contains(&format!("{module}/mod.rs")));
        assert!(!inventory.contains(&unknown));
        let mut output: BTreeMap<_, _> = inventory
            .iter()
            .filter(|path| **path != obsolete)
            .map(|path| (path.clone(), b"rendered".to_vec()))
            .collect();
        output.insert(format!("{module}/client.rs"), b"client".to_vec());
        assert!(!matches_output(root.path(), &output, &inventory));
        assert!(apply(root.path(), &output, &inventory, true).is_err());
        apply(root.path(), &output, &inventory, false).unwrap();
        assert!(!root.path().join(obsolete).exists());
        assert!(!root.path().join(&module).join("generated").exists());
        assert_eq!(fs::read(root.path().join(unknown)).unwrap(), b"custom");
        let inventory = managed_paths(root.path(), "2026-04").unwrap();
        assert!(matches_output(root.path(), &output, &inventory));
        fs::remove_file(root.path().join(format!("{module}/client.rs"))).unwrap();
        assert!(!matches_output(
            root.path(),
            &output,
            &managed_paths(root.path(), "2026-04").unwrap()
        ));
    }

    #[test]
    fn discovery_validates_every_document_and_filename() {
        let root = tempfile::tempdir().unwrap();
        let spec = br#"{"openapi":"3.1.0","info":{"title":"Fixture","version":"2026-04"},"paths":{},"components":{"schemas":{}}}"#;
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
            br#"{"openapi":"3.1.0","info":{"title":"Fixture","version":"2026-04"},"paths":{},"components":{"schemas":{}}}"#,
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
        assert!(facade.get("test").is_none());
        assert!(
            String::from_utf8(output["apps/polar-rsh/src/versions.rs"].clone())
                .unwrap()
                .contains("pub mod v2026_10")
        );

        // Registration is idempotent and does not interfere with releases.
        let file = "apps/polar-rsh/Cargo.toml";
        write(&root.path().join(file), &output[file]).unwrap();
        write(
            &root.path().join("Cargo.toml"),
            b"[workspace]\nmembers = [\"custom\"]\n[workspace.package]\nversion = \"1.2.4\"\n",
        )
        .unwrap();
        let after_release = integrations(root.path(), &versions).unwrap();
        assert_eq!(after_release[file], output[file]);

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
        assert!(facade.get("test").is_none());
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
                module_path: "apps/polar-rsh/src/v2026_04/../../../outside".into(),
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
            r#"{"openapi":"3.1.0","info":{"title":"Fixture","version":"2026-04"},"paths":{},"components":{"schemas":{}}}"#,
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
            "tasks/sdk-classify/Cargo.toml",
            "rust-toolchain.toml",
            "rustfmt.toml",
            "Cargo.lock",
            "apps/polar-rsh/src/lib.rs",
            "apps/polar-rsh/src/versions.rs",
            "apps/polar-rsh/Cargo.toml",
            "justfile",
            "Cargo.toml",
        ] {
            write(&root.path().join(name), b"fixture\n").unwrap();
        }
        let module_path = "apps/polar-rsh/src/v2026_04";
        let path = format!("{module_path}/mod.rs");
        write(&root.path().join(&path), b"// generated\n").unwrap();
        for path in managed_paths(root.path(), "2026-04").unwrap() {
            if !root.path().join(&path).exists() {
                write(&root.path().join(path), b"fixture\n").unwrap();
            }
        }
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
        };
        let tracking = Tracking {
            upstream_revision: git(&upstream, &["rev-parse", "HEAD"]).unwrap(),
            versions: BTreeMap::from([("2026-04".into(), entry)]),
            ..Tracking::default()
        };
        save(root.path(), &tracking).unwrap();
        (root, tracking)
    }

    #[test]
    fn validation_is_bound_to_sources_generator_and_workspace_inputs() {
        let (root, mut tracking) = fixture();
        current(root.path(), &tracking).unwrap();
        tracking.versions.get_mut("2026-04").unwrap().status = "generated".into();
        assert!(current(root.path(), &tracking).is_err());
        tracking.versions.get_mut("2026-04").unwrap().status = "validated".into();
        fs::write(root.path().join("Cargo.lock"), "changed").unwrap();
        assert!(current(root.path(), &tracking).is_err());
        fs::write(root.path().join("Cargo.lock"), "fixture\n").unwrap();
        current(root.path(), &tracking).unwrap();
        fs::remove_file(root.path().join("apps/polar-rsh/src/v2026_04/mod.rs")).unwrap();
        assert!(current(root.path(), &tracking).is_err());
    }

    #[test]
    fn validation_snapshot_compares_bytes_without_persisting_output_hashes() {
        let (root, tracking) = fixture();
        let before = snapshot(root.path(), &tracking).unwrap();
        let path = "apps/polar-rsh/src/v2026_04/mod.rs";
        fs::write(root.path().join(path), "edited during validation").unwrap();
        assert_ne!(snapshot(root.path(), &tracking).unwrap(), before);
        assert!(serde_json::to_value(&tracking).unwrap()["versions"]["2026-04"]["files"].is_null());
    }

    #[test]
    fn snapshot_includes_scaffolds_for_unselected_pending_versions() {
        let (root, mut tracking) = fixture();
        let path = format!("{}/mod.rs", module_path("2026-10"));
        write(&root.path().join(&path), b"scaffold").unwrap();
        tracking
            .versions
            .insert("2026-10".into(), Version { status: "pending".into(), ..Version::default() });
        let outputs = snapshot(root.path(), &tracking).unwrap();
        assert_eq!(outputs[&path], b"scaffold");
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
