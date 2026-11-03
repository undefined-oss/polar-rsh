pub mod enums;
pub mod get;
pub mod list;
pub mod types;
pub mod unions;
pub use self::{enums::*, get::*, list::*, types::*, unions::*};
pub struct Payments<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Payments<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get a payment by ID.
    ///
    /// **Scopes**: `payments:read`
    pub fn get(
        &self,
        id: crate::v2026_10::models::common::ids::PaymentId,
    ) -> crate::v2026_10::models::payments::PaymentsGet<'a, ClientService> {
        crate::v2026_10::models::payments::PaymentsGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List payments.
    ///
    /// **Scopes**: `payments:read`
    pub fn list(&self) -> crate::v2026_10::models::payments::PaymentsList<'a, ClientService> {
        crate::v2026_10::models::payments::PaymentsList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }
}
