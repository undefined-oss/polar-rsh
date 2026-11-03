use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;

use super::Operation;

pub(super) struct List {
    pub item: Type,
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
