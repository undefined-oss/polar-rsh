//! Discover UUIDs across the document, then resolve reusable identities.
//!
//! Every schema ending in `Event` uses `EventId` for its `id` and `parent_id`
//! (`parentId`) fields. Other fields, such as `customer_id`, retain their own
//! entity identities. This convention takes precedence over inferred ownership.

use std::collections::{BTreeMap, BTreeSet};

use heck::ToUpperCamelCase;

use super::ConversionError;
use crate::templates::uuid_ids::UuidId;

mod scan;
use scan::{Occurrence, Scan};

pub struct UuidCatalog {
    pub ids: Vec<UuidId>,
    /// Exact UUID schema locations and references mapped to shared types.
    pub type_paths: BTreeMap<String, String>,
}

pub fn collect_uuid_ids(document: &serde_json::Value) -> Result<UuidCatalog, ConversionError> {
    let scan = scan::discover(document)?;
    let owners = resolve_owners(&scan);
    let mut definitions: BTreeMap<String, Vec<&Occurrence>> = BTreeMap::new();
    let mut type_paths = BTreeMap::new();
    for occurrence in &scan.occurrences {
        let owner = owners.get(&occurrence.owner).unwrap_or(&occurrence.owner);
        let identity = if occurrence.owner.ends_with("Event")
            && matches!(occurrence.field.as_str(), "id" | "parent_id" | "parentId")
        {
            "Event".to_owned()
        } else if let Some(identity) = &occurrence.path_identity {
            identity.clone()
        } else if occurrence.field == "id" || occurrence.field == "parent_id" {
            owner.clone()
        } else if let Some(entity) = occurrence.field.strip_suffix("_id") {
            entity.to_upper_camel_case()
        } else {
            let singular = scan::singular(&occurrence.field).to_upper_camel_case();
            if scan.entities.contains(&singular) {
                singular
            } else {
                format!("{}{}", owner, occurrence.field.to_upper_camel_case())
            }
        };
        let name = if identity.ends_with("Id") { identity } else { format!("{identity}Id") };
        syn::parse_str::<syn::Ident>(&name)
            .map_err(|cause| fail(&occurrence.location, cause.to_string()))?;
        definitions.entry(name.clone()).or_default().push(occurrence);
        let type_path = format!("crate::models::common::ids::{name}");
        type_paths.insert(occurrence.location.clone(), type_path);
    }

    loop {
        let mut changed = false;
        for (source, target) in &scan.references {
            if !type_paths.contains_key(source) {
                if let Some(ty) = type_paths.get(target).cloned() {
                    type_paths.insert(source.clone(), ty);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let ids = definitions
        .into_iter()
        .map(|(name, mut occurrences)| {
            occurrences.sort_by_key(|occurrence| {
                (
                    occurrence
                        .description
                        .as_deref()
                        .is_none_or(|text| text == "The ID of the object."),
                    occurrence.location.clone(),
                )
            });
            UuidId {
                name,
                description: occurrences
                    .into_iter()
                    .find_map(|occurrence| occurrence.description.clone()),
            }
        })
        .collect();
    Ok(UuidCatalog { ids, type_paths })
}

fn resolve_owners(scan: &Scan) -> BTreeMap<String, String> {
    let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (a, b) in &scan.unions {
        graph.entry(a.clone()).or_default().insert(b.clone());
        graph.entry(b.clone()).or_default().insert(a.clone());
    }
    let mut result = BTreeMap::new();
    for owner in scan.occurrences.iter().map(|occurrence| &occurrence.owner) {
        if result.contains_key(owner) {
            continue;
        }
        let mut members = BTreeSet::new();
        let mut pending = vec![owner.clone()];
        while let Some(member) = pending.pop() {
            if members.insert(member.clone()) {
                pending.extend(graph.get(&member).into_iter().flatten().cloned());
            }
        }
        let mut identities = BTreeSet::new();
        for member in &members {
            identities.extend(scan.anchors.get(member).into_iter().flatten().cloned());
            if scan.entities.contains(member) {
                identities.insert(member.clone());
            }
        }
        // Conflicting evidence must not collapse unrelated identities. Keep the
        // union's schema name until its consumer supplies more context.
        let identity = if identities.len() == 1 {
            identities.into_iter().next().unwrap()
        } else {
            members.iter().min_by_key(|name| (name.len(), *name)).unwrap().clone()
        };
        for member in members {
            result.insert(member, identity.clone());
        }
    }
    result
}

fn fail(location: &str, message: impl Into<String>) -> ConversionError {
    ConversionError::InvalidUuidId { location: location.into(), message: message.into() }
}
