use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, ensure};

const SUBMODULE: &str = "vendor/polar";
const INIT: &str = "git submodule update --init --depth 1 -- vendor/polar";

fn git(directory: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(args)
        .output()
        .context("Run Git to verify the Polar submodule")?;
    ensure!(
        output.status.success(),
        "git {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

/// The parent repository's gitlink is the only source of the pinned revision.
pub fn load(root: &Path) -> Result<(PathBuf, String)> {
    let entry = git(root, &["ls-files", "--stage", "--", SUBMODULE])?;
    let fields: Vec<_> = entry.split_whitespace().collect();
    ensure!(
        fields.len() == 4 && fields[0] == "160000" && fields[2] == "0" && fields[3] == SUBMODULE,
        "Expected a staged Git submodule at {SUBMODULE}; resolve any submodule merge conflict \
         first"
    );
    let pinned = fields[1];
    let source = root.join(SUBMODULE);
    ensure!(source.join(".git").exists(), "Polar submodule is not initialized; run `{INIT}`");
    let actual = git(&source, &["rev-parse", "HEAD"])?;
    ensure!(
        actual == pinned,
        "Polar submodule is at {actual}, but the parent repository pins {pinned}; run `{INIT}` or \
         stage an intentional upgrade with `git add {SUBMODULE}`"
    );
    let status = git(
        &source,
        &["status", "--porcelain", "--untracked-files=all", "--ignore-submodules=none"],
    )?;
    ensure!(
        status.is_empty(),
        "Polar submodule has local changes; contract tests require a clean reference \
         checkout:\n{status}"
    );
    Ok((source, actual))
}
