use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type};

use super::syntax_error;
use crate::templates::operation_query::QueryConfig;

mod fields;
use fields::{EncodeFilter, Filter};

pub(super) struct Query {
    pub name: Ident,
    pub required: bool,
    fields: Vec<Filter>,
    sort: Option<Type>,
    sorting_docs: Vec<String>,
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, required, fields, sort, sorting_docs } = self;
        let encode = fields.iter().map(EncodeFilter);
        let metadata = fields.iter().filter_map(Filter::metadata);
        let sorting_field = sort.as_ref().map(|ty| {
            quote! {
                #(#[doc = #sorting_docs])*
                pub sorting: Vec<(#ty, crate::query::Sorting)>,
            }
        });
        let sorting_hook = sort.as_ref().map(|_| quote! {
            fn sorting_values(&self) -> Vec<String> {
                self.sorting.iter().map(|(field, direction)| direction.encode(field.as_str())).collect()
            }
        });
        let default = (!required).then(|| quote!(, Default));
        let constructor = required.then(|| {
            let required_name = format_ident!("{name}Required");
            let required_fields = fields.iter().filter(|field| field.required);
            let initializers = fields.iter().map(|field| {
                let name = &field.name;
                if field.required {
                    quote!(#name: required.#name,)
                } else {
                    quote!(#name: Default::default(),)
                }
            });
            let sorting = sort.as_ref().map(|_| quote!(sorting: Vec::new(),));
            quote! {
                #[derive(Debug, Clone)]
                pub struct #required_name { #(#required_fields)* }
                impl #name {
                    pub fn new(required: #required_name) -> Self {
                        Self { #(#initializers)* #sorting }
                    }
                }
            }
        });
        tokens.extend(quote! {
            #[derive(Debug, Clone #default)]
            pub struct #name { #(#fields)* #sorting_field }
            #constructor
            #[allow(deprecated)] // Deprecated API parameters still need wire encoding.
            impl crate::query::EncodeQuery for #name {
                #[allow(clippy::vec_init_then_push)] // Fields emit conditional or repeated pairs.
                fn scalar_params(&self) -> Vec<(&'static str, String)> {
                    let mut params = Vec::new();
                    #(#encode)*
                    params
                }
                #sorting_hook
                #(#metadata)*
            }
        });
    }
}

impl TryFrom<QueryConfig> for Query {
    type Error = rrgen::Error;

    fn try_from(config: QueryConfig) -> Result<Self, Self::Error> {
        let required = config.required();
        let mut names = BTreeSet::new();
        if config.sort_type.is_some() {
            names.insert("sorting".to_owned());
        }
        let mut wires = names.clone();
        let mut fields = Vec::new();
        for field in config.fields {
            if !names.insert(field.rust_name.trim_start_matches("r#").to_owned())
                || !wires.insert(field.wire_name.clone())
            {
                return Err(rrgen::Error::Message("duplicate or reserved query field".into()));
            }
            fields.push(Filter::new(field)?);
        }
        Ok(Self {
            name: syn::parse_str(&config.params_name).map_err(syntax_error)?,
            required,
            fields,
            sort: config
                .sort_type
                .as_deref()
                .map(syn::parse_str)
                .transpose()
                .map_err(syntax_error)?,
            sorting_docs: crate::templates::documentation_lines(
                config.sorting_description.as_deref(),
            ),
        })
    }
}
