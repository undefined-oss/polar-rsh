//! Operations without a request body.

use std::path::Path;

use super::operation::{OperationDefinition, Request, generate_operation};

pub enum BodylessMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

pub fn generate_bodyless_operation(
    staging_dir: &Path,
    module_path: &str,
    definition: OperationDefinition,
    method: BodylessMethod,
) -> Result<rrgen::GenResult, rrgen::Error> {
    generate_operation(staging_dir, module_path, definition, Request::Bodyless(method))
}
