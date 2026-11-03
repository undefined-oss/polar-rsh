use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use super::Operation;

pub(super) struct Execution<'a>(pub &'a Operation);

impl ToTokens for Execution<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Operation { name, error, union, response, success, responses, .. } = self.0;
        let untyped_errors = self.0.untyped_errors.iter().map(|status| {
            quote! {
                #status => Err(eros::ErrorUnion::new(RequestFailure::HttpStatus {
                    status: response_code,
                    body: body_slice,
                })),
            }
        });
        tokens.extend(quote! {
            impl<'a, ClientService, Body> #name<'a, ClientService>
            where
                ClientService: tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone,
                Body: http_body::Body<Data = bytes::Bytes>,
                Body::Error: Into<tower::BoxError>,
            {
                async fn execute(self) -> Result<#response, #error<ClientService::Error>> {
                    let request = self.build_request().map_err(|error| #error::from_failure(RequestFailure::BuildRequest(error)))?;
                    let response = self.client.send(request).await.map_err(#error::Transport)?;
                    let result: eros::Result<_, #union> = async {
                        let response_code = response.status();
                        let body_slice = response.into_body().collect().await
                            .map_err(|error| eros::ErrorUnion::new(RequestFailure::ReadBody(error.into())))?.to_bytes();
                        match response_code {
                            #(#success)*
                            #(#responses)*
                            #(#untyped_errors)*
                            _ => Err(eros::ErrorUnion::new(RequestFailure::UnexpectedStatus { status: response_code, body: body_slice })),
                        }
                    }.await;
                    result.map_err(Into::into)
                }

                pub async fn send(self) -> Result<#response, #error<ClientService::Error>> {
                    let duration = self.options.timeout.unwrap_or_else(|| self.client.default_timeout());
                    tokio::time::timeout(duration, self.execute()).await
                        .map_err(|error| #error::from_failure(RequestFailure::Timeout(error)))?
                }
            }
        });
    }
}

pub(super) struct IntoFuture<'a>(pub &'a Operation);

impl ToTokens for IntoFuture<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Operation { name, error, response, .. } = self.0;
        tokens.extend(quote! {
            impl<'a, ClientService, Body> std::future::IntoFuture for #name<'a, ClientService>
            where
                ClientService: tower::Service<http::Request<RequestBody>, Response = http::Response<Body>> + Clone + 'a,
                ClientService::Future: 'a,
                ClientService::Error: 'a,
                Body: http_body::Body<Data = bytes::Bytes> + 'a,
                Body::Error: Into<tower::BoxError>,
            {
                type Output = Result<#response, #error<ClientService::Error>>;
                type IntoFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + 'a>>;
                fn into_future(self) -> Self::IntoFuture {
                    Box::pin(self.send())
                }
            }
        });
    }
}
