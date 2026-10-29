use std::collections::BTreeSet;

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Type};

use super::{super::super::list_operation::ListConfig, Operation, syntax_error};

pub(super) struct List {
    pub params: Ident,
    sort: Ident,
    variants: Vec<Ident>,
    values: Vec<String>,
    filters: Vec<Filter>,
    item: Type,
}

struct Filter {
    name: Ident,
    wire: String,
    ty: Type,
    docs: Vec<String>,
}

impl ToTokens for Filter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, ty, docs, .. } = self;
        tokens.extend(quote! { #(#[doc = #docs])* pub #name: Option<#ty>, });
    }
}

struct EncodeFilter<'a>(&'a Filter);
impl ToTokens for EncodeFilter<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Filter { name, wire, .. } = self.0;
        tokens.extend(quote! {
            if let Some(value) = &self.#name { params.push((#wire, value.to_string())); }
        });
    }
}

impl ToTokens for List {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { params, sort, variants, values, filters, .. } = self;
        let encode = filters.iter().map(EncodeFilter);
        tokens.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum #sort { #(#variants,)* }
            impl #sort {
                pub fn as_str(&self) -> &'static str { match self { #(Self::#variants => #values,)* } }
            }
            impl std::fmt::Display for #sort {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.as_str()) }
            }
            #[derive(Debug, Clone, Default)]
            pub struct #params {
                #(#filters)*
                pub page: Option<std::num::NonZeroU64>,
                pub limit: Option<crate::models::pagination::PageLimit>,
                pub sorting: Vec<(#sort, crate::query::Sorting)>,
            }
            impl crate::query::EncodeQuery for #params {
                fn scalar_params(&self) -> Vec<(&'static str, String)> {
                    let mut params = Vec::new();
                    #(#encode)*
                    if let Some(page) = self.page { params.push(("page", page.to_string())); }
                    if let Some(limit) = self.limit { params.push(("limit", limit.to_string())); }
                    params
                }
                fn sorting_values(&self) -> Vec<String> {
                    self.sorting.iter().map(|(field, direction)| direction.encode(field.as_str())).collect()
                }
            }
        });
    }
}

pub(super) struct Streams<'a> {
    pub operation: &'a Operation,
    pub list: &'a List,
}

impl ToTokens for Streams<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Operation { name, error, response, fields, .. } = self.operation;
        let item = &self.list.item;
        let field_names: Vec<_> = fields.iter().map(|field| &field.name).collect();
        tokens.extend(quote! {
            impl<'a, ClientService, Body> #name<'a, ClientService>
            where
                ClientService: tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
                ClientService::Future: 'a,
                ClientService::Error: 'a,
                Body: http_body::Body<Data = bytes::Bytes> + 'a,
                Body::Error: Into<tower::BoxError>,
            {
                pub fn pages(self) -> impl futures_util::Stream<Item = Result<#response, #error<ClientService::Error>>> + 'a {
                    futures_util::stream::try_unfold(Some(self), |state| async move {
                        let Some(mut request) = state else { return Ok(None); };
                        let mut params = request.params.take().unwrap_or_default();
                        let current = params.page.unwrap_or(std::num::NonZeroU64::MIN);
                        params.page = Some(current);
                        let client = request.client;
                        let options = request.options.clone();
                        #(let #field_names = request.#field_names.clone();)*
                        let response = request.params(params.clone()).send().await?;
                        let next = response.pagination.next_page(current).map(|page| {
                            params.page = Some(page);
                            Self { client, options, params: Some(params), #(#field_names,)* }
                        });
                        Ok(Some((response, next)))
                    })
                }
                pub fn items(self) -> impl futures_util::Stream<Item = Result<#item, #error<ClientService::Error>>> + 'a {
                    use futures_util::TryStreamExt;
                    self.pages().map_ok(|page| futures_util::stream::iter(page.items.into_iter().map(Ok))).try_flatten()
                }
            }
        });
    }
}

impl TryFrom<ListConfig> for List {
    type Error = rrgen::Error;

    fn try_from(config: ListConfig) -> Result<Self, Self::Error> {
        let params: Ident = syn::parse_str(&config.params_name).map_err(syntax_error)?;
        let sort: Ident = syn::parse_str(&config.sort_name).map_err(syntax_error)?;
        if params == sort {
            return Err(rrgen::Error::Message(
                "list parameter and sorting type names must differ".into(),
            ));
        }
        let mut names = BTreeSet::new();
        let mut variants = Vec::new();
        for value in &config.sort_values {
            let variant: Ident =
                syn::parse_str(&value.to_upper_camel_case()).map_err(syntax_error)?;
            if !names.insert(variant.to_string()) {
                return Err(rrgen::Error::Message("duplicate sorting variant".into()));
            }
            variants.push(variant);
        }
        let mut names: BTreeSet<String> = ["page", "limit", "sorting"].map(String::from).into();
        let mut wires = names.clone();
        let mut filters = Vec::new();
        for field in config.filters {
            let name: Ident = syn::parse_str(&field.rust_name).map_err(syntax_error)?;
            if !names.insert(name.to_string().trim_start_matches("r#").into())
                || !wires.insert(field.wire_name.clone())
            {
                return Err(rrgen::Error::Message("duplicate or reserved query field".into()));
            }
            filters.push(Filter {
                name,
                wire: field.wire_name,
                ty: syn::parse_str(&field.rust_type).map_err(syntax_error)?,
                docs: super::super::super::documentation_lines(field.description.as_deref()),
            });
        }
        Ok(Self {
            params,
            sort,
            variants,
            values: config.sort_values,
            filters,
            item: syn::parse_str(&config.item_type).map_err(syntax_error)?,
        })
    }
}
