//! String enum emission from resolved Rust names and wire values.

use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

use heck::ToUpperCamelCase;
use serde::Serialize;

const TEMPLATE: &str = include_str!("../../templates/string_enums.rs.tera");
const MACRO: &str = include_str!("../../templates/macros/string_enum.rs.tera");

/// Names and derive paths are trusted Rust syntax supplied by the resolver.
/// `Default` is managed separately through `default_value`.
pub struct StringEnum {
    pub name: String,
    pub description: Option<String>,
    pub derives: Vec<String>,
    pub variants: Vec<StringVariant>,
    /// Original OpenAPI wire value, not a Rust variant name.
    pub default_value: Option<String>,
}

pub struct StringVariant {
    pub description: Option<String>,
    pub wire_value: String,
}

#[derive(Serialize)]
struct EnumContext {
    name: String,
    documentation: Vec<String>,
    derives: Vec<String>,
    variants: Vec<VariantContext>,
}

#[derive(Serialize)]
struct VariantContext {
    name: String,
    documentation: Vec<String>,
    wire_literal: String,
    is_default: bool,
}

#[derive(Serialize)]
struct Context {
    module_path: String,
    enums: Vec<EnumContext>,
}

/// Render a complete shared enum file in staging. Placement and semantic
/// deduplication belong to the caller; declaration order is stable by name.
pub fn generate_string_enums(
    staging_dir: &Path,
    module_path: &str,
    mut enums: Vec<StringEnum>,
) -> Result<rrgen::GenResult, rrgen::Error> {
    if !module_path.ends_with(".rs")
        || !Path::new(module_path).components().all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(rrgen::Error::Message("enums require a relative Rust file path".into()));
    }
    enums.sort_by(|a, b| a.name.cmp(&b.name));
    let mut names = BTreeSet::new();
    let mut contexts = Vec::new();
    for definition in enums {
        let fail =
            |reason: &str| rrgen::Error::Message(format!("enum {}: {reason}", definition.name));
        if !names.insert(definition.name.clone()) {
            return Err(fail("duplicate enum name"));
        }
        if definition.variants.is_empty() {
            return Err(fail("at least one variant is required"));
        }
        if definition.derives.iter().any(|derive| derive == "Default") {
            return Err(fail("specify Default through default_value"));
        }
        if let Some(default) = &definition.default_value {
            if !definition.variants.iter().any(|variant| &variant.wire_value == default) {
                return Err(fail("default_value must match an existing wire value"));
            }
        }
        let mut variant_names = BTreeSet::new();
        let mut wire_values = BTreeSet::new();
        for variant in &definition.variants {
            let name = variant_name(&variant.wire_value);
            if name.is_empty() {
                return Err(fail("wire value does not produce a Rust variant name"));
            }
            if !variant_names.insert(name) || !wire_values.insert(&variant.wire_value) {
                return Err(fail("generated variant names and wire values must be unique"));
            }
        }
        let mut derives = definition.derives;
        if definition.default_value.is_some() {
            derives.push("Default".into());
        }
        contexts.push(EnumContext {
            documentation: super::documentation_lines(definition.description.as_deref()),
            name: definition.name,
            derives,
            variants: definition
                .variants
                .into_iter()
                .map(|variant| VariantContext {
                    documentation: super::documentation_lines(variant.description.as_deref()),
                    is_default: definition.default_value.as_ref() == Some(&variant.wire_value),
                    name: variant_name(&variant.wire_value),
                    // Rust's Debug string representation produces a quoted Rust
                    // literal, including Rust-compatible control/Unicode escapes.
                    wire_literal: format!("{:?}", variant.wire_value),
                })
                .collect(),
        });
    }

    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    tera.add_raw_templates([
        ("macros/string_enum.rs.tera", MACRO),
        ("string_enums.rs.tera", TEMPLATE),
    ])?;
    let result = rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        TEMPLATE,
        &serde_json::to_value(Context {
            module_path: serde_json::to_string(module_path)?,
            enums: contexts,
        })?,
    )?;
    super::format_rust_file(&staging_dir.join(module_path))?;
    Ok(result)
}

// Convert wire spelling once, before rendering. Keep the original wire value
// for serde, string conversions, and default matching.
fn variant_name(wire_value: &str) -> String {
    // Preserve signed numeric suffixes, such as Etc/GMT+1 and Etc/GMT-1.
    let mut spelling = String::new();
    let mut chars = wire_value.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '+' if chars.peek().is_some_and(char::is_ascii_digit) => spelling.push_str(" Plus "),
            '-' if chars.peek().is_some_and(char::is_ascii_digit) => spelling.push_str(" Minus "),
            _ => spelling.push(character),
        }
    }
    let name = spelling.to_upper_camel_case();
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("Value{name}")
    } else if name == "Self" {
        "SelfValue".into()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::test_support::{TestResult, assert_generated};

    #[test]
    fn string_enums() -> TestResult {
        assert_generated("string_enums", "models/common/enums.rs", |output| {
            generate_string_enums(
                output,
                "models/common/enums.rs",
                vec![
                    StringEnum {
                        name: "SubscriptionStatus".into(),
                        description: Some("The subscription's status.".into()),
                        derives: ["Debug", "Clone", "Copy", "Deserialize", "Serialize"]
                            .map(String::from)
                            .into(),
                        variants: vec![
                            StringVariant { wire_value: "active".into(), description: None },
                            StringVariant {
                                wire_value: "past_due".into(),
                                description: Some(
                                    "Payment is overdue.\r\nA retry may follow.".into(),
                                ),
                            },
                        ],
                        default_value: Some("active".into()),
                    },
                    StringEnum {
                        name: "CheckoutStatus".into(),
                        description: None,
                        derives: ["Debug", "Clone", "Deserialize", "Serialize"]
                            .map(String::from)
                            .into(),
                        variants: vec![StringVariant {
                            wire_value: "confirmed".into(),
                            description: None,
                        }],
                        default_value: None,
                    },
                ],
            )
        })
    }

    #[test]
    fn string_enum_variant_names() -> TestResult {
        assert_generated("string_enum_variant_names", "models/common/enums.rs", |output| {
            generate_string_enums(
                output,
                "models/common/enums.rs",
                vec![StringEnum {
                    name: "WireValue".into(),
                    description: None,
                    derives: ["Debug", "Clone", "Deserialize", "Serialize"]
                        .map(String::from)
                        .into(),
                    variants: [
                        "Etc/GMT+1",
                        "Etc/GMT-1",
                        "self",
                        "123",
                        "with\"quote",
                        "line\nbreak",
                    ]
                    .into_iter()
                    .map(|value| StringVariant { wire_value: value.into(), description: None })
                    .collect(),
                    default_value: None,
                }],
            )
        })
    }
}
