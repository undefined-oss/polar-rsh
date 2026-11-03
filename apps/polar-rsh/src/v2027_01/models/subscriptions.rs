pub mod create;
pub mod enums;
pub mod export;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod revoke;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, enums::*, export::*, get::*, list::*, newtypes::*, revoke::*, types::*, unions::*,
    update::*,
};
pub struct Subscriptions<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Subscriptions<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a subscription programmatically.
    ///
    /// This endpoint only allows to create subscription on free products.
    /// For paid products, use the checkout flow.
    ///
    /// No initial order will be created and no confirmation email will be sent.
    ///
    /// **Scopes**: `subscriptions:write`
    pub fn create(
        &self,
        payload: crate::v2027_01::models::subscriptions::unions::SubscriptionsCreateRequestUnion,
    ) -> crate::v2027_01::models::subscriptions::SubscriptionsCreate<'a, ClientService> {
        crate::v2027_01::models::subscriptions::SubscriptionsCreate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Export subscriptions as a CSV file.
    ///
    /// **Scopes**: `subscriptions:read` `subscriptions:write`
    pub fn export(
        &self,
    ) -> crate::v2027_01::models::subscriptions::SubscriptionsExport<'a, ClientService> {
        crate::v2027_01::models::subscriptions::SubscriptionsExport {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Get a subscription by ID.
    ///
    /// **Scopes**: `subscriptions:read` `subscriptions:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::SubscriptionId,
    ) -> crate::v2027_01::models::subscriptions::SubscriptionsGet<'a, ClientService> {
        crate::v2027_01::models::subscriptions::SubscriptionsGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// List subscriptions.
    ///
    /// **Scopes**: `subscriptions:read` `subscriptions:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::subscriptions::SubscriptionsList<'a, ClientService> {
        crate::v2027_01::models::subscriptions::SubscriptionsList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Revoke a subscription, i.e cancel immediately.
    ///
    /// **Scopes**: `subscriptions:write`
    pub fn revoke(
        &self,
        id: crate::v2027_01::models::common::ids::SubscriptionId,
    ) -> crate::v2027_01::models::subscriptions::SubscriptionsRevoke<'a, ClientService> {
        crate::v2027_01::models::subscriptions::SubscriptionsRevoke {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Update a subscription.
    ///
    /// **Scopes**: `subscriptions:write`
    pub fn update(
        &self,
        id: crate::v2027_01::models::common::ids::SubscriptionId,
        payload: crate::v2027_01::models::subscriptions::unions::SubscriptionUpdate,
    ) -> crate::v2027_01::models::subscriptions::SubscriptionsUpdate<'a, ClientService> {
        crate::v2027_01::models::subscriptions::SubscriptionsUpdate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
