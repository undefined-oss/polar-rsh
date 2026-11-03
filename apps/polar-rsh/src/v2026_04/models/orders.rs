pub mod create;
pub mod enums;
pub mod export;
pub mod finalize;
pub mod generate_invoice;
pub mod get;
pub mod invoice;
pub mod list;
pub mod newtypes;
pub mod receipt;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, enums::*, export::*, finalize::*, generate_invoice::*, get::*, invoice::*, list::*,
    newtypes::*, receipt::*, types::*, unions::*, update::*,
};
pub struct Orders<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Orders<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a draft order for an off-session charge against a saved payment
    /// method. The order is created with `status=draft` and no invoice number;
    /// call `POST /v1/orders/{id}/finalize` to attempt the charge.
    ///
    /// The organization must have the `off_session_charges_enabled` feature
    /// flag.
    ///
    /// **Scopes**: `orders:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::orders::types::OrderCreate,
    ) -> crate::v2026_04::models::orders::OrdersCreate<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Export orders as a CSV file.
    ///
    /// **Scopes**: `orders:read`
    pub fn export(&self) -> crate::v2026_04::models::orders::OrdersExport<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersExport {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Finalize a draft order and synchronously attempt an off-session charge.
    ///
    /// On success, the order transitions to `paid` and benefit grants fire
    /// before the response returns. On failure (decline, missing payment
    /// method, SCA challenge), the order stays in `draft` and a 4xx error
    /// is returned.
    ///
    /// The request fails with 412 if the order is not in `draft` status.
    ///
    /// **Scopes**: `orders:write`
    pub fn finalize(
        &self,
        id: crate::v2026_04::models::common::ids::OrderId,
    ) -> crate::v2026_04::models::orders::OrdersFinalize<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersFinalize {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload: None,
            id,
        }
    }

    /// Trigger generation of an order's invoice.
    ///
    /// **Scopes**: `orders:read`
    pub fn generate_invoice(
        &self,
        id: crate::v2026_04::models::common::ids::OrderId,
    ) -> crate::v2026_04::models::orders::OrdersGenerateInvoice<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersGenerateInvoice {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get an order by ID.
    ///
    /// **Scopes**: `orders:read`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::OrderId,
    ) -> crate::v2026_04::models::orders::OrdersGet<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get an order's invoice data.
    ///
    /// **Scopes**: `orders:read`
    pub fn invoice(
        &self,
        id: crate::v2026_04::models::common::ids::OrderId,
    ) -> crate::v2026_04::models::orders::OrdersInvoice<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersInvoice {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List orders.
    ///
    /// **Scopes**: `orders:read`
    pub fn list(&self) -> crate::v2026_04::models::orders::OrdersList<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Get a presigned URL to download an order's receipt PDF.
    ///
    /// **Scopes**: `orders:read`
    pub fn receipt(
        &self,
        id: crate::v2026_04::models::common::ids::OrderId,
    ) -> crate::v2026_04::models::orders::OrdersReceipt<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersReceipt {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Update an order.
    ///
    /// **Scopes**: `orders:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::OrderId,
        payload: crate::v2026_04::models::orders::types::OrderUpdate,
    ) -> crate::v2026_04::models::orders::OrdersUpdate<'a, ClientService> {
        crate::v2026_04::models::orders::OrdersUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
