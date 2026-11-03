use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Type};

use super::super::syntax_error;
use crate::templates::operation_query::{QueryEncoding, QueryField};

pub(super) struct Filter {
    pub(super) name: Ident,
    wire: String,
    ty: Type,
    docs: Vec<String>,
    deprecated: bool,
    pub(super) required: bool,
    encoding: Encoding,
}

enum Encoding {
    Scalar,
    Repeated,
    Metadata { ty: Box<Type>, variants: Vec<(Ident, bool)> },
}

impl Filter {
    pub(super) fn new(field: QueryField) -> Result<Self, rrgen::Error> {
        let encoding = match field.encoding {
            QueryEncoding::Scalar => Encoding::Scalar,
            QueryEncoding::Repeated => Encoding::Repeated,
            QueryEncoding::Metadata { value_type, variants } => Encoding::Metadata {
                ty: Box::new(syn::parse_str(&value_type).map_err(syntax_error)?),
                variants: variants
                    .into_iter()
                    .map(|variant| {
                        Ok((syn::parse_str(&variant.name).map_err(syntax_error)?, variant.repeated))
                    })
                    .collect::<Result<_, rrgen::Error>>()?,
            },
        };
        Ok(Self {
            name: syn::parse_str(&field.rust_name).map_err(syntax_error)?,
            wire: field.wire_name,
            ty: syn::parse_str(&field.rust_type).map_err(syntax_error)?,
            docs: crate::templates::documentation_lines(field.description.as_deref()),
            deprecated: field.deprecated,
            required: field.required,
            encoding,
        })
    }

    pub(super) fn metadata(&self) -> Option<TokenStream> {
        let Encoding::Metadata { ty, variants } = &self.encoding else { return None };
        let name = &self.name;
        let arms = variants.iter().map(|(variant, repeated)| {
            let values = if *repeated {
                quote!(values.iter().map(ToString::to_string).collect())
            } else {
                quote!(vec![values.to_string()])
            };
            quote!(#ty::#variant(values) => #values,)
        });
        Some(quote! {
            fn metadata_params(&self) -> Vec<(String, Vec<String>)> {
                self.#name.iter().map(|(key, value)| {
                    let values = match value { #(#arms)* };
                    (key.clone(), values)
                }).collect()
            }
        })
    }
}

impl ToTokens for Filter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, ty, docs, deprecated, encoding, required, .. } = self;
        let ty = if !required && matches!(encoding, Encoding::Scalar) {
            quote!(Option<#ty>)
        } else {
            quote!(#ty)
        };
        let deprecated = deprecated.then(|| quote!(#[deprecated]));
        tokens.extend(quote! { #(#[doc = #docs])* #deprecated pub #name: #ty, });
    }
}

pub(super) struct EncodeFilter<'a>(pub &'a Filter);
impl ToTokens for EncodeFilter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Filter { name, wire, encoding, required, .. } = self.0;
        match encoding {
            Encoding::Scalar if *required => tokens.extend(quote! {
                params.push((#wire, self.#name.to_string()));
            }),
            Encoding::Scalar => tokens.extend(quote! {
                if let Some(value) = &self.#name { params.push((#wire, value.to_string())); }
            }),
            Encoding::Repeated => tokens.extend(quote! {
                for value in &self.#name { params.push((#wire, value.to_string())); }
            }),
            Encoding::Metadata { .. } => {}
        }
    }
}
