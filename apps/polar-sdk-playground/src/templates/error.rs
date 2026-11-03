//! Emit the shared error module.

use std::path::Path;

const TEMPLATE: &str = include_str!("../../templates/error.rs.tera");

/// Write the shared error module beneath an explicit staging directory.
pub fn generate_error(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    let result =
        rrgen::RRgen::with_working_dir(staging_dir).generate(TEMPLATE, &serde_json::json!({}))?;
    super::format_rust_file(&staging_dir.join("error.rs"))?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    #[test]
    fn error() -> TestResult {
        assert_generated("error", "error.rs", generate_error)
    }
}
