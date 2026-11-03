//! Resolve operations supported by the current request templates.

use std::collections::{BTreeMap, BTreeSet};

use heck::{ToSnakeCase, ToUpperCamelCase};
use serde_json::Value;

use super::{
    ConversionError, enums::EnumCatalog, models::ModelCatalog, references::dereference,
    uuid_ids::UuidCatalog,
};
use crate::templates::{
    bodyless_operation::BodylessMethod,
    client::ResourceAccessor,
    list_operation::ListConfig,
    operation::{
        ErrorResponse, OperationDefinition, PathParameter, PayloadMethod, RequestPayload,
        ResponseBody, SuccessResponse,
    },
    operation_errors::ApiError,
    resource::{Argument, Resource, ResourceMethod},
};

mod documentation;
mod errors;
mod list;
mod query;
mod query_types;
mod resources;

#[derive(Default)]
pub struct OperationCatalog {
    pub operations: BTreeMap<String, Operation>,
    pub resources: BTreeMap<String, Resource>,
    pub accessors: Vec<ResourceAccessor>,
    pub error_types: BTreeSet<String>,
    pub generated: BTreeMap<String, String>,
    pub deferred: BTreeMap<String, String>,
    pub excluded: BTreeSet<String>,
}

pub struct Operation {
    pub definition: OperationDefinition,
    pub request: OperationRequest,
}

pub enum OperationRequest {
    Bodyless { method: BodylessMethod },
    Json { method: PayloadMethod, payload: RequestPayload },
    Form { method: PayloadMethod, payload: RequestPayload },
    List(ListConfig),
}

pub fn collect_operations(
    spec: &oas3::Spec,
    document: &Value,
    models: &ModelCatalog,
    ids: &UuidCatalog,
    enums: &EnumCatalog,
) -> Result<OperationCatalog, ConversionError> {
    let mut catalog = OperationCatalog::default();
    for (path, item) in spec.paths.iter().flatten() {
        for (method, operation) in item.methods() {
            let method = format!("{method:?}").to_lowercase();
            let location = format!("#/paths/{}/{}", escape(path), method);
            let group =
                operation.extensions.get("speakeasy-group").and_then(Value::as_str).ok_or_else(
                    || {
                        ConversionError::InvalidOperation(format!(
                            "{location}: missing x-speakeasy-group"
                        ))
                    },
                )?;
            let value = document.pointer(location.strip_prefix('#').unwrap()).unwrap();
            let resolved =
                resolve(path, &method, group, value, &location, document, models, ids, enums);
            let (domain, method_name, operation) = match resolved {
                Ok(resolved) => resolved,
                Err(reason) => {
                    catalog.deferred.insert(location, reason);
                    continue;
                }
            };
            let definition = &operation.definition;
            let module = format!("models/{}/{method_name}.rs", domain.replace("::", "/"));
            let operation_type = format!("crate::models::{domain}::{}", definition.name);
            if catalog.operations.contains_key(&module)
                || matches!(
                    method_name.as_str(),
                    "resource"
                        | "types"
                        | "enums"
                        | "aliases"
                        | "unions"
                        | "ids"
                        | "serde_helpers"
                        | "newtypes"
                        | "verification"
                        | "mod"
                )
            {
                return Err(ConversionError::InvalidOperation(format!(
                    "{location}: duplicate or reserved operation module {module}"
                )));
            }
            let resource = catalog
                .resources
                .entry(format!("models/{}.rs", domain.replace("::", "/")))
                .or_insert_with(|| resources::new(&domain, models));
            let mut arguments: Vec<_> = definition
                .parameters
                .iter()
                .map(|parameter| Argument {
                    name: parameter.rust_name.clone(),
                    field: parameter.rust_name.clone(),
                    rust_type: parameter.rust_type.clone(),
                })
                .collect();
            let mut optional_payload = false;
            let required_query = definition.query.as_ref().is_some_and(|query| query.required());
            if let Some(query) = definition.query.as_ref().filter(|query| query.required()) {
                arguments.push(Argument {
                    name: "params".into(),
                    field: "params".into(),
                    rust_type: format!("crate::models::{domain}::{}", query.params_name),
                });
            }
            if let OperationRequest::Json { payload, .. } | OperationRequest::Form { payload, .. } =
                &operation.request
            {
                optional_payload = !payload.required;
                if payload.required {
                    arguments.push(Argument {
                        name: "payload".into(),
                        field: "payload".into(),
                        rust_type: payload.rust_type.clone(),
                    });
                }
            }
            resource.methods.push(ResourceMethod {
                name: method_name,
                description: definition.description.clone(),
                operation_type: operation_type.clone(),
                arguments,
                has_params: matches!(&operation.request, OperationRequest::List(_))
                    || definition.query.is_some() && !required_query,
                optional_payload,
            });
            catalog
                .error_types
                .extend(definition.errors.iter().map(|error| error.error.payload_type.clone()));
            catalog.operations.insert(module, operation);
            catalog.generated.insert(location, operation_type);
        }
    }
    catalog.accessors = resources::connect(&mut catalog.resources, models);
    Ok(catalog)
}

fn resolve(
    path: &str,
    method: &str,
    group: &str,
    operation: &Value,
    location: &str,
    document: &Value,
    models: &ModelCatalog,
    ids: &UuidCatalog,
    enums: &EnumCatalog,
) -> Result<(String, String, Operation), String> {
    if !matches!(method, "get" | "post" | "put" | "patch" | "delete") {
        return Err("request method is not connected to generation yet".into());
    }
    let customer_portal = operation
        .get("operationId")
        .and_then(Value::as_str)
        .is_some_and(|id| id.starts_with("customer_portal:"));
    let mut request = resolve_request(method, operation, location, document, models)?;
    let segments: Vec<_> = group.split('.').map(ToSnakeCase::to_snake_case).collect();
    for segment in &segments {
        syn::parse_str::<syn::Ident>(segment).map_err(|cause| cause.to_string())?;
    }
    // Portal operations reuse the SDK group names of organization operations.
    // Their operationId namespace keeps those resource methods distinct.
    let domain = if customer_portal {
        format!("customer_portal::{}", segments.join("::"))
    } else {
        segments.join("::")
    };
    let method_name = operation
        .get("x-speakeasy-name-override")
        .and_then(Value::as_str)
        .ok_or("missing x-speakeasy-name-override")?
        .to_snake_case();
    syn::parse_str::<syn::Ident>(&method_name).map_err(|cause| cause.to_string())?;
    let operation_name = domain_type_name(
        &domain,
        &format!("{}{}", domain.to_upper_camel_case(), method_name.to_upper_camel_case()),
        "Operation",
        models,
    );
    let path_location = format!("#/paths/{}", escape(path));
    let path_item = document.pointer(path_location.strip_prefix('#').unwrap()).unwrap();
    let mut parameters = BTreeMap::new();
    for (owner, base) in [(path_item, path_location.as_str()), (operation, location)] {
        for (index, parameter) in
            owner.get("parameters").and_then(Value::as_array).into_iter().flatten().enumerate()
        {
            let (parameter, base) =
                dereference(parameter, format!("{base}/parameters/{index}"), document)?;
            let name =
                parameter.get("name").and_then(Value::as_str).ok_or("parameter has no name")?;
            let place =
                parameter.get("in").and_then(Value::as_str).ok_or("parameter has no location")?;
            parameters.insert((place.to_owned(), name.to_owned()), (parameter, base));
        }
    }
    let mut query = None;
    if operation.get("x-polar-pagination").is_some()
        || operation.get("x-speakeasy-pagination").is_some()
    {
        if method != "get" || operation.get("requestBody").is_some() {
            return Err("page streams require GET without a request body".into());
        }
        let resolver = query_types::QueryResolver { document, models, enums, ids };
        request = OperationRequest::List(list::resolve(
            operation,
            &operation_name,
            &mut parameters,
            &resolver,
        )?);
    } else if parameters.keys().any(|(place, _)| place == "query") {
        let resolver = query_types::QueryResolver { document, models, enums, ids };
        query = Some(query::resolve(&operation_name, &mut parameters, &resolver)?);
    }
    let mut parameters = parameters
        .into_iter()
        .map(|((place, name), (parameter, base))| {
            if place != "path" {
                return Err(format!(
                    "{place} parameters are not connected to this operation adapter"
                ));
            }
            if parameter.get("required") != Some(&Value::Bool(true)) {
                return Err("path parameter must be required".into());
            }
            if parameter.get("style").and_then(Value::as_str).is_some_and(|style| style != "simple")
            {
                return Err(format!(
                    "{base}: only simple path parameter serialization is supported"
                ));
            }
            let schema_location = format!("{base}/schema");
            let (rust_type, percent_encode) = if let Some(ty) = ids.type_paths.get(&schema_location)
            {
                (ty.clone(), false)
            } else {
                let schema = parameter
                    .get("schema")
                    .ok_or_else(|| format!("{base}: missing path parameter schema"))?;
                let (schema, _) = dereference(schema, schema_location, document)?;
                if schema.get("type").and_then(Value::as_str) != Some("string")
                    || schema.get("enum").is_some()
                    || schema.get("const").is_some()
                    || ["anyOf", "oneOf", "allOf"].iter().any(|key| schema.get(key).is_some())
                    || schema
                        .get("format")
                        .and_then(Value::as_str)
                        .is_some_and(|format| format.starts_with("uuid"))
                {
                    return Err(format!(
                        "{base}: path parameter requires a resolved UUID or string"
                    ));
                }
                ("String".into(), true)
            };
            Ok(PathParameter { rust_name: name.to_snake_case(), name, rust_type, percent_encode })
        })
        .collect::<Result<Vec<_>, String>>()?;
    parameters.sort_by_key(|parameter| path.find(&format!("{{{}}}", parameter.name)));
    let responses =
        operation.get("responses").and_then(Value::as_object).ok_or("missing responses")?;
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    let mut untyped_errors = Vec::new();
    let mut error_types = BTreeSet::new();
    for (status, response) in responses {
        let status: u16 = status
            .parse()
            .map_err(|_| "response ranges and default responses are not supported")?;
        let (response, response_location) =
            dereference(response, format!("{location}/responses/{status}"), document)?;
        let no_content = response
            .get("content")
            .is_none_or(|content| content.as_object().is_some_and(|content| content.is_empty()));
        if (200..300).contains(&status) && no_content {
            if matches!(&request, OperationRequest::List(_)) {
                return Err("list success requires a JSON page".into());
            }
            successes.push(SuccessResponse {
                status,
                description: response.get("description").and_then(Value::as_str).map(str::to_owned),
                body: ResponseBody::Empty,
            });
            continue;
        }
        if (400..600).contains(&status) && no_content {
            untyped_errors.push(status);
            continue;
        }
        if (200..300).contains(&status)
            && response.pointer("/content/text~1csv/schema/type").and_then(Value::as_str)
                == Some("string")
        {
            if matches!(&request, OperationRequest::List(_)) {
                return Err("list success requires a JSON page".into());
            }
            successes.push(SuccessResponse {
                status,
                description: response.get("description").and_then(Value::as_str).map(str::to_owned),
                body: ResponseBody::Bytes { media_type: "text/csv".into() },
            });
            continue;
        }
        let schema = response
            .pointer("/content/application~1json/schema")
            .ok_or("response requires an application/json schema")?;
        if (200..300).contains(&status) {
            let schema_location = format!("{response_location}/content/application~1json/schema");
            let ty = match &request {
                OperationRequest::List(config) => {
                    list::response_type(response, operation, config, document)?
                }
                _ if models.union_type_paths.contains_key(&schema_location) => {
                    models.union_type_paths[&schema_location].clone()
                }
                _ if schema.get("type").and_then(Value::as_str) == Some("array") => {
                    let items = schema.get("items").ok_or("response array requires items")?;
                    format!("Vec<{}>", component_type(items, models, "response item")?.1)
                }
                _ if schema.as_object().is_some_and(|schema| schema.is_empty()) => {
                    "serde_json::Value".into()
                }
                _ => component_type(schema, models, "response")?.1,
            };
            successes.push(SuccessResponse {
                status,
                description: response.get("description").and_then(Value::as_str).map(str::to_owned),
                body: ResponseBody::Json(ty),
            });
        } else if (400..600).contains(&status) {
            let description =
                response.get("description").and_then(Value::as_str).map(str::to_owned);
            for (name, ty) in errors::payloads(
                schema,
                &format!("{response_location}/content/application~1json/schema"),
                document,
                models,
            )? {
                if !error_types.insert((status, ty.clone())) {
                    continue;
                }
                errors.push(ErrorResponse {
                    status,
                    error: ApiError {
                        variant: name.to_upper_camel_case(),
                        payload_type: ty,
                        message: description.clone().unwrap_or_else(|| format!("HTTP {status}")),
                        description: description.clone(),
                    },
                });
            }
        } else {
            return Err("redirect responses are not supported".into());
        }
    }
    if successes.is_empty() {
        return Err("missing success response".into());
    }
    if matches!(&request, OperationRequest::List(_)) && successes.len() != 1 {
        return Err("paginated operations require one success response".into());
    }
    Ok((
        domain.clone(),
        method_name.clone(),
        Operation {
            request,
            definition: OperationDefinition {
                name: operation_name,
                description: documentation::description(operation, document),
                path: path.into(),
                parameters,
                query,
                successes,
                errors,
                untyped_errors,
            },
        },
    ))
}

fn resolve_request(
    method: &str,
    operation: &Value,
    location: &str,
    document: &Value,
    models: &ModelCatalog,
) -> Result<OperationRequest, String> {
    let Some(body) = operation.get("requestBody") else {
        let method = match method {
            "get" => BodylessMethod::Get,
            "post" => BodylessMethod::Post,
            "patch" => BodylessMethod::Patch,
            "put" => BodylessMethod::Put,
            "delete" => BodylessMethod::Delete,
            _ => return Err(format!("unsupported bodyless request method: {method}")),
        };
        return Ok(OperationRequest::Bodyless { method });
    };
    if matches!(method, "get" | "delete") {
        return Err(format!("{} request bodies are not supported", method.to_uppercase()));
    }
    let (body, location) = dereference(body, format!("{location}/requestBody"), document)?;
    let required = body.get("required") == Some(&Value::Bool(true));
    let (media_type, schema) = ["application/json", "application/x-www-form-urlencoded"]
        .into_iter()
        .find_map(|media_type| {
            body.get("content")?.get(media_type)?.get("schema").map(|schema| (media_type, schema))
        })
        .ok_or("request body requires a supported JSON or form schema")?;
    let schema_location = format!("{location}/content/{}/schema", escape(media_type));
    let payload_type = if let Some(ty) = models.union_type_paths.get(&schema_location) {
        let nullable = ["anyOf", "oneOf"].iter().any(|keyword| {
            schema.get(keyword).and_then(Value::as_array).is_some_and(|branches| {
                branches
                    .iter()
                    .any(|branch| branch.get("type").and_then(Value::as_str) == Some("null"))
            })
        });
        if nullable { format!("Option<{ty}>") } else { ty.clone() }
    } else {
        if let Some(reason) = models.deferred.get(&schema_location) {
            return Err(reason.clone());
        }
        component_type(schema, models, "request")?.1
    };
    let method = match method {
        "post" => PayloadMethod::Post,
        "patch" => PayloadMethod::Patch,
        "put" => PayloadMethod::Put,
        _ => return Err(format!("unsupported payload request method: {method}")),
    };
    let payload = RequestPayload { rust_type: payload_type, required };
    Ok(if media_type == "application/x-www-form-urlencoded" {
        OperationRequest::Form { method, payload }
    } else {
        OperationRequest::Json { method, payload }
    })
}

fn component_type(
    schema: &Value,
    models: &ModelCatalog,
    context: &str,
) -> Result<(String, String), String> {
    let name = schema
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix("#/components/schemas/"))
        .ok_or_else(|| format!("inline {context} schemas require operation-level type discovery"))?
        .replace("~1", "/")
        .replace("~0", "~");
    let ty = models
        .type_paths
        .get(&name)
        .ok_or_else(|| format!("{context} model {name} is not generated"))?
        .clone();
    Ok((name, ty))
}

// Domain exports share a namespace with their models.
fn domain_type_name(domain: &str, name: &str, suffix: &str, models: &ModelCatalog) -> String {
    let prefix = format!("crate::models::{domain}::");
    let mut name = name.to_owned();
    while models
        .type_paths
        .values()
        .chain(models.union_type_paths.values())
        .any(|path| path.starts_with(&prefix) && path.rsplit("::").next() == Some(name.as_str()))
    {
        name.push_str(suffix);
    }
    name
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
