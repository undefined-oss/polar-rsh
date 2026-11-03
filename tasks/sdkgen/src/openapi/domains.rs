//! Track schema usage through operation and webhook references.

use std::collections::{BTreeMap, BTreeSet};

use heck::ToSnakeCase;
use serde_json::Value;

use super::ConversionError;

pub struct SchemaDomains {
    pub active: BTreeMap<String, BTreeSet<String>>,
}

pub fn schema_domains(spec: &oas3::Spec) -> Result<SchemaDomains, ConversionError> {
    let document = serde_json::to_value(spec).map_err(|cause| fail(cause.to_string()))?;
    let mut usage = BTreeMap::new();
    for (path, item) in spec.paths.iter().flatten() {
        for (method, operation) in item.methods() {
            let group = operation
                .extensions
                .get("speakeasy-group")
                .and_then(Value::as_str)
                .ok_or_else(|| fail(format!("{method:?} {path}: missing x-speakeasy-group")))?;
            // Nested groups such as customers.members belong to the customers
            // domain.
            let domain = group.split('.').next().unwrap_or(group).to_snake_case();
            syn::parse_str::<syn::Ident>(&domain)
                .map_err(|_| fail(format!("invalid domain from x-speakeasy-group: {group}")))?;
            let operation =
                serde_json::to_value(operation).map_err(|cause| fail(cause.to_string()))?;
            let parameters =
                serde_json::to_value(&item.parameters).map_err(|cause| fail(cause.to_string()))?;
            let mut visited = BTreeSet::new();
            visit(&operation, &document, &domain, &mut visited, &mut usage)?;
            visit(&parameters, &document, &domain, &mut visited, &mut usage)?;
        }
    }
    if let Some(webhooks) = document.get("webhooks") {
        let mut webhook_usage = BTreeMap::new();
        visit(webhooks, &document, "webhooks", &mut BTreeSet::new(), &mut webhook_usage)?;
        // Webhook envelopes belong to webhooks; their data reuses the owning
        // API domain's models instead of moving those models to common.
        for (schema, domains) in webhook_usage {
            usage.entry(schema).or_insert(domains);
        }
    }
    Ok(SchemaDomains { active: usage })
}

fn visit(
    value: &Value,
    document: &Value,
    domain: &str,
    visited: &mut BTreeSet<String>,
    usage: &mut BTreeMap<String, BTreeSet<String>>,
) -> Result<(), ConversionError> {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str)
                && visited.insert(reference.to_owned())
            {
                let pointer = reference.strip_prefix('#').ok_or_else(|| {
                    fail(format!("external reference is unsupported: {reference}"))
                })?;
                let target = document
                    .pointer(pointer)
                    .ok_or_else(|| fail(format!("unresolved reference: {reference}")))?;
                if let Some(name) = pointer.strip_prefix("/components/schemas/")
                    && !name.contains('/')
                {
                    let name = name.replace("~1", "/").replace("~0", "~");
                    usage.entry(name).or_default().insert(domain.to_owned());
                }
                visit(target, document, domain, visited, usage)?;
            }
            for (key, child) in object {
                // Map entries are names, including `enum` properties and
                // `default` responses.
                if matches!(key.as_str(), "properties" | "patternProperties" | "responses")
                    && let Value::Object(entries) = child
                {
                    for entry in entries.values() {
                        visit(entry, document, domain, visited, usage)?;
                    }
                    continue;
                }
                // Example payloads and tooling metadata do not establish type
                // ownership.
                if matches!(key.as_str(), "example" | "examples" | "default" | "enum")
                    || (key.starts_with("x-") && key != "x-polar-pagination")
                {
                    continue;
                }
                visit(child, document, domain, visited, usage)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                visit(value, document, domain, visited, usage)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn fail(message: String) -> ConversionError {
    ConversionError::InvalidDomain { message }
}
