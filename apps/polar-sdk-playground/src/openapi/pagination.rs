//! Resolve one shared page limit for all page/limit operations.

use oas3::spec::{ObjectOrReference, ObjectSchema, ParameterIn, Schema, SchemaType, SchemaTypeSet};

use super::ConversionError;
use crate::templates::pagination::PageLimitConfig;

// Polar documents this cap in parameter descriptions but omits the schema
// maximum.
const POLAR_MAXIMUM_PAGE_LIMIT: u8 = 100;

pub fn page_limit_config(spec: &oas3::Spec) -> Result<Option<PageLimitConfig>, ConversionError> {
    let mut shared: Option<(PageLimitConfig, String)> = None;

    for (path, path_item) in spec.paths.iter().flatten() {
        for (method, operation) in path_item.methods() {
            // oas3 strips the `x-` prefix from extension keys.
            let pagination = operation.extensions.get("polar-pagination");
            let location = format!("{method:?} {path}");
            let error = |message: String| ConversionError::InvalidPagination {
                location: location.clone(),
                message,
            };
            if pagination.is_some_and(|pagination| {
                pagination.get("type").and_then(serde_json::Value::as_str) != Some("page_limit")
            }) {
                return Err(error(
                    "unsupported x-polar-pagination; expected type page_limit".into(),
                ));
            }

            // Operation parameters override path parameters with the same name
            // and location.
            let mut limit = None;
            let mut has_page = false;
            for parameter in operation.parameters.iter().chain(&path_item.parameters) {
                let parameter =
                    parameter.resolve(spec).map_err(|cause| error(cause.to_string()))?;
                if parameter.location == ParameterIn::Query {
                    has_page |= parameter.name == "page";
                    if parameter.name == "limit" && limit.is_none() {
                        limit = Some(parameter);
                    }
                }
            }
            // Mixed page/cursor responses can expose page and limit without a
            // pagination extension. They still share the same PageLimit.
            if pagination.is_none() && !(has_page && limit.is_some()) {
                continue;
            }
            let parameter = limit.ok_or_else(|| error("missing limit query parameter".into()))?;
            let schema = parameter
                .schema
                .ok_or_else(|| error("limit query parameter has no schema".into()))?
                .resolve(spec)
                .map_err(|cause| error(cause.to_string()))?;
            let Schema::Object(schema) = schema else {
                return Err(error("limit requires an integer schema".into()));
            };
            let ObjectOrReference::Object(schema) = *schema else {
                return Err(error("unresolved limit schema reference".into()));
            };
            let config = resolve_limit(&schema).map_err(error)?;

            if let Some((expected, first_location)) = &shared {
                if *expected != config {
                    return Err(ConversionError::InvalidPagination {
                        location,
                        message: format!(
                            "limit constraints {config:?} conflict with {first_location} \
                             {expected:?}; all paginated operations must share one PageLimit"
                        ),
                    });
                }
            } else {
                shared = Some((config, location));
            }
        }
    }

    Ok(shared.map(|(config, _)| config))
}

fn resolve_limit(schema: &ObjectSchema) -> Result<PageLimitConfig, String> {
    let is_integer = match &schema.schema_type {
        Some(SchemaTypeSet::Single(SchemaType::Integer)) => true,
        Some(SchemaTypeSet::Multiple(types)) => types == &[SchemaType::Integer],
        _ => false,
    };
    if !is_integer {
        return Err("limit requires a non-nullable integer schema".into());
    }

    let inclusive_minimum = schema.minimum.as_ref().and_then(|n| n.as_f64()).map(f64::ceil);
    let exclusive_minimum =
        schema.exclusive_minimum.as_ref().and_then(|n| n.as_f64()).map(|n| n.floor() + 1.0);
    let minimum = tighter_bound(inclusive_minimum, exclusive_minimum, f64::max)
        .ok_or("limit schema must declare a minimum or exclusiveMinimum")?;

    let inclusive_maximum = schema.maximum.as_ref().and_then(|n| n.as_f64()).map(f64::floor);
    let exclusive_maximum =
        schema.exclusive_maximum.as_ref().and_then(|n| n.as_f64()).map(|n| n.ceil() - 1.0);
    let maximum = tighter_bound(inclusive_maximum, exclusive_maximum, f64::min)
        .unwrap_or(f64::from(POLAR_MAXIMUM_PAGE_LIMIT));

    let default = schema
        .default
        .as_ref()
        .and_then(serde_json::Value::as_f64)
        .ok_or("limit schema must declare a numeric default")?;
    if minimum < 1.0
        || maximum > 255.0
        || minimum > maximum
        || default.fract() != 0.0
        || default < minimum
        || default > maximum
    {
        return Err(format!(
            "limit requires integer values with 1 <= minimum <= default <= maximum <= 255; got \
             minimum={minimum}, default={default}, maximum={maximum}"
        ));
    }

    Ok(PageLimitConfig { minimum: minimum as u8, maximum: maximum as u8, default: default as u8 })
}

fn tighter_bound(a: Option<f64>, b: Option<f64>, combine: fn(f64, f64) -> f64) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(combine(a, b)),
        (a, b) => a.or(b),
    }
}
