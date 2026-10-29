//! Model declarations from resolved names, types, and field presence rules.

use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use quote::quote;

mod fragments;
use fragments::ModelFragment;

const TEMPLATE: &str = include_str!("../../templates/models.rs.tera");
const HELPERS: &str = include_str!("../../templates/serde_helpers.rs.tera");

pub struct Model {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub derive_default: bool,
}

/// `rust_type` is the resolved underlying type, without the presence wrapper.
/// Names and type paths are resolved before emission; wire names remain
/// unchanged.
pub struct Field {
    pub rust_name: String,
    pub wire_name: String,
    pub rust_type: String,
    pub description: Option<String>,
    pub required: bool,
    pub nullable: bool,
}

fn syntax_error(error: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid model syntax: {error}"))
}

/// Emit model declarations and their shared Serde helpers into staging.
/// `module_path` is a relative Rust file path, such as
/// `models/organization/types.rs`.
pub fn generate_models(
    staging_dir: &Path,
    module_path: &str,
    mut models: Vec<Model>,
) -> Result<(), rrgen::Error> {
    if !module_path.ends_with(".rs")
        || module_path == "models/common/serde_helpers.rs"
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message(
            "models require a relative Rust file path distinct from the Serde helpers".into(),
        ));
    }
    models.sort_by(|a, b| a.name.cmp(&b.name));
    let declarations =
        models.into_iter().map(ModelFragment::try_from).collect::<Result<Vec<_>, _>>()?;
    let mut names = BTreeSet::new();
    for declaration in &declarations {
        for name in std::iter::once(declaration.name.clone()).chain(declaration.required_name()) {
            let name = name.to_string().trim_start_matches("r#").to_owned();
            if !names.insert(name.clone()) {
                return Err(rrgen::Error::Message(format!(
                    "duplicate generated model name: {name}"
                )));
            }
        }
    }
    let file: syn::File = syn::parse2(quote!(#(#declarations)*)).map_err(syntax_error)?;
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    let generator = rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera);
    generator.generate(
        TEMPLATE,
        &serde_json::json!({
            // rrgen's frontmatter is YAML; quote the destination independently of Rust syntax.
            "module_path": serde_json::to_string(module_path)?,
            "declarations": prettyplease::unparse(&file),
        }),
    )?;
    generator.generate(HELPERS, &serde_json::json!({}))?;
    Ok(())
}
