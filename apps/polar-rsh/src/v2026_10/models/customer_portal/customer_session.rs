pub mod get_authenticated_user;
pub mod introspect;
pub use self::{get_authenticated_user::*, introspect::*};
pub struct CustomerPortalCustomerSession<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalCustomerSession<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get information about the currently authenticated portal user.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get_authenticated_user(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customer_session::CustomerPortalCustomerSessionGetAuthenticatedUser<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customer_session::CustomerPortalCustomerSessionGetAuthenticatedUser {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
        }
    }

    /// Introspect the current session and return its information.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn introspect(
        &self,
    ) -> crate::v2026_10::models::customer_portal::customer_session::CustomerPortalCustomerSessionIntrospect<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::customer_session::CustomerPortalCustomerSessionIntrospect {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
        }
    }
}
