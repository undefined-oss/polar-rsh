//! Paginated GET operations with typed query parameters and lazy streams.
use std::path::Path;

use super::operation::{OperationDefinition, Request, ResponseBody, generate_operation};

pub struct ListConfig {
    pub query: super::operation_query::QueryConfig,
    pub item_type: String,
}

pub fn generate_list_operation(
    staging_dir: &Path,
    module_path: &str,
    mut definition: OperationDefinition,
    config: ListConfig,
) -> Result<rrgen::GenResult, rrgen::Error> {
    let item: syn::Type = syn::parse_str(&config.item_type)
        .map_err(|error| rrgen::Error::Message(format!("invalid list item type: {error}")))?;
    let [success] = definition.successes.as_mut_slice() else {
        return Err(rrgen::Error::Message(
            "paginated operations require one success response".into(),
        ));
    };
    success.body = ResponseBody::Json(
        quote::quote!(crate::models::pagination::ListResource<#item>).to_string(),
    );
    generate_operation(staging_dir, module_path, definition, Request::List(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        operation::{
            PathParameter,
            test_support::{definition, query_field},
        },
        operation_query::{MetadataVariant, QueryConfig, QueryEncoding, QueryField},
        test_support::{TestResult, assert_generated},
    };

    #[test]
    fn pagination_filters_and_sorting() -> TestResult {
        assert_generated("operation_list", "models/customers/list.rs", |output| {
            let mut operation = definition(
                "ListCustomers",
                "/v1/organizations/{organization_id}/customers",
                ResponseBody::Empty,
            );
            operation.parameters = vec![PathParameter {
                name: "organization_id".into(),
                rust_name: "organization_id".into(),
                rust_type: "crate::models::common::OrganizationId".into(),
                percent_encode: false,
            }];
            generate_list_operation(
                output,
                "models/customers/list.rs",
                operation,
                ListConfig {
                    item_type: "crate::models::customers::types::Customer".into(),
                    query: QueryConfig {
                        params_name: "ListCustomersParams".into(),
                        fields: vec![
                            query_field("page", "std::num::NonZeroU64"),
                            query_field("limit", "crate::models::pagination::PageLimit"),
                            QueryField {
                                wire_name: "email".into(),
                                encoding: QueryEncoding::Repeated,
                                ..query_field("emails", "Vec<String>")
                            },
                            QueryField { deprecated: true, ..query_field("active", "bool") },
                            QueryField {
                                encoding: QueryEncoding::Metadata {
                                    value_type: "crate::models::common::unions::MetadataValue"
                                        .into(),
                                    variants: vec![
                                        MetadataVariant { name: "String".into(), repeated: false },
                                        MetadataVariant { name: "Strings".into(), repeated: true },
                                    ],
                                },
                                ..query_field(
                                    "metadata",
                                    "std::collections::BTreeMap<String, \
                                     crate::models::common::unions::MetadataValue>",
                                )
                            },
                        ],
                        sort_type: Some(
                            "crate::models::customers::enums::CustomerSortField".into(),
                        ),
                        sorting_description: Some("Sort customers by one or more fields.".into()),
                    },
                },
            )
        })
    }
}
