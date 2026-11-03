//! Emit pagination helpers using resolved page-limit constraints.

use std::path::Path;

use serde::Serialize;

const TEMPLATE: &str = include_str!("../../templates/pagination.rs.tera");

/// Bounds for the playground's `u8` page-limit representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PageLimitConfig {
    pub minimum: u8,
    pub maximum: u8,
    pub default: u8,
}

impl Default for PageLimitConfig {
    fn default() -> Self {
        Self { minimum: 1, maximum: 100, default: 10 }
    }
}

/// Write pagination types after validating the inclusive bounds and default.
pub fn generate_pagination(
    staging_dir: &Path,
    config: PageLimitConfig,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if config.minimum == 0
        || config.minimum > config.maximum
        || !(config.minimum..=config.maximum).contains(&config.default)
    {
        return Err(rrgen::Error::Message(
            "page limits require 1 <= minimum <= default <= maximum <= 255".into(),
        ));
    }
    let result = rrgen::RRgen::with_working_dir(staging_dir)
        .generate(TEMPLATE, &serde_json::to_value(config)?)?;
    super::format_rust_file(&staging_dir.join("models/pagination.rs"))?;
    Ok(result)
}
