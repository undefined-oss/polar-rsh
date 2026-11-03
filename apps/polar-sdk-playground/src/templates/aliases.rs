//! Named schema aliases for scalars, collections, and nullable values.

use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Alias {
    pub name: String,
    pub description: Option<String>,
    pub rust_type: String,
}

struct AliasFragment {
    name: syn::Ident,
    docs: Vec<String>,
    ty: syn::Type,
}

impl ToTokens for AliasFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, docs, ty } = self;
        tokens.extend(quote! {
            #(#[doc = #docs])*
            pub type #name = #ty;
        });
    }
}

pub fn generate_aliases(
    staging_dir: &Path,
    module_path: &str,
    mut definitions: Vec<Alias>,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if !module_path.ends_with(".rs")
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message("aliases require a relative Rust file path".into()));
    }
    definitions.sort_by(|a, b| a.name.cmp(&b.name));
    let mut names = BTreeSet::new();
    let fragments = definitions
        .into_iter()
        .map(|definition| {
            if !names.insert(definition.name.clone()) {
                return Err(rrgen::Error::Message(format!("duplicate alias {}", definition.name)));
            }
            Ok(AliasFragment {
                name: syn::parse_str(&definition.name).map_err(syntax_error)?,
                docs: super::documentation_lines(definition.description.as_deref()),
                ty: syn::parse_str(&definition.rust_type).map_err(syntax_error)?,
            })
        })
        .collect::<Result<Vec<_>, rrgen::Error>>()?;
    let file = syn::parse2(quote!(#(#fragments)*)).map_err(syntax_error)?;
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        include_str!("../../templates/aliases.rs.tera"),
        &serde_json::json!({"module_path": serde_json::to_string(module_path)?, "declarations": super::format_rust(file)}),
    )
}

fn syntax_error(cause: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid alias syntax: {cause}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    #[test]
    fn scalars_collections_and_nullable_values() -> TestResult {
        assert_generated("aliases", "models/common/aliases.rs", |output| {
            generate_aliases(
                output,
                "models/common/aliases.rs",
                vec![
                    Alias {
                        name: "Tags".into(),
                        description: Some("Tags attached to a resource.".into()),
                        rust_type: "Vec<String>".into(),
                    },
                    Alias {
                        name: "Metadata".into(),
                        description: None,
                        rust_type: "std::collections::BTreeMap<String, serde_json::Value>".into(),
                    },
                    Alias {
                        name: "NullableCount".into(),
                        description: Some("A count, or null when unavailable.".into()),
                        rust_type: "Option<u64>".into(),
                    },
                    Alias { name: "Count".into(), description: None, rust_type: "u64".into() },
                ],
            )
        })
    }
}
