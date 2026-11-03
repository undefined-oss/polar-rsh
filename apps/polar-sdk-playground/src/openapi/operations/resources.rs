//! Keep nested SDK groups in their domain and expose each child on its parent.

use std::collections::BTreeMap;

use heck::ToUpperCamelCase;

use super::{ModelCatalog, domain_type_name};
use crate::templates::{client::ResourceAccessor, resource::Resource};

pub(super) fn new(scope: &str, models: &ModelCatalog) -> Resource {
    Resource {
        name: domain_type_name(scope, &scope.to_upper_camel_case(), "Resource", models),
        methods: Vec::new(),
        accessors: Vec::new(),
    }
}

pub(super) fn connect(
    resources: &mut BTreeMap<String, Resource>,
    models: &ModelCatalog,
) -> Vec<ResourceAccessor> {
    // A namespace may have child resources without any operations of its own.
    for module in resources.keys().cloned().collect::<Vec<_>>() {
        let mut scope = scope(&module);
        while let Some((parent, _)) = scope.rsplit_once("::") {
            resources
                .entry(format!("models/{}.rs", parent.replace("::", "/")))
                .or_insert_with(|| new(parent, models));
            scope = parent.into();
        }
    }
    let entries: Vec<_> = resources
        .iter()
        .map(|(module, resource)| {
            let scope = scope(module);
            let (parent, name) = scope
                .rsplit_once("::")
                .map_or((None, scope.as_str()), |(parent, name)| (Some(parent.to_owned()), name));
            (
                parent,
                ResourceAccessor {
                    name: name.into(),
                    rust_type: format!("crate::models::{scope}::{}", resource.name),
                },
            )
        })
        .collect();
    let mut roots = Vec::new();
    for (parent, accessor) in entries {
        if let Some(parent) = parent {
            resources
                .get_mut(&format!("models/{}.rs", parent.replace("::", "/")))
                .unwrap()
                .accessors
                .push(accessor);
        } else {
            roots.push(accessor);
        }
    }
    for resource in resources.values_mut() {
        resource.methods.sort_by(|a, b| a.name.cmp(&b.name));
        resource.accessors.sort_by(|a, b| a.name.cmp(&b.name));
    }
    roots.sort_by(|a, b| a.name.cmp(&b.name));
    roots
}

fn scope(module: &str) -> String {
    module.strip_prefix("models/").unwrap().strip_suffix(".rs").unwrap().replace('/', "::")
}
