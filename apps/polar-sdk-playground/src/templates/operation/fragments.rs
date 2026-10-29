use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type};

use super::{
    super::operation_errors::{OperationErrors, error_tokens},
    OperationDefinition, Request, syntax_error,
};

mod execution;
use execution::{Execution, IntoFuture};
mod list;
use list::{List, Streams};

pub(super) struct Operation {
    name: Ident,
    error: Ident,
    union: Ident,
    docs: Vec<String>,
    fields: Vec<Parameter>,
    path: TokenStream,
    response: Type,
    success: u16,
    responses: Vec<ResponseArm>,
    errors: TokenStream,
    method: Ident,
    payload: Option<Type>,
    list: Option<List>,
}

impl ToTokens for Operation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, docs, fields, errors, .. } = self;
        let payload = self.payload.as_ref().map(|ty| quote!(pub(super) payload: #ty,));
        let request = RequestBuilder(self);
        let execute = Execution(self);
        let into_future = IntoFuture(self);
        let list = &self.list;
        let params = list.as_ref().map(|list| {
            let name = &list.params;
            quote!(pub(super) params: Option<#name>,)
        });
        let streams = list.as_ref().map(|list| Streams { operation: self, list });
        tokens.extend(quote! {
            #list
            #errors
            #(#[doc = #docs])*
            pub struct #name<'a, ClientService> {
                pub(super) client: &'a PolarClient<ClientService>,
                #(#fields)*
                #payload
                #params
                pub(super) options: RequestOptions,
            }
            #request
            #execute
            #into_future
            #streams
        });
    }
}

struct Parameter {
    name: Ident,
    ty: Type,
}

impl ToTokens for Parameter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, ty } = self;
        tokens.extend(quote!(pub(super) #name: #ty,));
    }
}

struct RequestBuilder<'a>(&'a Operation);

impl ToTokens for RequestBuilder<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Operation { name, path, method, payload, .. } = self.0;
        let serialization = payload.as_ref().map(|_| JsonBody);
        let body = if payload.is_some() { quote!(Some(body.into())) } else { quote!(None) };
        let path = if self.0.list.is_some() {
            quote! {
                let query_params = self.params.as_ref().map_or_default(|params| crate::query::EncodeQuery::encode_query(params));
                let mut path = #path;
                if !query_params.is_empty() { path.push_str(format!("?{query_params}").as_ref()); }
            }
        } else {
            quote!(let path = #path;)
        };
        let params_setter = self.0.list.as_ref().map(|list| {
            let params = &list.params;
            quote! { pub fn params(mut self, params: #params) -> Self { self.params = Some(params); self } }
        });
        tokens.extend(quote! {
            impl<'a, ClientService> #name<'a, ClientService> {
                pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
                    #path
                    #serialization
                    self.client.build_request(http::Method::#method, &path, #body, &self.options)
                }
                pub fn options(mut self, options: RequestOptions) -> Self {
                    self.options = options;
                    self
                }
                #params_setter
            }
        });
    }
}

struct JsonBody;

impl ToTokens for JsonBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! { let body = serde_json::to_vec(&self.payload)?; });
    }
}

struct ResponseArm {
    status: u16,
    payloads: Vec<Type>,
}

impl ToTokens for ResponseArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { status, payloads } = self;
        let (declaration, payload, convert) = if payloads.len() == 1 {
            let payload = &payloads[0];
            (quote!(), quote!(#payload), quote!(eros::ErrorUnion::new(error)))
        } else {
            let variants: Vec<_> =
                (0..payloads.len()).map(|index| format_ident!("Payload{index}")).collect();
            (
                quote! {
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    enum ResponseError { #(#variants(#payloads),)* }
                },
                quote!(ResponseError),
                quote! {
                    match error { #(ResponseError::#variants(error) => eros::ErrorUnion::new(error),)* }
                },
            )
        };
        tokens.extend(quote! {
            #status => {
                #declaration
                let error = serde_json::from_slice::<#payload>(&body_slice)
                    .map_err(|error| eros::ErrorUnion::new(RequestFailure::DecodeResponse(error)))?;
                Err(#convert)
            },
        });
    }
}

impl Operation {
    pub(super) fn new(
        definition: OperationDefinition,
        request: Request,
    ) -> Result<Self, rrgen::Error> {
        let (method, payload, list) = match request {
            Request::Get => (format_ident!("GET"), None, None),
            Request::Post(ty) => {
                (format_ident!("POST"), Some(syn::parse_str(&ty).map_err(syntax_error)?), None)
            }
            Request::Patch(ty) => {
                (format_ident!("PATCH"), Some(syn::parse_str(&ty).map_err(syntax_error)?), None)
            }
            Request::List(config) => (format_ident!("GET"), None, Some(List::try_from(config)?)),
        };
        let name: Ident = syn::parse_str(&definition.name).map_err(syntax_error)?;
        let response = syn::parse_str(&definition.response_type).map_err(syntax_error)?;
        if !(200..300).contains(&definition.success_status) {
            return Err(rrgen::Error::Message("operation success status must be 2xx".into()));
        }
        let mut fields = Vec::new();
        let mut parameters = BTreeMap::new();
        let mut names: BTreeSet<String> = ["client".into(), "options".into()].into();
        if payload.is_some() {
            names.insert("payload".into());
        }
        if list.is_some() {
            names.insert("params".into());
        }
        for parameter in definition.parameters {
            let ident: Ident = syn::parse_str(&parameter.rust_name).map_err(syntax_error)?;
            if !names.insert(ident.to_string().trim_start_matches("r#").into())
                || parameters.insert(parameter.name, ident.clone()).is_some()
            {
                return Err(rrgen::Error::Message("duplicate or reserved path parameter".into()));
            }
            fields.push(Parameter {
                name: ident,
                ty: syn::parse_str(&parameter.rust_type).map_err(syntax_error)?,
            });
        }
        let path = path_expression(&definition.path, &parameters)?;
        let mut responses = BTreeMap::<u16, Vec<Type>>::new();
        let mut api_errors = Vec::new();
        for response in definition.errors {
            if http::StatusCode::from_u16(response.status).is_err()
                || response.status == definition.success_status
            {
                return Err(rrgen::Error::Message("invalid or duplicate response status".into()));
            }
            responses
                .entry(response.status)
                .or_default()
                .push(syn::parse_str(&response.error.payload_type).map_err(syntax_error)?);
            api_errors.push(response.error);
        }
        let errors = error_tokens(OperationErrors {
            operation: definition.name,
            description: None,
            api_errors,
        })?;
        Ok(Self {
            error: format_ident!("{}Error", name),
            union: format_ident!("{}Errors", name),
            name,
            docs: super::super::documentation_lines(definition.description.as_deref()),
            fields,
            path,
            response,
            success: definition.success_status,
            responses: responses
                .into_iter()
                .map(|(status, payloads)| ResponseArm { status, payloads })
                .collect(),
            errors,
            method,
            payload,
            list,
        })
    }
}

fn path_expression(
    path: &str,
    parameters: &BTreeMap<String, Ident>,
) -> Result<TokenStream, rrgen::Error> {
    let mut remaining = path.trim_start_matches('/');
    let mut format = String::new();
    let mut arguments = Vec::new();
    let mut used = BTreeSet::new();
    while let Some((prefix, tail)) = remaining.split_once('{') {
        if prefix.contains('}') {
            return Err(rrgen::Error::Message("unmatched path brace".into()));
        }
        format.push_str(prefix);
        let (name, suffix) = tail
            .split_once('}')
            .ok_or_else(|| rrgen::Error::Message("unclosed path parameter".into()))?;
        let ident = parameters
            .get(name)
            .ok_or_else(|| rrgen::Error::Message(format!("unknown path parameter: {name}")))?;
        used.insert(name.to_owned());
        format.push_str("{}");
        arguments.push(quote!(self.#ident));
        remaining = suffix;
    }
    if remaining.contains('}') || used.len() != parameters.len() {
        return Err(rrgen::Error::Message("path parameters do not match the endpoint".into()));
    }
    format.push_str(remaining);
    Ok(quote!(format!(#format, #(#arguments),*)))
}
