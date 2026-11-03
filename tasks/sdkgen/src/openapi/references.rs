use std::collections::BTreeSet;

use serde_json::Value;

pub(super) fn dereference<'a>(
    mut value: &'a Value,
    mut location: String,
    document: &'a Value,
) -> Result<(&'a Value, String), String> {
    let mut visited = BTreeSet::new();
    while let Some(reference) = value.get("$ref").and_then(Value::as_str) {
        if !visited.insert(reference) {
            return Err(format!("recursive reference {reference}"));
        }
        value = document
            .pointer(reference.strip_prefix('#').ok_or("external reference is unsupported")?)
            .ok_or_else(|| format!("unresolved reference {reference}"))?;
        location = reference.into();
    }
    Ok((value, location))
}
