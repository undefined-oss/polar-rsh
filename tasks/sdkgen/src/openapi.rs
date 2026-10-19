//! The OpenAPI structures consumed by the generator. Unknown schema keywords
//! are retained so query overlays do not discard upstream constraints.

use std::fmt;

use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{IgnoredAny, MapAccess, Visitor},
};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Document {
    pub openapi: String,
    pub paths: IndexMap<String, PathItem>,
    #[serde(default)]
    pub webhooks: IndexMap<String, PathItem>,
    pub components: Components,
}

impl Document {
    pub fn operation(&self, path: &str, method: &str) -> Result<&Operation> {
        self.paths
            .get(path)
            .and_then(|item| item.operations.get(&method.to_ascii_lowercase()))
            .with_context(|| format!("Missing OpenAPI operation {method} {path}"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Components {
    pub schemas: IndexMap<String, Schema>,
}

#[derive(Debug, Default)]
pub struct PathItem {
    // Preserve source order, which determines overlay and webhook output order.
    pub operations: IndexMap<String, Operation>,
}

impl<'de> Deserialize<'de> for PathItem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PathVisitor;
        impl<'de> Visitor<'de> for PathVisitor {
            type Value = PathItem;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an OpenAPI path item")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<PathItem, M::Error> {
                let mut item = PathItem::default();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "get" | "put" | "post" | "delete" | "options" | "head" | "patch"
                        | "trace" => {
                            if item.operations.insert(key, map.next_value()?).is_some() {
                                return Err(serde::de::Error::custom("duplicate HTTP operation"));
                            }
                        }
                        // References and inherited parameters need resolution before
                        // generation; never silently generate incomplete operations.
                        "$ref" | "parameters" => {
                            return Err(serde::de::Error::custom(
                                "path references and inherited parameters are not supported",
                            ));
                        }
                        _ => {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                Ok(item)
            }
        }
        deserializer.deserialize_map(PathVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub operation_id: String,
    pub summary: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    pub request_body: Option<RequestBody>,
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub content: IndexMap<String, MediaType>,
}

#[derive(Debug, Deserialize)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    Form,
    Simple,
    Matrix,
    Label,
    SpaceDelimited,
    PipeDelimited,
    DeepObject,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "in")]
    pub location: Location,
    #[serde(default)]
    pub required: bool,
    pub schema: Schema,
    pub style: Option<Style>,
    pub explode: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Schema {
    Boolean(bool),
    Object(Box<SchemaObject>),
}

impl Schema {
    pub fn object(&self) -> Option<&SchemaObject> {
        match self {
            Self::Object(object) => Some(object),
            Self::Boolean(_) => None,
        }
    }

    pub fn is_type(&self, kind: PrimitiveType) -> bool {
        self.object().is_some_and(|s| s.kind == Some(SchemaType::Single(kind)))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    Integer,
    String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum SchemaType {
    Single(PrimitiveType),
    Union(Vec<PrimitiveType>),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct SchemaObject {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<SchemaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Schema>,
    #[serde(rename = "anyOf", skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<Schema>>,
    // JSON Schema defaults are arbitrary JSON, and an explicit null differs
    // from the absence of a default during normalization.
    #[serde(default, deserialize_with = "present_value", skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(flatten)]
    pub extensions: IndexMap<String, Value>,
}

fn present_value<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Value>, D::Error> {
    Value::deserialize(deserializer).map(Some)
}

impl SchemaObject {
    pub fn merge(&mut self, branch: &Self) {
        if branch.reference.is_some() {
            self.reference.clone_from(&branch.reference);
        }
        if branch.kind.is_some() {
            self.kind.clone_from(&branch.kind);
        }
        if branch.format.is_some() {
            self.format.clone_from(&branch.format);
        }
        if branch.items.is_some() {
            self.items.clone_from(&branch.items);
        }
        if branch.any_of.is_some() {
            self.any_of.clone_from(&branch.any_of);
        }
        if branch.default.is_some() {
            self.default.clone_from(&branch.default);
        }
        self.extensions.extend(branch.extensions.clone());
    }
}

#[cfg(test)]
pub fn fixture(paths: Value) -> Document {
    serde_json::from_value(serde_json::json!({
        "openapi": "3.1.0", "paths": paths, "components": {"schemas": {}}
    }))
    .unwrap()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn all_specs_produce_equivalent_overlays() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../apps/polar-rsh/src");
        for entry in std::fs::read_dir(root).unwrap() {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                continue;
            }
            let Some(version) = path.file_name().unwrap().to_str().unwrap().strip_prefix('v')
            else {
                continue;
            };
            let version = version.replace('_', "-");
            let spec = std::fs::read(path.join(format!("{version}.openapi.json"))).unwrap();
            let mut document: Document = serde_json::from_slice(&spec).unwrap();
            let overlay = crate::schema::normalize_queries(&mut document).unwrap();
            let expected: Value = serde_json::from_slice(
                &std::fs::read(path.join("query-parameters.overlay.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(serde_json::to_value(overlay).unwrap(), expected, "overlay for {version}");
        }
    }

    #[test]
    fn malformed_consumed_fields_fail_at_deserialization() {
        for operation in [
            json!({"parameters": []}),
            json!({"operationId": 42}),
            json!({"operationId": "items:list", "parameters": null}),
            json!({"operationId": "items:list", "parameters": [{"name":"id", "schema":{}}]}),
            json!({"operationId": "items:list", "parameters": [{"name":"id", "in":"query", "required":"false", "schema":{}}]}),
            json!({"operationId": "items:list", "parameters": [{"name":"id", "in":"query", "schema":{"anyOf":"invalid"}}]}),
        ] {
            assert!(serde_json::from_value::<Operation>(operation).is_err());
        }
        for item in [json!({"$ref":"#/components/pathItems/items"}), json!({"parameters":[]})] {
            assert!(serde_json::from_value::<PathItem>(item).is_err());
        }
    }

    #[test]
    fn schema_round_trip_retains_defaults_extensions_and_boolean_schemas() {
        for value in [
            json!(false),
            json!({"type":["string","null"],"default":null,"x-extra":{"a":1},"minLength":2}),
            json!({"type":"array","items":true}),
        ] {
            let schema: Schema = serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(schema).unwrap(), value);
        }
    }
}
