//! Emit the shared query module.

use std::path::Path;

const TEMPLATE: &str = include_str!("../../templates/query.rs.tera");

/// Write the shared query module beneath an explicit staging directory.
pub fn generate_query(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    rrgen::RRgen::with_working_dir(staging_dir).generate(TEMPLATE, &serde_json::json!({}))
}
