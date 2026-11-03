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
        super::operation::Request::Bodyless(super::bodyless_operation::BodylessMethod::Get),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        operation::{
            ResponseBody, SuccessResponse,
            test_support::{api_error, definition, query_field},
        },
        operation_query::{QueryConfig, QueryField},
        test_support::{TestResult, assert_generated},
    };

    #[test]
    fn get_paths_and_typed_errors() -> TestResult {
        assert_generated("operation_get", "models/customers/get.rs", |output| {
            let mut operation = definition(
                "GetCustomer",
                "/v1/organizations/{organization_id}/customers/external/{external_id}",
                ResponseBody::Json("crate::models::customers::types::Customer".into()),
            );
            operation.description =
                Some("Get a customer by external ID.\n\nRequired scopes: `customers:read`.".into());
            operation.parameters = vec![
                PathParameter {
                    name: "organization_id".into(),
                    rust_name: "organization_id".into(),
                    rust_type: "crate::models::common::OrganizationId".into(),
                    percent_encode: false,
                },
                PathParameter {
                    name: "external_id".into(),
                    rust_name: "external_id".into(),
                    rust_type: "String".into(),
                    percent_encode: true,
                },
            ];
            operation.query = Some(QueryConfig {
                params_name: "GetCustomerParams".into(),
                fields: vec![query_field("include_deleted", "bool")],
                sort_type: None,
                sorting_description: None,
            });
            operation.errors = vec![
                ErrorResponse {
                    status: 401,
                    error: api_error("Unauthorized", "crate::models::common::errors::Unauthorized"),
                },
                ErrorResponse {
                    status: 403,
                    error: api_error("Unauthorized", "crate::models::common::errors::Unauthorized"),
                },
                ErrorResponse {
                    status: 404,
                    error: api_error("NotFound", "crate::models::common::errors::NotFound"),
                },
                ErrorResponse {
                    status: 422,
                    error: api_error(
                        "Validation",
                        "crate::models::common::errors::HTTPValidationError",
                    ),
                },
                ErrorResponse {
                    status: 422,
                    error: api_error(
                        "InvalidRequest",
                        "crate::models::common::errors::InvalidRequest",
                    ),
                },
            ];
            operation.untyped_errors = vec![429, 503];
            generate_get_operation(output, "models/customers/get.rs", operation)
        })
    }

    #[test]
    fn binary_response() -> TestResult {
        assert_generated("operation_binary", "models/orders/download.rs", |output| {
            generate_get_operation(
                output,
                "models/orders/download.rs",
                definition(
                    "DownloadInvoice",
                    "/v1/invoices/download",
                    ResponseBody::Bytes { media_type: "application/pdf".into() },
                ),
            )
        })
    }

    #[test]
    fn multiple_success_statuses() -> TestResult {
        assert_generated("operation_success_statuses", "models/orders/receipt.rs", |output| {
            let mut operation =
                definition("GetReceipt", "/v1/receipts", ResponseBody::Json("String".into()));
            operation.successes[0].description = Some("The receipt is ready.".into());
            operation.successes.push(SuccessResponse {
                status: 202,
                description: Some("The receipt is being generated.".into()),
                body: ResponseBody::Empty,
            });
            generate_get_operation(output, "models/orders/receipt.rs", operation)
        })
    }

    #[test]
    fn required_query_parameters() -> TestResult {
        assert_generated("operation_required_query", "models/oauth2/authorize.rs", |output| {
            let mut operation = definition(
                "Authorize",
                "/v1/oauth2/authorize",
                ResponseBody::Json("serde_json::Value".into()),
            );
            operation.query = Some(QueryConfig {
                params_name: "AuthorizeParams".into(),
                fields: vec![
                    QueryField {
                        required: true,
                        description: Some("The OAuth client identifier.".into()),
                        ..query_field("client_id", "String")
                    },
                    QueryField {
                        wire_name: "redirect_uri".into(),
                        ..query_field("redirect", "url::Url")
                    },
                    query_field("state", "String"),
                ],
                sort_type: None,
                sorting_description: None,
            });
            generate_get_operation(output, "models/oauth2/authorize.rs", operation)
        })
    }
}
