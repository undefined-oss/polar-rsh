pub mod create;
pub mod enums;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub use self::{create::*, enums::*, list::*, newtypes::*, types::*, unions::*};
pub struct Refunds<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Refunds<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a refund.
    ///
    /// **Scopes**: `refunds:write`
    pub fn create(
        &self,
        payload: crate::v2026_10::models::refunds::types::RefundCreate,
    ) -> crate::v2026_10::models::refunds::RefundsCreate<'a, ClientService> {
        crate::v2026_10::models::refunds::RefundsCreate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// List refunds.
    ///
    /// **Scopes**: `refunds:read` `refunds:write`
    pub fn list(&self) -> crate::v2026_10::models::refunds::RefundsList<'a, ClientService> {
        crate::v2026_10::models::refunds::RefundsList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }
}
