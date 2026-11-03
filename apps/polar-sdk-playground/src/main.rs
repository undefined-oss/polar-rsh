use std::{collections::BTreeSet, error::Error, fs, path::Path};

mod client;
mod error;
mod models;
mod openapi;
mod query;
mod templates;

fn main() -> Result<(), Box<dyn Error>> {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(project_dir.join("src/2026-10.openapi.json"))?;
    let spec = oas3::from_json(&source)?;
    let document: serde_json::Value = serde_json::from_str(&source)?;
    let client = openapi::ClientInput::try_from(&spec)?;
    let page_limit = openapi::pagination::page_limit_config(&spec)?;
    let enums = openapi::enums::collect_enums(&spec, &document)?;
    let ids = openapi::uuid_ids::collect_uuid_ids(&document)?;
    let newtypes = openapi::newtypes::collect_newtypes(&spec, &document, &enums)?;
    let mut model_catalog =
        openapi::models::collect_models(&spec, &document, &enums, &ids, &newtypes)?;
    let operations =
        openapi::operations::collect_operations(&spec, &document, &model_catalog, &ids, &enums)?;
    for (module, models) in &mut model_catalog.modules {
        let prefix = module.trim_end_matches(".rs").replace('/', "::");
        for model in models {
            model.derive_error =
                operations.error_types.contains(&format!("crate::{prefix}::{}", model.name));
        }
    }
    let output_dir = project_dir.join("generated");
    let mut model_files: BTreeSet<String> =
        ["models/common/ids.rs".into(), "models/common/serde_helpers.rs".into()].into();
    model_files.extend(
        enums
            .modules
            .keys()
            .chain(model_catalog.modules.keys())
            .chain(model_catalog.aliases.keys())
            .chain(model_catalog.unions.keys())
            .chain(newtypes.modules.keys())
            .chain(operations.operations.keys())
            .cloned(),
    );

    let id_count = ids.ids.len();
    templates::uuid_ids::generate_uuid_ids(&output_dir, ids.ids)?;
    fs::write(
        output_dir.join("uuid-mappings.json"),
        serde_json::to_string_pretty(&ids.type_paths)?,
    )?;
    println!("Resolved {} UUID schema mappings", ids.type_paths.len());
    println!(
        "Generated {} UUID types in {}",
        id_count,
        output_dir.join("models/common/ids.rs").display()
    );

    templates::client::generate_client(
        &output_dir,
        &client.version,
        &client.servers,
        &operations.accessors,
    )?;
    println!("Generated {}", output_dir.join("client.rs").display());

    templates::error::generate_error(&output_dir)?;
    println!("Generated {}", output_dir.join("error.rs").display());

    templates::query::generate_query(&output_dir)?;
    templates::path::generate_path(&output_dir)?;
    println!("Generated {}", output_dir.join("query.rs").display());

    templates::serde_helpers::generate_serde_helpers(&output_dir)?;
    println!("Generated {}", output_dir.join("models/common/serde_helpers.rs").display());

    if let Some(config) = page_limit {
        model_files.insert("models/pagination.rs".into());
        templates::pagination::generate_pagination(&output_dir, config)?;
        println!("Generated {}", output_dir.join("models/pagination.rs").display());
    }

    println!(
        "Generating {} distinct enum types",
        enums.modules.values().map(Vec::len).sum::<usize>()
    );
    // Remove enum and newtype modules no longer emitted by this schema.
    for entry in fs::read_dir(output_dir.join("models"))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let path = entry.path().join("enums.rs");
        if path.is_file() && !enums.modules.keys().any(|module| output_dir.join(module) == path) {
            fs::remove_file(&path)?;
            println!("Removed {}", path.display());
        }
        let path = entry.path().join("newtypes.rs");
        if path.is_file() && !newtypes.modules.keys().any(|module| output_dir.join(module) == path)
        {
            fs::remove_file(&path)?;
            println!("Removed {}", path.display());
        }
    }
    for (module_path, definitions) in enums.modules {
        templates::string_enums::generate_string_enums(&output_dir, &module_path, definitions)?;
        println!("Generated {}", output_dir.join(module_path).display());
    }

    fs::write(
        output_dir.join("newtype-mappings.json"),
        serde_json::to_string_pretty(&newtypes.type_paths)?,
    )?;
    println!(
        "Generating {} validated scalar types for {} schema locations",
        newtypes.modules.values().map(Vec::len).sum::<usize>(),
        newtypes.type_paths.len()
    );
    for (module_path, definitions) in newtypes.modules {
        templates::newtypes::generate_newtypes(&output_dir, &module_path, definitions)?;
    }

    fs::write(
        output_dir.join("model-report.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "generated": model_catalog.type_paths,
            "deferred": model_catalog.deferred,
            "unions": model_catalog.union_type_paths,
            "inline_enums": enums.inline_type_paths,
        }))?,
    )?;
    println!(
        "Generating {} named model types; {} schemas deferred",
        model_catalog.type_paths.len(),
        model_catalog.deferred.len()
    );
    for entry in fs::read_dir(output_dir.join("models"))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let path = entry.path().join("types.rs");
        if path.is_file()
            && !model_catalog.modules.keys().any(|module| output_dir.join(module) == path)
        {
            fs::remove_file(&path)?;
        }
        let path = entry.path().join("literals.rs");
        if path.is_file() {
            fs::remove_file(&path)?;
        }
        let path = entry.path().join("unions.rs");
        if path.is_file()
            && !model_catalog.unions.keys().any(|module| output_dir.join(module) == path)
        {
            fs::remove_file(&path)?;
        }
        let path = entry.path().join("aliases.rs");
        if path.is_file()
            && !model_catalog.aliases.keys().any(|module| output_dir.join(module) == path)
        {
            fs::remove_file(&path)?;
        }
    }
    if model_catalog
        .unions
        .get("models/webhooks/events.rs")
        .is_some_and(|unions| unions.iter().any(|union| union.name == "WebhookPayload"))
    {
        templates::webhooks::generate_verification(&output_dir)?;
        model_files.insert("models/webhooks/verification.rs".into());
    }
    for (module_path, definitions) in model_catalog.unions {
        templates::unions::generate_unions(&output_dir, &module_path, definitions)?;
        println!("Generated {}", output_dir.join(module_path).display());
    }
    for (module_path, definitions) in model_catalog.aliases {
        templates::aliases::generate_aliases(&output_dir, &module_path, definitions)?;
        println!("Generated {}", output_dir.join(module_path).display());
    }
    for (module_path, definitions) in model_catalog.modules {
        templates::models::generate_models(&output_dir, &module_path, definitions)?;
        println!("Generated {}", output_dir.join(module_path).display());
    }

    fs::write(
        output_dir.join("operation-report.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "generated": operations.generated,
            "deferred": operations.deferred,
            "excluded": operations.excluded,
        }))?,
    )?;
    for (module, operation) in operations.operations {
        match operation.request {
            openapi::operations::OperationRequest::List(config) => {
                templates::list_operation::generate_list_operation(
                    &output_dir,
                    &module,
                    operation.definition,
                    config,
                )?;
            }
            openapi::operations::OperationRequest::Bodyless { method } => {
                templates::bodyless_operation::generate_bodyless_operation(
                    &output_dir,
                    &module,
                    operation.definition,
                    method,
                )?;
            }
            openapi::operations::OperationRequest::Json { method, payload } => {
                templates::json_operation::generate_json_operation(
                    &output_dir,
                    &module,
                    operation.definition,
                    method,
                    payload,
                )?;
            }
            openapi::operations::OperationRequest::Form { method, payload } => {
                templates::form_operation::generate_form_operation(
                    &output_dir,
                    &module,
                    operation.definition,
                    method,
                    payload,
                )?;
            }
        }
    }
    templates::modules::generate_modules(
        &output_dir,
        "models",
        &model_files,
        operations.resources,
    )?;
    println!("Generated {}", output_dir.join("models.rs").display());
    templates::modules::generate_library(&output_dir)?;

    Ok(())
}
