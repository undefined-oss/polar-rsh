//! Validated scalar values. OpenAPI discovery supplies names and normalized
//! bounds.

use std::path::{Component, Path};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

mod fragments;

#[derive(Clone, Debug, PartialEq)]
pub enum Scalar {
    String { minimum: Option<u64>, maximum: Option<u64> },
    Url { minimum: Option<u64>, maximum: Option<u64> },
    Integer { rust_type: String, minimum: Option<i128>, maximum: Option<i128> },
    Number { minimum: Option<NumberBound>, maximum: Option<NumberBound> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberBound {
    pub value: f64,
    pub exclusive: bool,
}

pub struct Newtype {
    pub name: String,
    pub description: Option<String>,
    pub scalar: Scalar,
}

struct NewtypeFragment {
    name: syn::Ident,
    error: syn::Ident,
    docs: Vec<String>,
    ty: syn::Type,
    derives: TokenStream,
    checks: Vec<fragments::Check>,
    preparation: TokenStream,
    constructor: TokenStream,
    conversions: TokenStream,
}

impl TryFrom<Newtype> for NewtypeFragment {
    type Error = rrgen::Error;

    fn try_from(definition: Newtype) -> Result<Self, Self::Error> {
        let name: syn::Ident = syn::parse_str(&definition.name).map_err(syntax_error)?;
        let error = format_ident!("{name}ValidationError");
        let ty = match &definition.scalar {
            Scalar::String { .. } => syn::parse_quote!(String),
            Scalar::Url { .. } => syn::parse_quote!(url::Url),
            Scalar::Integer { rust_type, .. } => syn::parse_str(rust_type).map_err(syntax_error)?,
            Scalar::Number { .. } => syn::parse_quote!(f64),
        };
        let derives = match &definition.scalar {
            Scalar::String { .. } | Scalar::Url { .. } => quote!(Eq, Ord, Hash),
            Scalar::Integer { .. } => quote!(Copy, Eq, Ord, Hash),
            Scalar::Number { .. } => quote!(Copy),
        };
        let checks = fragments::checks(&definition.scalar);
        let preparation = fragments::preparation(&definition.scalar);
        let constructor = fragments::constructor(&definition.scalar, &ty, &error);
        let conversions = fragments::conversions(&definition.scalar, &name, &error);
        Ok(Self {
            name,
            error,
            docs: super::documentation_lines(definition.description.as_deref()),
            ty,
            derives,
            checks,
            preparation,
            constructor,
            conversions,
        })
    }
}

impl ToTokens for NewtypeFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, error, docs, ty, derives, checks, preparation, constructor, conversions } =
            self;
        let validations = checks.iter().map(|check| check.validation(error));
        let errors = checks.iter().map(fragments::Check::error_variant);
        tokens.extend(quote! {
            #(#[doc = #docs])*
            #[derive(Debug, Clone, PartialEq, PartialOrd, #derives,
                serde::Serialize, derive_more::Display, derive_more::Deref)]
            #[serde(transparent)]
            pub struct #name(#ty);

            #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
            pub enum #error {
                #(#errors)*
            }

            impl #name {
                #constructor

                /// Take the underlying value.
                pub fn into_inner(self) -> #ty {
                    self.0
                }
            }

            impl TryFrom<#ty> for #name {
                type Error = #error;

                fn try_from(value: #ty) -> Result<Self, Self::Error> {
                    #preparation
                    #(#validations)*
                    Ok(Self(value))
                }
            }

            impl From<#name> for #ty {
                fn from(value: #name) -> Self {
                    value.0
                }
            }

            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let value = <#ty as serde::Deserialize>::deserialize(deserializer)?;
                    Self::try_from(value).map_err(serde::de::Error::custom)
                }
            }

            #conversions
        });
    }
}

pub fn generate_newtypes(
    staging_dir: &Path,
    module_path: &str,
    mut definitions: Vec<Newtype>,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if !module_path.ends_with(".rs")
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message("newtypes require a relative Rust file path".into()));
    }
    definitions.sort_by(|a, b| a.name.cmp(&b.name));
    let fragments =
        definitions.into_iter().map(NewtypeFragment::try_from).collect::<Result<Vec<_>, _>>()?;
    let file = syn::parse2(quote!(#(#fragments)*)).map_err(syntax_error)?;
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        include_str!("../../templates/newtypes.rs.tera"),
        &serde_json::json!({
            "module_path": serde_json::to_string(module_path)?,
            "declarations": super::format_rust(file),
        }),
    )
}

fn syntax_error(cause: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid newtype syntax: {cause}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    #[test]
    fn bounded_string() -> TestResult {
        assert_generated("newtype_string", "models/organizations/newtypes.rs", |output| {
            generate_newtypes(
                output,
                "models/organizations/newtypes.rs",
                vec![Newtype {
                    name: "OrganizationName".into(),
                    description: Some("The organization's name.".into()),
                    scalar: Scalar::String { minimum: Some(3), maximum: Some(100) },
                }],
            )
        })
    }

    #[test]
    fn bounded_url() -> TestResult {
        assert_generated("newtype_url", "models/webhooks/newtypes.rs", |output| {
            generate_newtypes(
                output,
                "models/webhooks/newtypes.rs",
                vec![Newtype {
                    name: "WebhookEndpointUrl".into(),
                    description: None,
                    scalar: Scalar::Url { minimum: Some(1), maximum: Some(2083) },
                }],
            )
        })
    }

    #[test]
    fn signed_and_unsigned_bounds() -> TestResult {
        assert_generated("newtype_integers", "models/common/newtypes.rs", |output| {
            generate_newtypes(
                output,
                "models/common/newtypes.rs",
                vec![
                    Newtype {
                        name: "Adjustment".into(),
                        description: None,
                        scalar: Scalar::Integer {
                            rust_type: "i64".into(),
                            minimum: Some(-100),
                            maximum: Some(100),
                        },
                    },
                    Newtype {
                        name: "Quantity".into(),
                        description: None,
                        scalar: Scalar::Integer {
                            rust_type: "u64".into(),
                            minimum: Some(1),
                            maximum: None,
                        },
                    },
                    Newtype {
                        name: "Capacity".into(),
                        description: None,
                        scalar: Scalar::Integer {
                            rust_type: "u32".into(),
                            minimum: None,
                            maximum: Some(1000),
                        },
                    },
                ],
            )
        })
    }

    #[test]
    fn inclusive_and_exclusive_number_bounds() -> TestResult {
        assert_generated("newtype_numbers", "models/common/newtypes.rs", |output| {
            generate_newtypes(
                output,
                "models/common/newtypes.rs",
                vec![
                    Newtype {
                        name: "Ratio".into(),
                        description: None,
                        scalar: Scalar::Number {
                            minimum: Some(NumberBound { value: 0.0, exclusive: false }),
                            maximum: Some(NumberBound { value: 1.0, exclusive: true }),
                        },
                    },
                    Newtype {
                        name: "Weight".into(),
                        description: None,
                        scalar: Scalar::Number {
                            minimum: Some(NumberBound { value: 0.0, exclusive: true }),
                            maximum: Some(NumberBound { value: 100.0, exclusive: false }),
                        },
                    },
                ],
            )
        })
    }
}
