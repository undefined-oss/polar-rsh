//! Keep operation builder and resource method documentation consistent.

use std::collections::BTreeSet;

use serde_json::Value;

pub(super) fn description(operation: &Value, document: &Value) -> Option<String> {
    let mut description = ["description", "summary"]
        .into_iter()
        .filter_map(|key| operation.get(key).and_then(Value::as_str))
        .find(|text| !text.trim().is_empty())
        .unwrap_or_default()
        .to_owned();

    // Polar already documents scopes in most descriptions. Preserve that
    // section, including its wording, instead of emitting it twice.
    if !description.lines().any(|line| line.trim_start().starts_with("**Scopes**:")) {
        // An explicit operation security array overrides the document default,
        // including an empty array. Token schemes often repeat the same scopes.
        let scopes: BTreeSet<_> = operation
            .get("security")
            .or_else(|| document.get("security"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_object)
            .flat_map(|requirement| requirement.values())
            .filter_map(Value::as_array)
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        if !scopes.is_empty() {
            if !description.is_empty() {
                description = description.trim_end().to_owned();
                description.push_str("\n\n");
            }
            description.push_str("**Scopes**: ");
            description.push_str(
                &scopes.into_iter().map(|scope| format!("`{scope}`")).collect::<Vec<_>>().join(" "),
            );
        }
    }

    (!description.is_empty()).then_some(description)
}
