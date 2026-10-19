use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, ensure};
use regex::Regex;
use serde::Serialize;

use crate::{
    metadata::Bindings,
    openapi::{Document, Schema},
    schema,
};

#[derive(Default, Serialize)]
pub struct Resources {
    pub params: Vec<Params>,
    pub accessors: Vec<Accessor>,
    pub resources: Vec<Resource>,
}

#[derive(Serialize)]
pub struct Params {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Serialize)]
pub struct Accessor {
    pub name: String,
    pub resource_type: String,
}

#[derive(Serialize)]
pub struct Resource {
    pub name: String,
    pub children: Vec<Accessor>,
    pub methods: Vec<Method>,
}

#[derive(Serialize)]
pub struct Method {
    pub name: String,
    pub documentation: String,
    pub arguments: String,
    pub output: String,
    pub error: String,
    pub generated_name: String,
    pub call_arguments: String,
}

#[derive(Serialize)]
pub struct Webhooks {
    pub events: Vec<Event>,
}

#[derive(Serialize)]
pub struct Event {
    pub name: String,
    pub schema: String,
    pub variant: String,
}

pub fn resources(spec: &Document, bindings: &Bindings) -> Result<Resources> {
    bindings.validate()?;
    let methods: Vec<_> = bindings
        .symbols
        .iter()
        .filter(|symbol| {
            symbol.path.starts_with("client::HttpClient::")
                && symbol.signature.as_ref().is_some_and(|signature| signature.asynchronous)
                && symbol.operation.as_ref().is_some_and(|operation| !operation.webhook)
        })
        .collect();
    let expected: BTreeSet<_> =
        schema::operations(spec).map(|op| op.operation_id.as_str()).collect();
    let actual: BTreeSet<_> = methods
        .iter()
        .map(|symbol| symbol.operation.as_ref().unwrap().source_operation_id.as_str())
        .collect();
    ensure!(actual == expected, "Bindings do not cover every HTTP operation");
    let return_type = Regex::new(r"^Result < (.*) , ApiOpError < (.*) > >$")?;
    let mut groups: BTreeMap<Vec<String>, Vec<Method>> = BTreeMap::new();
    let mut context = Resources::default();
    for symbol in methods {
        let origin = symbol.operation.as_ref().unwrap();
        let operation = spec.operation(&origin.source_path, &origin.source_method)?;
        let mut group: Vec<_> = origin.source_operation_id.split(':').map(snake).collect();
        let name = group.pop().context("Operation has no name")?;
        ensure!(!group.is_empty(), "Operation has no resource: {}", origin.source_operation_id);
        for length in 1..=group.len() {
            groups.entry(group[..length].to_vec()).or_default();
        }
        let signature = symbol.signature.as_ref().unwrap();
        let result = return_type
            .captures(&signature.return_type)
            .with_context(|| format!("Unsupported return type: {}", signature.return_type))?;
        let mut required = vec!["&self".to_owned()];
        let mut optional = Vec::new();
        let mut calls = Vec::new();
        let schemas: BTreeMap<_, _> = operation
            .parameters
            .iter()
            .map(|parameter| (snake(&parameter.name), &parameter.schema))
            .collect();
        for arg in &signature.arguments {
            let is_optional = arg.rust_type.starts_with("Option < ");
            let value = if is_optional { format!("params.{}", arg.name) } else { arg.name.clone() };
            let schema = schemas.get(arg.name.trim_start_matches("r#")).copied();
            let (typ, expression) = parameter_type(&arg.rust_type, schema, &value);
            let field = format!("{}: {typ}", arg.name);
            if is_optional {
                optional.push(field);
            } else {
                required.push(field);
            }
            calls.push(expression);
        }
        if !optional.is_empty() {
            let name = format!("{}Params", pascal(&symbol.name));
            required.push(format!("params: {name}"));
            context.params.push(Params { name, fields: optional });
        }
        groups.get_mut(&group).unwrap().push(Method {
            name: name.clone(),
            documentation: operation.summary.as_deref().unwrap_or(&name).replace(['\n', '\r'], " "),
            arguments: required.join(", "),
            output: result[1].to_owned(),
            error: result[2].to_owned(),
            generated_name: symbol.name.clone(),
            call_arguments: calls.join(", "),
        });
    }
    let keys: Vec<_> = groups.keys().cloned().collect();
    for (group, methods) in groups {
        if group.len() == 1 {
            context
                .accessors
                .push(Accessor { name: group[0].clone(), resource_type: pascal(&group[0]) });
        }
        let children: Vec<_> = keys
            .iter()
            .filter(|child| child.len() == group.len() + 1 && child.starts_with(&group))
            .map(|child| Accessor {
                name: child.last().unwrap().clone(),
                resource_type: pascal(&child.join("_")),
            })
            .collect();
        let names: BTreeSet<_> = methods
            .iter()
            .map(|method| &method.name)
            .chain(children.iter().map(|child| &child.name))
            .collect();
        ensure!(
            names.len() == methods.len() + children.len(),
            "Resource member collision: {}",
            group.join(".")
        );
        context.resources.push(Resource { name: pascal(&group.join("_")), children, methods });
    }
    Ok(context)
}

pub fn webhooks(spec: &Document) -> Result<Webhooks> {
    let mut events = Vec::new();
    let mut variants = BTreeSet::new();
    for (name, item) in &spec.webhooks {
        let reference = item
            .operations
            .get("post")
            .and_then(|op| op.request_body.as_ref())
            .and_then(|body| body.content.get("application/json"))
            .and_then(|media| media.schema.object())
            .and_then(|schema| schema.reference.as_deref())
            .with_context(|| format!("Missing payload schema for webhook {name}"))?;
        let schema = reference
            .strip_prefix("#/components/schemas/")
            .context("Expected a local webhook schema")?;
        ensure!(spec.components.schemas.contains_key(schema), "Missing webhook schema {schema}");
        let variant = pascal(name);
        ensure!(variants.insert(variant.clone()), "Duplicate webhook variant {variant}");
        events.push(Event { name: name.clone(), schema: schema.to_owned(), variant });
    }
    Ok(Webhooks { events })
}

fn snake(name: &str) -> String {
    name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect()
}

fn pascal(name: &str) -> String {
    name.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| format!("{}{}", part[..1].to_ascii_uppercase(), &part[1..]))
        .collect()
}

/// Convert generated string arguments to native scalar types in the public
/// facade.
fn parameter_type(rust_type: &str, schema: Option<&Schema>, value: &str) -> (String, String) {
    let rust_type = rust_type.replace("impl AsRef < str >", "String");
    if let Some(inner) = rust_type.strip_prefix("Option < ").and_then(|s| s.strip_suffix(" >")) {
        let (inner, expression) = parameter_type(inner, schema, "v");
        return (
            format!("Option < {inner} >"),
            if expression == "v" {
                value.to_owned()
            } else {
                format!("{value}.map(|v| {expression})")
            },
        );
    }
    if let Some(inner) = rust_type.strip_prefix("Vec < ").and_then(|s| s.strip_suffix(" >")) {
        let (inner, expression) = parameter_type(
            inner,
            schema.and_then(Schema::object).and_then(|s| s.items.as_ref()),
            "v",
        );
        return (
            format!("Vec < {inner} >"),
            if expression == "v" {
                value.to_owned()
            } else {
                format!("{value}.into_iter().map(|v| {expression}).collect()")
            },
        );
    }
    if rust_type == "String" {
        let typed = match schema.and_then(Schema::object).and_then(|s| s.format.as_deref()) {
            Some("uuid" | "uuid4" | "uuid_v4" | "UUID") => Some(("uuid::Uuid", "to_string")),
            Some("date-time") => Some(("chrono::DateTime<chrono::Utc>", "to_rfc3339")),
            Some("date") => Some(("chrono::NaiveDate", "to_string")),
            Some("uri" | "url") => Some(("url::Url", "to_string")),
            _ => None,
        };
        if let Some((typ, conversion)) = typed {
            return (typ.to_owned(), format!("{value}.{conversion}()"));
        }
    }
    (rust_type, value.to_owned())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn uuid_vectors_and_optional_dates_keep_native_public_types() {
        let (typ, expression) = parameter_type(
            "Option < Vec < String > >",
            Some(
                &serde_json::from_value(json!({"type":"array","items":{"format":"uuid4"}}))
                    .unwrap(),
            ),
            "params.id",
        );
        assert_eq!(typ, "Option < Vec < uuid::Uuid > >");
        assert_eq!(expression, "params.id.map(|v| v.into_iter().map(|v| v.to_string()).collect())");
        let (typ, expression) = parameter_type(
            "Option < impl AsRef < str > >",
            Some(&serde_json::from_value(json!({"format":"date-time"})).unwrap()),
            "params.since",
        );
        assert_eq!(typ, "Option < chrono::DateTime<chrono::Utc> >");
        assert_eq!(expression, "params.since.map(|v| v.to_rfc3339())");
    }

    #[test]
    fn missing_operation_bindings_fail_instead_of_silently_dropping_methods() {
        let spec = crate::openapi::fixture(json!({"/items":{"get":{"operationId":"items:list"}}}));
        assert!(resources(&spec, &crate::metadata::fixture(Vec::new())).is_err());
    }
}
