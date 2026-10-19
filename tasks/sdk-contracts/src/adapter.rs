use std::{fmt::Debug, time::Duration};

use anyhow::{Result, bail, ensure};
use presence_rs::Presence;
use serde::{Serialize, de::DeserializeOwned};

use self::sdk::{
    Polar, Request, RequestError, RequestOptions,
    generated::client::ApiOpError,
    resources::{CustomersDeleteParams, ProductsListParams},
};
use crate::{
    Scenario,
    model::{Action, CustomerSummary, Failure, OneOrMany, Operation, Outcome},
};

fn field<T: Clone>(value: &Presence<T>) -> Option<T> {
    match value {
        Presence::Some(value) => Some(value.clone()),
        _ => None,
    }
}

fn array<T: DeserializeOwned>(value: &Presence<OneOrMany<String>>) -> Result<Option<Vec<T>>> {
    let values = match value {
        Presence::Some(OneOrMany::One(value)) => std::slice::from_ref(value),
        Presence::Some(OneOrMany::Many(values)) => values.as_slice(),
        _ => return Ok(None),
    };
    values
        .iter()
        .map(|value| {
            T::deserialize(serde::de::value::StrDeserializer::<serde::de::value::Error>::new(value))
                .map_err(Into::into)
        })
        .collect::<Result<Vec<_>>>()
        .map(Some)
}

async fn send<T: Serialize + Send + 'static, E: Debug + Send + 'static>(
    request: Request<T, E>,
    action: &Action,
) -> Result<Outcome> {
    let options = RequestOptions {
        access_token: action.access_token.clone(),
        timeout: action.timeout_ms.map(Duration::from_millis),
    };
    match request.options(options).await {
        Ok(value) => {
            let mut value = serde_json::to_value(value)?;
            if action.operation.returns_customer() {
                let customer: CustomerSummary = serde_json::from_value(value)?;
                value = serde_json::to_value(customer)?;
            }
            Ok(Outcome::Ok(value))
        }
        Err(RequestError::Timeout(_)) => Ok(Outcome::Error(Failure::Timeout)),
        Err(RequestError::Api(ApiOpError::Api(error))) => {
            let mut retry_after = None;
            let mut body = Presence::Absent;
            if error.status == 429 {
                retry_after = Some(
                    error
                        .headers
                        .get("retry-after")
                        .context("Missing Retry-After")?
                        .to_str()?
                        .parse::<u64>()?,
                );
            } else if error.status < 500 {
                ensure!(
                    error.typed.is_some(),
                    "Expected typed API error, got {:?}",
                    error.parse_error
                );
                body = serde_json::from_str(&error.body)?;
            }
            Ok(Outcome::Error(Failure::Http { status: error.status, body, retry_after }))
        }
        Err(error) => bail!("Unexpected Rust error: {error:?}"),
    }
}
use anyhow::Context;

pub async fn run(scenario: &Scenario, base_url: &str) -> Result<Vec<Outcome>> {
    let mut builder = Polar::builder("contract-token")
        .base_url(base_url.parse()?)
        .http_client(reqwest::Client::builder().no_proxy().build()?)
        .timeout(Duration::from_millis(scenario.timeout_ms.unwrap_or(5000)));
    if let Some(org) = &scenario.organization_id {
        builder = builder.organization_id(org.parse()?);
    }
    let polar = builder.build()?;
    let mut outcomes = Vec::new();
    for action in &scenario.actions {
        let outcome = match &action.operation {
            Operation::ProductsList { query } => {
                // New API versions may add optional filters outside the shared
                // scenarios. Leave those absent while exercising every input
                // specified by this contract driver.
                #[allow(clippy::needless_update)]
                let parameters = ProductsListParams {
                    id: array(&query.id)?,
                    organization_id: array(&query.organization_id)?,
                    query: field(&query.query),
                    is_archived: field(&query.is_archived),
                    is_recurring: field(&query.is_recurring),
                    benefit_id: array(&query.benefit_id)?,
                    visibility: array(&query.visibility)?,
                    page: field(&query.page),
                    limit: field(&query.limit),
                    sorting: array(&query.sorting)?,
                    metadata: field(&query.metadata)
                        .map(|value| serde_json::from_value(serde_json::to_value(value)?))
                        .transpose()?,
                    ..Default::default()
                };
                send(polar.products().list(parameters), action).await?
            }
            Operation::CustomersGet { id } => {
                send(polar.customers().get(id.parse()?), action).await?
            }
            Operation::CustomersGetExternal { id } => {
                send(polar.customers().get_external(id.clone()), action).await?
            }
            Operation::CustomersUpdate { id, body } => {
                send(
                    polar.customers().update(id.parse()?, serde_json::from_value(body.clone())?),
                    action,
                )
                .await?
            }
            Operation::CustomersDelete { id, query } => {
                send(
                    polar.customers().delete(
                        id.parse()?,
                        CustomersDeleteParams { anonymize: field(&query.anonymize) },
                    ),
                    action,
                )
                .await?
            }
        };
        outcomes.push(outcome);
    }
    Ok(outcomes)
}
