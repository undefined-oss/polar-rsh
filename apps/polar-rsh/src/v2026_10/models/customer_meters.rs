pub mod enums;
pub mod get;
pub mod list;
pub mod types;
pub use self::{enums::*, get::*, list::*, types::*};
pub struct CustomerMeters<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerMeters<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get a customer meter by ID.
    ///
    /// **Scopes**: `customer_meters:read`
    pub fn get(
        &self,
        id: crate::v2026_10::models::common::ids::CustomerMeterId,
    ) -> crate::v2026_10::models::customer_meters::CustomerMetersGet<'a, ClientService> {
        crate::v2026_10::models::customer_meters::CustomerMetersGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List customer meters.
    ///
    /// **Scopes**: `customer_meters:read`
    pub fn list(
        &self,
    ) -> crate::v2026_10::models::customer_meters::CustomerMetersList<'a, ClientService> {
        crate::v2026_10::models::customer_meters::CustomerMetersList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }
}
