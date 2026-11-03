pub mod cancel;
pub mod get;
pub mod list;
pub mod update;
pub use self::{cancel::*, get::*, list::*, update::*};
pub struct CustomerPortalSubscriptions<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalSubscriptions<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Cancel a subscription of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn cancel(
        &self,
        id: crate::v2027_01::models::common::ids::SubscriptionId,
    ) -> crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsCancel<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsCancel {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a subscription for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::SubscriptionId,
    ) -> crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsGet<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// List subscriptions of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsList<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a subscription of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn update(
        &self,
        id: crate::v2027_01::models::common::ids::SubscriptionId,
        payload: crate::v2027_01::models::subscriptions::unions::CustomerSubscriptionUpdate,
    ) -> crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsUpdate<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::subscriptions::CustomerPortalSubscriptionsUpdate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
