//! Discover bounded scalar leaves once, then resolve model fields by exact
//! path.

use std::collections::{BTreeMap, BTreeSet};

use heck::ToUpperCamelCase;
use serde_json::Value;

use super::{ConversionError, domains, enums::EnumCatalog};
use crate::templates::newtypes::{Newtype, Scalar};

mod bounds;
pub(super) use bounds::integer_type;

pub struct NewtypeCatalog {
    pub modules: BTreeMap<String, Vec<Newtype>>,
    pub type_paths: BTreeMap<String, String>,
}

struct Candidate {
    location: String,
    original_name: String,
    domain: String,
    description: Option<String>,
    scalar: Scalar,
}

pub fn collect_newtypes(
    spec: &oas3::Spec,
    document: &Value,
    enums: &EnumCatalog,
) -> Result<NewtypeCatalog, ConversionError> {
    let usage = domains::schema_domains(spec)?;
    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .ok_or_else(|| fail("missing component schemas".into()))?;
    let mut candidates = BTreeMap::new();
    for (name, schema) in schemas {
        if !usage.active.contains_key(name) && usage.excluded.contains(name) {
            continue;
        }
        let domain = usage
            .active
            .get(name)
            .filter(|domains| domains.len() == 1)
            .and_then(|domains| domains.first())
            .map(String::as_str)
            .unwrap_or("common");
        // Create/update fields share an entity name only when that entity
        // exists. Registration below also requires identical scalar bounds.
        let entity = ["Create", "Update"]
            .into_iter()
            .find_map(|suffix| name.strip_suffix(suffix).filter(|base| schemas.contains_key(*base)))
            .unwrap_or(name);
        visit(
            schema,
            &format!("#/components/schemas/{}", escape(name)),
            &entity.to_upper_camel_case(),
            &name.to_upper_camel_case(),
            domain,
            None,
            &mut candidates,
        )?;
    }

    let mut reserved: BTreeSet<String> = schemas
        .keys()
        .map(|name| name.to_upper_camel_case())
        .chain(
            enums
                .inline_type_paths
                .values()
                .filter_map(|path| path.rsplit("::").next())
                .map(str::to_owned),
        )
        .collect();
    let mut catalog = NewtypeCatalog { modules: BTreeMap::new(), type_paths: BTreeMap::new() };
    for (name, definitions) in candidates {
        if definitions.iter().all(|definition| definition.scalar == definitions[0].scalar) {
            register(name, definitions, &mut reserved, &mut catalog)?;
        } else {
            // Different constraints retain their original schema identity.
            let mut separate: BTreeMap<String, Vec<Candidate>> = BTreeMap::new();
            for definition in definitions {
                separate.entry(definition.original_name.clone()).or_default().push(definition);
            }
            for (name, definitions) in separate {
                register(name, definitions, &mut reserved, &mut catalog)?;
            }
        }
    }
    Ok(catalog)
}

fn visit(
    schema: &Value,
    location: &str,
    name: &str,
    original_name: &str,
    domain: &str,
    description: Option<&str>,
    candidates: &mut BTreeMap<String, Vec<Candidate>>,
) -> Result<(), ConversionError> {
    if schema.get("format").and_then(Value::as_str) == Some("email") {
        return Ok(());
    }
    let description = schema.get("description").and_then(Value::as_str).or(description);
    if let Some(scalar) = bounds::scalar(schema, location).map_err(fail)? {
        candidates.entry(name.to_owned()).or_default().push(Candidate {
            location: location.to_owned(),
            original_name: original_name.to_owned(),
            domain: domain.to_owned(),
            description: description.map(str::to_owned),
            scalar,
        });
    }
    for (field, child) in schema.get("properties").and_then(Value::as_object).into_iter().flatten()
    {
        if field == "email" || field.ends_with("_email") {
            continue;
        }
        let suffix = field.to_upper_camel_case();
        visit(
            child,
            &format!("{location}/properties/{}", escape(field)),
            &format!("{name}{suffix}"),
            &format!("{original_name}{suffix}"),
            domain,
            None,
            candidates,
        )?;
    }
    for keyword in ["anyOf", "oneOf", "allOf", "prefixItems"] {
        let Some(branches) = schema.get(keyword).and_then(Value::as_array) else { continue };
        let non_null = branches
            .iter()
            .filter(|branch| branch.get("type").and_then(Value::as_str) != Some("null"))
            .count();
        for (index, branch) in branches.iter().enumerate() {
            let suffix = if non_null == 1 {
                String::new()
            } else if let Some(kind) = branch.get("type").and_then(Value::as_str) {
                let same_kind =
                    branches.iter().filter(|other| other.get("type") == branch.get("type")).count();
                if same_kind == 1 {
                    kind.to_upper_camel_case()
                } else {
                    format!("{}{index}", kind.to_upper_camel_case())
                }
            } else {
                format!("{}{index}", keyword.to_upper_camel_case())
            };
            visit(
                branch,
                &format!("{location}/{keyword}/{index}"),
                &format!("{name}{suffix}"),
                &format!("{original_name}{suffix}"),
                domain,
                description,
                candidates,
            )?;
        }
    }
    for (keyword, suffix) in
        [("items", "Item"), ("additionalProperties", "Value"), ("propertyNames", "Key")]
    {
        if let Some(child) = schema.get(keyword) {
            visit(
                child,
                &format!("{location}/{keyword}"),
                &format!("{name}{suffix}"),
                &format!("{original_name}{suffix}"),
                domain,
                None,
                candidates,
            )?;
        }
    }
    Ok(())
}

fn register(
    mut name: String,
    definitions: Vec<Candidate>,
    reserved: &mut BTreeSet<String>,
    catalog: &mut NewtypeCatalog,
) -> Result<(), ConversionError> {
    if !definitions.iter().all(|definition| definition.scalar == definitions[0].scalar) {
        return Err(fail(format!("conflicting bounds for {name}")));
    }
    if reserved.contains(&name) {
        name.push_str("Value");
    }
    syn::parse_str::<syn::Ident>(&name).map_err(|cause| fail(format!("{name}: {cause}")))?;
    if !reserved.insert(name.clone()) || !reserved.insert(format!("{name}ValidationError")) {
        return Err(fail(format!("duplicate newtype name {name}")));
    }
    let domains: BTreeSet<_> =
        definitions.iter().map(|definition| definition.domain.as_str()).collect();
    let domain = if domains.len() == 1 { *domains.first().unwrap() } else { "common" };
    let module = format!("models/{domain}/newtypes.rs");
    let type_path = format!("crate::models::{domain}::newtypes::{name}");
    let scalar = definitions[0].scalar.clone();
    let description = definitions.iter().find_map(|definition| definition.description.clone());
    for definition in definitions {
        catalog.type_paths.insert(definition.location, type_path.clone());
    }
    catalog.modules.entry(module).or_default().push(Newtype { name, description, scalar });
    Ok(())
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn fail(message: String) -> ConversionError {
    ConversionError::InvalidModel(format!("bounded scalar: {message}"))
}
