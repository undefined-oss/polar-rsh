use std::{error::Error, fs, path::Path};

mod client;
mod error;
mod generator;
mod models;
mod openapi;
mod query;
mod templates;

fn main() -> Result<(), Box<dyn Error>> {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(project_dir.join("src/2026-10.openapi.json"))?;
    generator::generate(&source, &project_dir.join("generated"))
}
