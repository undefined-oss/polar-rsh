pub mod create_dashboard;
pub mod delete_dashboard;
pub mod enums;
pub mod export;
pub mod get;
pub mod get_dashboard;
pub mod limits;
pub mod list_dashboards;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update_dashboard;
pub use self::{
    create_dashboard::*, delete_dashboard::*, enums::*, export::*, get::*, get_dashboard::*,
    limits::*, list_dashboards::*, newtypes::*, types::*, unions::*, update_dashboard::*,
};
pub struct MetricsResource<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> MetricsResource<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a user-defined metric dashboard.
    ///
    /// **Scopes**: `metrics:write`
    pub fn create_dashboard(
        &self,
        payload: crate::v2027_01::models::metrics::types::MetricDashboardCreate,
    ) -> crate::v2027_01::models::metrics::MetricsCreateDashboard<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsCreateDashboard {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a user-defined metric dashboard.
    ///
    /// **Scopes**: `metrics:write`
    pub fn delete_dashboard(
        &self,
        id: crate::v2027_01::models::common::ids::DashboardId,
    ) -> crate::v2027_01::models::metrics::MetricsDeleteDashboard<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsDeleteDashboard {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Export metrics as a CSV file.
    ///
    /// **Scopes**: `metrics:read`
    pub fn export(
        &self,
        params: crate::v2027_01::models::metrics::MetricsExportParams,
    ) -> crate::v2027_01::models::metrics::MetricsExport<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsExport {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params,
        }
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

    /// Get a user-defined metric dashboard by ID.
    ///
    /// **Scopes**: `metrics:read`
    pub fn get_dashboard(
        &self,
        id: crate::v2027_01::models::common::ids::DashboardId,
    ) -> crate::v2027_01::models::metrics::MetricsGetDashboard<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsGetDashboard {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
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

    /// List user-defined metric dashboards.
    ///
    /// **Scopes**: `metrics:read`
    pub fn list_dashboards(
        &self,
    ) -> crate::v2027_01::models::metrics::MetricsListDashboards<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsListDashboards {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a user-defined metric dashboard.
    ///
    /// **Scopes**: `metrics:write`
    pub fn update_dashboard(
        &self,
        id: crate::v2027_01::models::common::ids::DashboardId,
        payload: crate::v2027_01::models::metrics::types::MetricDashboardUpdate,
    ) -> crate::v2027_01::models::metrics::MetricsUpdateDashboard<'a, ClientService> {
        crate::v2027_01::models::metrics::MetricsUpdateDashboard {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
