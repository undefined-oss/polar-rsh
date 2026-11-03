//! Resolve component enums and place them according to domain usage.

use std::collections::BTreeMap;

use oas3::spec::{ObjectOrReference, Schema, SchemaType, SchemaTypeSet};

use super::{ConversionError, domains};
use crate::templates::string_enums::{StringEnum, StringVariant};

mod inline;

pub struct EnumCatalog {
    pub modules: BTreeMap<String, Vec<StringEnum>>,
    /// Original component name to its generated Rust type path.
    pub type_paths: BTreeMap<String, String>,
    /// Exact inline schema location to its generated Rust type path.
    pub inline_type_paths: BTreeMap<String, String>,
}

pub fn collect_enums(
    spec: &oas3::Spec,
    document: &serde_json::Value,
) -> Result<EnumCatalog, ConversionError> {
    let usage = domains::schema_domains(spec)?;
    let mut modules: BTreeMap<String, Vec<StringEnum>> = BTreeMap::new();
    let mut type_paths = BTreeMap::new();
    for (name, schema) in spec.components.iter().flat_map(|components| &components.schemas) {
        // Webhook-only types are deferred; webhook references do not make an
        // otherwise domain-local enum common.
        if !usage.active.contains_key(name) && usage.excluded.contains(name) {
            continue;
        }
        let error = |message: String| ConversionError::InvalidEnum { name: name.clone(), message };
        let resolved = schema.resolve(spec).map_err(|cause| error(cause.to_string()))?;
        let Schema::Object(resolved) = resolved else { continue };
        let ObjectOrReference::Object(schema) = *resolved else {
            return Err(error("unresolved schema reference".into()));
        };
        if schema.enum_values.is_empty() {
            continue;
        }
        if schema.schema_type != Some(SchemaTypeSet::Single(SchemaType::String)) {
            return Err(error("only string component enums are currently supported".into()));
        }
        syn::parse_str::<syn::Ident>(name).map_err(|cause| error(cause.to_string()))?;
        let variants = schema
            .enum_values
            .iter()
            .map(|value| {
                let wire_value =
                    value.as_str().ok_or_else(|| error("enum values must be strings".into()))?;
                Ok(StringVariant { description: None, wire_value: wire_value.to_owned() })
            })
            .collect::<Result<Vec<_>, ConversionError>>()?;
        // Sorting direction is represented by query::Sorting.
        let variants =
            variants.into_iter().filter(|variant| !variant.wire_value.starts_with('-')).collect();
        let default_value = schema
            .default
            .as_ref()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| error("enum default must be a string".into()))
            })
            .transpose()?;

        // Unreferenced components remain available in common until ownership is
        // known.
        let domain = usage
            .active
            .get(name)
            .filter(|domains| domains.len() == 1)
            .and_then(|domains| domains.first())
            .map(String::as_str)
            .unwrap_or("common");
        let module_path = format!("models/{domain}/enums.rs");
        type_paths.insert(name.clone(), format!("crate::models::{domain}::enums::{name}"));
        modules.entry(module_path).or_default().push(StringEnum {
            name: name.clone(),
            description: schema.description,
            derives: derives(),
            variants,
            default_value,
        });
    }
    let mut catalog = EnumCatalog { modules, type_paths, inline_type_paths: BTreeMap::new() };
    inline::collect(document, &usage, &mut catalog)?;
    Ok(catalog)
}

fn derives() -> Vec<String> {
    [
        "Debug",
        "Clone",
        "Copy",
        "PartialEq",
        "Eq",
        "PartialOrd",
        "Ord",
        "Hash",
        "Deserialize",
        "Serialize",
    ]
    .map(String::from)
    .into()
}
