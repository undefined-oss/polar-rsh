//! POST, PUT and PATCH operations with typed JSON payloads.

use std::path::Path;

use super::operation::{
    OperationDefinition, PayloadMethod, Request, RequestPayload, generate_operation,
};

pub fn generate_json_operation(
    staging_dir: &Path,
    module_path: &str,
    definition: OperationDefinition,
    method: PayloadMethod,
    payload: RequestPayload,
) -> Result<rrgen::GenResult, rrgen::Error> {
    generate_operation(staging_dir, module_path, definition, Request::Json { method, payload })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        operation::{ResponseBody, test_support::definition},
        test_support::{TestResult, assert_generated},
    };

    #[test]
    fn json_payloads() -> TestResult {
        for (snapshot, method, required) in [
            ("operation_json_post", PayloadMethod::Post, true),
            ("operation_json_patch_optional", PayloadMethod::Patch, false),
            ("operation_json_put", PayloadMethod::Put, true),
        ] {
            assert_generated(snapshot, "models/customers/save.rs", |output| {
                let mut operation = definition(
                    "SaveCustomer",
                    "/v1/customers",
                    ResponseBody::Json("crate::models::customers::types::Customer".into()),
                );
                operation.successes[0].status = 201;
                generate_json_operation(
                    output,
                    "models/customers/save.rs",
                    operation,
                    method,
                    RequestPayload {
                        rust_type: "crate::models::customers::types::CustomerCreate".into(),
                        required,
                    },
                )
            })?;
        }
        Ok(())
    }
}
