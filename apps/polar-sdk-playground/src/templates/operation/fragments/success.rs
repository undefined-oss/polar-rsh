use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use super::super::{ResponseBody, SuccessResponse, status_code, syntax_error};

pub(super) struct Success {
    pub ty: Type,
    pub declaration: TokenStream,
    pub arms: Vec<TokenStream>,
    pub accept: Option<String>,
}

impl Success {
    pub fn new(operation: &Ident, responses: Vec<SuccessResponse>) -> Result<Self, rrgen::Error> {
        if responses.is_empty() {
            return Err(rrgen::Error::Message("operation requires a success response".into()));
        }
        let multiple = responses.len() > 1;
        let name = format_ident!("{operation}Response");
        let mut variants = Vec::new();
        let mut arms = Vec::new();
        let mut statuses = BTreeSet::new();
        let mut media_types = BTreeSet::new();
        let mut ty = syn::parse_quote!(());
        for response in responses {
            if !(200..300).contains(&response.status) || !statuses.insert(response.status) {
                return Err(rrgen::Error::Message("invalid or duplicate success status".into()));
            }
            let status = status_code::pattern(response.status);
            let variant = status_code::variant(response.status);
            let docs = crate::templates::documentation_lines(response.description.as_deref());
            let empty = matches!(&response.body, ResponseBody::Empty);
            let (payload, decode): (Type, TokenStream) = match response.body {
                ResponseBody::Json(value) => {
                    media_types.insert("application/json".to_owned());
                    let ty: Type = syn::parse_str(&value).map_err(syntax_error)?;
                    let decode = quote! {
                        serde_json::from_slice::<#ty>(&body_slice)
                            .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error)))
                    };
                    (ty, decode)
                }
                ResponseBody::Bytes { media_type } => {
                    media_types.insert(media_type);
                    (syn::parse_quote!(bytes::Bytes), quote!(Ok(body_slice)))
                }
                ResponseBody::Empty => (syn::parse_quote!(()), quote!(Ok(()))),
            };
            let result = if multiple {
                if empty {
                    variants.push(quote! { #(#[doc = #docs])* #variant, });
                    quote!(Ok(#name::#variant))
                } else {
                    variants.push(quote! { #(#[doc = #docs])* #variant(#payload), });
                    quote!((#decode).map(#name::#variant))
                }
            } else {
                decode
            };
            ty = payload;
            arms.push(quote!(#status => #result,));
        }
        let declaration = if multiple {
            ty = syn::parse_quote!(#name);
            quote! {
                #[derive(Debug, Clone)]
                pub enum #name { #(#variants)* }
            }
        } else {
            TokenStream::new()
        };
        let accept = media_types
            .iter()
            .any(|media| media != "application/json")
            .then(|| media_types.into_iter().collect::<Vec<_>>().join(", "));
        Ok(Self { ty, declaration, arms, accept })
    }
}
