pub mod get;
pub mod list;
pub use self::{get::*, list::*};
pub struct CustomerPortalCustomerMeters<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalCustomerMeters<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get a meter by ID for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::MeterId,
    ) -> crate::v2027_01::models::customer_portal::customer_meters::CustomerPortalCustomerMetersGet<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::customer_meters::CustomerPortalCustomerMetersGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// List meters of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::customer_portal::customer_meters::CustomerPortalCustomerMetersList<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::customer_meters::CustomerPortalCustomerMetersList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }
}
