//! Emit shared URL path handling.

use std::path::Path;

pub fn generate_path(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    let result = rrgen::RRgen::with_working_dir(staging_dir)
        .generate(include_str!("../../templates/path.rs.tera"), &serde_json::json!({}))?;
    super::format_rust_file(&staging_dir.join("path.rs"))?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    #[test]
    fn path() -> TestResult {
        assert_generated("path", "path.rs", generate_path)
    }
}
