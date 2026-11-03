pub mod enums;
pub mod get;
pub mod ingest;
pub mod list;
pub mod list_names;
pub mod newtypes;
pub mod types;
pub mod unions;
pub use self::{
    enums::*, get::*, ingest::*, list::*, list_names::*, newtypes::*, types::*, unions::*,
};
pub struct Events<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Events<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get an event by ID.
    ///
    /// **Scopes**: `events:read` `events:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::EventId,
    ) -> crate::v2027_01::models::events::EventsGet<'a, ClientService> {
        crate::v2027_01::models::events::EventsGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Ingest batch of events.
    ///
    /// **Scopes**: `events:write`
    pub fn ingest(
        &self,
        payload: crate::v2027_01::models::events::types::EventsIngest,
    ) -> crate::v2027_01::models::events::EventsIngestOperation<'a, ClientService> {
        crate::v2027_01::models::events::EventsIngestOperation {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// List events.
    ///
    /// **Scopes**: `events:read` `events:write`
    pub fn list(&self) -> crate::v2027_01::models::events::EventsList<'a, ClientService> {
        crate::v2027_01::models::events::EventsList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// List event names.
    ///
    /// **Scopes**: `events:read` `events:write`
    pub fn list_names(
        &self,
    ) -> crate::v2027_01::models::events::EventsListNames<'a, ClientService> {
        crate::v2027_01::models::events::EventsListNames {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }
}
