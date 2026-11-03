use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Type};

use super::{FieldFragment, Presence};

pub(super) struct RequiredFields<'a> {
    pub name: &'a Ident,
    pub fields: &'a [FieldFragment],
}

impl ToTokens for RequiredFields<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name;
        let fields = self.fields.iter().filter(|field| field.requires_argument()).map(|field| {
            let FieldFragment { name, ty, docs, .. } = field;
            quote! { #docs pub #name: #ty, }
        });
        tokens.extend(quote! {
            #[derive(Debug, Clone)]
            pub struct #name {
                #(#fields)*
            }
        });
    }
}

pub(super) struct Constructor<'a> {
    pub required_name: Option<&'a Ident>,
    pub fields: &'a [FieldFragment],
    pub use_default: bool,
}

impl ToTokens for Constructor<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let argument = self.required_name.map(|name| quote!(required: #name));
        let body = if self.use_default {
            quote!(Self::default())
        } else {
            let fields = self.fields.iter().map(Initializer);
            quote!(Self { #(#fields)* })
        };
        tokens.extend(quote! {
            pub fn new(#argument) -> Self {
                #body
            }
        });
    }
}

struct Initializer<'a>(&'a FieldFragment);

impl ToTokens for Initializer<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.0.name;
        if self.0.presence.is_required() && self.0.constant {
            tokens.extend(quote!(#name: Default::default(),));
            return;
        }
        let value = match self.0.presence {
            Presence::Required | Presence::RequiredNullable => quote!(required.#name),
            Presence::Optional => quote!(None),
            Presence::OptionalNullable => quote!(presence_rs::Presence::Absent),
            Presence::AdditionalProperties => quote!(Default::default()),
        };
        tokens.extend(quote!(#name: #value,));
    }
}

pub(super) struct Setter<'a>(pub &'a FieldFragment);

impl ToTokens for Setter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldFragment { name, base, docs, presence, .. } = self.0;
        let (parameter, value) = if is_string(base) {
            (quote!(impl Into<#base>), quote!(value.into()))
        } else {
            (quote!(#base), quote!(value))
        };
        let assignment = match presence {
            Presence::OptionalNullable => quote!(presence_rs::Presence::Some(#value)),
            Presence::Optional | Presence::RequiredNullable => quote!(Some(#value)),
            Presence::Required | Presence::AdditionalProperties => value,
        };
        tokens.extend(quote! {
            #docs
            pub fn #name(mut self, value: #parameter) -> Self {
                self.#name = #assignment;
                self
            }
        });
    }
}

fn is_string(ty: &Type) -> bool {
    let Type::Path(path) = ty else { return false };
    if path.qself.is_some()
        || path.path.segments.iter().any(|segment| !segment.arguments.is_empty())
    {
        return false;
    }
    let segments: Vec<_> =
        path.path.segments.iter().map(|segment| segment.ident.to_string()).collect();
    segments == ["String"]
        || segments == ["std", "string", "String"]
        || segments == ["alloc", "string", "String"]
}
