//! One discovery pass over schema definitions and operation schemas.

use std::collections::{BTreeMap, BTreeSet};

use heck::ToUpperCamelCase;
use serde_json::Value;

use super::{ConversionError, fail};

pub(super) struct Occurrence {
    pub location: String,
    pub owner: String,
    pub field: String,
    pub description: Option<String>,
    pub path_identity: Option<String>,
}

#[derive(Default)]
pub(super) struct Scan {
    pub occurrences: Vec<Occurrence>,
    pub references: Vec<(String, String)>,
    pub unions: Vec<(String, String)>,
    pub anchors: BTreeMap<String, BTreeSet<String>>,
    pub entities: BTreeSet<String>,
    relations: Vec<(String, String)>,
    visited: BTreeSet<String>,
}

pub(super) fn discover(spec: &oas3::Spec, document: &Value) -> Result<Scan, ConversionError> {
    // Preserve schema keywords that oas3 does not model, including
    // propertyNames.
    let usage = super::super::domains::schema_domains(spec)?;
    let mut scan = Scan::default();
    if let Some(schemas) = document.pointer("/components/schemas").and_then(Value::as_object) {
        for (name, schema) in schemas {
            if !usage.active.contains_key(name) && usage.excluded.contains(name) {
                continue;
            }
            let location = format!("#/components/schemas/{}", escape(name));
            scan.schema(schema, &location, name, "", None, None);
        }
    }
    if let Some(paths) = document.get("paths").and_then(Value::as_object) {
        for (path, item) in paths {
            let parent = format!("#/paths/{}", escape(path));
            for method in ["get", "post", "put", "patch", "delete", "head", "options", "trace"] {
                let Some(operation) = item.get(method) else { continue };
                if operation
                    .get("x-speakeasy-group")
                    .and_then(Value::as_str)
                    .is_some_and(|group| group.split('.').next() == Some("webhooks"))
                {
                    continue;
                }
                let location = format!("{parent}/{method}");
                let owner = operation
                    .get("operationId")
                    .and_then(Value::as_str)
                    .unwrap_or(path)
                    .to_upper_camel_case();
                let mut seen = BTreeSet::new();
                for (parameters, base) in
                    [(operation.get("parameters"), &location), (item.get("parameters"), &parent)]
                {
                    for (index, parameter) in
                        parameters.and_then(Value::as_array).into_iter().flatten().enumerate()
                    {
                        let parameter_location = format!("{base}/parameters/{index}");
                        let parameter = dereference(parameter, document, &parameter_location)?;
                        let name = parameter.get("name").and_then(Value::as_str).unwrap_or("");
                        let kind = parameter.get("in").and_then(Value::as_str).unwrap_or("");
                        if !seen.insert((name.to_owned(), kind.to_owned())) {
                            continue;
                        }
                        let identity = if kind == "path" {
                            Some(path_identity(path, name))
                        } else if matches!(name, "id" | "ids") {
                            Some(resource_identity(path))
                        } else {
                            None
                        };
                        if let Some(schema) = parameter.get("schema") {
                            let schema_location = format!("{parameter_location}/schema");
                            scan.schema(
                                schema,
                                &schema_location,
                                &owner,
                                name,
                                parameter.get("description").and_then(Value::as_str),
                                identity.as_deref(),
                            );
                        }
                    }
                }
                for key in ["requestBody", "responses", "callbacks"] {
                    if let Some(value) = operation.get(key) {
                        scan.schema_containers(value, &format!("{location}/{key}"), &owner);
                    }
                }
                if let Some(item) = operation.pointer("/x-polar-pagination/item_schema") {
                    let identity = resource_identity(path);
                    for target in schema_refs(item) {
                        scan.anchors.entry(target).or_default().insert(identity.clone());
                    }
                }
                // An item GET identifies the entity returned for its path ID.
                if method == "get" && path.ends_with('}') {
                    if let Some(parameter) = path
                        .rsplit('/')
                        .next()
                        .and_then(|part| part.strip_prefix('{'))
                        .and_then(|part| part.strip_suffix('}'))
                    {
                        let identity = path_identity(path, parameter);
                        if let Some(responses) =
                            operation.get("responses").and_then(Value::as_object)
                        {
                            for (status, response) in responses {
                                if status.starts_with('2') {
                                    if let Some(schema) =
                                        response.pointer("/content/application~1json/schema")
                                    {
                                        for target in schema_refs(schema) {
                                            scan.anchors
                                                .entry(target)
                                                .or_default()
                                                .insert(identity.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // Shared request/response/parameter components may contain inline schemas
    // too.
    if let Some(components) = document.get("components").and_then(Value::as_object) {
        for kind in ["parameters", "requestBodies", "responses", "headers"] {
            if let Some(entries) = components.get(kind).and_then(Value::as_object) {
                for (name, value) in entries {
                    scan.schema_containers(
                        value,
                        &format!("#/components/{kind}/{}", escape(name)),
                        &name.to_upper_camel_case(),
                    );
                }
            }
        }
    }
    for occurrence in &scan.occurrences {
        if let Some(name) = occurrence.field.strip_suffix("_id").filter(|name| *name != "parent") {
            scan.entities.insert(name.to_upper_camel_case());
        }
        if let Some(identity) = &occurrence.path_identity {
            scan.entities.insert(identity.clone());
        }
    }
    for (field, target) in &scan.relations {
        let entity = singular(field).to_upper_camel_case();
        if scan.entities.contains(&entity) {
            scan.anchors.entry(target.clone()).or_default().insert(entity);
        }
    }
    for occurrence in &scan.occurrences {
        if occurrence.field == "id" {
            if let Some(entity) = occurrence.description.as_deref().and_then(described_identity) {
                if scan.entities.contains(&entity) {
                    scan.anchors.entry(occurrence.owner.clone()).or_default().insert(entity);
                }
            }
        }
    }
    Ok(scan)
}

impl Scan {
    fn schema(
        &mut self,
        value: &Value,
        location: &str,
        owner: &str,
        field: &str,
        description: Option<&str>,
        path_identity: Option<&str>,
    ) {
        if !self.visited.insert(location.to_owned()) {
            return;
        }
        let description = value.get("description").and_then(Value::as_str).or(description);
        if let Some(reference) = value.get("$ref").and_then(Value::as_str) {
            self.references.push((location.into(), reference.into()));
        }
        if value.get("format").and_then(Value::as_str).is_some_and(|format| {
            matches!(format.to_ascii_lowercase().as_str(), "uuid" | "uuid4" | "uuid_v4")
        }) {
            self.occurrences.push(Occurrence {
                location: location.into(),
                owner: owner.to_upper_camel_case(),
                field: field.into(),
                description: description.map(str::to_owned),
                path_identity: path_identity.map(str::to_owned),
            });
        }
        if let Some(properties) = value.get("properties").and_then(Value::as_object) {
            for (name, schema) in properties {
                for target in schema_refs(schema) {
                    self.relations.push((name.clone(), target));
                }
                let child_location = format!("{location}/properties/{}", escape(name));
                let child_owner = if field.is_empty() {
                    owner.to_owned()
                } else {
                    format!("{owner}{}", field.to_upper_camel_case())
                };
                self.schema(schema, &child_location, &child_owner, name, None, None);
                if let Some(entity) = name.strip_suffix("_id") {
                    if let Some(related) = properties.get(entity) {
                        for target in schema_refs(related) {
                            self.anchors
                                .entry(target)
                                .or_default()
                                .insert(entity.to_upper_camel_case());
                        }
                    }
                }
            }
        }
        for keyword in ["anyOf", "oneOf", "allOf", "prefixItems"] {
            if let Some(branches) = value.get(keyword).and_then(Value::as_array) {
                for (index, branch) in branches.iter().enumerate() {
                    if field.is_empty()
                        && matches!(keyword, "anyOf" | "oneOf")
                        && value.get("properties").is_none()
                    {
                        for target in schema_refs(branch) {
                            self.unions.push((owner.to_upper_camel_case(), target));
                        }
                    }
                    self.schema(
                        branch,
                        &format!("{location}/{keyword}/{index}"),
                        owner,
                        field,
                        description,
                        path_identity,
                    );
                }
            }
        }
        for keyword in ["items", "additionalProperties", "contains", "not"] {
            if let Some(child) = value.get(keyword).filter(|value| value.is_object()) {
                self.schema(
                    child,
                    &format!("{location}/{keyword}"),
                    owner,
                    field,
                    description,
                    path_identity,
                );
            }
        }
        if let Some(keys) = value.get("propertyNames") {
            let key_location = format!("{location}/propertyNames");
            self.schema(keys, &key_location, owner, field, description, path_identity);
        }
    }

    fn schema_containers(&mut self, value: &Value, location: &str, owner: &str) {
        match value {
            Value::Object(object) => {
                for (key, child) in object {
                    if key == "schema" {
                        let location = format!("{location}/schema");
                        self.schema(child, &location, owner, "", None, None);
                    } else if key == "$ref" {
                        if let Some(target) = child.as_str() {
                            self.references.push((location.into(), target.into()));
                        }
                    } else if !key.starts_with("x-")
                        && !matches!(key.as_str(), "examples" | "example")
                    {
                        self.schema_containers(
                            child,
                            &format!("{location}/{}", escape(key)),
                            owner,
                        );
                    }
                }
            }
            Value::Array(values) => {
                for (index, child) in values.iter().enumerate() {
                    self.schema_containers(child, &format!("{location}/{index}"), owner);
                }
            }
            _ => {}
        }
    }
}

fn schema_refs(schema: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(name) = schema
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|r| r.strip_prefix("#/components/schemas/"))
    {
        if !name.contains('/') {
            refs.push(name.replace("~1", "/").replace("~0", "~").to_upper_camel_case());
        }
    }
    for key in ["anyOf", "oneOf"] {
        for branch in schema.get(key).and_then(Value::as_array).into_iter().flatten() {
            refs.extend(schema_refs(branch));
        }
    }
    if let Some(items) = schema.get("items") {
        refs.extend(schema_refs(items));
    }
    refs
}

fn described_identity(description: &str) -> Option<String> {
    let text = description.trim().trim_end_matches('.');
    let entity = text
        .strip_prefix("The ID of the ")
        .or_else(|| text.strip_prefix("ID of the "))
        .or_else(|| text.strip_prefix("The ").and_then(|text| text.strip_suffix(" ID")))?;
    Some(entity.to_upper_camel_case())
}

fn resource_identity(path: &str) -> String {
    singular(path.trim_end_matches('/').rsplit('/').next().unwrap_or(path)).to_upper_camel_case()
}

fn path_identity(path: &str, parameter: &str) -> String {
    if parameter != "id" {
        return parameter.strip_suffix("_id").unwrap_or(parameter).to_upper_camel_case();
    }
    let marker = format!("{{{parameter}}}");
    let mut previous = "";
    for segment in path.split('/') {
        if segment == marker {
            return singular(previous).to_upper_camel_case();
        }
        previous = segment;
    }
    path.to_upper_camel_case()
}

pub(super) fn singular(value: &str) -> String {
    if let Some(stem) = value.strip_suffix("ies") {
        format!("{stem}y")
    } else if !value.ends_with("ss") {
        value.strip_suffix('s').unwrap_or(value).to_owned()
    } else {
        value.to_owned()
    }
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn dereference<'a>(
    mut value: &'a Value,
    document: &'a Value,
    location: &str,
) -> Result<&'a Value, ConversionError> {
    let mut seen = BTreeSet::new();
    while let Some(reference) = value.get("$ref").and_then(Value::as_str) {
        if !seen.insert(reference) {
            return Err(fail(location, "cyclic parameter reference"));
        }
        value = reference
            .strip_prefix('#')
            .and_then(|pointer| document.pointer(pointer))
            .ok_or_else(|| fail(location, format!("unresolved reference {reference}")))?;
    }
    Ok(value)
}
