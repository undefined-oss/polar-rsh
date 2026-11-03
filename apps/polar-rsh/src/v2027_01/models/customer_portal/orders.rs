pub mod confirm_retry_payment;
pub mod generate_invoice;
pub mod get;
pub mod get_payment_status;
pub mod invoice;
pub mod list;
pub mod receipt;
pub mod update;
pub use self::{
    confirm_retry_payment::*, generate_invoice::*, get::*, get_payment_status::*, invoice::*,
    list::*, receipt::*, update::*,
};
pub struct CustomerPortalOrders<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalOrders<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Confirm a retry payment using a Stripe confirmation token.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn confirm_retry_payment(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
        payload: crate::v2027_01::models::orders::types::CustomerOrderConfirmPayment,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersConfirmRetryPayment<
        'a,
        ClientService,
    >{
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersConfirmRetryPayment {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }

    /// Trigger generation of an order's invoice.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn generate_invoice(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersGenerateInvoice<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersGenerateInvoice {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Get an order by ID for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersGet<'a, ClientService>
    {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersGet {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Get the current payment status for an order.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get_payment_status(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersGetPaymentStatus<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersGetPaymentStatus {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Get an order's invoice data.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn invoice(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersInvoice<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersInvoice {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// List orders of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersList<'a, ClientService>
    {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Get a presigned URL to download an order's receipt PDF.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn receipt(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersReceipt<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersReceipt {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }

    /// Update an order for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn update(
        &self,
        id: crate::v2027_01::models::common::ids::OrderId,
        payload: crate::v2027_01::models::orders::types::CustomerOrderUpdate,
    ) -> crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersUpdate<
        'a,
        ClientService,
    > {
        crate::v2027_01::models::customer_portal::orders::CustomerPortalOrdersUpdate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
