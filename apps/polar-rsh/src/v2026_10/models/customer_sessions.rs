pub mod create;
pub mod newtypes;
pub mod types;
pub mod unions;
pub use self::{create::*, newtypes::*, types::*, unions::*};
pub struct CustomerSessions<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerSessions<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a customer session.
    ///
    /// For organizations with `member_model_enabled`, this will automatically
    /// create a member session for the owner member of the customer.
    ///
    /// **Scopes**: `customer_sessions:write`
    pub fn create(
        &self,
        payload: crate::v2026_10::models::customer_sessions::unions::CustomerSessionsCreateRequestUnion,
    ) -> crate::v2026_10::models::customer_sessions::CustomerSessionsCreate<'a, ClientService> {
        crate::v2026_10::models::customer_sessions::CustomerSessionsCreate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }
}
