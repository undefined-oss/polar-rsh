use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Type, ext::IdentExt};

use super::{AdditionalProperties, Field, Model, syntax_error};

mod builders;
use builders::{Constructor, RequiredFields, Setter};

pub(super) struct ModelFragment {
    pub name: Ident,
    docs: Documentation,
    default: Option<TokenStream>,
    error: Option<TokenStream>,
    fields: Vec<FieldFragment>,
}

impl ToTokens for ModelFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, docs, default, error, fields } = self;
        let required_name = self.required_name();
        let required = required_name.as_ref().map(|name| RequiredFields { name, fields });
        let constructor = Constructor {
            required_name: required_name.as_ref(),
            fields,
            use_default: default.is_some(),
        };
        let setters = fields.iter().filter(|field| !field.presence.is_required()).map(Setter);
        let deny_unknown = (!fields.iter().any(|field| field.presence.is_flattened()))
            .then(|| quote!(#[serde(deny_unknown_fields)]));
        tokens.extend(quote! {
            #required
            #docs
            #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
            #deny_unknown
            #default
            #error
            pub struct #name {
                #(#fields)*
            }

            impl #name {
                #constructor
                #(#setters)*
            }
        });
    }
}

impl ModelFragment {
    pub(super) fn required_name(&self) -> Option<Ident> {
        self.fields
            .iter()
            .any(FieldFragment::requires_argument)
            .then(|| quote::format_ident!("{}Required", self.name))
    }
}

pub(super) struct FieldFragment {
    name: Ident,
    wire_name: String,
    docs: Documentation,
    ty: TokenStream,
    serde: TokenStream,
    base: Type,
    presence: Presence,
    constant: bool,
}

impl ToTokens for FieldFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.declaration(true));
    }
}

impl FieldFragment {
    fn requires_argument(&self) -> bool {
        self.presence.is_required() && !self.constant
    }

    pub(super) fn declaration(&self, public: bool) -> TokenStream {
        let Self { name, wire_name, docs, ty, serde, .. } = self;
        let visibility = public.then(|| quote!(pub));
        let rename = (!self.presence.is_flattened() && name.unraw().to_string() != *wire_name)
            .then(|| quote!(#[serde(rename = #wire_name)]));
        quote! {
            #docs
            #rename
            #serde
            #visibility #name: #ty,
        }
    }
}

struct Documentation(Vec<String>);

impl Documentation {
    fn new(description: Option<&str>) -> Self {
        Self(super::super::documentation_lines(description))
    }
}

impl ToTokens for Documentation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lines = &self.0;
        tokens.extend(quote! { #(#[doc = #lines])* });
    }
}

enum Presence {
    Required,
    RequiredNullable,
    Optional,
    OptionalNullable,
    AdditionalProperties,
}

impl Presence {
    fn is_flattened(&self) -> bool {
        matches!(self, Self::AdditionalProperties)
    }

    fn is_required(&self) -> bool {
        matches!(self, Self::Required | Self::RequiredNullable)
    }

    fn new(required: bool, nullable: bool) -> Self {
        match (required, nullable) {
            (true, false) => Self::Required,
            (true, true) => Self::RequiredNullable,
            (false, false) => Self::Optional,
            (false, true) => Self::OptionalNullable,
        }
    }

    fn field_type(&self, base: &Type) -> TokenStream {
        match self {
            Self::Required | Self::AdditionalProperties => quote!(#base),
            Self::RequiredNullable | Self::Optional => quote!(Option<#base>),
            Self::OptionalNullable => quote!(presence_rs::Presence<#base>),
        }
    }

    fn serde_attributes(&self) -> TokenStream {
        match self {
            Self::Required => quote!(),
            Self::RequiredNullable => quote! {
                #[serde(deserialize_with = "crate::models::common::serde_helpers::deserialize_required_nullable")]
            },
            Self::Optional => quote! {
                #[serde(default, skip_serializing_if = "Option::is_none",
                    deserialize_with = "crate::models::common::serde_helpers::deserialize_optional_value")]
            },
            Self::OptionalNullable => quote! {
                #[serde(default, skip_serializing_if = "presence_rs::Presence::is_absent")]
            },
            Self::AdditionalProperties => quote!(#[serde(flatten, default)]),
        }
    }
}

impl TryFrom<Field> for FieldFragment {
    type Error = rrgen::Error;

    fn try_from(field: Field) -> Result<Self, Self::Error> {
        let name = syn::parse_str(&field.rust_name).map_err(syntax_error)?;
        let base = syn::parse_str(&field.rust_type).map_err(syntax_error)?;
        let presence = Presence::new(field.required, field.nullable);
        Ok(Self {
            name,
            wire_name: field.wire_name,
            docs: Documentation::new(field.description.as_deref()),
            ty: presence.field_type(&base),
            serde: presence.serde_attributes(),
            base,
            presence,
            constant: field.constant,
        })
    }
}

impl TryFrom<AdditionalProperties> for FieldFragment {
    type Error = rrgen::Error;

    fn try_from(properties: AdditionalProperties) -> Result<Self, Self::Error> {
        let name = syn::parse_str(&properties.rust_name).map_err(syntax_error)?;
        let base = syn::parse_str(&properties.rust_type).map_err(syntax_error)?;
        let presence = Presence::AdditionalProperties;
        Ok(Self {
            name,
            wire_name: properties.rust_name,
            docs: Documentation::new(None),
            ty: presence.field_type(&base),
            serde: presence.serde_attributes(),
            base,
            presence,
            constant: false,
        })
    }
}

impl TryFrom<Model> for ModelFragment {
    type Error = rrgen::Error;

    fn try_from(model: Model) -> Result<Self, Self::Error> {
        let name = syn::parse_str(&model.name).map_err(syntax_error)?;
        if model.derive_default && model.fields.iter().any(|field| field.required) {
            return Err(rrgen::Error::Message(format!(
                "model {name}: Default requires all fields to be optional"
            )));
        }
        let mut fields =
            model.fields.into_iter().map(FieldFragment::try_from).collect::<Result<Vec<_>, _>>()?;
        if let Some(properties) = model.additional_properties {
            fields.push(FieldFragment::try_from(properties)?);
        }
        let mut names = BTreeSet::new();
        let mut wire_names = BTreeSet::new();
        for field in &fields {
            let canonical_name = field.name.to_string().trim_start_matches("r#").to_owned();
            if canonical_name == "new" && !field.presence.is_required() {
                return Err(rrgen::Error::Message(format!(
                    "model {name}: setter name conflicts with constructor: new"
                )));
            }
            if !names.insert(canonical_name) || !wire_names.insert(&field.wire_name) {
                return Err(rrgen::Error::Message(format!(
                    "model {name}: duplicate Rust or wire field name"
                )));
            }
        }
        Ok(Self {
            name,
            docs: Documentation::new(model.description.as_deref()),
            default: model.derive_default.then(|| quote!(#[derive(Default)])),
            error: model
                .derive_error
                .then(|| quote!(#[derive(thiserror::Error)] #[error("{self:?}")])),
            fields,
        })
    }
}
