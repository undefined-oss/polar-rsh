//! Version-independent fixture and report formats. API bodies and metadata
//! remain arbitrary JSON; each version's SDK validates those at the boundary.

use std::collections::BTreeMap;

use presence_rs::Presence;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub name: String,
    pub actions: Vec<Action>,
    pub responses: Vec<Response>,
    pub expected: Vec<Outcome>,
    pub timeout_ms: Option<u64>,
    pub organization_id: Option<String>,
    #[serde(default)]
    pub known_default_query: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Action {
    #[serde(flatten)]
    pub operation: Operation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", deny_unknown_fields)]
pub enum Operation {
    #[serde(rename = "products.list")]
    ProductsList {
        #[serde(default)]
        query: Box<ProductQuery>,
    },
    #[serde(rename = "customers.get")]
    CustomersGet { id: String },
    #[serde(rename = "customers.get_external")]
    CustomersGetExternal { id: String },
    #[serde(rename = "customers.update")]
    CustomersUpdate { id: String, body: Value },
    #[serde(rename = "customers.delete")]
    CustomersDelete {
        id: String,
        #[serde(default)]
        query: DeleteQuery,
    },
}

impl Operation {
    pub fn returns_customer(&self) -> bool {
        matches!(
            self,
            Self::CustomersGet { .. }
                | Self::CustomersGetExternal { .. }
                | Self::CustomersUpdate { .. }
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductQuery {
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub id: Presence<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub organization_id: Presence<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub query: Presence<String>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub is_archived: Presence<bool>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub is_recurring: Presence<bool>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub benefit_id: Presence<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub visibility: Presence<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub page: Presence<i64>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub limit: Presence<i64>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub sorting: Presence<OneOrMany<String>>,
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub metadata: Presence<BTreeMap<String, Value>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteQuery {
    #[serde(default, skip_serializing_if = "Presence::is_absent")]
    pub anonymize: Presence<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub status: u16,
    #[serde(default)]
    pub body: Value,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    #[serde(default)]
    pub delay_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Ok(Value),
    Error(Failure),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
pub enum Failure {
    Timeout,
    Http {
        status: u16,
        #[serde(default, skip_serializing_if = "Presence::is_absent")]
        body: Presence<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_after: Option<u64>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CapturedRequest {
    pub method: String,
    pub path: Vec<String>,
    pub query: BTreeMap<String, Vec<String>>,
    pub headers: BTreeMap<String, String>,
    pub body: Value,
}

#[derive(Default, Serialize)]
pub struct CaseReport {
    pub name: String,
    pub known_default_query: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_outcomes: Option<Vec<Outcome>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_requests: Option<Vec<CapturedRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typescript_outcomes: Option<Vec<Outcome>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typescript_requests: Option<Vec<CapturedRequest>>,
    pub passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct Report<'a> {
    pub upstream_revision: &'a str,
    pub api_version: &'a str,
    pub cases: &'a [CaseReport],
}

#[derive(Serialize)]
pub struct TypescriptInput<'a> {
    pub scenario: &'a Scenario,
    pub base_url: &'a str,
    pub api_version: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct CustomerSummary {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub created_at: String,
    pub organization_id: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn fixture_actions_round_trip_preserving_absent_null_scalar_and_array_inputs() {
        let fixtures: Vec<Value> = serde_json::from_str(include_str!("../scenarios.json")).unwrap();
        for fixture in fixtures {
            for action in fixture["actions"].as_array().unwrap() {
                let parsed: Action = serde_json::from_value(action.clone()).unwrap();
                assert_eq!(serde_json::to_value(parsed).unwrap(), *action);
            }
            let outcomes: Vec<Outcome> =
                serde_json::from_value(fixture["expected"].clone()).unwrap();
            assert_eq!(serde_json::to_value(outcomes).unwrap(), fixture["expected"]);
        }
    }

    #[test]
    fn malformed_actions_fail_before_running_the_harness() {
        for value in [
            json!({"operation":"unknown"}),
            json!({"operation":"customers.get"}),
            json!({"operation":"customers.get","id":42}),
            json!({"operation":"products.list","query":{"page":"1"}}),
            json!({"operation":"products.list","query":{"is_archived":"false"}}),
            json!({"operation":"products.list","query":{"id":[123]}}),
            json!({"operation":"products.list","query":{"unknown":true}}),
            json!({"operation":"customers.get","id":"test","timeout_ms":-1}),
            json!({"operation":"customers.get","id":"test","access_toke":"typo"}),
        ] {
            assert!(serde_json::from_value::<Action>(value.clone()).is_err(), "accepted {value}");
        }
    }

    #[test]
    fn outcomes_reject_malformed_envelopes_and_preserve_null_bodies() {
        for value in [json!({"ok":null}), json!({"error":{"kind":"http","status":422,"body":null}})]
        {
            let parsed: Outcome = serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), value);
        }
        for value in [
            json!({}),
            json!({"error":{"kind":"unknown"}}),
            json!({"error":{"kind":"http"}}),
            json!({"ok":{},"error":{"kind":"timeout"}}),
        ] {
            assert!(serde_json::from_value::<Outcome>(value).is_err());
        }
    }
}
