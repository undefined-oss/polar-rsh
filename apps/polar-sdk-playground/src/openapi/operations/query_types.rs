use std::collections::BTreeSet;

use serde_json::Value;

use super::{ModelCatalog, UuidCatalog, dereference};
use crate::{
    openapi::enums::EnumCatalog,
    templates::operation_query::{MetadataVariant, QueryEncoding},
};

pub(super) enum QueryType {
    Scalar(String),
    Repeated(String),
    Metadata { value_type: String, variants: Vec<MetadataVariant> },
}

impl QueryType {
    pub fn field(self) -> (String, QueryEncoding) {
        match self {
            Self::Scalar(ty) => (ty, QueryEncoding::Scalar),
            Self::Repeated(ty) => (format!("Vec<{ty}>"), QueryEncoding::Repeated),
            Self::Metadata { value_type, variants } => (
                format!("std::collections::BTreeMap<String, {value_type}>"),
                QueryEncoding::Metadata { value_type, variants },
            ),
        }
    }
}

pub(super) struct QueryResolver<'a> {
    pub document: &'a Value,
    pub models: &'a ModelCatalog,
    pub enums: &'a EnumCatalog,
    pub ids: &'a UuidCatalog,
}

impl QueryResolver<'_> {
    pub fn resolve(&self, schema: &Value, location: &str) -> Result<QueryType, String> {
        self.resolve_inner(schema, location, &mut BTreeSet::new())
    }

    fn resolve_inner(
        &self,
        schema: &Value,
        location: &str,
        visiting: &mut BTreeSet<String>,
    ) -> Result<QueryType, String> {
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            if let Some(name) = reference.strip_prefix("#/components/schemas/") {
                let name = name.replace("~1", "/").replace("~0", "~");
                if let Some(ty) = self.enums.type_paths.get(&name) {
                    return Ok(QueryType::Scalar(ty.clone()));
                }
            }
            let (target, path) = dereference(schema, location.into(), self.document)?;
            if !visiting.insert(path.clone()) {
                return Err(format!("{path}: recursive query schema"));
            }
            let result = self.resolve_inner(target, &path, visiting);
            visiting.remove(&path);
            return result;
        }
        if let Some(ty) = self.enums.inline_type_paths.get(location) {
            return Ok(QueryType::Scalar(ty.clone()));
        }
        if schema.get("enum").is_some()
            || schema.get("oneOf").is_some()
            || schema.get("allOf").is_some()
        {
            return Err(format!("{location}: unresolved query enum or composition"));
        }
        if let Some(branches) = schema.get("anyOf").and_then(Value::as_array) {
            let mut types = Vec::new();
            for (i, branch) in branches.iter().enumerate() {
                if branch.get("type").and_then(Value::as_str) != Some("null") {
                    types.push(self.resolve_inner(
                        branch,
                        &format!("{location}/anyOf/{i}"),
                        visiting,
                    )?);
                }
            }
            if types.len() == 1 {
                return Ok(types.remove(0));
            }
            let mut scalar = None;
            let mut repeated = false;
            for ty in types {
                let ty = match ty {
                    QueryType::Scalar(ty) => ty,
                    QueryType::Repeated(ty) => {
                        repeated = true;
                        ty
                    }
                    QueryType::Metadata { .. } => {
                        return Err(format!("{location}: mixed query object union"));
                    }
                };
                if scalar.as_ref().is_some_and(|previous| previous != &ty) {
                    return Err(format!("{location}: query union has different element types"));
                }
                scalar = Some(ty);
            }
            let ty =
                scalar.ok_or_else(|| format!("{location}: query schema has no non-null type"))?;
            return Ok(if repeated { QueryType::Repeated(ty) } else { QueryType::Scalar(ty) });
        }
        match schema.get("type").and_then(Value::as_str) {
            Some("array") => {
                let items = schema.get("items").ok_or("query array requires items")?;
                match self.resolve_inner(items, &format!("{location}/items"), visiting)? {
                    QueryType::Scalar(ty) => Ok(QueryType::Repeated(ty)),
                    _ => Err(format!("{location}: nested query collections are unsupported")),
                }
            }
            Some("object") => self.metadata(location),
            Some("string") => {
                let ty = match schema.get("format").and_then(Value::as_str) {
                    Some(format) if format.starts_with("uuid") => self
                        .ids
                        .type_paths
                        .get(location)
                        .ok_or_else(|| format!("{location}: UUID query type was not resolved"))?
                        .clone(),
                    Some("date-time") => "jiff::Timestamp".into(),
                    Some("date") => "jiff::civil::Date".into(),
                    _ => "String".into(),
                };
                Ok(QueryType::Scalar(ty))
            }
            Some("boolean") => Ok(QueryType::Scalar("bool".into())),
            Some("integer") => Ok(QueryType::Scalar("i64".into())),
            Some("number") => Ok(QueryType::Scalar("f64".into())),
            _ => Err(format!("{location}: unsupported query schema")),
        }
    }

    fn metadata(&self, location: &str) -> Result<QueryType, String> {
        let ty = self
            .models
            .union_type_paths
            .get(&format!("{location}/additionalProperties"))
            .ok_or_else(|| format!("{location}: metadata value union is not generated"))?;
        let (module, name) = ty.strip_prefix("crate::").unwrap().rsplit_once("::").unwrap();
        let union = self
            .models
            .unions
            .get(&(module.replace("::", "/") + ".rs"))
            .and_then(|unions| unions.iter().find(|union| union.name == name))
            .ok_or("metadata union definition is missing")?;
        if union.exclusive || union.discriminator.is_some() {
            return Err("metadata requires an untagged anyOf value union".into());
        }
        let variants = union
            .variants
            .iter()
            .map(|variant| {
                let array =
                    variant.rust_type.strip_prefix("Vec<").and_then(|ty| ty.strip_suffix('>'));
                let scalar = array.unwrap_or(&variant.rust_type);
                if !matches!(scalar, "String" | "i64" | "f64" | "bool") {
                    return Err("unsupported metadata scalar type".to_owned());
                }
                Ok(MetadataVariant { name: variant.name.clone(), repeated: array.is_some() })
            })
            .collect::<Result<_, _>>()?;
        Ok(QueryType::Metadata { value_type: ty.clone(), variants })
    }
}
