//! Emit a client bound to a single API version at generation time.

use std::{collections::BTreeSet, path::Path};

use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};

const TEMPLATE: &str = include_str!("../../templates/client.rs.tera");

/// The supported fields of an OpenAPI server entry. The Polar extension is the
/// source of environment identity; other extensions do not affect naming.
#[derive(Clone, Deserialize)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    #[serde(rename = "x-polar-environment")]
    pub environment: String,
}

#[derive(Serialize)]
struct EnvironmentContext {
    name: String,
    documentation: Vec<String>,
    url_literal: String,
    is_default: bool,
}

#[derive(Serialize)]
struct ClientContext {
    version_literal: String,
    environments: Vec<EnvironmentContext>,
}

pub fn generate_client(
    staging_dir: &Path,
    version: &str,
    servers: &[Server],
) -> Result<rrgen::GenResult, rrgen::Error> {
    if version.is_empty() || http::HeaderValue::from_str(version).is_err() {
        return Err(rrgen::Error::Message(
            "API version must be a nonempty HTTP header value".into(),
        ));
    }
    let mut names = BTreeSet::new();
    let mut environments = Vec::new();
    for server in servers {
        let name = server.environment.to_upper_camel_case();
        if name.is_empty() || name == "Self" || name.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(rrgen::Error::Message(format!(
                "invalid environment name: {:?}",
                server.environment
            )));
        }
        if !names.insert(name.clone()) {
            return Err(rrgen::Error::Message(format!("duplicate generated environment: {name}")));
        }
        let url = url::Url::parse(&server.url).map_err(|error| {
            rrgen::Error::Message(format!("invalid server URL {:?}: {error}", server.url))
        })?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || server.url.contains(['{', '}'])
        {
            return Err(rrgen::Error::Message(
                "server URL must be an absolute HTTP(S) URL without unresolved variables".into(),
            ));
        }
        environments.push(EnvironmentContext {
            name,
            documentation: super::documentation_lines(server.description.as_deref()),
            url_literal: format!("{:?}", server.url),
            is_default: server.environment == "production",
        });
    }
    if !environments.iter().any(|environment| environment.is_default) {
        return Err(rrgen::Error::Message(
            "servers must include the production environment".into(),
        ));
    }
    environments.sort_by(|a, b| a.name.cmp(&b.name));
    let mut tera = tera::Tera::default();
    tera.autoescape_on(vec![]);
    rrgen::RRgen::with_working_dir(staging_dir).add_template_engine(tera).generate(
        TEMPLATE,
        &serde_json::to_value(ClientContext {
            version_literal: format!("{version:?}"),
            environments,
        })?,
    )
}
