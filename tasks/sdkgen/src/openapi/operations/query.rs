use std::collections::BTreeMap;

use heck::ToSnakeCase;
use serde_json::Value;

use super::query_types::{QueryResolver, QueryType};
use crate::templates::operation_query::{QueryConfig, QueryEncoding, QueryField};

pub(super) fn resolve(
    name: &str,
    parameters: &mut BTreeMap<(String, String), (&Value, String)>,
    resolver: &QueryResolver<'_>,
) -> Result<QueryConfig, String> {
    let mut config = QueryConfig {
        params_name: format!("{name}Params"),
        fields: Vec::new(),
        sort_type: None,
        sorting_description: None,
    };
    let queries: Vec<_> =
        parameters.keys().filter(|(place, _)| place == "query").cloned().collect();
    let page_limit = ["page", "limit"]
        .iter()
        .all(|name| parameters.contains_key(&("query".into(), (*name).into())));
    for key in queries {
        let (parameter, location) = parameters.remove(&key).unwrap();
        let wire = key.1;
        let required = parameter.get("required") == Some(&Value::Bool(true));
        let schema = parameter.get("schema").ok_or("query parameter has no schema")?;
        let ty = if page_limit && matches!(wire.as_str(), "page" | "limit") {
            let (schema, _) =
                super::dereference(schema, format!("{location}/schema"), resolver.document)?;
            QueryType::Scalar(super::list::pagination_type(&wire, schema)?.into())
        } else {
            resolver.resolve(schema, &format!("{location}/schema"))?
        };
        let metadata = matches!(ty, QueryType::Metadata { .. });
        let style = parameter.get("style").and_then(Value::as_str).unwrap_or("form");
        if style != if metadata { "deepObject" } else { "form" }
            || parameter.get("explode") == Some(&Value::Bool(false))
            || parameter.get("allowReserved") == Some(&Value::Bool(true))
        {
            return Err(format!("{location}: unsupported query serialization"));
        }
        if wire == "sorting" {
            let QueryType::Repeated(ty) = ty else {
                return Err("sorting must be an array of enum values".into());
            };
            if required {
                return Err("required sorting is not supported".into());
            }
            if !resolver
                .enums
                .type_paths
                .values()
                .chain(resolver.enums.inline_type_paths.values())
                .any(|path| path == &ty)
            {
                return Err("sorting requires a generated enum".into());
            }
            config.sort_type = Some(ty);
            config.sorting_description = description(parameter, schema);
            continue;
        }
        let (rust_type, encoding) = ty.field();
        if matches!(encoding, QueryEncoding::Metadata { .. }) && wire != "metadata" {
            return Err("deepObject filtering currently requires the metadata parameter".into());
        }
        let mut rust_name = wire.to_snake_case();
        if matches!(rust_name.as_str(), "self" | "super" | "crate") {
            rust_name.push_str("_value");
        } else if syn::parse_str::<syn::Ident>(&rust_name).is_err() {
            rust_name = format!("r#{rust_name}");
        }
        config.fields.push(QueryField {
            rust_name,
            wire_name: wire,
            rust_type,
            encoding,
            description: description(parameter, schema),
            deprecated: parameter.get("deprecated") == Some(&Value::Bool(true))
                || schema.get("deprecated") == Some(&Value::Bool(true)),
            required,
        });
    }
    Ok(config)
}

pub(super) fn description(parameter: &Value, schema: &Value) -> Option<String> {
    parameter
        .get("description")
        .or_else(|| schema.get("description"))
        .and_then(Value::as_str)
        .map(str::to_owned)
}
