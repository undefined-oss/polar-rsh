pub mod enums;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod update;
pub use self::{enums::*, list::*, newtypes::*, types::*, update::*};
pub struct EventTypes<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> EventTypes<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// List event types with aggregated statistics.
    ///
    /// **Scopes**: `events:read` `events:write`
    pub fn list(&self) -> crate::v2026_10::models::event_types::EventTypesList<'a, ClientService> {
        crate::v2026_10::models::event_types::EventTypesList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update an event type's label.
    ///
    /// **Scopes**: `events:write`
    pub fn update(
        &self,
        id: crate::v2026_10::models::common::ids::EventTypeId,
        payload: crate::v2026_10::models::event_types::types::EventTypeUpdate,
    ) -> crate::v2026_10::models::event_types::EventTypesUpdate<'a, ClientService> {
        crate::v2026_10::models::event_types::EventTypesUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
