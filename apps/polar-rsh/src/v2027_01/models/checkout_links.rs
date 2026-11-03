pub mod create;
pub mod delete;
pub mod enums;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, delete::*, enums::*, get::*, list::*, newtypes::*, types::*, unions::*, update::*,
};
pub struct CheckoutLinks<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CheckoutLinks<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a checkout link.
    ///
    /// **Scopes**: `checkout_links:write`
    pub fn create(
        &self,
        payload: crate::v2027_01::models::checkout_links::unions::CheckoutLinkCreate,
    ) -> crate::v2027_01::models::checkout_links::CheckoutLinksCreate<'a, ClientService> {
        crate::v2027_01::models::checkout_links::CheckoutLinksCreate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a checkout link.
    ///
    /// **Scopes**: `checkout_links:write`
    pub fn delete(
        &self,
        id: crate::v2027_01::models::common::ids::CheckoutLinkId,
    ) -> crate::v2027_01::models::checkout_links::CheckoutLinksDelete<'a, ClientService> {
        crate::v2027_01::models::checkout_links::CheckoutLinksDelete {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a checkout link by ID.
    ///
    /// **Scopes**: `checkout_links:read` `checkout_links:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::CheckoutLinkId,
    ) -> crate::v2027_01::models::checkout_links::CheckoutLinksGet<'a, ClientService> {
        crate::v2027_01::models::checkout_links::CheckoutLinksGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// List checkout links.
    ///
    /// **Scopes**: `checkout_links:read` `checkout_links:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::checkout_links::CheckoutLinksList<'a, ClientService> {
        crate::v2027_01::models::checkout_links::CheckoutLinksList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a checkout link.
    ///
    /// **Scopes**: `checkout_links:write`
    pub fn update(
        &self,
        id: crate::v2027_01::models::common::ids::CheckoutLinkId,
        payload: crate::v2027_01::models::checkout_links::types::CheckoutLinkUpdate,
    ) -> crate::v2027_01::models::checkout_links::CheckoutLinksUpdate<'a, ClientService> {
        crate::v2027_01::models::checkout_links::CheckoutLinksUpdate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
