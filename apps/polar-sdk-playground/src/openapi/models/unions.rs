//! Discover named and inline unions before resolving their payload types.

use std::collections::{BTreeMap, BTreeSet};

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::{Map, Value};

use super::{
    super::{ConversionError, references::dereference},
    escape,
    types::Resolver,
};
use crate::templates::unions::{Union, Variant};

pub(super) struct UnionSchema {
    pub key: String,
    pub name: String,
    pub module_path: String,
    pub type_path: String,
    pub nullable: bool,
    pub schema: Value,
    pub keyword: &'static str,
    pub boxed_fields: BTreeSet<String>,
}

pub(super) fn collect(
    schemas: &Map<String, Value>,
    destinations: &BTreeMap<String, String>,
    spec: &oas3::Spec,
    document: &Value,
) -> Result<BTreeMap<String, UnionSchema>, ConversionError> {
    let mut unions = BTreeMap::new();
    for (name, module) in destinations {
        let domain = module.strip_prefix("models/").unwrap().strip_suffix("/types.rs").unwrap();
        visit(
            &schemas[name],
            &format!("#/components/schemas/{}", escape(name)),
            &name.to_upper_camel_case(),
            Some(name),
            domain,
            &mut unions,
        );
    }
    collect_requests(spec, document, &mut unions)?;
    collect_responses(spec, document, &mut unions)?;
    super::webhooks::collect(document, &mut unions)?;
    let mut names = BTreeSet::new();
    for definition in unions.values() {
        if !names.insert(&definition.type_path) {
            return Err(ConversionError::InvalidModel(format!(
                "duplicate union name {}",
                definition.type_path
            )));
        }
    }
    Ok(unions)
}

struct OperationSchema<'a> {
    schema: &'a Value,
    name: String,
    domains: BTreeSet<String>,
}

fn collect_requests(
    spec: &oas3::Spec,
    document: &Value,
    unions: &mut BTreeMap<String, UnionSchema>,
) -> Result<(), ConversionError> {
    let mut requests: BTreeMap<String, OperationSchema<'_>> = BTreeMap::new();
    for (path, item) in spec.paths.iter().flatten() {
        for (method, operation) in item.methods() {
            let method = format!("{method:?}").to_lowercase();
            if !matches!(method.as_str(), "post" | "patch" | "put") {
                continue;
            }
            let Some(group) = operation.extensions.get("speakeasy-group").and_then(Value::as_str)
            else {
                continue;
            };
            let location = format!("#/paths/{}/{}", escape(path), method);
            let operation = document.pointer(location.strip_prefix('#').unwrap()).unwrap();
            let customer_portal = operation
                .get("operationId")
                .and_then(Value::as_str)
                .is_some_and(|id| id.starts_with("customer_portal:"));
            let Some(body) = operation.get("requestBody") else { continue };
            let (body, location) = dereference(body, format!("{location}/requestBody"), document)
                .map_err(ConversionError::InvalidModel)?;
            let Some((media_type, schema)) =
                ["application/json", "application/x-www-form-urlencoded"].into_iter().find_map(
                    |media_type| {
                        body.get("content")?
                            .get(media_type)?
                            .get("schema")
                            .map(|schema| (media_type, schema))
                    },
                )
            else {
                continue;
            };
            let Some(method_name) =
                operation.get("x-speakeasy-name-override").and_then(Value::as_str)
            else {
                continue;
            };
            let domain = group.split('.').next().unwrap().to_snake_case();
            let name = format!(
                "{}{}{}Request",
                if customer_portal { "CustomerPortal" } else { "" },
                group.to_upper_camel_case(),
                method_name.to_upper_camel_case()
            );
            requests
                .entry(format!("{location}/content/{}/schema", escape(media_type)))
                .or_insert_with(|| OperationSchema { schema, name, domains: BTreeSet::new() })
                .domains
                .insert(domain);
        }
    }
    for (location, request) in requests {
        let domain = if request.domains.len() == 1 {
            request.domains.first().unwrap().as_str()
        } else {
            "common"
        };
        visit(request.schema, &location, &request.name, None, domain, unions);
    }
    Ok(())
}

fn collect_responses(
    spec: &oas3::Spec,
    document: &Value,
    unions: &mut BTreeMap<String, UnionSchema>,
) -> Result<(), ConversionError> {
    let mut responses: BTreeMap<String, OperationSchema<'_>> = BTreeMap::new();
    for (path, item) in spec.paths.iter().flatten() {
        for (method, operation) in item.methods() {
            let Some(group) = operation.extensions.get("speakeasy-group").and_then(Value::as_str)
            else {
                continue;
            };
            let location =
                format!("#/paths/{}/{}", escape(path), format!("{method:?}").to_lowercase());
            let operation = document.pointer(location.strip_prefix('#').unwrap()).unwrap();
            let Some(method_name) =
                operation.get("x-speakeasy-name-override").and_then(Value::as_str)
            else {
                continue;
            };
            let portal = operation
                .get("operationId")
                .and_then(Value::as_str)
                .is_some_and(|id| id.starts_with("customer_portal:"));
            let domain = group.split('.').next().unwrap().to_snake_case();
            for (status, response) in
                operation.get("responses").and_then(Value::as_object).into_iter().flatten()
            {
                if !status.parse::<u16>().is_ok_and(|status| (200..300).contains(&status)) {
                    continue;
                }
                let (response, location) =
                    dereference(response, format!("{location}/responses/{status}"), document)
                        .map_err(ConversionError::InvalidModel)?;
                let Some(schema) = response.pointer("/content/application~1json/schema") else {
                    continue;
                };
                let name = format!(
                    "{}{}{}Response{status}",
                    if portal { "CustomerPortal" } else { "" },
                    group.to_upper_camel_case(),
                    method_name.to_upper_camel_case()
                );
                responses
                    .entry(format!("{location}/content/application~1json/schema"))
                    .or_insert_with(|| OperationSchema { schema, name, domains: BTreeSet::new() })
                    .domains
                    .insert(domain.clone());
            }
        }
    }
    for (location, response) in responses {
        let domain = if response.domains.len() == 1 {
            response.domains.first().unwrap().as_str()
        } else {
            "common"
        };
        visit(response.schema, &location, &response.name, None, domain, unions);
    }
    Ok(())
}

fn visit(
    schema: &Value,
    location: &str,
    name: &str,
    root: Option<&str>,
    domain: &str,
    unions: &mut BTreeMap<String, UnionSchema>,
) {
    for keyword in ["anyOf", "oneOf"] {
        if let Some(branches) = schema.get(keyword).and_then(Value::as_array) {
            let nullable = branches.iter().any(is_null);
            if branches.iter().filter(|branch| !is_null(branch)).count() > 1
                && !(root.is_some() && nullable)
            {
                let name = if root.is_some() { name.to_owned() } else { format!("{name}Union") };
                unions.insert(
                    location.into(),
                    UnionSchema {
                        key: root.unwrap_or(location).into(),
                        type_path: format!("crate::models::{domain}::unions::{name}"),
                        name,
                        module_path: format!("models/{domain}/unions.rs"),
                        nullable,
                        schema: schema.clone(),
                        keyword,
                        boxed_fields: BTreeSet::new(),
                    },
                );
            }
        }
    }
    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        for (field, schema) in properties {
            visit(
                schema,
                &format!("{location}/properties/{}", escape(field)),
                &format!("{name}{}", field.to_upper_camel_case()),
                None,
                domain,
                unions,
            );
        }
    }
    for keyword in ["anyOf", "oneOf", "allOf", "prefixItems"] {
        for (index, branch) in
            schema.get(keyword).and_then(Value::as_array).into_iter().flatten().enumerate()
        {
            visit(
                branch,
                &format!("{location}/{keyword}/{index}"),
                &format!("{name}{}{index}", keyword.to_upper_camel_case()),
                None,
                domain,
                unions,
            );
        }
    }
    for keyword in ["items", "additionalProperties", "propertyNames"] {
        if let Some(child) = schema.get(keyword) {
            visit(
                child,
                &format!("{location}/{keyword}"),
                &format!("{name}{}", keyword.to_upper_camel_case()),
                None,
                domain,
                unions,
            );
        }
    }
}

pub(super) fn resolve(
    location: &str,
    definition: &UnionSchema,
    resolver: &Resolver<'_>,
) -> Result<(Union, BTreeSet<String>), String> {
    let fail = |message: &str| format!("{location}: {message}");
    let schema = &definition.schema;
    let branches = schema[definition.keyword].as_array().unwrap();
    let discriminator = schema.get("discriminator");
    let property = discriminator
        .map(|value| {
            value
                .get("propertyName")
                .and_then(Value::as_str)
                .ok_or_else(|| fail("discriminator requires propertyName"))
        })
        .transpose()?;
    let mapping = discriminator.and_then(|value| value.get("mapping")).and_then(Value::as_object);
    if let Some(mapping) = mapping {
        for target in mapping.values() {
            if !branches.iter().any(|branch| branch.get("$ref") == Some(target)) {
                return Err(fail("discriminator mapping targets a non-member schema"));
            }
        }
    }
    let mut dependencies = BTreeSet::new();
    let mut names = BTreeSet::new();
    let mut variants = Vec::new();
    for (index, branch) in branches.iter().enumerate() {
        if is_null(branch) {
            continue;
        }
        let reference = branch.get("$ref").and_then(Value::as_str);
        let tags = if property.is_some() {
            let reference = reference
                .ok_or_else(|| fail("tagged union branches must reference component schemas"))?;
            let tags: Vec<_> = if let Some(mapping) = mapping {
                mapping
                    .iter()
                    .filter(|(_, target)| target.as_str() == Some(reference))
                    .map(|(tag, _)| tag.clone())
                    .collect()
            } else {
                vec![reference.rsplit('/').next().unwrap().into()]
            };
            if tags.is_empty() {
                return Err(fail("discriminator mapping does not cover every branch"));
            }
            tags
        } else {
            Vec::new()
        };
        let mut name = tags
            .first()
            .map(|tag| tag.to_upper_camel_case())
            .unwrap_or_else(|| variant_name(branch));
        if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) {
            name = format!("Value{name}");
        }
        if name == "Self" {
            name = "SelfValue".into();
        }
        if !names.insert(name.clone()) {
            name = format!("{name}{}", index + 1);
            if !names.insert(name.clone()) {
                return Err(fail("union variant names collide"));
            }
        }
        let ty = resolver.resolve(
            branch,
            &format!("{location}/{}/{index}", definition.keyword),
            &mut dependencies,
            &mut BTreeSet::new(),
        )?;
        let fields = if let (Some(property), Some(reference), [tag]) =
            (property, reference, tags.as_slice())
        {
            let target_name =
                reference.rsplit('/').next().unwrap().replace("~1", "/").replace("~0", "~");
            let target = &resolver.schemas[&target_name];
            if target
                .get("properties")
                .and_then(|fields| fields.get(property))
                .and_then(|field| field.get("const"))
                .and_then(Value::as_str)
                == Some(tag.as_str())
            {
                let (mut model, model_dependencies) =
                    super::resolve_model(&target_name, target, resolver)?;
                if model.additional_properties.is_none() {
                    dependencies.extend(model_dependencies);
                    model.fields.retain(|field| field.wire_name != property);
                    for field in &mut model.fields {
                        if definition.boxed_fields.contains(&field.wire_name) {
                            let ty: syn::Type = syn::parse_str(&field.rust_type)
                                .map_err(|cause| fail(&cause.to_string()))?;
                            field.rust_type = quote::quote!(Box<#ty>).to_string();
                        }
                    }
                    Some(model.fields)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        if !definition.boxed_fields.is_empty() && fields.is_none() {
            return Err(fail("boxed event data requires a tagged object variant"));
        }
        variants.push(Variant {
            name,
            description: branch.get("description").and_then(Value::as_str).map(str::to_owned),
            rust_type: ty.nested(),
            tags,
            fields,
        });
    }
    Ok((
        Union {
            name: definition.name.clone(),
            description: schema.get("description").and_then(Value::as_str).map(str::to_owned),
            exclusive: definition.keyword == "oneOf",
            discriminator: property.map(str::to_owned),
            variants,
        },
        dependencies,
    ))
}

fn variant_name(schema: &Value) -> String {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        return reference.rsplit('/').next().unwrap().to_upper_camel_case();
    }
    if let Some(value) = schema.get("const").and_then(Value::as_str) {
        return value.to_upper_camel_case();
    }
    let kind = schema.get("type").and_then(Value::as_str).unwrap_or("Variant");
    if kind == "array" {
        if let Some(items) = schema.get("items") {
            return format!("{}Array", variant_name(items));
        }
    }
    kind.to_upper_camel_case()
}

fn is_null(schema: &Value) -> bool {
    schema.get("type").and_then(Value::as_str) == Some("null")
}
