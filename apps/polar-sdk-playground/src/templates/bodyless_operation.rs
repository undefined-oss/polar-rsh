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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        operation::{PathParameter, ResponseBody, test_support::definition},
        test_support::{TestResult, assert_generated},
    };

    #[test]
    fn bodyless_mutations() -> TestResult {
        for (snapshot, method, status) in [
            ("operation_bodyless_post", BodylessMethod::Post, 202),
            ("operation_bodyless_patch", BodylessMethod::Patch, 200),
            ("operation_bodyless_put", BodylessMethod::Put, 200),
            ("operation_delete", BodylessMethod::Delete, 204),
        ] {
            assert_generated(snapshot, "models/customers/action.rs", |output| {
                let mut operation =
                    definition("CustomerAction", "/v1/customers/{id}", ResponseBody::Empty);
                operation.parameters = vec![PathParameter {
                    name: "id".into(),
                    rust_name: "id".into(),
                    rust_type: "crate::models::common::CustomerId".into(),
                    percent_encode: false,
                }];
                operation.successes[0].status = status;
                generate_bodyless_operation(output, "models/customers/action.rs", operation, method)
            })?;
        }
        Ok(())
    }
}
