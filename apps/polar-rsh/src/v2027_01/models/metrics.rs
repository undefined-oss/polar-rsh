pub mod enums;
pub mod get;
pub mod limits;
pub mod types;
pub mod unions;
pub use self::{enums::*, get::*, limits::*, types::*, unions::*};
pub struct MetricsResource<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> MetricsResource<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get metrics about your orders and subscriptions.
    ///
    /// Currency values are output in cents.
    ///
    /// **Scopes**: `metrics:read`
    pub fn get(
        &self,
        params: crate::v2027_01::models::metrics::MetricsGetParams,
    ) -> crate::v2027_01::models::metrics::MetricsGet<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params,
        }
    }

    /// Get the interval limits for the metrics endpoint.
    ///
    /// **Scopes**: `metrics:read`
    pub fn limits(
        &self,
    ) -> crate::v2027_01::models::metrics::MetricsLimitsOperation<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsLimitsOperation {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
        }
    }
}
