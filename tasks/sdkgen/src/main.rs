use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, ensure};
use clap::Parser;

mod bindings;
mod metadata;
mod openapi;
mod pipeline;
mod presence;
mod query;
mod schema;
mod template;

const GENERATOR_VERSION: &str = "openapi-to-rust 0.22.0";

/// Generate Polar SDK resources and webhook dispatch from a versioned OpenAPI
/// spec.
#[derive(Parser)]
struct Args {
    /// API version of the specification in the supplied module directory.
    #[arg(long, default_value = "2026-04")]
    api_version: String,
    /// Check generated files without changing the workspace.
    #[arg(long)]
    check: bool,
}

fn main() -> Result<()> {
    let args = pipeline::Cli::parse();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    pipeline::run(workspace, &args)
}

fn run(root: &Path, args: &Args) -> Result<()> {
    ensure!(
        valid_version(&args.api_version),
        "API version must have the form YYYY-MM (month 01–12)"
    );
    let spec_name = format!("{}.openapi.json", args.api_version);
    let spec_path = root.join(&spec_name);
    ensure!(spec_path.is_file(), "No specification found at {}", spec_path.display());
    let version = Command::new("openapi-to-rust").arg("--version").output().context(
        "Install the generator: cargo install --locked openapi-to-rust --version 0.22.0",
    )?;
    ensure!(version.status.success(), "openapi-to-rust --version failed");
    let version = String::from_utf8(version.stdout)?;
    ensure!(
        version.trim() == GENERATOR_VERSION,
        "Expected {GENERATOR_VERSION}; found {}",
        version.trim()
    );

    // Stage the complete result for both modes: rendering errors leave existing
    // files intact.
    let temporary = tempfile::tempdir()?;
    let staging = temporary.path();
    fs::copy(&spec_path, staging.join(&spec_name))?;
    fs::copy(root.join("openapi-to-rust.toml"), staging.join("openapi-to-rust.toml"))?;
    for name in ["rustfmt.toml", "rust-toolchain.toml"] {
        if let Some(source) = root.ancestors().map(|dir| dir.join(name)).find(|file| file.is_file())
        {
            fs::copy(source, staging.join(name))?;
        }
    }
    let mut spec: openapi::Document = serde_json::from_slice(&fs::read(&spec_path)?)?;
    let overlay = schema::normalize_queries(&mut spec)?;
    let overlay_name = "query-parameters.overlay.json";
    fs::write(
        staging.join(overlay_name),
        format!("{}\n", serde_json::to_string_pretty(&overlay)?),
    )?;
    checked_command(Command::new("openapi-to-rust").current_dir(staging).args([
        "generate",
        "--config",
        "openapi-to-rust.toml",
    ]))?;
    let types_path = staging.join("generated/types.rs");
    let bindings_path = staging.join("generated/bindings.json");
    let mut metadata: metadata::Bindings = serde_json::from_slice(&fs::read(&bindings_path)?)?;
    metadata.validate()?;
    let types = presence::generate(&fs::read_to_string(&types_path)?, &mut metadata)?;
    // Upstream schema documentation and generated builder/union shapes are
    // retained.
    let mut types_file = syn::parse_file(&types)?;
    types_file.attrs.push(syn::parse_quote!(#![allow(
        clippy::double_must_use,
        clippy::collapsible_if,
        clippy::doc_lazy_continuation
    )]));
    fs::write(types_path, prettyplease::unparse(&types_file))?;
    fs::write(&bindings_path, format!("{}\n", serde_json::to_string_pretty(&metadata)?))?;
    let dependencies_path = staging.join("generated/REQUIRED_DEPS.toml");
    let mut dependencies = fs::read_to_string(&dependencies_path)?;
    dependencies.push_str("presence-rs = { version = \"=0.2.0\", features = [\"serde\"] }\n");
    dependencies.push_str("serde_qs = \"=1.1.3\"\n");
    fs::write(dependencies_path, dependencies)?;
    let resources = bindings::resources(&spec, &metadata)?;
    let webhooks = bindings::webhooks(&spec)?;
    let templates = template::Templates::new()?;
    fs::write(staging.join("resources.rs"), templates.resources(&resources)?)?;
    fs::write(staging.join("webhook_events.rs"), templates.webhooks(&webhooks)?)?;
    let client_path = staging.join("generated/client.rs");
    let client = query::generate(&spec, &metadata, &fs::read_to_string(&client_path)?)?;
    let client = presence::client_builders(&client, &types)?;
    // These shapes are imposed by the upstream generator's public API.
    let mut client = syn::parse_file(&client)?;
    client.attrs.push(syn::parse_quote!(#![allow(
        clippy::double_must_use,
        clippy::result_large_err,
        clippy::too_many_arguments,
        clippy::match_single_binding
    )]));
    fs::write(client_path, prettyplease::unparse(&client))?;
    fs::write(staging.join("generated/query.rs"), include_str!("../templates/query.rs"))?;
    let modules_path = staging.join("generated/mod.rs");
    let mut modules = fs::read_to_string(&modules_path)?;
    modules.push_str("\nmod query;\n");
    fs::write(modules_path, modules)?;
    checked_command(Command::new("rustfmt").current_dir(staging).args([
        "--edition",
        "2024",
        "resources.rs",
        "webhook_events.rs",
        "generated/client.rs",
        "generated/types.rs",
        "generated/query.rs",
    ]))?;
    let mut outputs =
        vec![PathBuf::from(overlay_name), "resources.rs".into(), "webhook_events.rs".into()];
    for entry in fs::read_dir(staging.join("generated"))? {
        let entry = entry?;
        ensure!(
            entry.file_type()?.is_file(),
            "Unexpected generated directory: {}",
            entry.path().display()
        );
        outputs.push(Path::new("generated").join(entry.file_name()));
    }
    outputs.sort();
    let changed = sync_outputs(staging, root, &outputs, args.check)?;
    if args.check {
        ensure!(changed.is_empty(), "Stale generated output: {}", changed.join(", "));
        println!("Generated output is up to date for API {}.", args.api_version);
    } else {
        println!("Generated API {}: {} files updated.", args.api_version, changed.len());
    }
    Ok(())
}

fn checked_command(command: &mut Command) -> Result<()> {
    // Large versioned SDKs otherwise retain gigabytes of duplicate incremental
    // state for each feature combination in the conformance matrix.
    command.env("CARGO_INCREMENTAL", "0");
    let status = command.status().with_context(|| format!("Failed to execute {command:?}"))?;
    ensure!(status.success(), "Command failed ({status}): {command:?}");
    Ok(())
}

fn valid_version(version: &str) -> bool {
    version.len() == 7
        && version.as_bytes()[4] == b'-'
        && version[..4].bytes().all(|byte| byte.is_ascii_digit())
        && matches!(
            &version[5..],
            "01" | "02" | "03" | "04" | "05" | "06" | "07" | "08" | "09" | "10" | "11" | "12"
        )
}

fn sync_outputs(
    staging: &Path,
    root: &Path,
    outputs: &[PathBuf],
    check: bool,
) -> Result<Vec<String>> {
    let mut changed = Vec::new();
    for relative in outputs {
        let content = fs::read(staging.join(relative))?;
        let destination = root.join(relative);
        let existing = match fs::read(&destination) {
            Ok(content) => Some(content),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.into()),
        };
        if existing.as_deref() != Some(content.as_slice()) {
            changed.push(relative.display().to_string());
            if !check {
                fs::create_dir_all(destination.parent().unwrap())?;
                fs::write(destination, content)?;
            }
        }
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_rejects_paths_and_invalid_months() {
        for invalid in ["../test", "2026-00", "2026-13", "2026-4", "éé-04"] {
            assert!(!valid_version(invalid));
        }
        assert!(valid_version("2026-04"));
        assert!(valid_version("2026-10"));
    }

    #[test]
    fn check_detects_missing_and_stale_files_without_writing() {
        let staging = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::write(staging.path().join("existing"), "new").unwrap();
        fs::write(staging.path().join("missing"), "new").unwrap();
        fs::write(root.path().join("existing"), "old").unwrap();
        let outputs = vec!["existing".into(), "missing".into()];
        assert_eq!(
            sync_outputs(staging.path(), root.path(), &outputs, true).unwrap(),
            ["existing", "missing"]
        );
        assert_eq!(fs::read_to_string(root.path().join("existing")).unwrap(), "old");
        assert!(!root.path().join("missing").exists());
        sync_outputs(staging.path(), root.path(), &outputs, false).unwrap();
        assert!(sync_outputs(staging.path(), root.path(), &outputs, true).unwrap().is_empty());
    }
}
