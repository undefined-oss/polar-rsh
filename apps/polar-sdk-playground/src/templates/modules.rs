//! Module declarations for the model files emitted by the current generation.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path, PathBuf},
};

use quote::quote;

use super::resource::{Resource, resource_tokens};

pub fn generate_modules(
    staging_dir: &Path,
    root: &str,
    files: &BTreeSet<String>,
    mut resources: BTreeMap<String, Resource>,
) -> Result<(), rrgen::Error> {
    let mut modules: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    let mut declarations = BTreeMap::new();
    syn::parse_str::<syn::Ident>(root)
        .map_err(|cause| rrgen::Error::Message(format!("invalid module root: {cause}")))?;
    modules.insert(PathBuf::from(root), BTreeSet::new());
    for file in files {
        let path = Path::new(file);
        if !path.starts_with(root)
            || path.extension().and_then(|value| value.to_str()) != Some("rs")
            || path.file_name().and_then(|value| value.to_str()) == Some("mod.rs")
            || !path.components().all(|part| matches!(part, Component::Normal(_)))
        {
            return Err(rrgen::Error::Message(format!("invalid model module file: {file}")));
        }
        let mut child = path.to_path_buf();
        let mut name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        while let Some(parent) = child.parent() {
            if parent.as_os_str().is_empty() {
                break;
            }
            let key = (parent.to_path_buf(), name.clone());
            if declarations.insert(key, child.clone()).is_some_and(|previous| previous != child) {
                return Err(rrgen::Error::Message(format!("conflicting module paths for {name}")));
            }
            modules.entry(parent.to_path_buf()).or_default().insert(name);
            if parent == Path::new(root) {
                break;
            }
            name = parent.file_name().unwrap().to_str().unwrap().to_owned();
            child = parent.to_path_buf();
        }
    }
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    let generator = rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera);
    if let Some(module) = resources.keys().find(|module| {
        !modules.keys().any(|directory| directory.with_extension("rs") == Path::new(module))
    }) {
        return Err(rrgen::Error::Message(format!("resource has no domain modules: {module}")));
    }
    let resource_names: BTreeMap<_, _> = resources
        .iter()
        .map(|(path, resource)| (PathBuf::from(path), resource.name.clone()))
        .collect();
    for (directory, names) in modules {
        let module_path = directory.with_extension("rs");
        let resource =
            resources.remove(module_path.to_str().unwrap()).map(resource_tokens).transpose()?;
        let names = names
            .iter()
            .map(|name| syn::parse_str::<syn::Ident>(name))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|cause| rrgen::Error::Message(format!("invalid module name: {cause}")))?;
        let exports: Vec<_> = names
            .iter()
            .filter(|name| directory != Path::new(root) && *name != "serde_helpers")
            .map(|name| {
                let child = directory.join(name.to_string()).with_extension("rs");
                if let Some(resource) = resource_names.get(&child) {
                    let resource = syn::parse_str::<syn::Ident>(resource)?;
                    // Child resources own their operation names; exporting
                    // their modules with a glob makes
                    // get/list/update ambiguous here.
                    Ok(quote!(pub use self::#name::#resource;))
                } else {
                    Ok(quote!(pub use self::#name::*;))
                }
            })
            .collect::<Result<_, syn::Error>>()
            .map_err(|cause| rrgen::Error::Message(format!("invalid resource export: {cause}")))?;
        let file = syn::parse2(quote! {
            #(pub mod #names;)*
            #(#exports)*
            #resource
        })
        .map_err(|cause| rrgen::Error::Message(format!("invalid module declarations: {cause}")))?;
        generator.generate(
            include_str!("../../templates/modules.rs.tera"),
            &serde_json::json!({
                "module_path": serde_json::to_string(&module_path)?,
                "declarations": super::format_rust(file),
            }),
        )?;
        let old_module = staging_dir.join(&directory).join("mod.rs");
        if old_module.is_file() {
            std::fs::remove_file(old_module)?;
        }
        if resource.is_some() {
            let old_resource = staging_dir.join(&directory).join("resource.rs");
            if old_resource.is_file() {
                std::fs::remove_file(old_resource)?;
            }
        }
    }
    Ok(())
}

pub fn generate_library(staging_dir: &Path) -> Result<rrgen::GenResult, rrgen::Error> {
    rrgen::RRgen::with_working_dir(staging_dir)
        .generate(include_str!("../../templates/lib.rs.tera"), &serde_json::json!({}))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::templates::{
        client::ResourceAccessor,
        resource::ResourceMethod,
        test_support::{TestResult, assert_generated, assert_source},
    };

    #[test]
    fn library() -> TestResult {
        assert_generated("library", "lib.rs", generate_library)
    }

    fn collect_sources(
        directory: &Path,
        root: &Path,
        sources: &mut BTreeMap<String, String>,
    ) -> std::io::Result<()> {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                collect_sources(&entry.path(), root, sources)?;
            } else {
                let path = entry.path();
                let relative =
                    path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
                sources.insert(relative, fs::read_to_string(path)?);
            }
        }
        Ok(())
    }

    fn assert_modules(
        name: &str,
        files: &[&str],
        resources: BTreeMap<String, Resource>,
    ) -> TestResult {
        let output = tempfile::tempdir()?;
        let files = files.iter().map(|file| (*file).to_owned()).collect();
        generate_modules(output.path(), "models", &files, resources)?;
        let mut sources = BTreeMap::new();
        collect_sources(output.path(), output.path(), &mut sources)?;
        let source = sources
            .into_iter()
            .map(|(path, source)| format!("// {path}\n{source}"))
            .collect::<Vec<_>>()
            .join("\n");
        assert_source(name, &source);
        Ok(())
    }

    #[test]
    fn shared_and_domain_exports() -> TestResult {
        assert_modules(
            "modules_exports",
            &[
                "models/organizations/types.rs",
                "models/organizations/enums.rs",
                "models/organizations/newtypes.rs",
                "models/common/serde_helpers.rs",
                "models/common/ids.rs",
                "models/pagination.rs",
            ],
            BTreeMap::new(),
        )
    }

    #[test]
    fn resource_in_domain_module() -> TestResult {
        assert_modules(
            "modules_resource",
            &["models/organizations/list.rs", "models/organizations/types.rs"],
            BTreeMap::from([(
                "models/organizations.rs".into(),
                Resource {
                    name: "Organizations".into(),
                    accessors: vec![],
                    methods: vec![ResourceMethod {
                        name: "list".into(),
                        description: Some("List organizations.".into()),
                        operation_type: "crate::models::organizations::ListOrganizations".into(),
                        arguments: vec![],
                        has_params: true,
                        optional_payload: false,
                    }],
                },
            )]),
        )
    }

    #[test]
    fn nested_resources_export_only_resource_names() -> TestResult {
        let customers = Resource { name: "Customers".into(), methods: vec![], accessors: vec![] };
        let subscriptions =
            Resource { name: "Subscriptions".into(), methods: vec![], accessors: vec![] };
        let portal = Resource {
            name: "CustomerPortal".into(),
            methods: vec![],
            accessors: vec![
                ResourceAccessor {
                    name: "customers".into(),
                    rust_type: "crate::models::customer_portal::customers::Customers".into(),
                },
                ResourceAccessor {
                    name: "subscriptions".into(),
                    rust_type: "crate::models::customer_portal::subscriptions::Subscriptions"
                        .into(),
                },
            ],
        };
        assert_modules(
            "modules_nested_resources",
            &[
                "models/customer_portal/customers/get.rs",
                "models/customer_portal/customers/types.rs",
                "models/customer_portal/subscriptions/get.rs",
                "models/customer_portal/subscriptions/types.rs",
                "models/customer_portal/enums.rs",
            ],
            BTreeMap::from([
                ("models/customer_portal.rs".into(), portal),
                ("models/customer_portal/customers.rs".into(), customers),
                ("models/customer_portal/subscriptions.rs".into(), subscriptions),
            ]),
        )
    }
}
