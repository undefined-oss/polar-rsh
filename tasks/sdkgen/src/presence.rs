//! Adapt optional nullable model fields to explicit presence semantics.
//! Parse Rust syntax so nested generic types, constructors and builders stay
//! aligned.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, bail, ensure};
use quote::ToTokens;
use syn::{
    Expr, GenericArgument, Item, Member, Meta, PathArguments, Type, parse_quote,
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
};

use crate::metadata::Bindings;

fn option_inner(typ: &Type) -> Option<&Type> {
    let Type::Path(path) = typ else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else { return None };
    match args.args.first()? {
        GenericArgument::Type(inner) if args.args.len() == 1 => Some(inner),
        _ => None,
    }
}

fn presence_field(field: &mut syn::Field) -> Result<bool> {
    let Some(inner) = option_inner(&field.ty).and_then(option_inner).cloned() else {
        return Ok(false);
    };
    let mut skip = false;
    let mut default = false;
    let mut deserialize = false;
    for attribute in &mut field.attrs {
        if !attribute.path().is_ident("serde") {
            continue;
        }
        let metas =
            attribute.parse_args_with(Punctuated::<Meta, syn::Token![,]>::parse_terminated)?;
        let mut updated = Vec::new();
        for meta in metas {
            if meta.path().is_ident("skip_serializing_if") {
                ensure!(
                    meta.to_token_stream().to_string()
                        == "skip_serializing_if = \"Option::is_none\"",
                    "Unexpected tri-state skip predicate"
                );
                updated
                    .push(parse_quote!(skip_serializing_if = "presence_rs::Presence::is_absent"));
                skip = true;
            } else if meta.path().is_ident("deserialize_with") {
                ensure!(
                    meta.to_token_stream().to_string()
                        == "deserialize_with = \"tri_state_serde::deserialize\"",
                    "Unsupported tri-state deserializer"
                );
                deserialize = true;
            } else {
                ensure!(
                    !meta.path().is_ident("with") && !meta.path().is_ident("serialize_with"),
                    "Custom tri-state codecs require a Presence adapter"
                );
                if meta.path().is_ident("default") {
                    ensure!(matches!(meta, Meta::Path(_)), "Unexpected tri-state default");
                    default = true;
                }
                updated.push(meta);
            }
        }
        *attribute = parse_quote!(#[serde(#(#updated),*)]);
    }
    ensure!(skip && default && deserialize, "Unexpected generated tri-state field attributes");
    field.ty = parse_quote!(presence_rs::Presence<#inner>);
    Ok(true)
}

pub fn generate(source: &str, metadata: &mut Bindings) -> Result<String> {
    metadata.validate()?;
    let mut file =
        syn::parse_file(source).context("Parse generated models for Presence conversion")?;
    let mut fields = BTreeMap::<String, BTreeSet<String>>::new();
    let mut changed = BTreeMap::new();
    for item in &mut file.items {
        let Item::Struct(model) = item else { continue };
        for field in &mut model.fields {
            if presence_field(field)? {
                let name = field.ident.as_ref().context("Unnamed tri-state field")?.to_string();
                fields.entry(model.ident.to_string()).or_default().insert(name.clone());
                changed.insert((format!("types::{}", model.ident), name), field.clone());
            }
        }
    }
    ensure!(!changed.is_empty(), "No generated tri-state fields found; review generator output");
    let mut matched = BTreeSet::new();
    for symbol in &mut metadata.symbols {
        for binding in &mut symbol.fields {
            let Some(name) = &binding.name else {
                continue;
            };
            let key = (symbol.path.clone(), name.clone());
            if let Some(field) = changed.get(&key) {
                binding.rust_type = field.ty.to_token_stream().to_string();
                binding.attributes = field
                    .attrs
                    .iter()
                    .filter(|attr| !attr.path().is_ident("doc"))
                    .map(|attr| attr.to_token_stream().to_string())
                    .collect();
                ensure!(matched.insert(key), "Duplicate model field binding");
            }
        }
    }
    ensure!(matched.len() == changed.len(), "Presence model fields and binding metadata disagree");

    for item in &mut file.items {
        let Item::Impl(implementation) = item else { continue };
        let Type::Path(path) = &*implementation.self_ty else { continue };
        let Some(segment) = path.path.segments.last() else { continue };
        let owner = segment.ident.to_string();
        let builder_model = owner.strip_suffix("Builder").filter(|name| fields.contains_key(*name));
        let mut visitor = Initializers {
            fields: &fields,
            owner: &owner,
            builder_model,
            member: "self . value",
            error: None,
        };
        visitor.visit_item_impl_mut(implementation);
        if let Some(error) = visitor.error {
            return Err(error);
        }
    }
    file.items
        .retain(|item| !matches!(item, Item::Mod(module) if module.ident == "tri_state_serde"));
    let output = prettyplease::unparse(&file);
    // Fail closed if new generator constructs were not handled by this adapter.
    ensure!(!output.contains("tri_state_serde"), "Unconverted tri-state codec reference");
    let mut remaining = NestedOptions(false);
    remaining.visit_file_mut(&mut file);
    ensure!(!remaining.0, "Unconverted nested Option type in generated models");
    Ok(output)
}

/// HTTP operation builders embed request models separately from model builders.
pub fn client_builders(source: &str, models: &str) -> Result<String> {
    let models = syn::parse_file(models)?;
    let mut fields = BTreeMap::<String, BTreeSet<String>>::new();
    for item in &models.items {
        let Item::Struct(model) = item else { continue };
        for field in &model.fields {
            if let Type::Path(path) = &field.ty
                && path.path.segments.last().is_some_and(|part| part.ident == "Presence")
            {
                fields
                    .entry(model.ident.to_string())
                    .or_default()
                    .insert(field.ident.as_ref().context("Unnamed presence field")?.to_string());
            }
        }
    }
    // Request body names can alias another generated model, including alias
    // chains.
    loop {
        let previous = fields.len();
        for item in &models.items {
            if let Item::Type(alias) = item
                && let Type::Path(path) = &*alias.ty
                && let Some(segment) = path.path.segments.last()
                && let Some(target) = fields.get(&segment.ident.to_string()).cloned()
            {
                fields.insert(alias.ident.to_string(), target);
            }
        }
        if fields.len() == previous {
            break;
        }
    }
    let mut file = syn::parse_file(source)?;
    let mut builders = BTreeMap::new();
    for item in &file.items {
        let Item::Struct(builder) = item else { continue };
        for field in &builder.fields {
            if field.ident.as_ref().is_some_and(|name| name == "request")
                && let Type::Path(path) = option_inner(&field.ty).unwrap_or(&field.ty)
                && let Some(segment) = path.path.segments.last()
                && fields.contains_key(&segment.ident.to_string())
            {
                builders.insert(
                    builder.ident.to_string(),
                    (segment.ident.to_string(), option_inner(&field.ty).is_some()),
                );
            }
        }
    }
    for item in &mut file.items {
        let Item::Impl(implementation) = item else { continue };
        let Type::Path(path) = &*implementation.self_ty else { continue };
        let Some(segment) = path.path.segments.last() else { continue };
        let owner = segment.ident.to_string();
        let mut visitor = Initializers {
            fields: &fields,
            owner: &owner,
            builder_model: builders.get(&owner).map(|(model, _)| model.as_str()),
            member: if builders.get(&owner).is_some_and(|(_, optional)| *optional) {
                "request"
            } else {
                "self . request"
            },
            error: None,
        };
        visitor.visit_item_impl_mut(implementation);
        if let Some(error) = visitor.error {
            return Err(error);
        }
    }
    Ok(prettyplease::unparse(&file))
}

struct NestedOptions(bool);

impl VisitMut for NestedOptions {
    fn visit_type_mut(&mut self, typ: &mut Type) {
        self.0 |= option_inner(typ).and_then(option_inner).is_some();
        visit_mut::visit_type_mut(self, typ);
    }
}

fn is_none(expr: &Expr) -> bool {
    matches!(expr, Expr::Path(path) if path.path.is_ident("None"))
}

fn some_inner(expr: &Expr) -> Option<&Expr> {
    let Expr::Call(call) = expr else { return None };
    match &*call.func {
        Expr::Path(path) if path.path.is_ident("Some") && call.args.len() == 1 => call.args.first(),
        _ => None,
    }
}

fn initializer(expr: &Expr) -> Result<Expr> {
    if is_none(expr) {
        return Ok(parse_quote!(presence_rs::Presence::Absent));
    }
    if let Some(inner) = some_inner(expr) {
        if is_none(inner) {
            return Ok(parse_quote!(presence_rs::Presence::Null));
        }
        if let Some(value) = some_inner(inner) {
            return Ok(parse_quote!(presence_rs::Presence::Some(#value)));
        }
    }
    bail!("Unsupported tri-state initializer: {}", expr.to_token_stream())
}

struct Initializers<'a> {
    fields: &'a BTreeMap<String, BTreeSet<String>>,
    owner: &'a str,
    builder_model: Option<&'a str>,
    member: &'a str,
    error: Option<anyhow::Error>,
}

impl Initializers<'_> {
    fn replace(&mut self, expr: &mut Expr) {
        match initializer(expr) {
            Ok(updated) => *expr = updated,
            Err(error) => self.error = Some(error),
        }
    }
}

impl VisitMut for Initializers<'_> {
    fn visit_expr_struct_mut(&mut self, expr: &mut syn::ExprStruct) {
        let name = expr.path.segments.last().unwrap().ident.to_string();
        let model = if name == "Self" { self.owner } else { &name };
        if let Some(fields) = self.fields.get(model) {
            for field in &mut expr.fields {
                if let Member::Named(name) = &field.member
                    && fields.contains(&name.to_string())
                {
                    self.replace(&mut field.expr);
                }
            }
        }
        visit_mut::visit_expr_struct_mut(self, expr);
    }

    fn visit_expr_assign_mut(&mut self, expr: &mut syn::ExprAssign) {
        if let Some(model) = self.builder_model
            && let Expr::Field(field) = &*expr.left
            && field.base.to_token_stream().to_string() == self.member
            && let Member::Named(name) = &field.member
            && self.fields[model].contains(&name.to_string())
        {
            self.replace(&mut expr.right);
        }
        visit_mut::visit_expr_assign_mut(self, expr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unexpected_codecs_and_initializers() {
        let mut field: syn::Field = parse_quote! {
            #[serde(default, skip_serializing_if = "Option::is_none", with = "bytes::double_option")]
            value: Option<Option<Vec<u8>>>
        };
        assert!(presence_field(&mut field).is_err());
        assert!(initializer(&parse_quote!(Some(value))).is_err());
    }

    #[test]
    fn updates_models_builders_and_metadata_together() {
        let source = r#"
            mod tri_state_serde {}
            struct Model {
                #[serde(rename = "wire", default, skip_serializing_if = "Option::is_none", deserialize_with = "tri_state_serde::deserialize")]
                value: Option<Option<Vec<Option<String>>>>,
                optional: Option<String>,
                required_nullable: Option<String>,
            }
            impl Model {
                fn new() -> Self { Self { value: None, optional: None, required_nullable: None } }
            }
            impl ModelBuilder {
                fn set(&mut self, value: Vec<Option<String>>) { self.value.value = Some(Some(value)); }
                fn null(&mut self) { self.value.value = Some(None); }
                fn absent(&mut self) { self.value.value = None; }
                fn optional(&mut self, value: String) { self.value.optional = Some(value); }
            }
        "#;
        let mut symbol = crate::metadata::test_symbol("types::Model", "Model");
        symbol.fields.push(
            serde_json::from_value(serde_json::json!({
                "name":"value", "index":0, "public":true, "wire_name":null,
                "rust_type":"Option < Option < Vec < Option < String > > > >", "attributes":[]
            }))
            .unwrap(),
        );
        let mut metadata = crate::metadata::fixture(vec![symbol]);
        let output = generate(source, &mut metadata).unwrap();
        assert!(output.contains("value: presence_rs::Presence<Vec<Option<String>>>"));
        assert!(output.contains("rename = \"wire\""));
        assert!(output.contains("skip_serializing_if = \"presence_rs::Presence::is_absent\""));
        assert!(output.contains("value: presence_rs::Presence::Absent"));
        assert!(output.contains("self.value.value = presence_rs::Presence::Some(value)"));
        assert!(output.contains("self.value.value = presence_rs::Presence::Null"));
        assert!(output.contains("self.value.value = presence_rs::Presence::Absent"));
        assert!(output.contains("self.value.optional = Some(value)"));
        assert!(output.contains("required_nullable: Option<String>"));
        assert!(!output.contains("tri_state_serde"));
        assert!(metadata.symbols[0].fields[0].rust_type.starts_with("presence_rs :: Presence"));
        assert!(generate(source, &mut crate::metadata::fixture(Vec::new())).is_err());

        let client = r#"
            struct UpdateBuilder { request: Model }
            impl UpdateBuilder {
                fn set(&mut self, value: Vec<Option<String>>) { self.request.value = Some(Some(value)); }
                fn null(&mut self) { self.request.value = Some(None); }
                fn absent(&mut self) { self.request.value = None; }
                fn optional(&mut self, value: String) { self.request.optional = Some(value); }
            }
        "#;
        let client = client_builders(client, &output).unwrap();
        assert!(client.contains("self.request.value = presence_rs::Presence::Some(value)"));
        assert!(client.contains("self.request.value = presence_rs::Presence::Null"));
        assert!(client.contains("self.request.value = presence_rs::Presence::Absent"));
        assert!(client.contains("self.request.optional = Some(value)"));

        let models = format!("{output}\ntype Alias = Model;\ntype RequestAlias = Alias;");
        let client = r#"
            struct OptionalBuilder { request: Option<RequestAlias> }
            impl OptionalBuilder {
                fn set(&mut self, value: Vec<Option<String>>) {
                    let request = self.request.get_or_insert_with(Default::default);
                    request.value = Some(Some(value));
                }
                fn null(&mut self) {
                    let request = self.request.get_or_insert_with(Default::default);
                    request.value = Some(None);
                }
                fn absent(&mut self) {
                    if let Some(request) = self.request.as_mut() { request.value = None; }
                }
            }
        "#;
        let client = client_builders(client, &models).unwrap();
        assert!(client.contains("request.value = presence_rs::Presence::Some(value)"));
        assert!(client.contains("request.value = presence_rs::Presence::Null"));
        assert!(client.contains("request.value = presence_rs::Presence::Absent"));
    }
}
