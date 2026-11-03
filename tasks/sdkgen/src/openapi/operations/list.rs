use std::collections::BTreeMap;

use serde_json::Value;

use super::{component_type, dereference, query_types::QueryResolver};
use crate::templates::{
    list_operation::ListConfig,
    operation_query::{QueryEncoding, QueryField},
};

pub(super) fn resolve(
    operation: &Value,
    name: &str,
    parameters: &mut BTreeMap<(String, String), (&Value, String)>,
    resolver: &QueryResolver<'_>,
) -> Result<ListConfig, String> {
    let pagination =
        operation.get("x-polar-pagination").ok_or("pagination requires x-polar-pagination")?;
    if pagination.get("type").and_then(Value::as_str) != Some("page_limit") {
        return Err("unsupported Polar pagination type".into());
    }
    if let Some(other) = operation.get("x-speakeasy-pagination") {
        if other.get("type").and_then(Value::as_str) != Some("offsetLimit")
            || other.pointer("/outputs/results").and_then(Value::as_str) != Some("$.items")
            || other.pointer("/outputs/numPages").and_then(Value::as_str)
                != Some("$.pagination.max_page")
        {
            return Err("pagination extensions disagree on the response layout".into());
        }
        let inputs =
            other.get("inputs").and_then(Value::as_array).ok_or("pagination inputs are missing")?;
        for field in ["page", "limit"] {
            if !inputs.iter().any(|input| {
                input.get("name").and_then(Value::as_str) == Some(field)
                    && input.get("in").and_then(Value::as_str) == Some("parameters")
                    && input.get("type").and_then(Value::as_str) == Some(field)
            }) {
                return Err(format!("pagination extension does not map {field}"));
            }
        }
    }
    let item_schema = pagination.get("item_schema").ok_or("pagination item schema is missing")?;
    let (_, item_type) = component_type(item_schema, resolver.models, "pagination item")?;
    let mut pagination_fields = Vec::new();
    for field in ["page", "limit"] {
        let (parameter, location) = parameters
            .remove(&("query".into(), field.into()))
            .ok_or_else(|| format!("pagination requires {field}"))?;
        if parameter.get("required") == Some(&Value::Bool(true)) {
            return Err("pagination parameters must be optional".into());
        }
        let schema = parameter.get("schema").ok_or("pagination parameter has no schema")?;
        let (schema, _) = dereference(schema, format!("{location}/schema"), resolver.document)?;
        let rust_type = pagination_type(field, schema)?;
        pagination_fields.push(QueryField {
            rust_name: field.into(),
            wire_name: field.into(),
            rust_type: rust_type.into(),
            description: super::query::description(parameter, schema),
            encoding: QueryEncoding::Scalar,
            deprecated: false,
            required: false,
        });
    }
    let mut query = super::query::resolve(name, parameters, resolver)?;
    if query.required() {
        return Err("pagination filters must be optional".into());
    }
    query.fields.extend(pagination_fields);
    let config = ListConfig { query, item_type };
    Ok(config)
}

pub(super) fn response_type(
    response: &Value,
    operation: &Value,
    config: &ListConfig,
    document: &Value,
) -> Result<String, String> {
    let schema = response
        .pointer("/content/application~1json/schema")
        .ok_or("list response requires JSON")?;
    let (schema, _) = dereference(schema, "list response".into(), document)?;
    if schema.pointer("/properties/items/items/$ref")
        != operation.pointer("/x-polar-pagination/item_schema/$ref")
        || schema.pointer("/properties/pagination/$ref").and_then(Value::as_str)
            != Some("#/components/schemas/Pagination")
    {
        return Err("list response does not match x-polar-pagination".into());
    }
    Ok(format!("crate::models::pagination::ListResource<{}>", config.item_type))
}

pub(super) fn pagination_type(field: &str, schema: &Value) -> Result<&'static str, String> {
    if schema.get("type").and_then(Value::as_str) != Some("integer") {
        return Err("pagination parameters must be integers".into());
    }
    if field == "page"
        && (schema.get("default").and_then(Value::as_u64) != Some(1)
            || !(schema.get("exclusiveMinimum").and_then(Value::as_f64) == Some(0.0)
                || schema.get("minimum").and_then(Value::as_f64) == Some(1.0)))
    {
        return Err("page must default to 1 and start at 1".into());
    }
    Ok(if field == "page" {
        "std::num::NonZeroU64"
    } else {
        "crate::models::pagination::PageLimit"
    })
}
