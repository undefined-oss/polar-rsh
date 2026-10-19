use anyhow::{Context, Result, bail, ensure};
use serde::Serialize;
use serde_json::Value;

use crate::openapi::{Document, Location, Operation, PrimitiveType, Schema, SchemaObject, Style};

#[derive(Serialize)]
pub struct Overlay {
    overlay: &'static str,
    info: OverlayInfo,
    actions: Vec<Action>,
}

#[derive(Serialize)]
struct OverlayInfo {
    title: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
struct Action {
    target: String,
    #[serde(flatten)]
    change: Change,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Change {
    Remove(bool),
    Update(Update),
}

#[derive(Serialize)]
#[serde(untagged)]
enum Update {
    Schema { schema: Box<SchemaObject> },
    Encoding { style: Style, explode: bool },
}

pub fn normalize_queries(spec: &mut Document) -> Result<Overlay> {
    let mut actions = Vec::new();
    for (path, item) in &mut spec.paths {
        for (method, operation) in &mut item.operations {
            for (index, parameter) in operation.parameters.iter_mut().enumerate() {
                if parameter.location != Location::Query {
                    continue;
                }
                let Some(schema) = parameter.schema.object() else {
                    continue;
                };
                let Some(union) = &schema.any_of else {
                    continue;
                };
                ensure!(
                    !parameter.required,
                    "Review required union query: {path} {}",
                    parameter.name
                );
                let branches: Vec<_> =
                    union.iter().filter(|branch| !branch.is_type(PrimitiveType::Null)).collect();
                let array_of = |array: &Schema, scalar: &Schema| {
                    array.is_type(PrimitiveType::Array)
                        && array.object().and_then(|s| s.items.as_ref()) == Some(scalar)
                };
                let selected = match branches.as_slice() {
                    [single] => *single,
                    [a, b] if array_of(a, b) => *a,
                    [a, b] if array_of(b, a) => *b,
                    _ => bail!("Unsupported query union: {path} {}", parameter.name),
                };
                let mut normalized = schema.clone();
                normalized.any_of = None;
                normalized.merge(selected.object().context("Invalid union branch")?);
                if normalized.default.as_ref().is_some_and(Value::is_null) {
                    normalized.default = None;
                }
                let is_array = normalized.kind
                    == Some(crate::openapi::SchemaType::Single(PrimitiveType::Array));
                if is_array
                    && let Some(default) = &mut normalized.default
                    && !default.is_array()
                {
                    *default = Value::Array(vec![default.take()]);
                }
                let target = format!(
                    "$['paths'][{}][{}]['parameters'][{index}]",
                    serde_json::to_string(path)?,
                    serde_json::to_string(method)?
                );
                actions.push(Action {
                    target: format!("{target}['schema']"),
                    change: Change::Remove(true),
                });
                actions.push(Action {
                    target: target.clone(),
                    change: Change::Update(Update::Schema { schema: Box::new(normalized.clone()) }),
                });
                parameter.schema = Schema::Object(Box::new(normalized));
                if is_array {
                    parameter.style = Some(Style::Form);
                    parameter.explode = Some(true);
                    actions.push(Action {
                        target,
                        change: Change::Update(Update::Encoding {
                            style: Style::Form,
                            explode: true,
                        }),
                    });
                }
            }
        }
    }
    Ok(Overlay {
        overlay: "1.1.0",
        info: OverlayInfo {
            title: "Polar query parameters for Rust code generation",
            version: "1.0.0",
        },
        actions,
    })
}

pub fn operations(spec: &Document) -> impl Iterator<Item = &Operation> {
    spec.paths.values().flat_map(|item| item.operations.values())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn normalizes_only_query_unions_and_preserves_body_nullability() {
        let nullable = json!({"anyOf": [{"type":"string","format":"uuid4"}, {"type":"array","items":{"type":"string","format":"uuid4"}}, {"type":"null"}], "default": "one", "x-extra": {"keep": true}});
        let mut spec = crate::openapi::fixture(
            json!({"/items":{"get":{"operationId":"items:list", "parameters":[{"name":"id","in":"query","schema":nullable}],"requestBody":{"content":{"application/json":{"schema":nullable}}}}}}),
        );
        let body = spec.paths["/items"].operations["get"].request_body.as_ref().unwrap().content
            ["application/json"]
            .schema
            .clone();
        let overlay = normalize_queries(&mut spec).unwrap();
        let parameter = &spec.paths["/items"].operations["get"].parameters[0];
        assert!(parameter.schema.is_type(PrimitiveType::Array));
        let schema = parameter.schema.object().unwrap();
        assert_eq!(schema.default, Some(json!(["one"])));
        assert_eq!(parameter.explode, Some(true));
        assert!(schema.any_of.is_none());
        assert_eq!(schema.extensions["x-extra"], json!({"keep":true}));
        assert_eq!(
            spec.paths["/items"].operations["get"].request_body.as_ref().unwrap().content
                ["application/json"]
                .schema,
            body
        );
        assert_eq!(overlay.actions.len(), 3);
        let serialized = serde_json::to_value(overlay).unwrap();
        assert_eq!(serialized["actions"][0]["remove"], true);
        assert_eq!(serialized["actions"][1]["update"]["schema"]["default"], json!(["one"]));
    }

    #[test]
    fn rejects_unrelated_scalar_and_array_branches() {
        let mut spec = crate::openapi::fixture(
            json!({"/items":{"get":{"operationId":"items:list","parameters":[{"in":"query","name":"id","schema":{"anyOf":[{"type":"boolean"},{"type":"array","items":{"type":"string"}},{"type":"null"}]}}]}}}),
        );
        assert!(normalize_queries(&mut spec).is_err());
    }

    #[test]
    fn removes_explicit_null_defaults_without_losing_other_schema_keywords() {
        let mut spec = crate::openapi::fixture(
            json!({"/items":{"get":{"operationId":"items:list","parameters":[{"in":"query","name":"id","schema":{"anyOf":[{"type":"string","minLength":1},{"type":"null"}],"default":null,"description":"ID"}}]}}}),
        );
        normalize_queries(&mut spec).unwrap();
        let schema = spec.paths["/items"].operations["get"].parameters[0].schema.object().unwrap();
        assert!(schema.default.is_none());
        assert_eq!(schema.extensions["minLength"], 1);
        assert_eq!(schema.extensions["description"], "ID");
    }
}
