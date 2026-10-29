use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type};

use super::{OperationErrors, syntax_error};

pub(super) struct OperationErrorsFragment {
    name: Ident,
    union_name: Ident,
    docs: Vec<String>,
    api_errors: Vec<ApiErrorFragment>,
}

impl ToTokens for OperationErrorsFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, union_name, docs, api_errors } = self;
        let shared = SharedVariants;
        let conversion = UnionConversion { name, union_name, api_errors };
        tokens.extend(quote! {
            #(#[doc = #docs])*
            #[derive(Debug, thiserror::Error)]
            pub enum #name<TransportError> {
                #[error("HTTP transport failed: {0}")]
                Transport(#[source] TransportError),
                #shared
                #(#api_errors)*
            }
            #conversion
        });
    }
}

struct ApiErrorFragment {
    variant: Ident,
    payload: Type,
    docs: Vec<String>,
    message: String,
}

impl ToTokens for ApiErrorFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { variant, payload, docs, message } = self;
        tokens.extend(quote! {
            #(#[doc = #docs])*
            #[error("{message}: {0}", message = #message)]
            #variant(#[source] #payload),
        });
    }
}

struct SharedVariants;

impl ToTokens for SharedVariants {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            #[error("failed to build request: {0}")]
            BuildRequest(#[source] crate::error::BuildRequestError),
            #[error("failed to read response body: {0}")]
            ReadBody(#[source] tower::BoxError),
            #[error("failed to decode response: {0}")]
            DecodeResponse(#[source] serde_json::Error),
            #[error("unexpected HTTP status: {status}")]
            UnexpectedStatus { status: http::StatusCode, body: bytes::Bytes },
            #[error("request timed out")]
            Timeout(#[source] tokio::time::error::Elapsed),
        });
    }
}

struct UnionConversion<'a> {
    name: &'a Ident,
    union_name: &'a Ident,
    api_errors: &'a [ApiErrorFragment],
}

impl ToTokens for UnionConversion<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, union_name, api_errors } = self;
        let payloads = api_errors.iter().map(|error| &error.payload);
        let union_enum = format_ident!("E{}", api_errors.len() + 1);
        let arms = api_errors.iter().enumerate().map(|(index, error)| {
            let slot = format_ident!("{}", char::from(b'B' + index as u8));
            let variant = &error.variant;
            quote!(eros::#union_enum::#slot(error) => Self::#variant(error),)
        });
        let failures = FailureArms;
        tokens.extend(quote! {
            pub(super) type #union_name = (crate::error::RequestFailure, #(#payloads,)*);

            impl<T> From<eros::ErrorUnion<#union_name>> for #name<T> {
                fn from(error: eros::ErrorUnion<#union_name>) -> Self {
                    match error.into_enum() {
                        eros::#union_enum::A(error) => match error { #failures },
                        #(#arms)*
                    }
                }
            }

            impl<T> #name<T> {
                pub(super) fn from_failure(error: crate::error::RequestFailure) -> Self {
                    let union: eros::ErrorUnion<#union_name> = eros::ErrorUnion::new(error);
                    union.into()
                }
            }
        });
    }
}

struct FailureArms;

impl ToTokens for FailureArms {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            crate::error::RequestFailure::BuildRequest(error) => Self::BuildRequest(error),
            crate::error::RequestFailure::ReadBody(error) => Self::ReadBody(error),
            crate::error::RequestFailure::DecodeResponse(error) => Self::DecodeResponse(error),
            crate::error::RequestFailure::UnexpectedStatus { status, body } => Self::UnexpectedStatus { status, body },
            crate::error::RequestFailure::Timeout(error) => Self::Timeout(error),
        });
    }
}

impl TryFrom<OperationErrors> for OperationErrorsFragment {
    type Error = rrgen::Error;

    fn try_from(definition: OperationErrors) -> Result<Self, Self::Error> {
        let operation: Ident = syn::parse_str(&definition.operation).map_err(syntax_error)?;
        // Eros 0.8 supports E1 through E26; RequestFailure occupies the first
        // slot.
        if definition.api_errors.len() > 25 {
            return Err(rrgen::Error::Message(
                "Eros supports at most 25 API error payloads per operation".into(),
            ));
        }
        let mut variants: BTreeSet<String> = [
            "Transport",
            "BuildRequest",
            "ReadBody",
            "DecodeResponse",
            "UnexpectedStatus",
            "Timeout",
        ]
        .map(String::from)
        .into();
        let mut payloads = BTreeSet::new();
        let mut api_errors = Vec::new();
        for error in definition.api_errors {
            let variant: Ident = syn::parse_str(&error.variant).map_err(syntax_error)?;
            let payload: Type = syn::parse_str(&error.payload_type).map_err(syntax_error)?;
            if !variants.insert(variant.to_string().trim_start_matches("r#").to_owned()) {
                return Err(rrgen::Error::Message(format!(
                    "duplicate operation error variant: {variant}"
                )));
            }
            if !payloads.insert(payload.to_token_stream().to_string()) {
                return Err(rrgen::Error::Message(
                    "API error payload types must be distinct".into(),
                ));
            }
            api_errors.push(ApiErrorFragment {
                variant,
                payload,
                docs: super::super::documentation_lines(error.description.as_deref()),
                message: error.message,
            });
        }
        Ok(Self {
            name: format_ident!("{}Error", operation),
            union_name: format_ident!("{}Errors", operation),
            docs: super::super::documentation_lines(definition.description.as_deref()),
            api_errors,
        })
    }
}
