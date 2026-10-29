//! UUID ID emission from resolved Rust names.

use std::path::Path;

use serde::Serialize;

const UUID_IDS: &str = include_str!("../../templates/uuid_ids.rs.tera");
const UUID_ID_MACRO: &str = include_str!("../../templates/macros/uuid_id.rs.tera");

/// A resolved Rust ID name. Semantic identity is determined before rendering.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UuidId {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
struct UuidIdContext {
    name: String,
    documentation: Vec<String>,
}

#[derive(Serialize)]
struct UuidIds {
    ids: Vec<UuidIdContext>,
}

/// Write the complete shared ID file beneath an explicit staging directory.
///
/// Callers supply resolved, valid Rust type names. Repeated names represent the
/// same ID; ordering and duplicate uses do not affect the emitted file.
pub fn generate_uuid_ids(
    staging_dir: &Path,
    mut ids: Vec<UuidId>,
) -> Result<rrgen::GenResult, rrgen::Error> {
    ids.sort();
    ids.dedup();
    if ids.windows(2).any(|pair| pair[0].name == pair[1].name) {
        return Err(rrgen::Error::Message("duplicate ID has conflicting descriptions".into()));
    }
    let ids = ids
        .into_iter()
        .map(|id| UuidIdContext {
            documentation: super::documentation_lines(id.description.as_deref()),
            name: id.name,
        })
        .collect();

    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_templates([
        ("macros/uuid_id.rs.tera", UUID_ID_MACRO),
        ("uuid_ids.rs.tera", UUID_IDS),
    ])?;

    rrgen::RRgen::with_working_dir(staging_dir)
        .add_template_engine(tera)
        .generate(UUID_IDS, &serde_json::to_value(UuidIds { ids })?)
}
