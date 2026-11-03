//! Build the incoming payload union from the document's webhook declarations.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use super::{super::references::dereference, ConversionError, escape, unions::UnionSchema};

pub(super) fn collect(
    document: &Value,
    unions: &mut BTreeMap<String, UnionSchema>,
) -> Result<(), ConversionError> {
    let Some(webhooks) = document.get("webhooks").and_then(Value::as_object) else {
        return Ok(());
    };
    let mut branches = Vec::new();
    let mut mapping = BTreeMap::new();
    let mut payloads = BTreeSet::new();
    for (name, webhook) in webhooks {
        let location = format!("#/webhooks/{}", escape(name));
        let fail = |message: &str| ConversionError::InvalidModel(format!("{location}: {message}"));
        let (webhook, location) = dereference(webhook, location.clone(), document)
            .map_err(ConversionError::InvalidModel)?;
        let operation =
            webhook.get("post").ok_or_else(|| fail("webhook requires a POST declaration"))?;
        let body =
            operation.get("requestBody").ok_or_else(|| fail("webhook requires a request body"))?;
        let (body, _) = dereference(body, format!("{location}/post/requestBody"), document)
            .map_err(ConversionError::InvalidModel)?;
        let schema = body
            .pointer("/content/application~1json/schema")
            .ok_or_else(|| fail("webhook requires an application/json schema"))?;
        let reference = schema
            .get("$ref")
            .and_then(Value::as_str)
            .filter(|reference| reference.starts_with("#/components/schemas/"))
            .ok_or_else(|| fail("webhook payload must reference a component schema"))?;
        let (payload, _) = dereference(schema, reference.into(), document)
            .map_err(ConversionError::InvalidModel)?;
        let tag = payload
            .pointer("/properties/type/const")
            .and_then(Value::as_str)
            .ok_or_else(|| fail("webhook payload requires a constant type discriminator"))?;
        let required = payload
            .get("required")
            .and_then(Value::as_array)
            .ok_or_else(|| fail("webhook payload requires type and data"))?;
        if !["type", "data"]
            .into_iter()
            .all(|field| required.iter().any(|value| value.as_str() == Some(field)))
            || payload.pointer("/properties/data").is_none()
        {
            return Err(fail("webhook payload requires type and data"));
        }
        if mapping.insert(tag.to_owned(), reference.to_owned()).is_some() {
            return Err(fail("duplicate webhook event discriminator"));
        }
        if !payloads.insert(reference) {
            return Err(fail("webhook payload is declared more than once"));
        }
        let description = operation.get("description").or_else(|| payload.get("description"));
        branches.push(json!({ "$ref": reference, "description": description }));
    }
    if branches.is_empty() {
        return Ok(());
    }
    unions.insert(
        "#/webhooks".into(),
        UnionSchema {
            key: "#/webhooks".into(),
            name: "WebhookPayload".into(),
            module_path: "models/webhooks/events.rs".into(),
            type_path: "crate::models::webhooks::events::WebhookPayload".into(),
            nullable: false,
            keyword: "oneOf",
            boxed_fields: BTreeSet::from(["data".into()]),
            schema: json!({
                "description": "An incoming webhook payload.",
                "oneOf": branches,
                "discriminator": { "propertyName": "type", "mapping": mapping },
            }),
        },
    );
    Ok(())
}
