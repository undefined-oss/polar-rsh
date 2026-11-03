//! Shared operations with typed paths and response bodies.

use std::path::{Component, Path};

use quote::ToTokens;

use super::operation_errors::ApiError;

mod fragments;
mod status_code;

pub struct OperationDefinition {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub parameters: Vec<PathParameter>,
    pub query: Option<super::operation_query::QueryConfig>,
    pub successes: Vec<SuccessResponse>,
    pub errors: Vec<ErrorResponse>,
    pub untyped_errors: Vec<u16>,
}

pub struct SuccessResponse {
    pub status: u16,
    pub description: Option<String>,
    pub body: ResponseBody,
}

pub enum ResponseBody {
    Json(String),
    Bytes { media_type: String },
    Empty,
}

pub enum PayloadMethod {
    Post,
    Patch,
    Put,
}

pub struct RequestPayload {
    pub rust_type: String,
    pub required: bool,
}

pub struct PathParameter {
    pub name: String,
    pub rust_name: String,
    pub rust_type: String,
    pub percent_encode: bool,
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
        &serde_json::json!({"module_path": serde_json::to_string(module_path)?, "declarations": super::format_rust(file)}),
    )
}

pub(super) enum Request {
    Bodyless(super::bodyless_operation::BodylessMethod),
    Json { method: PayloadMethod, payload: RequestPayload },
    Form { method: PayloadMethod, payload: RequestPayload },
    List(super::list_operation::ListConfig),
}
