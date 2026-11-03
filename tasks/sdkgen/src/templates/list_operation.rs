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
