use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, ext::IdentExt};

use super::{
    super::{
        bodyless_operation::BodylessMethod,
        operation_errors::{OperationErrors, error_tokens},
    },
    OperationDefinition, PayloadMethod, Request, status_code, syntax_error,
};

mod execution;
use execution::{Execution, IntoFuture};
mod list;
use list::{List, Streams};
mod query;
use query::Query;
mod success;
use success::Success;

pub(super) struct Operation {
    name: Ident,
    error: Ident,
    union: Ident,
    docs: Vec<String>,
    fields: Vec<Parameter>,
    path: TokenStream,
    path_bindings: Vec<TokenStream>,
    response: Type,
    success: Vec<TokenStream>,
    response_declaration: TokenStream,
    responses: Vec<ResponseArm>,
    untyped_errors: Vec<TokenStream>,
    errors: TokenStream,
    method: Ident,
    payload: Option<Payload>,
    list: Option<List>,
    query: Option<Query>,
    accept: Option<String>,
}

impl ToTokens for Operation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, docs, fields, errors, response_declaration, .. } = self;
        let payload = self.payload.as_ref().map(|payload| {
            let ty = &payload.ty;
            if payload.required {
                quote!(pub(super) payload: #ty,)
            } else {
                quote!(pub(super) payload: Option<#ty>,)
            }
        });
        let request = RequestBuilder(self);
        let execute = Execution(self);
        let into_future = IntoFuture(self);
        let list = &self.list;
        let query = &self.query;
        let params = query.as_ref().map(|query| {
            let name = &query.name;
            if query.required {
                quote!(pub(super) params: #name,)
            } else {
                quote!(pub(super) params: Option<#name>,)
            }
        });
        let streams = list.as_ref().map(|list| Streams { operation: self, list });
        tokens.extend(quote! {
            #query
            #response_declaration
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

struct Payload {
    ty: Type,
    required: bool,
    form: bool,
}

impl ToTokens for RequestBuilder<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Operation { name, path, path_bindings, method, payload, .. } = self.0;
        let serialization = payload.as_ref().map(PayloadBody);
        let body = match payload {
            Some(payload) if payload.required => quote!(Some(body.into())),
            Some(_) => quote!(body.map(Into::into)),
            None => quote!(None),
        };
        let payload_setter = payload.as_ref().filter(|payload| !payload.required).map(|payload| {
            let ty = &payload.ty;
            quote! {
                /// Set the optional request body.
                pub fn payload(mut self, payload: #ty) -> Self {
                    self.payload = Some(payload);
                    self
                }
            }
        });
        let path = if let Some(query) = &self.0.query {
            let encode = if query.required {
                quote!(crate::query::EncodeQuery::encode_query(&self.params))
            } else {
                quote!(
                    self.params
                        .as_ref()
                        .map_or_else(String::new, crate::query::EncodeQuery::encode_query)
                )
            };
            quote! {
                let query_params = #encode;
                let mut path = #path;
                if !query_params.is_empty() { path.push_str(format!("?{query_params}").as_ref()); }
            }
        } else {
            quote!(let path = #path;)
        };
        let params_setter = self.0.query.as_ref().map(|query| {
            let params = &query.name;
            let value = if query.required { quote!(params) } else { quote!(Some(params)) };
            quote! { pub fn params(mut self, params: #params) -> Self { self.params = #value; self } }
        });
        let build =
            quote!(self.client.build_request(http::Method::#method, &path, #body, &self.options));
        let accept = self.0.accept.as_ref().map(|accept| quote! {
            request.headers_mut().insert(http::header::ACCEPT, http::HeaderValue::from_static(#accept));
        });
        let content_type = payload.as_ref().filter(|payload| payload.form).map(|payload| {
            let header = quote! {
                request.headers_mut().insert(http::header::CONTENT_TYPE,
                    http::HeaderValue::from_static("application/x-www-form-urlencoded"));
            };
            if payload.required {
                header
            } else {
                quote! { if self.payload.is_some() { #header } }
            }
        });
        let build = if accept.is_some() || content_type.is_some() {
            quote! {
                let mut request = #build?;
                #accept
                #content_type
                Ok(request)
            }
        } else {
            build
        };
        tokens.extend(quote! {
            impl<'a, ClientService> #name<'a, ClientService> {
                pub(crate) fn build_request(&self) -> Result<http::Request<RequestBody>, BuildRequestError> {
                    #(#path_bindings)*
                    #path
                    #serialization
                    #build
                }
                pub fn options(mut self, options: RequestOptions) -> Self {
                    self.options = options;
                    self
                }
                #params_setter
                #payload_setter
            }
        });
    }
}

struct PayloadBody<'a>(&'a Payload);

impl ToTokens for PayloadBody<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let serializer = if self.0.form {
            quote!(serde_urlencoded::to_string)
        } else {
            quote!(serde_json::to_vec)
        };
        if self.0.required {
            tokens.extend(quote! { let body = #serializer(&self.payload)?; });
        } else {
            tokens.extend(quote! {
                let body = self.payload.as_ref().map(#serializer).transpose()?;
            });
        }
    }
}

struct ResponseArm {
    status: TokenStream,
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
        mut definition: OperationDefinition,
        request: Request,
    ) -> Result<Self, rrgen::Error> {
        let form = matches!(&request, Request::Form { .. });
        let (method, payload, list) = match request {
            Request::Bodyless(method) => {
                let method = match method {
                    BodylessMethod::Get => format_ident!("GET"),
                    BodylessMethod::Post => format_ident!("POST"),
                    BodylessMethod::Patch => format_ident!("PATCH"),
                    BodylessMethod::Put => format_ident!("PUT"),
                    BodylessMethod::Delete => format_ident!("DELETE"),
                };
                (method, None, None)
            }
            Request::Json { method, payload } | Request::Form { method, payload } => {
                let method = match method {
                    PayloadMethod::Post => format_ident!("POST"),
                    PayloadMethod::Patch => format_ident!("PATCH"),
                    PayloadMethod::Put => format_ident!("PUT"),
                };
                let payload = Payload {
                    ty: syn::parse_str(&payload.rust_type).map_err(syntax_error)?,
                    required: payload.required,
                    form,
                };
                (method, Some(payload), None)
            }
            Request::List(config) => {
                definition.query = Some(config.query);
                let item = syn::parse_str(&config.item_type).map_err(syntax_error)?;
                (format_ident!("GET"), None, Some(List { item }))
            }
        };
        let query = definition.query.map(Query::try_from).transpose()?;
        let name: Ident = syn::parse_str(&definition.name).map_err(syntax_error)?;
        let success_statuses: BTreeSet<_> =
            definition.successes.iter().map(|response| response.status).collect();
        let success = Success::new(&name, definition.successes)?;
        let mut fields = Vec::new();
        let mut parameters = BTreeMap::new();
        let mut path_bindings = Vec::new();
        let mut names: BTreeSet<String> = ["client".into(), "options".into()].into();
        if payload.is_some() {
            names.insert("payload".into());
        }
        if query.is_some() {
            names.insert("params".into());
        }
        for parameter in definition.parameters {
            let ident: Ident = syn::parse_str(&parameter.rust_name).map_err(syntax_error)?;
            let expression = if parameter.percent_encode {
                let encoded = format_ident!("encoded_{}", ident.unraw());
                path_bindings
                    .push(quote!(let #encoded = crate::path::encode_segment(&self.#ident);));
                quote!(#encoded)
            } else {
                quote!(self.#ident)
            };
            if !names.insert(ident.to_string().trim_start_matches("r#").into())
                || parameters.insert(parameter.name, expression).is_some()
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
        let mut error_types = BTreeSet::new();
        let untyped_errors: BTreeSet<_> = definition.untyped_errors.into_iter().collect();
        if untyped_errors.iter().any(|status| !(400..600).contains(status)) {
            return Err(rrgen::Error::Message("untyped error statuses must be 4xx or 5xx".into()));
        }
        for response in definition.errors {
            if http::StatusCode::from_u16(response.status).is_err()
                || success_statuses.contains(&response.status)
                || untyped_errors.contains(&response.status)
            {
                return Err(rrgen::Error::Message("invalid or duplicate response status".into()));
            }
            responses
                .entry(response.status)
                .or_default()
                .push(syn::parse_str(&response.error.payload_type).map_err(syntax_error)?);
            // Eros needs each payload type once, even when several statuses use
            // it.
            if error_types.insert(response.error.payload_type.clone()) {
                api_errors.push(response.error);
            }
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
            path_bindings,
            response: success.ty,
            response_declaration: success.declaration,
            success: success.arms,
            responses: responses
                .into_iter()
                .map(|(status, payloads)| ResponseArm {
                    status: status_code::pattern(status),
                    payloads,
                })
                .collect(),
            untyped_errors: untyped_errors.into_iter().map(status_code::pattern).collect(),
            errors,
            method,
            payload,
            list,
            query,
            accept: success.accept,
        })
    }
}

fn path_expression(
    path: &str,
    parameters: &BTreeMap<String, TokenStream>,
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
        let expression = parameters
            .get(name)
            .ok_or_else(|| rrgen::Error::Message(format!("unknown path parameter: {name}")))?;
        used.insert(name.to_owned());
        format.push_str("{}");
        arguments.push(expression);
        remaining = suffix;
    }
    if remaining.contains('}') || used.len() != parameters.len() {
        return Err(rrgen::Error::Message("path parameters do not match the endpoint".into()));
    }
    format.push_str(remaining);
    Ok(if arguments.is_empty() {
        quote!(#format.to_owned())
    } else {
        quote!(format!(#format, #(#arguments),*))
    })
}
