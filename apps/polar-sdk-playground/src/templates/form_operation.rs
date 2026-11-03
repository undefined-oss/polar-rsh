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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        operation::{ResponseBody, test_support::definition},
        test_support::{TestResult, assert_generated},
    };

    #[test]
    fn form_payloads() -> TestResult {
        for (snapshot, required) in [("operation_form", true), ("operation_form_optional", false)] {
            assert_generated(snapshot, "models/oauth2/token.rs", |output| {
                generate_form_operation(
                    output,
                    "models/oauth2/token.rs",
                    definition(
                        "RequestToken",
                        "/v1/oauth2/token",
                        ResponseBody::Json("serde_json::Value".into()),
                    ),
                    PayloadMethod::Post,
                    RequestPayload {
                        rust_type: "std::collections::BTreeMap<String, String>".into(),
                        required,
                    },
                )
            })?;
        }
        Ok(())
    }
}
