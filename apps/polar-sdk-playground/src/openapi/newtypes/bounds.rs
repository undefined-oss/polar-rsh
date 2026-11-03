use serde_json::Value;

use crate::templates::newtypes::{NumberBound, Scalar};

pub(crate) fn integer_type(schema: &Value) -> &'static str {
    let nonnegative = schema
        .get("minimum")
        .or_else(|| schema.get("exclusiveMinimum"))
        .and_then(Value::as_f64)
        .is_some_and(|minimum| minimum >= 0.0);
    match schema.get("format").and_then(Value::as_str) {
        Some("int32") => "i32",
        Some("uint32") => "u32",
        Some("uint64") => "u64",
        _ if nonnegative => "u64",
        _ => "i64",
    }
}

pub(super) fn scalar(schema: &Value, location: &str) -> Result<Option<Scalar>, String> {
    // Literal and enum schemas already have a finite, typed set of values.
    if schema.get("const").is_some() || schema.get("enum").is_some() {
        return Ok(None);
    }
    let kind = schema
        .get("type")
        .and_then(Value::as_str)
        .or_else(|| location.ends_with("/propertyNames").then_some("string"));
    match kind {
        Some("string") => {
            let length = |keyword| {
                schema
                    .get(keyword)
                    .map(|value| {
                        value.as_u64().ok_or_else(|| {
                            format!("{location}/{keyword}: expected a nonnegative integer")
                        })
                    })
                    .transpose()
            };
            let minimum = length("minLength")?;
            let maximum = length("maxLength")?;
            if minimum.is_none() && maximum.is_none() {
                return Ok(None);
            }
            if minimum.zip(maximum).is_some_and(|(min, max)| min > max) {
                return Err(format!("{location}: minimum length exceeds maximum length"));
            }
            Ok(Some(match schema.get("format").and_then(Value::as_str) {
                Some("email") => return Ok(None),
                Some("uri") => Scalar::Url { minimum, maximum },
                Some("uuid" | "uuid4" | "uuid_v4" | "UUID" | "date" | "date-time") => {
                    return Err(format!(
                        "{location}: length bounds on this formatted string need a dedicated \
                         validator"
                    ));
                }
                _ => Scalar::String { minimum, maximum },
            }))
        }
        Some("integer") => {
            let mut minimum = None;
            let mut maximum = None;
            for (keyword, lower, exclusive) in [
                ("minimum", true, false),
                ("exclusiveMinimum", true, true),
                ("maximum", false, false),
                ("exclusiveMaximum", false, true),
            ] {
                let Some(value) = schema.get(keyword) else { continue };
                let (floor, ceil) = integer_edges(value)
                    .map_err(|cause| format!("{location}/{keyword}: {cause}"))?;
                let bound = match (lower, exclusive) {
                    (true, false) => ceil,
                    (true, true) => floor
                        .checked_add(1)
                        .ok_or_else(|| format!("{location}: integer bound overflow"))?,
                    (false, false) => floor,
                    (false, true) => ceil
                        .checked_sub(1)
                        .ok_or_else(|| format!("{location}: integer bound overflow"))?,
                };
                if lower {
                    minimum = Some(minimum.map_or(bound, |other: i128| other.max(bound)));
                } else {
                    maximum = Some(maximum.map_or(bound, |other: i128| other.min(bound)));
                }
            }
            if minimum.is_none() && maximum.is_none() {
                return Ok(None);
            }
            if minimum.zip(maximum).is_some_and(|(min, max)| min > max) {
                return Err(format!("{location}: empty integer range"));
            }
            Ok(Some(Scalar::Integer { rust_type: integer_type(schema).into(), minimum, maximum }))
        }
        Some("number") => {
            let minimum = number_bound(schema, "minimum", "exclusiveMinimum", true)?;
            let maximum = number_bound(schema, "maximum", "exclusiveMaximum", false)?;
            if minimum.is_none() && maximum.is_none() {
                return Ok(None);
            }
            if minimum.as_ref().zip(maximum.as_ref()).is_some_and(|(min, max)| {
                min.value > max.value
                    || (min.value == max.value && (min.exclusive || max.exclusive))
            }) {
                return Err(format!("{location}: empty number range"));
            }
            Ok(Some(Scalar::Number { minimum, maximum }))
        }
        _ => Ok(None),
    }
}

fn number_bound(
    schema: &Value,
    inclusive: &str,
    exclusive: &str,
    lower: bool,
) -> Result<Option<NumberBound>, String> {
    let mut result: Option<NumberBound> = None;
    for (keyword, exclusive) in [(inclusive, false), (exclusive, true)] {
        let Some(value) = schema.get(keyword) else { continue };
        let value = value
            .as_f64()
            .filter(|value| value.is_finite())
            .ok_or_else(|| format!("{keyword}: expected a finite number"))?;
        if result.as_ref().is_none_or(|other| {
            (lower && value > other.value)
                || (!lower && value < other.value)
                || (value == other.value && exclusive)
        }) {
            result = Some(NumberBound { value, exclusive });
        }
    }
    Ok(result)
}

/// Round decimal schema bounds without converting integer values to floats.
/// Large bounds (including scientific notation) must retain adjacent integers.
fn integer_edges(value: &Value) -> Result<(i128, i128), String> {
    let number = value.as_number().ok_or("expected a number")?.to_string();
    let (mantissa, exponent) = number.split_once(['e', 'E']).unwrap_or((&number, "0"));
    let exponent: i32 = exponent.parse().map_err(|_| "invalid decimal exponent")?;
    let fractional = mantissa.split_once('.').map_or(0, |(_, digits)| digits.len() as i32);
    let coefficient: i128 =
        mantissa.replace('.', "").parse().map_err(|_| "integer bound is too large")?;
    let scale = exponent.checked_sub(fractional).ok_or("decimal scale overflow")?;
    let power =
        10_i128.checked_pow(scale.unsigned_abs()).ok_or("integer bound scale is too large")?;
    if scale >= 0 {
        let integer = coefficient.checked_mul(power).ok_or("integer bound overflow")?;
        Ok((integer, integer))
    } else {
        let floor = coefficient.div_euclid(power);
        let ceil = floor + i128::from(coefficient.rem_euclid(power) != 0);
        Ok((floor, ceil))
    }
}
