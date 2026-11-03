//! Model declarations from resolved names, types, and field presence rules.
//! Additional properties are captured only when explicitly allowed by the
//! schema.

use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use quote::quote;

mod fragments;
use fragments::{FieldFragment, ModelFragment};

const TEMPLATE: &str = include_str!("../../templates/models.rs.tera");

pub struct Model {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
    pub derive_default: bool,
    pub derive_error: bool,
    pub additional_properties: Option<AdditionalProperties>,
}

pub struct AdditionalProperties {
    pub rust_name: String,
    pub rust_type: String,
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
    /// The schema fixes this field to a single literal value.
    pub constant: bool,
}

fn syntax_error(error: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid model syntax: {error}"))
}

pub(super) fn field_declarations(
    fields: Vec<Field>,
) -> Result<proc_macro2::TokenStream, rrgen::Error> {
    let fields = fields.into_iter().map(FieldFragment::try_from).collect::<Result<Vec<_>, _>>()?;
    let fields = fields.iter().map(|field| field.declaration(false));
    Ok(quote!(#(#fields)*))
}

/// Emit model declarations into staging; shared helpers are generated
/// separately. `module_path` is a relative Rust file path, such as
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
            "declarations": super::format_rust(file),
        }),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    fn field(name: &str, rust_type: &str) -> Field {
        Field {
            rust_name: name.into(),
            wire_name: name.into(),
            rust_type: rust_type.into(),
            description: None,
            required: true,
            nullable: false,
            constant: false,
        }
    }

    #[test]
    fn presence_literals_and_wire_names() -> TestResult {
        assert_generated("model_presence", "models/customers/types.rs", |output| {
            generate_models(
                output,
                "models/customers/types.rs",
                vec![Model {
                    name: "CustomerCreate".into(),
                    description: Some(
                        "Create a customer.\r\n\r\nOptional values may be supplied with setters."
                            .into(),
                    ),
                    fields: vec![
                        Field {
                            description: Some("The customer's name.".into()),
                            ..field("name", "String")
                        },
                        Field { nullable: true, ..field("external_id", "String") },
                        Field { required: false, ..field("email", "String") },
                        Field { required: false, nullable: true, ..field("nickname", "String") },
                        Field { required: false, ..field("enabled", "bool") },
                        Field {
                            wire_name: "displayName".into(),
                            required: false,
                            ..field("display_name", "String")
                        },
                        Field {
                            rust_name: "r#type".into(),
                            constant: true,
                            ..field("type", "monostate::MustBe!(\"individual\")")
                        },
                    ],
                    derive_default: false,
                    derive_error: false,
                    additional_properties: None,
                }],
            )
        })
    }

    #[test]
    fn optional_fields_and_additional_properties() -> TestResult {
        assert_generated("model_additional_properties", "models/customers/types.rs", |output| {
            generate_models(
                output,
                "models/customers/types.rs",
                vec![Model {
                    name: "CustomerUpdate".into(),
                    description: None,
                    fields: vec![
                        Field { required: false, ..field("name", "String") },
                        Field { required: false, nullable: true, ..field("external_id", "String") },
                    ],
                    derive_default: true,
                    derive_error: false,
                    additional_properties: Some(AdditionalProperties {
                        rust_name: "additional_properties".into(),
                        rust_type: "std::collections::BTreeMap<String, serde_json::Value>".into(),
                    }),
                }],
            )
        })
    }

    #[test]
    fn only_required_literals() -> TestResult {
        assert_generated("model_literals", "models/common/types.rs", |output| {
            generate_models(
                output,
                "models/common/types.rs",
                vec![Model {
                    name: "Confirmation".into(),
                    description: None,
                    fields: vec![
                        Field {
                            constant: true,
                            ..field("status", "monostate::MustBe!(\"confirmed\")")
                        },
                        Field { constant: true, ..field("confirmed", "monostate::MustBe!(true)") },
                    ],
                    derive_default: false,
                    derive_error: false,
                    additional_properties: None,
                }],
            )
        })
    }

    #[test]
    fn error_model() -> TestResult {
        assert_generated("model_error", "models/common/errors.rs", |output| {
            generate_models(
                output,
                "models/common/errors.rs",
                vec![Model {
                    name: "NotFound".into(),
                    description: Some("The requested resource does not exist.".into()),
                    fields: vec![
                        Field {
                            constant: true,
                            ..field("error", "monostate::MustBe!(\"not_found\")")
                        },
                        field("detail", "String"),
                    ],
                    derive_default: false,
                    derive_error: true,
                    additional_properties: None,
                }],
            )
        })
    }
}
