//! POST and PATCH operations with typed JSON payloads.

use std::path::Path;

use super::operation::{OperationDefinition, Request, generate_operation};

pub enum JsonMethod {
    Post,
    Patch,
}

pub fn generate_json_operation(
    staging_dir: &Path,
    module_path: &str,
    definition: OperationDefinition,
    method: JsonMethod,
    payload_type: &str,
) -> Result<rrgen::GenResult, rrgen::Error> {
    let request = match method {
        JsonMethod::Post => Request::Post(payload_type.into()),
        JsonMethod::Patch => Request::Patch(payload_type.into()),
    };
    generate_operation(staging_dir, module_path, definition, request)
}
