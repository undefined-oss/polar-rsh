//! Operations with application/x-www-form-urlencoded request bodies.

use std::path::Path;

use super::operation::{
    OperationDefinition, PayloadMethod, Request, RequestPayload, generate_operation,
};

pub fn generate_form_operation(
    staging_dir: &Path,
    module_path: &str,
    definition: OperationDefinition,
    method: PayloadMethod,
    payload: RequestPayload,
) -> Result<rrgen::GenResult, rrgen::Error> {
    generate_operation(staging_dir, module_path, definition, Request::Form { method, payload })
}
