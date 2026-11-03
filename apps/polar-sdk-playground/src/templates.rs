//! Experimental template rendering, independent of OpenAPI parsing.

pub mod aliases;
pub mod bodyless_operation;
pub mod client;
pub mod error;
pub mod form_operation;
pub mod get_operation;
pub mod json_operation;
pub mod list_operation;
pub mod models;
pub mod modules;
pub mod newtypes;
pub mod operation;
pub mod operation_errors;
pub mod operation_query;
pub mod pagination;
pub mod path;
pub mod query;
pub mod resource;
pub mod serde_helpers;
pub mod string_enums;
pub mod unions;
pub mod uuid_ids;
pub mod webhooks;

fn format_rust_file(path: &std::path::Path) -> Result<(), rrgen::Error> {
    let source = std::fs::read_to_string(path)?;
    let file = syn::parse_file(&source).map_err(|error| {
        rrgen::Error::Message(format!("invalid generated {}: {error}", path.display()))
    })?;
    std::fs::write(path, format_rust(file))?;
    Ok(())
}

/// Format declarations separately to retain space between top-level items.
fn format_rust(mut file: syn::File) -> String {
    let items = std::mem::take(&mut file.items);
    let mut output = prettyplease::unparse(&file);
    file.shebang = None;
    file.attrs.clear();
    let mut previous_is_import = false;
    let mut previous_is_module = false;

    for item in items {
        let is_import = matches!(&item, syn::Item::Use(_));
        let is_module = matches!(&item, syn::Item::Mod(module) if module.content.is_none());
        if !output.is_empty()
            && !(previous_is_import && is_import || previous_is_module && is_module)
        {
            output.push('\n');
        }
        file.items.push(item);
        output.push_str(&prettyplease::unparse(&file));
        file.items.clear();
        previous_is_import = is_import;
        previous_is_module = is_module;
    }

    output
}

/// Normalize line endings while preserving Markdown and paragraph breaks.
fn documentation_lines(description: Option<&str>) -> Vec<String> {
    description
        .unwrap_or_default()
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(String::from)
        .collect()
}
