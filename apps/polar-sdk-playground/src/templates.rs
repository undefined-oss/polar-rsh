//! Experimental template rendering, independent of OpenAPI parsing.

pub mod client;
pub mod error;
pub mod get_operation;
pub mod json_operation;
pub mod list_operation;
pub mod models;
pub mod operation;
pub mod operation_errors;
pub mod pagination;
pub mod query;
pub mod resource;
pub mod string_enums;
pub mod uuid_ids;

/// Normalize line endings while preserving Markdown and paragraph breaks.
fn documentation_lines(description: Option<&str>) -> Vec<String> {
    description
        .unwrap_or_default()
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(String::from)
        .collect()
}
