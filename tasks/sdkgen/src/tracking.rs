use std::collections::{BTreeMap, BTreeSet};

use anyhow::Result;
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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Tracking {
    pub format: u32,
    pub upstream_revision: String,
    pub versions: BTreeMap<String, Version>,
}

impl Default for Tracking {
    fn default() -> Self {
        Self { format: 3, upstream_revision: String::new(), versions: BTreeMap::new() }
    }
}

// Historical inventories are decoded strictly, then discarded. Format 3 uses
// BLAKE3, so SHA-256 generation and validation records cannot be reused.
#[derive(Deserialize)]
struct Header {
    format: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireTracking<I> {
    #[serde(rename = "format")]
    _format: u32,
    upstream_revision: String,
    versions: BTreeMap<String, WireVersion<I>>,
    #[serde(rename = "shared")]
    _shared: I,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireVersion<I> {
    source: String,
    #[serde(rename = "observed_hash")]
    _observed_hash: String,
    #[serde(alias = "crate_path")]
    module_path: String,
    #[serde(rename = "status")]
    _status: String,
    #[serde(rename = "generated_hash")]
    _generated_hash: Option<String>,
    #[serde(rename = "validated_hash")]
    _validated_hash: Option<String>,
    #[serde(rename = "generator_fingerprint")]
    _generator_fingerprint: Option<String>,
    #[serde(rename = "validation_fingerprint")]
    _validation_fingerprint: Option<String>,
    #[serde(rename = "files")]
    _files: I,
}

fn migrate<I>(legacy: WireTracking<I>) -> Tracking {
    Tracking {
        upstream_revision: legacy.upstream_revision,
        versions: legacy
            .versions
            .into_iter()
            .map(|(name, record)| {
                (
                    name,
                    Version {
                        source: record.source,
                        module_path: record.module_path,
                        status: "pending".into(),
                        ..Version::default()
                    },
                )
            })
            .collect(),
        ..Tracking::default()
    }
}

pub fn decode_tracking(bytes: &[u8]) -> Result<Tracking> {
    let header: Header = serde_json::from_slice(bytes)?;
    match header.format {
        3 => Ok(serde_json::from_slice(bytes)?),
        2 => Ok(migrate(serde_json::from_slice::<WireTracking<BTreeSet<String>>>(bytes)?)),
        1 => Ok(migrate(serde_json::from_slice::<WireTracking<BTreeMap<String, String>>>(bytes)?)),
        other => anyhow::bail!("Unsupported tracking format {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_formats_drop_inventories_and_invalidate_sha256_records() {
        for format in [1, 2] {
            let inventory = if format == 1 {
                serde_json::json!({"old.rs": "old-output-hash"})
            } else {
                serde_json::json!(["old.rs"])
            };
            let legacy = serde_json::json!({
                "format": format,
                "upstream_revision": "revision",
                "shared": inventory,
                "versions": {"2026-04": {
                    "source": "spec.json",
                    "module_path": "module",
                    "observed_hash": "old-sha256",
                    "status": "validated",
                    "generated_hash": "old-sha256",
                    "validated_hash": "old-sha256",
                    "generator_fingerprint": "old-fingerprint",
                    "validation_fingerprint": "old-fingerprint",
                    "files": inventory,
                }},
            });
            let migrated = decode_tracking(&serde_json::to_vec(&legacy).unwrap()).unwrap();
            assert_eq!(migrated.format, 3);
            let version = &migrated.versions["2026-04"];
            assert_eq!(version.status, "pending");
            assert!(version.observed_hash.is_empty());
            assert!(version.generated_hash.is_none());
            assert!(version.validated_hash.is_none());
            assert!(version.generator_fingerprint.is_none());
            assert!(version.validation_fingerprint.is_none());
            let encoded = serde_json::to_value(&migrated).unwrap();
            assert!(encoded.get("shared").is_none());
            assert!(encoded["versions"]["2026-04"].get("files").is_none());
            assert_eq!(decode_tracking(&serde_json::to_vec(&encoded).unwrap()).unwrap(), migrated);
        }
    }

    #[test]
    fn rejects_unknown_formats_and_malformed_inventories() {
        for source in [
            r#"{"format":4}"#,
            r#"{"format":3,"upstream_revision":"","versions":{},"shared":[]}"#,
            r#"{"format":"2"}"#,
            r#"{"format":2,"upstream_revision":"","versions":{},"shared":{}}"#,
            r#"{"format":1,"upstream_revision":"","versions":{},"shared":[]}"#,
            r#"{"format":2,"upstream_revision":"","versions":{},"shared":[],"typo":true}"#,
        ] {
            assert!(decode_tracking(source.as_bytes()).is_err());
        }
    }
}
