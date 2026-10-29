//! Operation errors composed from shared failures and resolved API payloads.

use std::path::{Component, Path};

use quote::ToTokens;

mod fragments;

const TEMPLATE: &str = include_str!("../../templates/operation_errors.rs.tera");

pub struct OperationErrors {
    pub operation: String,
    pub description: Option<String>,
    pub api_errors: Vec<ApiError>,
}

pub struct ApiError {
    pub variant: String,
    pub payload_type: String,
    pub description: Option<String>,
    pub message: String,
}

fn syntax_error(error: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid operation error syntax: {error}"))
}

pub(super) fn error_tokens(
    definition: OperationErrors,
) -> Result<proc_macro2::TokenStream, rrgen::Error> {
    Ok(fragments::OperationErrorsFragment::try_from(definition)?.into_token_stream())
}

pub fn generate_operation_errors(
    staging_dir: &Path,
    module_path: &str,
    definition: OperationErrors,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if !module_path.ends_with(".rs")
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message(
            "operation errors require a relative Rust file path".into(),
        ));
    }
    let file = syn::parse2(error_tokens(definition)?).map_err(syntax_error)?;
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        TEMPLATE,
        &serde_json::json!({
            "module_path": serde_json::to_string(module_path)?,
            "declarations": prettyplease::unparse(&file),
        }),
    )
}
