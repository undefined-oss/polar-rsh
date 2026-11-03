pub mod aliases;
pub mod client_confirm;
pub mod client_get;
pub mod client_update;
pub mod create;
pub mod enums;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    aliases::*, client_confirm::*, client_get::*, client_update::*, create::*, enums::*, get::*,
    list::*, newtypes::*, types::*, unions::*, update::*,
};
pub struct Checkouts<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Checkouts<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Confirm a checkout session by client secret.
    ///
    /// Orders and subscriptions will be processed.
    pub fn client_confirm(
        &self,
        client_secret: String,
        payload: crate::v2026_10::models::checkouts::types::CheckoutConfirmStripe,
    ) -> crate::v2026_10::models::checkouts::CheckoutsClientConfirm<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsClientConfirm {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            client_secret,
            payload,
        }
    }

    /// Get a checkout session by client secret.
    pub fn client_get(
        &self,
        client_secret: String,
    ) -> crate::v2026_10::models::checkouts::CheckoutsClientGet<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsClientGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            client_secret,
        }
    }

    /// Update a checkout session by client secret.
    pub fn client_update(
        &self,
        client_secret: String,
        payload: crate::v2026_10::models::checkouts::types::CheckoutUpdatePublic,
    ) -> crate::v2026_10::models::checkouts::CheckoutsClientUpdate<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsClientUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            client_secret,
            payload,
        }
    }

    /// Create a checkout session.
    ///
    /// **Scopes**: `checkouts:write`
    pub fn create(
        &self,
        payload: crate::v2026_10::models::checkouts::aliases::CheckoutCreate,
    ) -> crate::v2026_10::models::checkouts::CheckoutsCreate<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsCreate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get a checkout session by ID.
    ///
    /// **Scopes**: `checkouts:read` `checkouts:write`
    pub fn get(
        &self,
        id: crate::v2026_10::models::common::ids::CheckoutId,
    ) -> crate::v2026_10::models::checkouts::CheckoutsGet<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List checkout sessions.
    ///
    /// **Scopes**: `checkouts:read` `checkouts:write`
    pub fn list(&self) -> crate::v2026_10::models::checkouts::CheckoutsList<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a checkout session.
    ///
    /// **Scopes**: `checkouts:write`
    pub fn update(
        &self,
        id: crate::v2026_10::models::common::ids::CheckoutId,
        payload: crate::v2026_10::models::checkouts::types::CheckoutUpdate,
    ) -> crate::v2026_10::models::checkouts::CheckoutsUpdate<'a, ClientService> {
        crate::v2026_10::models::checkouts::CheckoutsUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
