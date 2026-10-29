//! Emit the shared error module.

use std::path::Path;

const TEMPLATE: &str = include_str!("../../templates/error.rs.tera");

/// Write the shared error module beneath an explicit staging directory.
pub fn generate_error(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    rrgen::RRgen::with_working_dir(staging_dir).generate(TEMPLATE, &serde_json::json!({}))
}
