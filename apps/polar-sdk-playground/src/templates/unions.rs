//! Typed unions with discriminator-aware or structural deserialization.

use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Union {
    pub name: String,
    pub description: Option<String>,
    pub exclusive: bool,
    pub discriminator: Option<String>,
    pub variants: Vec<Variant>,
}

pub struct Variant {
    pub name: String,
    pub description: Option<String>,
    pub rust_type: String,
    pub tags: Vec<String>,
    /// Payload fields excluding a fixed discriminator, when available.
    pub fields: Option<Vec<super::models::Field>>,
}

struct UnionFragment {
    name: syn::Ident,
    docs: Vec<String>,
    exclusive: bool,
    discriminator: Option<String>,
    variants: Vec<VariantFragment>,
}

struct VariantFragment {
    name: syn::Ident,
    docs: Vec<String>,
    ty: syn::Type,
    tags: Vec<String>,
    fields: Option<TokenStream>,
}

impl ToTokens for VariantFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, ty, docs, .. } = self;
        tokens.extend(quote!(#(#[doc = #docs])*));
        if let Some(fields) = &self.fields {
            let tag = &self.tags[0];
            tokens.extend(quote! {
                #[serde(rename = #tag)]
                #name { #fields },
            });
        } else {
            tokens.extend(quote!(#name(#ty),));
        }
    }
}

impl UnionFragment {
    fn deserialize(&self) -> TokenStream {
        let name = self.name.to_string();
        if let Some(property) = &self.discriminator {
            let arms = self.variants.iter().map(|variant| {
                let VariantFragment { name, ty, tags, .. } = variant;
                quote! {
                    #(#tags)|* => serde_json::from_value::<#ty>(value)
                        .map(Self::#name).map_err(serde::de::Error::custom),
                }
            });
            return quote! {
                let tag = value.get(#property).and_then(serde_json::Value::as_str)
                    .ok_or_else(|| serde::de::Error::custom(concat!("missing or non-string discriminator ", #property, " for ", #name)))?
                    .to_owned();
                match tag.as_str() {
                    #(#arms)*
                    _ => Err(serde::de::Error::custom(format!("unknown {} discriminator {:?} for {}", #property, tag, #name))),
                }
            };
        }
        let attempts = self.variants.iter().map(|variant| {
            let VariantFragment { name: variant_name, ty, .. } = variant;
            quote! {
                if let Ok(payload) = serde_json::from_value::<#ty>(value.clone()) {
                    if matched.is_some() {
                        return Err(serde::de::Error::custom(concat!("ambiguous oneOf value for ", #name)));
                    }
                    matched = Some(Self::#variant_name(payload));
                }
            }
        });
        quote! {
            let mut matched = None;
            #(#attempts)*
            matched.ok_or_else(|| serde::de::Error::custom(concat!("no matching oneOf variant for ", #name)))
        }
    }
}

impl ToTokens for UnionFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, docs, variants, .. } = self;
        let tagged = self
            .discriminator
            .as_ref()
            .filter(|_| variants.iter().all(|variant| variant.fields.is_some()));
        if tagged.is_some() || (self.discriminator.is_none() && !self.exclusive) {
            let representation = match tagged {
                Some(property) => quote!(#[serde(tag = #property, deny_unknown_fields)]),
                None => quote!(#[serde(untagged)]),
            };
            tokens.extend(quote! {
                #(#[doc = #docs])*
                #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                #representation
                pub enum #name { #(#variants)* }
            });
            return;
        }
        let deserialize = self.deserialize();
        tokens.extend(quote! {
            #(#[doc = #docs])*
            #[derive(Debug, Clone, serde::Serialize)]
            #[serde(untagged)]
            pub enum #name { #(#variants)* }

            impl<'de> serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de> {
                    let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
                    #deserialize
                }
            }
        });
    }
}

pub fn generate_unions(
    staging_dir: &Path,
    module_path: &str,
    mut definitions: Vec<Union>,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if !module_path.ends_with(".rs")
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message("unions require a relative Rust file path".into()));
    }
    definitions.sort_by(|a, b| a.name.cmp(&b.name));
    let mut names = BTreeSet::new();
    let fragments = definitions
        .into_iter()
        .map(|definition| {
            if !names.insert(definition.name.clone()) || definition.variants.len() < 2 {
                return Err(rrgen::Error::Message(format!(
                    "invalid or duplicate union {}",
                    definition.name
                )));
            }
            let mut variant_names = BTreeSet::new();
            let tagged = definition.discriminator.is_some()
                && definition.variants.iter().all(|variant| variant.fields.is_some());
            let variants = definition
                .variants
                .into_iter()
                .map(|variant| {
                    if !variant_names.insert(variant.name.clone())
                        || (definition.discriminator.is_some() && variant.tags.is_empty())
                    {
                        return Err(rrgen::Error::Message(
                            "duplicate union variant or missing discriminator tags".into(),
                        ));
                    }
                    let ty: syn::Type = syn::parse_str(&variant.rust_type).map_err(syntax_error)?;
                    Ok(VariantFragment {
                        name: syn::parse_str(&variant.name).map_err(syntax_error)?,
                        docs: super::documentation_lines(variant.description.as_deref()),
                        ty,
                        tags: variant.tags,
                        fields: if tagged {
                            Some(super::models::field_declarations(variant.fields.unwrap())?)
                        } else {
                            None
                        },
                    })
                })
                .collect::<Result<Vec<_>, rrgen::Error>>()?;
            Ok(UnionFragment {
                name: syn::parse_str(&definition.name).map_err(syntax_error)?,
                docs: super::documentation_lines(definition.description.as_deref()),
                exclusive: definition.exclusive,
                discriminator: definition.discriminator,
                variants,
            })
        })
        .collect::<Result<Vec<_>, rrgen::Error>>()?;
    let file = syn::parse2(quote!(#(#fragments)*)).map_err(syntax_error)?;
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        include_str!("../../templates/unions.rs.tera"),
        &serde_json::json!({"module_path": serde_json::to_string(module_path)?, "declarations": super::format_rust(file)}),
    )
}

fn syntax_error(cause: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid union syntax: {cause}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        models::Field,
        test_support::{TestResult, assert_generated},
    };

    fn scalar_variants() -> Vec<Variant> {
        vec![
            Variant {
                name: "Text".into(),
                description: Some("A textual value.".into()),
                rust_type: "String".into(),
                tags: vec![],
                fields: None,
            },
            Variant {
                name: "Integer".into(),
                description: None,
                rust_type: "i64".into(),
                tags: vec![],
                fields: None,
            },
        ]
    }

    #[test]
    fn untagged_any_of() -> TestResult {
        assert_generated("union_untagged", "models/common/unions.rs", |output| {
            generate_unions(
                output,
                "models/common/unions.rs",
                vec![Union {
                    name: "TextOrInteger".into(),
                    description: Some("A value supplied as text or an integer.".into()),
                    exclusive: false,
                    discriminator: None,
                    variants: scalar_variants(),
                }],
            )
        })
    }

    #[test]
    fn exclusive_one_of() -> TestResult {
        assert_generated("union_exclusive", "models/common/unions.rs", |output| {
            generate_unions(
                output,
                "models/common/unions.rs",
                vec![Union {
                    name: "TextOrInteger".into(),
                    description: None,
                    exclusive: true,
                    discriminator: None,
                    variants: scalar_variants(),
                }],
            )
        })
    }

    #[test]
    fn tagged_webhook_with_boxed_data() -> TestResult {
        assert_generated("union_tagged_webhook", "models/webhooks/events.rs", |output| {
            let variants = [
                (
                    "CheckoutCreated",
                    "checkout.created",
                    "crate::models::checkouts::types::Checkout",
                ),
                ("ProductUpdated", "product.updated", "crate::models::products::types::Product"),
            ]
            .into_iter()
            .map(|(name, tag, payload)| Variant {
                name: name.into(),
                description: Some(format!("The {tag} event.")),
                rust_type: payload.into(),
                tags: vec![tag.into()],
                fields: Some(vec![
                    Field {
                        rust_name: "timestamp".into(),
                        wire_name: "timestamp".into(),
                        rust_type: "jiff::Timestamp".into(),
                        description: None,
                        required: true,
                        nullable: false,
                        constant: false,
                    },
                    Field {
                        rust_name: "api_version".into(),
                        wire_name: "api_version".into(),
                        rust_type: "String".into(),
                        description: None,
                        required: true,
                        nullable: false,
                        constant: false,
                    },
                    Field {
                        rust_name: "data".into(),
                        wire_name: "data".into(),
                        rust_type: format!("Box<{payload}>"),
                        description: Some("The affected resource.".into()),
                        required: true,
                        nullable: false,
                        constant: false,
                    },
                ]),
            })
            .collect();
            generate_unions(
                output,
                "models/webhooks/events.rs",
                vec![Union {
                    name: "WebhookPayload".into(),
                    description: Some("An incoming webhook payload.".into()),
                    exclusive: true,
                    discriminator: Some("type".into()),
                    variants,
                }],
            )
        })
    }

    #[test]
    fn custom_discriminator_with_shared_tags() -> TestResult {
        assert_generated("union_custom_discriminator", "models/meters/unions.rs", |output| {
            generate_unions(
                output,
                "models/meters/unions.rs",
                vec![Union {
                    name: "MeterAggregation".into(),
                    description: None,
                    exclusive: true,
                    discriminator: Some("func".into()),
                    variants: vec![
                        Variant {
                            name: "Count".into(),
                            description: None,
                            rust_type: "crate::models::meters::types::CountAggregation".into(),
                            tags: vec!["count".into()],
                            fields: None,
                        },
                        Variant {
                            name: "Property".into(),
                            description: Some("An aggregation over a property.".into()),
                            rust_type: "crate::models::meters::types::PropertyAggregation".into(),
                            tags: ["avg", "max", "min", "sum"].map(String::from).into(),
                            fields: None,
                        },
                    ],
                }],
            )
        })
    }
}
