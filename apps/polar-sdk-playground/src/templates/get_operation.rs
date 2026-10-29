//! GET operations with typed paths and JSON responses.

use std::path::Path;

pub use super::operation::{ErrorResponse, OperationDefinition as GetOperation, PathParameter};

pub fn generate_get_operation(
    staging_dir: &Path,
    module_path: &str,
    definition: GetOperation,
) -> Result<rrgen::GenResult, rrgen::Error> {
    super::operation::generate_operation(
        staging_dir,
        module_path,
        definition,
        super::operation::Request::Get,
    )
}
