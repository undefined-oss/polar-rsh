//! Resolve all supported object models before rendering any model files.

use std::collections::{BTreeMap, BTreeSet};

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::Value;

use super::{ConversionError, domains, enums::EnumCatalog, uuid_ids::UuidCatalog};
use crate::templates::{
    aliases::Alias,
    models::{AdditionalProperties, Field, Model},
};

mod types;
mod unions;
mod webhooks;
use types::Resolver;

pub struct ModelCatalog {
    pub aliases: BTreeMap<String, Vec<Alias>>,
    pub unions: BTreeMap<String, Vec<crate::templates::unions::Union>>,
    pub union_type_paths: BTreeMap<String, String>,
    pub modules: BTreeMap<String, Vec<Model>>,
    pub type_paths: BTreeMap<String, String>,
    pub deferred: BTreeMap<String, String>,
}

pub fn collect_models(
    spec: &oas3::Spec,
    document: &Value,
    enums: &EnumCatalog,
    ids: &UuidCatalog,
    newtypes: &super::newtypes::NewtypeCatalog,
) -> Result<ModelCatalog, ConversionError> {
    let usage = domains::schema_domains(spec)?;
    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .ok_or_else(|| ConversionError::InvalidModel("missing component schemas".into()))?;
    let mut destinations = BTreeMap::new();
    let mut type_paths = BTreeMap::new();
    for name in schemas.keys() {
        let domain = usage
            .active
            .get(name)
            .filter(|domains| domains.len() == 1)
            .and_then(|domains| domains.first())
            .map(String::as_str)
            .unwrap_or("common");
        destinations.insert(name.clone(), format!("models/{domain}/types.rs"));
        type_paths.insert(
            name.clone(),
            format!("crate::models::{domain}::types::{}", name.to_upper_camel_case()),
        );
    }
    let union_schemas = unions::collect(schemas, &destinations, spec, document)?;
    for union in union_schemas.values() {
        if type_paths.contains_key(&union.key) {
            type_paths.insert(union.key.clone(), union.type_path.clone());
        }
    }
    let alias_names: BTreeSet<_> = schemas
        .iter()
        .filter(|(name, schema)| {
            destinations.contains_key(*name)
                && !enums.type_paths.contains_key(*name)
                && *name != "Pagination"
                && types::list_item(schema).is_none()
                && schema.get("properties").is_none()
                && !union_schemas.contains_key(&format!("#/components/schemas/{}", escape(name)))
        })
        .map(|(name, _)| name.clone())
        .collect();
    for name in &alias_names {
        let path = type_paths.get_mut(name).unwrap();
        *path = path.replace("::types::", "::aliases::");
    }
    let resolver = Resolver {
        schemas,
        model_paths: &type_paths,
        enum_paths: &enums.type_paths,
        inline_enum_paths: &enums.inline_type_paths,
        uuid_paths: &ids.type_paths,
        newtype_paths: &newtypes.type_paths,
        union_paths: &union_schemas,
        aliases: &alias_names,
    };
    let mut models = BTreeMap::new();
    let mut deferred = BTreeMap::new();
    let mut union_definitions = BTreeMap::new();
    let mut alias_definitions = BTreeMap::new();
    for name in &alias_names {
        let schema = &schemas[name];
        let location = format!("#/components/schemas/{}", escape(name));
        let mut dependencies = BTreeSet::new();
        match resolver.resolve(schema, &location, &mut dependencies, &mut BTreeSet::new()) {
            Ok(ty) => {
                alias_definitions.insert(
                    name.clone(),
                    (
                        Alias {
                            name: name.to_upper_camel_case(),
                            description: schema
                                .get("description")
                                .and_then(Value::as_str)
                                .map(str::to_owned),
                            rust_type: ty.nested(),
                        },
                        dependencies,
                    ),
                );
            }
            Err(reason) => {
                deferred.insert(name.clone(), reason);
            }
        }
    }
    for (location, definition) in &union_schemas {
        match unions::resolve(location, definition, &resolver) {
            Ok(union) => {
                union_definitions.insert(definition.key.clone(), union);
            }
            Err(reason) => {
                deferred.insert(definition.key.clone(), reason);
            }
        }
    }
    for (name, schema) in schemas {
        if !destinations.contains_key(name) || enums.type_paths.contains_key(name) {
            continue;
        }
        if alias_names.contains(name)
            || name == "Pagination"
            || types::list_item(schema).is_some()
            || union_schemas.contains_key(&format!("#/components/schemas/{}", escape(name)))
        {
            continue;
        }
        match resolve_model(name, schema, &resolver) {
            Ok(model) => {
                models.insert(name.clone(), model);
            }
            Err(reason) => {
                deferred.insert(name.clone(), reason);
            }
        }
    }
    // Recursive graphs are deferred until the resolver handles indirection.
    let recursive: Vec<_> = models
        .keys()
        .filter(|name| reaches(name, name, &models, &mut BTreeSet::new()))
        .cloned()
        .collect();
    for name in recursive {
        models.remove(&name);
        deferred.insert(name, "recursive model graphs are not supported by this pass".into());
    }
    loop {
        let dependencies = models
            .iter()
            .map(|(name, (_, deps))| (name, deps))
            .chain(union_definitions.iter().map(|(name, (_, deps))| (name, deps)))
            .chain(alias_definitions.iter().map(|(name, (_, deps))| (name, deps)));
        let blocked: Vec<_> = dependencies
            .filter_map(|(name, deps)| {
                deps.iter()
                    .find(|dependency| {
                        !models.contains_key(*dependency)
                            && !union_definitions.contains_key(*dependency)
                            && !alias_definitions.contains_key(*dependency)
                    })
                    .map(|dependency| {
                        (name.clone(), format!("depends on deferred type {dependency}"))
                    })
            })
            .collect();
        if blocked.is_empty() {
            break;
        }
        for (name, reason) in blocked {
            models.remove(&name);
            union_definitions.remove(&name);
            alias_definitions.remove(&name);
            deferred.insert(name, reason);
        }
    }
    let mut modules: BTreeMap<String, Vec<Model>> = BTreeMap::new();
    type_paths.retain(|name, _| {
        models.contains_key(name)
            || union_definitions.contains_key(name)
            || alias_definitions.contains_key(name)
    });
    for (name, (model, _)) in models {
        modules.entry(destinations[&name].clone()).or_default().push(model);
    }
    let mut union_modules: BTreeMap<String, Vec<crate::templates::unions::Union>> = BTreeMap::new();
    let mut union_type_paths = BTreeMap::new();
    for (location, definition) in union_schemas {
        if let Some((union, _)) = union_definitions.remove(&definition.key) {
            union_modules.entry(definition.module_path).or_default().push(union);
            union_type_paths.insert(location, definition.type_path);
        }
    }
    let mut alias_modules: BTreeMap<String, Vec<Alias>> = BTreeMap::new();
    for (name, (alias, _)) in alias_definitions {
        let module =
            destinations[&name].strip_suffix("types.rs").unwrap().to_owned() + "aliases.rs";
        alias_modules.entry(module).or_default().push(alias);
    }
    Ok(ModelCatalog {
        aliases: alias_modules,
        unions: union_modules,
        union_type_paths,
        modules,
        type_paths,
        deferred,
    })
}

fn resolve_model(
    name: &str,
    schema: &Value,
    resolver: &Resolver<'_>,
) -> Result<(Model, BTreeSet<String>), String> {
    syn::parse_str::<syn::Ident>(&name.to_upper_camel_case()).map_err(|cause| cause.to_string())?;
    if ["anyOf", "oneOf", "allOf"].iter().any(|key| schema.get(key).is_some()) {
        return Err("requires a union or composition template".into());
    }
    if schema.get("type").and_then(Value::as_str) != Some("object") {
        return Err("requires a union or scalar-alias template".into());
    }
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or("object has no declared properties")?;
    let required: BTreeSet<_> = schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    let mut dependencies = BTreeSet::new();
    let mut fields = Vec::new();
    for (wire_name, field) in properties {
        let location =
            format!("#/components/schemas/{}/properties/{}", escape(name), escape(wire_name));
        let ty = resolver.resolve(field, &location, &mut dependencies, &mut BTreeSet::new())?;
        fields.push(Field {
            rust_name: rust_name(wire_name)?,
            wire_name: wire_name.clone(),
            rust_type: ty.base,
            description: field.get("description").and_then(Value::as_str).map(str::to_owned),
            required: required.contains(wire_name.as_str()),
            nullable: ty.nullable,
            constant: field.get("const").is_some(),
        });
    }
    let additional_properties = if schema
        .get("additionalProperties")
        .is_some_and(|value| value.is_object() || value == &Value::Bool(true))
    {
        let rust_type = resolver.resolve_map(
            schema,
            &format!("#/components/schemas/{}", escape(name)),
            &mut dependencies,
            &mut BTreeSet::new(),
        )?;
        let mut rust_name = "additional_properties".to_owned();
        while fields.iter().any(|field| field.rust_name.trim_start_matches("r#") == rust_name) {
            rust_name.push('_');
        }
        Some(AdditionalProperties { rust_name, rust_type })
    } else {
        None
    };
    Ok((
        Model {
            name: name.to_upper_camel_case(),
            description: schema.get("description").and_then(Value::as_str).map(str::to_owned),
            derive_default: fields.iter().all(|field| !field.required || field.constant),
            derive_error: false,
            fields,
            additional_properties,
        },
        dependencies,
    ))
}

fn reaches(
    start: &str,
    current: &str,
    models: &BTreeMap<String, (Model, BTreeSet<String>)>,
    visited: &mut BTreeSet<String>,
) -> bool {
    if !visited.insert(current.to_owned()) {
        return false;
    }
    models.get(current).is_some_and(|(_, dependencies)| {
        dependencies
            .iter()
            .any(|dependency| dependency == start || reaches(start, dependency, models, visited))
    })
}

fn rust_name(wire: &str) -> Result<String, String> {
    let name = wire.to_snake_case();
    let name = if matches!(name.as_str(), "self" | "super" | "crate") {
        format!("{name}_value")
    } else if syn::parse_str::<syn::Ident>(&name).is_err() {
        format!("r#{name}")
    } else {
        name
    };
    syn::parse_str::<syn::Ident>(&name).map_err(|cause| cause.to_string())?;
    Ok(name)
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
