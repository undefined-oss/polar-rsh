//! Emit shared helpers for optional and nullable model fields.

use std::path::Path;

const TEMPLATE: &str = include_str!("../../templates/serde_helpers.rs.tera");

pub fn generate_serde_helpers(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    let result =
        rrgen::RRgen::with_working_dir(staging_dir).generate(TEMPLATE, &serde_json::json!({}))?;
    super::format_rust_file(&staging_dir.join("models/common/serde_helpers.rs"))?;
    Ok(result)
}
