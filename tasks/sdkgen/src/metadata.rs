//! Binding metadata shared by every generator pass. Keep unknown upstream
//! fields when rewriting the file, but deserialize consumed fields strictly.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

type Extensions = IndexMap<String, Value>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bindings {
    pub schema_version: u32,
    pub generator_version: String,
    pub module_label: String,
    pub coverage: Vec<String>,
    pub modules: Vec<String>,
    pub reexports: Vec<String>,
    pub symbols: Vec<Symbol>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

impl Bindings {
    pub fn validate(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.schema_version == 1,
            "Unsupported binding metadata version {}",
            self.schema_version
        );
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    pub path: String,
    pub name: String,
    pub kind: String,
    pub attributes: Vec<String>,
    pub generics: String,
    pub impl_generics: Option<String>,
    pub impl_trait: Option<String>,
    pub rust_type: Option<String>,
    pub fields: Vec<Field>,
    pub variants: Vec<Variant>,
    pub signature: Option<Signature>,
    pub operation: Option<Operation>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Field {
    pub name: Option<String>,
    pub index: usize,
    pub public: bool,
    pub wire_name: Option<String>,
    pub rust_type: String,
    pub attributes: Vec<String>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Variant {
    pub name: String,
    pub wire_name: Option<String>,
    pub attributes: Vec<String>,
    pub fields: Vec<Field>,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub rust: String,
    pub asynchronous: bool,
    pub receiver: Option<String>,
    pub arguments: Vec<Argument>,
    pub return_type: String,
    pub generics: String,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Argument {
    pub name: String,
    pub rust_type: String,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Operation {
    pub operation_id: String,
    pub source_json_pointer: String,
    pub source_method: String,
    pub source_path: String,
    pub source_operation_id: String,
    pub webhook: bool,
    pub response_statuses: Vec<String>,
    pub response_excluded_statuses: Vec<String>,
    pub response_media_type: Option<String>,
    pub response_kind: String,
    pub consumption: String,
    #[serde(flatten)]
    pub extensions: Extensions,
}

#[cfg(test)]
pub fn fixture(symbols: Vec<Symbol>) -> Bindings {
    Bindings {
        schema_version: 1,
        generator_version: "0.22.0".into(),
        module_label: "test".into(),
        coverage: vec![],
        modules: vec![],
        reexports: vec![],
        symbols,
        extensions: IndexMap::new(),
    }
}

#[cfg(test)]
pub fn test_symbol(path: &str, name: &str) -> Symbol {
    Symbol {
        path: path.into(),
        name: name.into(),
        kind: "struct".into(),
        attributes: vec![],
        generics: String::new(),
        impl_generics: None,
        impl_trait: None,
        rust_type: None,
        fields: vec![],
        variants: vec![],
        signature: None,
        operation: None,
        extensions: IndexMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_checked_in_binding_file_round_trips_without_losing_fields() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../apps/polar-rsh/src");
        for entry in std::fs::read_dir(root).unwrap() {
            let path = entry.unwrap().path().join("generated/bindings.json");
            if !path.exists() {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            let metadata: Bindings = serde_json::from_str(&source).unwrap();
            metadata.validate().unwrap();
            assert_eq!(
                format!("{}\n", serde_json::to_string_pretty(&metadata).unwrap()),
                source,
                "Binding serialization changed {}",
                path.display()
            );
        }
    }

    #[test]
    fn rejects_malformed_known_fields_and_retains_extensions() {
        let mut value =
            serde_json::to_value(fixture(vec![test_symbol("types::Model", "Model")])).unwrap();
        value["future_metadata"] = serde_json::json!({"nested": [null, true]});
        value["symbols"][0]["future_symbol"] = serde_json::json!([1, 2]);
        let parsed: Bindings = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), value);
        value["symbols"][0]["fields"] = Value::Null;
        assert!(serde_json::from_value::<Bindings>(value).is_err());
        let mut unsupported = fixture(vec![]);
        unsupported.schema_version = 2;
        assert!(unsupported.validate().is_err());
    }
}
