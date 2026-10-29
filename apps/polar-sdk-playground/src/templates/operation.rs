//! Shared operations with typed paths and JSON responses.

use std::path::{Component, Path};

use quote::ToTokens;

use super::operation_errors::ApiError;

mod fragments;

pub struct OperationDefinition {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub parameters: Vec<PathParameter>,
    pub response_type: String,
    pub success_status: u16,
    pub errors: Vec<ErrorResponse>,
}

pub struct PathParameter {
    pub name: String,
    pub rust_name: String,
    pub rust_type: String,
}

pub struct ErrorResponse {
    pub status: u16,
    pub error: ApiError,
}

fn syntax_error(error: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid operation syntax: {error}"))
}

pub(super) fn generate_operation(
    staging_dir: &Path,
    module_path: &str,
    definition: OperationDefinition,
    request: Request,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if !module_path.ends_with(".rs")
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message("operations require a relative Rust file path".into()));
    }
    let fragment = fragments::Operation::new(definition, request)?;
    let file = syn::parse2(fragment.into_token_stream()).map_err(syntax_error)?;
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        include_str!("../../templates/operation.rs.tera"),
        &serde_json::json!({"module_path": serde_json::to_string(module_path)?, "declarations": prettyplease::unparse(&file)}),
    )
}

pub(super) enum Request {
    Get,
    Post(String),
    Patch(String),
    List(super::list_operation::ListConfig),
}
