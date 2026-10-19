//! Replace generator-owned query blocks with calls to the shared serde_qs
//! encoder. Binding metadata selects operations; the normalized spec selects
//! wire names.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, ensure};
use quote::{ToTokens, quote};
use syn::{
    FnArg, ImplItem, Item, Pat, Stmt, Type, parse_quote,
    visit_mut::{self, VisitMut},
};

use crate::{
    metadata::Bindings,
    openapi::{Document, Location, Style},
    schema,
};

pub fn generate(spec: &Document, metadata: &Bindings, source: &str) -> Result<String> {
    metadata.validate()?;
    let mut operations = BTreeMap::new();
    let expected: BTreeSet<_> = schema::operations(spec)
        .filter(|op| op.parameters.iter().any(|p| p.location == Location::Query))
        .map(|op| op.operation_id.as_str())
        .collect();
    let mut covered = BTreeSet::new();
    for binding in &metadata.symbols {
        let Some(origin) = &binding.operation else {
            continue;
        };
        let id = origin.source_operation_id.as_str();
        if !binding.path.starts_with("client::HttpClient::")
            || !binding.signature.as_ref().is_some_and(|s| s.asynchronous)
            || !expected.contains(id)
        {
            continue;
        }
        let operation = spec.operation(&origin.source_path, &origin.source_method)?;
        ensure!(operation.operation_id == id, "Query operation binding disagrees with spec");
        let parameters: Vec<_> =
            operation.parameters.iter().filter(|p| p.location == Location::Query).collect();
        for parameter in &parameters {
            let style = parameter.style.unwrap_or(Style::Form);
            ensure!(
                (style == Style::Form && parameter.explode != Some(false))
                    || (style == Style::DeepObject && parameter.name == "metadata"),
                "Unsupported query encoding for {id}: {}",
                parameter.name
            );
        }
        ensure!(
            operations.insert(binding.name.clone(), parameters).is_none(),
            "Duplicate query method"
        );
        covered.insert(id);
    }
    ensure!(covered == expected, "Bindings do not cover all query operations");

    let mut file = syn::parse_file(source)?;
    for item in &mut file.items {
        let Item::Impl(implementation) = item else { continue };
        if !matches!(&*implementation.self_ty, Type::Path(path) if path.path.is_ident("HttpClient"))
        {
            continue;
        }
        for item in &mut implementation.items {
            let ImplItem::Fn(method) = item else { continue };
            let Some(parameters) = operations.remove(&method.sig.ident.to_string()) else {
                continue;
            };
            let mut inserts = Vec::new();
            for parameter in &parameters {
                let wire = parameter.name.as_str();
                let rust_name: String = wire
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
                    .collect();
                let argument = method
                    .sig
                    .inputs
                    .iter()
                    .find_map(|arg| {
                        let FnArg::Typed(arg) = arg else { return None };
                        let Pat::Ident(ident) = &*arg.pat else { return None };
                        (ident.ident.to_string().trim_start_matches("r#") == rust_name)
                            .then_some((arg, &ident.ident))
                    })
                    .with_context(|| format!("No argument for query parameter {wire}"))?;
                let (argument, ident) = argument;
                let typ = argument.ty.to_token_stream().to_string();
                // The upstream string argument accepts AsRef<str>, not
                // Serialize.
                let value = match typ.as_str() {
                    "impl AsRef < str >" => quote!(#ident.as_ref()),
                    "Option < impl AsRef < str > >" => {
                        quote!(#ident.as_ref().map(|value| value.as_ref()))
                    }
                    _ => {
                        ensure!(!typ.contains("impl "), "Unsupported query argument type: {typ}");
                        quote!(#ident)
                    }
                };
                inserts.push(quote!(__polar_query.insert(#wire, &#value)?;));
            }

            let expected_form =
                usize::from(parameters.iter().any(|p| p.style != Some(Style::DeepObject)));
            let expected_deep =
                parameters.iter().filter(|p| p.style == Some(Style::DeepObject)).count();
            let mut form = 0;
            let mut deep = 0;
            let mut retained = Vec::new();
            for mut statement in std::mem::take(&mut method.block.stmts) {
                let mut usage = QueryUsage::default();
                usage.visit_stmt_mut(&mut statement);
                if usage.calls == 0 {
                    retained.push(statement);
                } else {
                    ensure!(
                        usage.calls == 1 && usage.form != usage.deep,
                        "Unexpected generated query block in {}",
                        method.sig.ident
                    );
                    form += usize::from(usage.form);
                    deep += usize::from(usage.deep);
                }
            }
            ensure!(
                form == expected_form && deep == expected_deep,
                "Query blocks disagree with spec for {}: expected {expected_form} \
                 form/{expected_deep} deepObject; found {form}/{deep}",
                method.sig.ident
            );
            let positions: Vec<_> = retained.iter().enumerate().filter_map(|(index, statement)| {
                matches!(statement, Stmt::Local(local) if matches!(&local.pat, Pat::Ident(ident) if ident.ident == "req")).then_some(index)
            }).collect();
            ensure!(positions.len() == 1, "Expected one request builder in {}", method.sig.ident);
            let setup: syn::Block = parse_quote!({
                let mut __polar_query = super::query::Query::default();
                #(#inserts)*
                let request_url = __polar_query.append_to(&request_url)?;
            });
            retained.splice(positions[0]..positions[0], setup.stmts);
            method.block.stmts = retained;
        }
    }
    ensure!(operations.is_empty(), "Missing generated query methods: {:?}", operations.keys());
    let mut remaining = QueryUsage::default();
    remaining.visit_file_mut(&mut file);
    ensure!(remaining.calls == 0, "Unconverted RequestBuilder::query call");
    Ok(prettyplease::unparse(&file))
}

#[derive(Default)]
struct QueryUsage {
    calls: usize,
    form: bool,
    deep: bool,
}

impl VisitMut for QueryUsage {
    fn visit_expr_method_call_mut(&mut self, expr: &mut syn::ExprMethodCall) {
        if expr.method == "query" {
            self.calls += 1;
        }
        visit_mut::visit_expr_method_call_mut(self, expr);
    }

    fn visit_local_mut(&mut self, local: &mut syn::Local) {
        let pattern = match &local.pat {
            Pat::Type(typed) => &*typed.pat,
            pattern => pattern,
        };
        if let Pat::Ident(ident) = pattern {
            self.form |= ident.ident == "query_params";
            self.deep |= ident.ident == "deep_params";
        }
        visit_mut::visit_local_mut(self, local);
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn fixtures() -> (Document, Bindings) {
        let spec = crate::openapi::fixture(
            json!({"/items":{"get":{"operationId":"items:list","parameters":[
                {"in":"query","name":"query","schema":{"type":"string"}},
                {"in":"query","name":"metadata","style":"deepObject","schema":{"type":"object"}}
            ]}}}),
        );
        let mut bindings = crate::metadata::fixture(Vec::new());
        let mut symbol =
            crate::metadata::test_symbol("client::HttpClient::items_list", "items_list");
        symbol.signature = Some(serde_json::from_value(json!({
            "rust":"", "asynchronous":true, "receiver":"& self", "arguments":[], "return_type":"()", "generics":""
        })).unwrap());
        symbol.operation = Some(
            serde_json::from_value(json!({
                "operation_id":"items:list", "source_json_pointer":"/paths/~1items/get",
                "source_operation_id":"items:list", "source_path":"/items", "source_method":"get",
                "webhook":false, "response_statuses":[], "response_excluded_statuses":[],
                "response_media_type":null, "response_kind":"json", "consumption":"buffered"
            }))
            .unwrap(),
        );
        bindings.symbols.push(symbol);
        (spec, bindings)
    }

    const CLIENT: &str = r#"
        impl HttpClient {
            async fn items_list(&self, query: Option<impl AsRef<str>>, metadata: Option<Metadata>) {
                let request_url = "https://example.com/items";
                let mut req = self.http_client.get(request_url);
                { let mut query_params: Vec<(String, String)> = Vec::new(); req = req.query(&query_params); }
                if let Some(value) = metadata { let mut deep_params = Vec::new(); req = req.query(&deep_params); }
                req = req.header("authorization", "token");
                req.send().await
            }
        }
    "#;

    #[test]
    fn replaces_query_blocks_preserving_transport_and_string_arguments() {
        let (spec, bindings) = fixtures();
        let output = generate(&spec, &bindings, CLIENT).unwrap();
        assert!(output.contains("query.as_ref().map(|value| value.as_ref())"));
        assert!(output.contains("__polar_query.insert(\"metadata\", &metadata)?"));
        assert!(output.contains("__polar_query.append_to(&request_url)?"));
        assert!(output.contains("req.header(\"authorization\", \"token\")"));
        assert!(!output.contains(".query("));
        assert!(!output.contains("deep_params"));
        assert!(!output.contains("query_params"));
        let setup = output.find("__polar_query.append_to").unwrap();
        assert!(setup < output.find("self.http_client.get(request_url)").unwrap());
    }

    #[test]
    fn rejects_incomplete_bindings_changed_output_and_unsupported_styles() {
        let (mut spec, bindings) = fixtures();
        assert!(generate(&spec, &crate::metadata::fixture(Vec::new()), CLIENT).is_err());
        assert!(
            generate(&spec, &bindings, &CLIENT.replace("req.query(&deep_params)", "req")).is_err()
        );
        spec.paths["/items"].operations["get"].parameters[0].style = Some(Style::SpaceDelimited);
        assert!(generate(&spec, &bindings, CLIENT).is_err());
    }
}
