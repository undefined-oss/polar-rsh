use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

pub(super) struct Resolver<'a> {
    pub schemas: &'a Map<String, Value>,
    pub model_paths: &'a BTreeMap<String, String>,
    pub enum_paths: &'a BTreeMap<String, String>,
    pub inline_enum_paths: &'a BTreeMap<String, String>,
    pub uuid_paths: &'a BTreeMap<String, String>,
    pub newtype_paths: &'a BTreeMap<String, String>,
    pub union_paths: &'a BTreeMap<String, super::unions::UnionSchema>,
    pub aliases: &'a BTreeSet<String>,
}

pub(super) struct ResolvedType {
    pub base: String,
    pub nullable: bool,
}

impl ResolvedType {
    fn new(base: impl Into<String>) -> Self {
        Self { base: base.into(), nullable: false }
    }

    pub(super) fn nested(self) -> String {
        if self.nullable { format!("Option<{}>", self.base) } else { self.base }
    }
}

impl Resolver<'_> {
    pub fn resolve(
        &self,
        schema: &Value,
        location: &str,
        dependencies: &mut BTreeSet<String>,
        visiting: &mut BTreeSet<String>,
    ) -> Result<ResolvedType, String> {
        let fail = |message: &str| format!("{location}: {message}");
        if let Some(value) = schema.get("const") {
            let ty = match value {
                Value::String(value) => quote::quote!(monostate::MustBe!(#value)),
                Value::Bool(value) => quote::quote!(monostate::MustBe!(#value)),
                _ => return Err(fail("only string and boolean literals are currently supported")),
            };
            return Ok(ResolvedType::new(ty.to_string()));
        }
        if let Some(union) = self.union_paths.get(location) {
            dependencies.insert(union.key.clone());
            return Ok(ResolvedType { base: union.type_path.clone(), nullable: union.nullable });
        }
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            let name = reference
                .strip_prefix("#/components/schemas/")
                .ok_or_else(|| fail("unsupported non-component schema reference"))?
                .replace("~1", "/")
                .replace("~0", "~");
            if let Some(ty) = self.enum_paths.get(&name) {
                return Ok(ResolvedType::new(ty.clone()));
            }
            if name == "Pagination" {
                return Ok(ResolvedType::new("crate::models::pagination::Pagination"));
            }
            let target =
                self.schemas.get(&name).ok_or_else(|| fail("unresolved schema reference"))?;
            if self.aliases.contains(&name) {
                if !visiting.insert(name.clone()) {
                    return Err(fail("recursive alias requires a distinct model type"));
                }
                let underlying = self.resolve(target, reference, dependencies, visiting)?;
                visiting.remove(&name);
                // Keep nullability at the field boundary, so a required
                // nullable alias cannot silently deserialize a
                // missing field as None.
                if underlying.nullable {
                    return Ok(underlying);
                }
            }
            if let Some(item) = list_item(target) {
                if !visiting.insert(name.clone()) {
                    return Err(fail("recursive list schema"));
                }
                let ty = self.resolve(
                    item,
                    &format!("{reference}/properties/items/items"),
                    dependencies,
                    visiting,
                )?;
                visiting.remove(&name);
                return Ok(ResolvedType::new(format!(
                    "crate::models::pagination::ListResource<{}>",
                    ty.nested()
                )));
            }
            let ty = self
                .model_paths
                .get(&name)
                .ok_or_else(|| fail("reference targets an excluded schema"))?;
            dependencies.insert(name);
            return Ok(ResolvedType::new(ty.clone()));
        }
        for keyword in ["anyOf", "oneOf"] {
            if let Some(branches) = schema.get(keyword).and_then(Value::as_array) {
                let mut nullable = false;
                let mut types = BTreeSet::new();
                for (index, branch) in branches.iter().enumerate() {
                    if branch.get("type").and_then(Value::as_str) == Some("null") {
                        nullable = true;
                        continue;
                    }
                    let ty = self.resolve(
                        branch,
                        &format!("{location}/{keyword}/{index}"),
                        dependencies,
                        visiting,
                    )?;
                    nullable |= ty.nullable;
                    types.insert(ty.base);
                }
                if types.len() != 1 {
                    return Err(fail("multiple non-null types require a union template"));
                }
                return Ok(ResolvedType { base: types.into_iter().next().unwrap(), nullable });
            }
        }
        if schema.get("allOf").is_some() {
            return Err(fail("allOf requires composition support"));
        }
        if schema.get("enum").is_some() {
            if let Some(ty) = self.inline_enum_paths.get(location) {
                return Ok(ResolvedType::new(ty.clone()));
            }
            return Err(fail("inline enum is missing from the enum catalog"));
        }
        if let Some(ty) = self.uuid_paths.get(location) {
            return Ok(ResolvedType::new(ty.clone()));
        }
        if let Some(ty) = self.newtype_paths.get(location) {
            return Ok(ResolvedType::new(ty.clone()));
        }
        let base = match schema.get("type").and_then(Value::as_str) {
            Some("string") => match schema.get("format").and_then(Value::as_str) {
                Some("uuid" | "uuid4" | "uuid_v4" | "UUID") => {
                    return Err(fail("UUID is missing from the identity catalog"));
                }
                Some("date-time") => "jiff::Timestamp",
                Some("date") => "jiff::civil::Date",
                Some("uri") => "url::Url",
                _ => "String",
            }
            .to_owned(),
            Some("integer") => super::super::newtypes::integer_type(schema).into(),
            Some("number") => "f64".into(),
            Some("boolean") => "bool".into(),
            // Unit accepts only JSON null. Keeping it as the base type lets
            // field presence distinguish required null from optional null.
            Some("null") => "()".into(),
            Some("array") => {
                if let Some(prefix) = schema.get("prefixItems") {
                    let prefix =
                        prefix.as_array().ok_or_else(|| fail("prefixItems must be an array"))?;
                    let length = prefix.len() as u64;
                    let minimum = schema.get("minItems").and_then(Value::as_u64).unwrap_or(0);
                    let maximum = schema.get("maxItems").and_then(Value::as_u64).or_else(|| {
                        (schema.get("items") == Some(&Value::Bool(false))).then_some(length)
                    });
                    if minimum != length || maximum != Some(length) {
                        return Err(fail(
                            "variable-length prefixItems require a dedicated array type",
                        ));
                    }
                    if prefix.len() > 12 {
                        return Err(fail(
                            "tuples with more than 12 items require a dedicated type",
                        ));
                    }
                    if prefix.is_empty() {
                        return Ok(ResolvedType::new("[serde_json::Value; 0]"));
                    }
                    let items = prefix
                        .iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let ty = self.resolve(
                                item,
                                &format!("{location}/prefixItems/{index}"),
                                dependencies,
                                visiting,
                            )?;
                            syn::parse_str::<syn::Type>(&ty.nested())
                                .map_err(|cause| format!("{location}/prefixItems/{index}: {cause}"))
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    return Ok(ResolvedType::new(quote::quote!((#(#items,)*)).to_string()));
                }
                let item = schema.get("items").ok_or_else(|| fail("array has no item schema"))?;
                let item =
                    self.resolve(item, &format!("{location}/items"), dependencies, visiting)?;
                format!("Vec<{}>", item.nested())
            }
            Some("object") => {
                if schema.get("properties").is_some() {
                    return Err(fail("inline object requires its own model"));
                }
                self.resolve_map(schema, location, dependencies, visiting)?
            }
            None if is_unconstrained(schema) => {
                return Ok(ResolvedType { base: "serde_json::Value".into(), nullable: true });
            }
            _ => return Err(fail("unsupported or missing schema type")),
        };
        Ok(ResolvedType::new(base))
    }

    pub(super) fn resolve_map(
        &self,
        schema: &Value,
        location: &str,
        dependencies: &mut BTreeSet<String>,
        visiting: &mut BTreeSet<String>,
    ) -> Result<String, String> {
        let value = match schema.get("additionalProperties") {
            Some(Value::Bool(false)) => {
                return Err(format!("{location}: closed inline object requires its own model"));
            }
            Some(value) if value.is_object() => self
                .resolve(
                    value,
                    &format!("{location}/additionalProperties"),
                    dependencies,
                    visiting,
                )?
                .nested(),
            _ => "serde_json::Value".into(),
        };
        let key = if let Some(keys) = schema.get("propertyNames") {
            // JSON object property names are strings even when their
            // constraint schema only declares length or pattern rules.
            let mut keys = keys.clone();
            if let Some(object) = keys.as_object_mut() {
                object.entry("type").or_insert_with(|| Value::String("string".into()));
            }
            self.resolve(&keys, &format!("{location}/propertyNames"), dependencies, visiting)?
                .nested()
        } else {
            "String".into()
        };
        Ok(format!("std::collections::BTreeMap<{key}, {value}>"))
    }
}

fn is_unconstrained(schema: &Value) -> bool {
    schema == &Value::Bool(true)
        || schema.as_object().is_some_and(|object| {
            // Annotations describe a value without restricting its shape.
            // Unknown validation keywords must still be deferred.
            object.keys().all(|key| {
                key.starts_with("x-")
                    || matches!(
                        key.as_str(),
                        "title"
                            | "description"
                            | "default"
                            | "examples"
                            | "example"
                            | "deprecated"
                            | "readOnly"
                            | "writeOnly"
                            | "externalDocs"
                            | "xml"
                            | "$comment"
                    )
            })
        })
}

pub(super) fn list_item(schema: &Value) -> Option<&Value> {
    let properties = schema.get("properties")?.as_object()?;
    if properties.len() != 2
        || properties.get("pagination")?.get("$ref")?.as_str()? != "#/components/schemas/Pagination"
    {
        return None;
    }
    let items = properties.get("items")?;
    if items.get("type")?.as_str()? != "array" {
        return None;
    }
    items.get("items")
}
