//! Webhook verification and parsing.

use std::path::Path;

pub fn generate_verification(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    let result = rrgen::RRgen::with_working_dir(staging_dir)
        .generate(include_str!("../../templates/webhooks.rs.tera"), &serde_json::json!({}))?;
    super::format_rust_file(&staging_dir.join("models/webhooks/verification.rs"))?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    #[test]
    fn webhook_verification() -> TestResult {
        assert_generated(
            "webhook_verification",
            "models/webhooks/verification.rs",
            generate_verification,
        )
    }
}
