//! Resource accessors that construct operation builders.
use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

pub struct Resource {
    pub name: String,
    pub methods: Vec<ResourceMethod>,
    pub accessors: Vec<super::client::ResourceAccessor>,
}
pub struct ResourceMethod {
    pub name: String,
    pub description: Option<String>,
    pub operation_type: String,
    pub arguments: Vec<Argument>,
    pub has_params: bool,
    pub optional_payload: bool,
}
pub struct Argument {
    pub name: String,
    pub field: String,
    pub rust_type: String,
}

struct MethodFragment {
    name: syn::Ident,
    docs: Vec<String>,
    operation: syn::Path,
    arguments: Vec<TokenStream>,
    fields: Vec<TokenStream>,
    params: Option<TokenStream>,
}
impl ToTokens for MethodFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, docs, operation, arguments, fields, params } = self;
        tokens.extend(quote! {
            #(#[doc = #docs])*
            pub fn #name(&self, #(#arguments),*) -> #operation<'a, ClientService> {
                #operation { client: self.client, options: crate::client::RequestOptions::default(), #params #(#fields,)* }
            }
        });
    }
}

fn syntax_error(error: syn::Error) -> rrgen::Error {
    rrgen::Error::Message(format!("invalid resource syntax: {error}"))
}

impl TryFrom<ResourceMethod> for MethodFragment {
    type Error = rrgen::Error;

    fn try_from(method: ResourceMethod) -> Result<Self, Self::Error> {
        let name = syn::parse_str(&method.name).map_err(syntax_error)?;
        let operation = syn::parse_str(&method.operation_type).map_err(syntax_error)?;
        let mut arguments = Vec::new();
        let mut fields = Vec::new();
        let mut names = BTreeSet::new();
        let mut field_names: BTreeSet<String> = ["client", "options"].map(String::from).into();
        if method.has_params {
            field_names.insert("params".into());
        }
        if method.optional_payload {
            field_names.insert("payload".into());
            fields.push(quote!(payload: None));
        }
        for arg in method.arguments {
            let name: syn::Ident = syn::parse_str(&arg.name).map_err(syntax_error)?;
            let field: syn::Ident = syn::parse_str(&arg.field).map_err(syntax_error)?;
            let ty: syn::Type = syn::parse_str(&arg.rust_type).map_err(syntax_error)?;
            if !names.insert(name.to_string().trim_start_matches("r#").to_owned())
                || !field_names.insert(field.to_string().trim_start_matches("r#").into())
            {
                return Err(rrgen::Error::Message(
                    "duplicate or reserved resource argument".into(),
                ));
            }
            arguments.push(quote!(#name: #ty));
            fields.push(quote!(#field: #name));
        }
        Ok(Self {
            name,
            docs: super::documentation_lines(method.description.as_deref()),
            operation,
            arguments,
            fields,
            params: method.has_params.then(|| quote!(params: None,)),
        })
    }
}

pub(super) fn resource_tokens(resource: Resource) -> Result<TokenStream, rrgen::Error> {
    let name: syn::Ident = syn::parse_str(&resource.name).map_err(syntax_error)?;
    let methods = resource
        .methods
        .into_iter()
        .map(MethodFragment::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    let mut names = BTreeSet::from(["new".to_owned()]);
    for method in &methods {
        if !names.insert(method.name.to_string().trim_start_matches("r#").into()) {
            return Err(rrgen::Error::Message("duplicate or reserved resource method".into()));
        }
    }
    let mut accessors = Vec::new();
    for accessor in resource.accessors {
        let name: syn::Ident = syn::parse_str(&accessor.name).map_err(syntax_error)?;
        let ty: syn::Path = syn::parse_str(&accessor.rust_type).map_err(syntax_error)?;
        if !names.insert(name.to_string().trim_start_matches("r#").into()) {
            return Err(rrgen::Error::Message("duplicate or reserved resource accessor".into()));
        }
        accessors.push(quote! {
            pub fn #name(&self) -> #ty<'a, ClientService> {
                #ty::new(self.client)
            }
        });
    }
    Ok(quote! {
        pub struct #name<'a, ClientService> { client: &'a crate::client::PolarClient<ClientService> }
        impl<'a, ClientService> #name<'a, ClientService> {
            pub(crate) fn new(client: &'a crate::client::PolarClient<ClientService>) -> Self { Self { client } }
            #(#methods)*
            #(#accessors)*
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{
        client::ResourceAccessor,
        test_support::{TestResult, assert_source},
    };

    fn assert_resource(name: &str, resource: Resource) -> TestResult {
        let file = syn::parse2(resource_tokens(resource)?)?;
        assert_source(name, &crate::templates::format_rust(file));
        Ok(())
    }

    #[test]
    fn domain_operations() -> TestResult {
        assert_resource(
            "resource_operations",
            Resource {
                name: "Organizations".into(),
                accessors: vec![],
                methods: vec![
                    ResourceMethod {
                        name: "get".into(),
                        description: Some(
                            "Get an organization.\n\nRequired scopes: `organizations:read`.".into(),
                        ),
                        operation_type: "crate::models::organizations::GetOrganization".into(),
                        arguments: vec![Argument {
                            name: "organization_id".into(),
                            field: "id".into(),
                            rust_type: "crate::models::common::OrganizationId".into(),
                        }],
                        has_params: false,
                        optional_payload: false,
                    },
                    ResourceMethod {
                        name: "list".into(),
                        description: Some("List organizations.".into()),
                        operation_type: "crate::models::organizations::ListOrganizations".into(),
                        arguments: vec![],
                        has_params: true,
                        optional_payload: false,
                    },
                    ResourceMethod {
                        name: "create".into(),
                        description: None,
                        operation_type: "crate::models::organizations::CreateOrganization".into(),
                        arguments: vec![Argument {
                            name: "body".into(),
                            field: "payload".into(),
                            rust_type: "crate::models::organizations::OrganizationCreate".into(),
                        }],
                        has_params: false,
                        optional_payload: false,
                    },
                    ResourceMethod {
                        name: "update".into(),
                        description: None,
                        operation_type: "crate::models::organizations::UpdateOrganization".into(),
                        arguments: vec![
                            Argument {
                                name: "organization_id".into(),
                                field: "id".into(),
                                rust_type: "crate::models::common::OrganizationId".into(),
                            },
                            Argument {
                                name: "payload".into(),
                                field: "payload".into(),
                                rust_type: "crate::models::organizations::OrganizationUpdate"
                                    .into(),
                            },
                        ],
                        has_params: false,
                        optional_payload: false,
                    },
                ],
            },
        )
    }

    #[test]
    fn required_params_and_optional_body() -> TestResult {
        assert_resource(
            "resource_required_params_optional_body",
            Resource {
                name: "Checkouts".into(),
                accessors: vec![],
                methods: vec![ResourceMethod {
                    name: "confirm".into(),
                    description: None,
                    operation_type: "crate::models::checkouts::ConfirmCheckout".into(),
                    arguments: vec![
                        Argument {
                            name: "checkout_id".into(),
                            field: "id".into(),
                            rust_type: "crate::models::common::CheckoutId".into(),
                        },
                        Argument {
                            name: "params".into(),
                            field: "params".into(),
                            rust_type: "crate::models::checkouts::ConfirmCheckoutParams".into(),
                        },
                    ],
                    has_params: false,
                    optional_payload: true,
                }],
            },
        )
    }

    #[test]
    fn nested_resource_accessors() -> TestResult {
        assert_resource(
            "resource_accessors",
            Resource {
                name: "CustomerPortal".into(),
                methods: vec![],
                accessors: vec![
                    ResourceAccessor {
                        name: "customers".into(),
                        rust_type: "crate::models::customer_portal::customers::Customers".into(),
                    },
                    ResourceAccessor {
                        name: "subscriptions".into(),
                        rust_type: "crate::models::customer_portal::subscriptions::Subscriptions"
                            .into(),
                    },
                ],
            },
        )
    }
}
