pub mod create;
pub mod enums;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod quantities;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, enums::*, get::*, list::*, newtypes::*, quantities::*, types::*, unions::*,
    update::*,
};
pub struct Meters<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Meters<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a meter.
    ///
    /// **Scopes**: `meters:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::meters::types::MeterCreate,
    ) -> crate::v2026_04::models::meters::MetersCreate<'a, ClientService> {
        crate::v2026_04::models::meters::MetersCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get a meter by ID.
    ///
    /// **Scopes**: `meters:read` `meters:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::MeterId,
    ) -> crate::v2026_04::models::meters::MetersGet<'a, ClientService> {
        crate::v2026_04::models::meters::MetersGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List meters.
    ///
    /// **Scopes**: `meters:read` `meters:write`
    pub fn list(&self) -> crate::v2026_04::models::meters::MetersList<'a, ClientService> {
        crate::v2026_04::models::meters::MetersList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Get quantities of a meter over a time period.
    ///
    /// **Scopes**: `meters:read` `meters:write`
    pub fn quantities(
        &self,
        id: crate::v2026_04::models::common::ids::MeterId,
        params: crate::v2026_04::models::meters::MetersQuantitiesParams,
    ) -> crate::v2026_04::models::meters::MetersQuantities<'a, ClientService> {
        crate::v2026_04::models::meters::MetersQuantities {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            params,
        }
    }

    /// Update a meter.
    ///
    /// **Scopes**: `meters:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::MeterId,
        payload: crate::v2026_04::models::meters::types::MeterUpdate,
    ) -> crate::v2026_04::models::meters::MetersUpdate<'a, ClientService> {
        crate::v2026_04::models::meters::MetersUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
