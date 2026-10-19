use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Version {
    pub source: String,
    pub observed_hash: String,
    #[serde(alias = "crate_path")]
    pub module_path: String,
    pub status: String,
    pub generated_hash: Option<String>,
    pub validated_hash: Option<String>,
    pub generator_fingerprint: Option<String>,
    pub validation_fingerprint: Option<String>,
    pub files: BTreeSet<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Tracking {
    pub format: u32,
    pub upstream_revision: String,
    pub versions: BTreeMap<String, Version>,
    pub shared: BTreeSet<String>,
}

impl Default for Tracking {
    fn default() -> Self {
        Self {
            format: 2,
            upstream_revision: String::new(),
            versions: BTreeMap::new(),
            shared: BTreeSet::new(),
        }
    }
}

// Decode each historical format into its own typed representation. This keeps
// legacy map inventories from being accepted accidentally in the current
// format.
#[derive(Deserialize)]
struct Header {
    format: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireTracking<I> {
    format: u32,
    upstream_revision: String,
    versions: BTreeMap<String, WireVersion<I>>,
    shared: I,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireVersion<I> {
    source: String,
    observed_hash: String,
    #[serde(alias = "crate_path")]
    module_path: String,
    status: String,
    generated_hash: Option<String>,
    validated_hash: Option<String>,
    generator_fingerprint: Option<String>,
    validation_fingerprint: Option<String>,
    files: I,
}

pub fn decode_tracking(bytes: &[u8]) -> Result<Tracking> {
    let header: Header = serde_json::from_slice(bytes)?;
    match header.format {
        2 => Ok(serde_json::from_slice(bytes)?),
        1 => {
            let legacy: WireTracking<BTreeMap<String, String>> = serde_json::from_slice(bytes)?;
            ensure!(legacy.format == 1, "Unsupported tracking format");
            Ok(Tracking {
                format: 2,
                upstream_revision: legacy.upstream_revision,
                shared: legacy.shared.into_keys().collect(),
                versions: legacy
                    .versions
                    .into_iter()
                    .map(|(name, record)| {
                        (
                            name,
                            Version {
                                source: record.source,
                                observed_hash: record.observed_hash,
                                module_path: record.module_path,
                                status: record.status,
                                generated_hash: record.generated_hash,
                                validated_hash: record.validated_hash,
                                generator_fingerprint: record.generator_fingerprint,
                                validation_fingerprint: record.validation_fingerprint,
                                files: record.files.into_keys().collect(),
                            },
                        )
                    })
                    .collect(),
            })
        }
        other => anyhow::bail!("Unsupported tracking format {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_formats_and_malformed_inventories() {
        for source in [
            r#"{"format":3}"#,
            r#"{"format":"2"}"#,
            r#"{"format":2,"upstream_revision":"","versions":{},"shared":{}}"#,
            r#"{"format":1,"upstream_revision":"","versions":{},"shared":[]}"#,
            r#"{"format":2,"upstream_revision":"","versions":{},"shared":[],"typo":true}"#,
        ] {
            assert!(decode_tracking(source.as_bytes()).is_err());
        }
    }
}
