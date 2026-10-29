//! Paginated GET operations with typed query parameters and lazy streams.
use std::path::Path;

use super::operation::{OperationDefinition, Request, generate_operation};

pub struct ListConfig {
    pub params_name: String,
    pub sort_name: String,
    pub sort_values: Vec<String>,
    pub filters: Vec<QueryField>,
    pub item_type: String,
}

pub struct QueryField {
    pub rust_name: String,
    pub wire_name: String,
    pub rust_type: String,
    pub description: Option<String>,
}

pub fn generate_list_operation(
    staging_dir: &Path,
    module_path: &str,
    mut definition: OperationDefinition,
    config: ListConfig,
) -> Result<rrgen::GenResult, rrgen::Error> {
    let item: syn::Type = syn::parse_str(&config.item_type)
        .map_err(|error| rrgen::Error::Message(format!("invalid list item type: {error}")))?;
    definition.response_type =
        quote::quote!(crate::models::pagination::ListResource<#item>).to_string();
    generate_operation(staging_dir, module_path, definition, Request::List(config))
}
