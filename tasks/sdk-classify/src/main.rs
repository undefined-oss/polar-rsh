use std::{
    collections::BTreeMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
};

use anyhow::{Context, Result, ensure};
use serde::Deserialize;

#[derive(Deserialize)]
struct Inventory {
    versions: BTreeMap<String, Spec>,
}

#[derive(Deserialize, PartialEq, Eq)]
struct Spec {
    observed_hash: String,
}

fn hashes(bytes: &[u8]) -> Result<BTreeMap<String, Spec>> {
    let inventory: Inventory = serde_json::from_slice(bytes).context("Invalid SDK inventory")?;
    ensure!(!inventory.versions.is_empty(), "SDK inventory has no API versions");
    ensure!(
        inventory.versions.values().all(|spec| {
            spec.observed_hash.len() == 64
                && spec.observed_hash.bytes().all(|byte| byte.is_ascii_hexdigit())
        }),
        "SDK inventory contains an invalid content hash"
    );
    Ok(inventory.versions)
}

fn classify(before: &[u8], after: &[u8]) -> Result<&'static str> {
    Ok(if hashes(before)? == hashes(after)? { "chore" } else { "feat" })
}

fn main() -> Result<()> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let [before, after] = args.as_slice() else {
        anyhow::bail!("Usage: sdk-classify BEFORE.json AFTER.json");
    };
    let kind = classify(&fs::read(before)?, &fs::read(after)?)?;
    if let Some(path) = env::var_os("GITHUB_OUTPUT") {
        let mut output = OpenOptions::new().append(true).open(path)?;
        writeln!(output, "kind={kind}")?;
    }
    println!("{kind}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inventory(versions: &[(&str, char)], revision: &str) -> String {
        let versions = versions
            .iter()
            .map(|(version, hash)| {
                format!(
                    r#""{version}":{{"observed_hash":"{}","validation_fingerprint":"{revision}"}}"#,
                    hash.to_string().repeat(64)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(r#"{{"upstream_revision":"{revision}","versions":{{{versions}}}}}"#)
    }

    #[test]
    fn pin_fingerprint_and_key_order_changes_are_chores() {
        let before = inventory(&[("2026-04", 'a'), ("2026-10", 'b')], "old");
        let after = inventory(&[("2026-10", 'b'), ("2026-04", 'a')], "new");
        assert_eq!(classify(before.as_bytes(), after.as_bytes()).unwrap(), "chore");
        assert_eq!(classify(before.as_bytes(), before.as_bytes()).unwrap(), "chore");
    }

    #[test]
    fn modified_added_and_removed_specs_are_features() {
        let before = inventory(&[("2026-04", 'a'), ("2026-10", 'b')], "old");
        for versions in [
            vec![("2026-04", 'c'), ("2026-10", 'b')],
            vec![("2026-04", 'a'), ("2026-10", 'b'), ("2027-01", 'c')],
            vec![("2026-04", 'a')],
        ] {
            let after = inventory(&versions, "new");
            assert_eq!(classify(before.as_bytes(), after.as_bytes()).unwrap(), "feat");
        }
    }

    #[test]
    fn malformed_or_missing_hashes_cannot_be_classified_as_safe() {
        let valid = inventory(&[("2026-04", 'a')], "old");
        for invalid in [
            "{}",
            r#"{"versions":{}}"#,
            r#"{"versions":{"2026-04":{}}}"#,
            r#"{"versions":{"2026-04":{"observed_hash":null}}}"#,
            r#"{"versions":{"2026-04":{"observed_hash":"bad"}}}"#,
            &inventory(&[("2026-04", 'z')], "old"),
        ] {
            assert!(classify(valid.as_bytes(), invalid.as_bytes()).is_err());
            assert!(classify(invalid.as_bytes(), valid.as_bytes()).is_err());
        }
    }
}
