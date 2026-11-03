//! Resolve anyOf error payloads into the operation's typed error variants.

use std::collections::BTreeSet;

use serde_json::Value;

use super::{ModelCatalog, component_type, dereference};

pub(super) fn payloads(
    schema: &Value,
    location: &str,
    document: &Value,
    models: &ModelCatalog,
) -> Result<Vec<(String, String)>, String> {
    resolve(schema, location, document, models, &mut BTreeSet::new())
}

fn resolve(
    schema: &Value,
    location: &str,
    document: &Value,
    models: &ModelCatalog,
    visiting: &mut BTreeSet<String>,
) -> Result<Vec<(String, String)>, String> {
    if schema.get("$ref").is_some() {
        if has_sibling_constraints(schema, "$ref") {
            return Err(format!(
                "{location}: error references with sibling constraints are not supported"
            ));
        }
        if let Ok((name, ty)) = component_type(schema, models, "error")
            && ty.contains("::types::")
        {
            return Ok(vec![(name, ty)]);
        }
        let (target, target_location) = dereference(schema, location.into(), document)?;
        if !visiting.insert(target_location.clone()) {
            return Err(format!("{target_location}: recursive error union"));
        }
        let result = resolve(target, &target_location, document, models, visiting);
        visiting.remove(&target_location);
        return result;
    }
    let branches = schema.get("anyOf").and_then(Value::as_array).ok_or_else(|| {
        format!("{location}: error responses require a component struct or anyOf")
    })?;
    if branches.is_empty() || has_sibling_constraints(schema, "anyOf") {
        return Err(format!(
            "{location}: error anyOf requires nonempty branches without a discriminator or \
             sibling constraints"
        ));
    }
    let mut payloads = Vec::new();
    for (index, branch) in branches.iter().enumerate() {
        payloads.extend(resolve(
            branch,
            &format!("{location}/anyOf/{index}"),
            document,
            models,
            visiting,
        )?);
    }
    Ok(payloads)
}

fn has_sibling_constraints(schema: &Value, keyword: &str) -> bool {
    schema.as_object().unwrap().keys().any(|key| {
        key != keyword
            && !matches!(
                key.as_str(),
                "title"
                    | "description"
                    | "deprecated"
                    | "readOnly"
                    | "writeOnly"
                    | "default"
                    | "examples"
                    | "example"
                    | "$comment"
                    | "$id"
                    | "$schema"
            )
            && !key.starts_with("x-")
    })
}
