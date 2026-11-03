use std::{path::Path, process::Command};

use anyhow::{Context, Result, ensure};
use clap::Parser;

mod pipeline;

const GENERATOR_VERSION: &str = "sdkgen-native-v1";

fn main() -> Result<()> {
    let args = pipeline::Cli::parse();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
    pipeline::run(workspace, &args)
}

fn checked_command(command: &mut Command) -> Result<()> {
    // Large versioned SDKs otherwise retain gigabytes of duplicate incremental
    // state for each feature combination in the feature matrix.
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
