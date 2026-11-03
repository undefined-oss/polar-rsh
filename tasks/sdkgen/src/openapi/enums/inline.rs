//! Name inline enums by their owning schema and field, without merging equal
//! values.

use std::collections::{BTreeMap, BTreeSet};

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::{Map, Value};

use super::{ConversionError, EnumCatalog, StringEnum, StringVariant, domains};

pub(super) fn collect(
    document: &Value,
    usage: &domains::SchemaDomains,
    catalog: &mut EnumCatalog,
) -> Result<(), ConversionError> {
    let schemas =
        document.pointer("/components/schemas").and_then(Value::as_object).ok_or_else(|| {
            ConversionError::InvalidEnum {
                name: "components/schemas".into(),
                message: "missing component schemas".into(),
            }
        })?;
    let mut collector = Collector { schemas, catalog };
    for (name, schema) in schemas {
        let domain = usage
            .active
            .get(name)
            .filter(|domains| domains.len() == 1)
            .and_then(|domains| domains.first())
            .map(String::as_str)
            .unwrap_or("common");
        collector.visit(
            schema,
            &format!("#/components/schemas/{}", escape(name)),
            &name.to_upper_camel_case(),
            domain,
            true,
        )?;
    }
    collect_parameters(document, &mut collector)?;
    Ok(())
}

fn collect_parameters(
    document: &Value,
    collector: &mut Collector<'_>,
) -> Result<(), ConversionError> {
    struct Parameter<'a> {
        schema: &'a Value,
        name: String,
        domains: BTreeSet<String>,
    }
    let mut parameters: BTreeMap<String, Parameter<'_>> = BTreeMap::new();
    for (path, item) in document.get("paths").and_then(Value::as_object).into_iter().flatten() {
        for method in ["get", "post", "patch", "put", "delete", "head", "options", "trace"] {
            let Some(operation) = item.get(method) else { continue };
            let Some(group) = operation.get("x-speakeasy-group").and_then(Value::as_str) else {
                continue;
            };
            let domain = group.split('.').next().unwrap().to_snake_case();
            let operation_name = operation
                .get("operationId")
                .and_then(Value::as_str)
                .unwrap_or(method)
                .to_upper_camel_case();
            let path_location = format!("#/paths/{}", escape(path));
            for (owner, base) in
                [(item, path_location.clone()), (operation, format!("{path_location}/{method}"))]
            {
                for (index, parameter) in owner
                    .get("parameters")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .enumerate()
                {
                    let location = format!("{base}/parameters/{index}");
                    let (parameter, location) = super::super::references::dereference(
                        parameter,
                        location.clone(),
                        document,
                    )
                    .map_err(|message| ConversionError::InvalidEnum { name: location, message })?;
                    let Some(schema) = parameter.get("schema") else { continue };
                    let field =
                        parameter.get("name").and_then(Value::as_str).unwrap_or("Parameter");
                    let name = if location.starts_with("#/components/parameters/") {
                        location.rsplit('/').next().unwrap().to_upper_camel_case()
                    } else {
                        format!("{operation_name}{}", field.to_upper_camel_case())
                    };
                    parameters
                        .entry(format!("{location}/schema"))
                        .or_insert_with(|| Parameter { schema, name, domains: BTreeSet::new() })
                        .domains
                        .insert(domain.clone());
                }
            }
        }
    }
    for (location, parameter) in parameters {
        let domain = if parameter.domains.len() == 1 {
            parameter.domains.first().unwrap().as_str()
        } else {
            "common"
        };
        collector.visit(parameter.schema, &location, &parameter.name, domain, false)?;
    }
    Ok(())
}

struct Collector<'a> {
    schemas: &'a Map<String, Value>,
    catalog: &'a mut EnumCatalog,
}

impl Collector<'_> {
    fn visit(
        &mut self,
        schema: &Value,
        location: &str,
        name: &str,
        domain: &str,
        component: bool,
    ) -> Result<(), ConversionError> {
        if !component && schema.get("enum").is_some() {
            self.register(schema, location, name, domain)?;
        }
        if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
            for (field, child) in properties {
                self.visit(
                    child,
                    &format!("{location}/properties/{}", escape(field)),
                    &format!("{name}{}", field.to_upper_camel_case()),
                    domain,
                    false,
                )?;
            }
        }
        for keyword in ["anyOf", "oneOf", "allOf", "prefixItems"] {
            let Some(branches) = schema.get(keyword).and_then(Value::as_array) else { continue };
            // A nullable wrapper does not add another part to the public name.
            // Its mapping still points to the exact enum leaf.
            let nullable = matches!(keyword, "anyOf" | "oneOf")
                && branches.len() == 2
                && branches
                    .iter()
                    .any(|branch| branch.get("type").and_then(Value::as_str) == Some("null"));
            for (index, branch) in branches.iter().enumerate() {
                let name = if nullable {
                    name.to_owned()
                } else {
                    format!("{name}{}{index}", keyword.to_upper_camel_case())
                };
                self.visit(branch, &format!("{location}/{keyword}/{index}"), &name, domain, false)?;
            }
        }
        for keyword in ["items", "additionalProperties", "propertyNames"] {
            if let Some(child) = schema.get(keyword) {
                self.visit(
                    child,
                    &format!("{location}/{keyword}"),
                    &format!("{name}{}", keyword.to_upper_camel_case()),
                    domain,
                    false,
                )?;
            }
        }
        Ok(())
    }

    fn register(
        &mut self,
        schema: &Value,
        location: &str,
        name: &str,
        domain: &str,
    ) -> Result<(), ConversionError> {
        let fail = |message: &str| ConversionError::InvalidEnum {
            name: location.into(),
            message: message.into(),
        };
        if schema.get("type").and_then(Value::as_str) != Some("string") {
            return Err(fail("only string inline enums are currently supported"));
        }
        // Preserve explicit component identity (for example CountryAlpha2).
        // Equal values without that identity still produce separate types.
        if let Some(title) = schema.get("title").and_then(Value::as_str)
            && let Some(path) = self.catalog.type_paths.get(title)
            && self.schemas.get(title).and_then(|value| value.get("enum")) == schema.get("enum")
        {
            self.catalog.inline_type_paths.insert(location.into(), path.clone());
            return Ok(());
        }
        syn::parse_str::<syn::Ident>(name).map_err(|cause| fail(&cause.to_string()))?;
        let variants: Vec<_> = schema
            .get("enum")
            .and_then(Value::as_array)
            .ok_or_else(|| fail("enum must be an array"))?
            .iter()
            .map(|value| {
                let wire_value =
                    value.as_str().ok_or_else(|| fail("enum values must be strings"))?;
                Ok(StringVariant { description: None, wire_value: wire_value.into() })
            })
            .collect::<Result<Vec<_>, ConversionError>>()?
            .into_iter()
            // Sorting direction is represented by query::Sorting.
            .filter(|variant| !variant.wire_value.starts_with('-'))
            .collect();
        let default_value = schema
            .get("default")
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| fail("enum default must be a string"))
            })
            .transpose()?
            // Upstream older API schemas can advertise a newer API-version
            // default outside their enum. Do not synthesize an invalid Default.
            .filter(|default| variants.iter().any(|variant| &variant.wire_value == default));
        let definitions =
            self.catalog.modules.entry(format!("models/{domain}/enums.rs")).or_default();
        if definitions.iter().any(|definition| definition.name == name) {
            return Err(fail(&format!(
                "generated enum name {name} conflicts with another declaration"
            )));
        }
        definitions.push(StringEnum {
            name: name.into(),
            description: schema.get("description").and_then(Value::as_str).map(str::to_owned),
            derives: super::derives(),
            variants,
            default_value,
        });
        self.catalog
            .inline_type_paths
            .insert(location.into(), format!("crate::models::{domain}::enums::{name}"));
        Ok(())
    }
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
