pub mod create;
pub mod delete;
pub mod enums;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub mod update_benefits;
pub use self::{
    create::*, delete::*, enums::*, get::*, list::*, newtypes::*, types::*, unions::*, update::*,
    update_benefits::*,
};
pub struct Products<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Products<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a product.
    ///
    /// **Scopes**: `products:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::products::unions::ProductCreate,
    ) -> crate::v2026_04::models::products::ProductsCreate<'a, ClientService> {
        crate::v2026_04::models::products::ProductsCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a product.
    ///
    /// Only products without orders, subscriptions, trials or discounts can be
    /// deleted. Products that are in use can only be archived.
    ///
    /// **Scopes**: `products:write`
    pub fn delete(
        &self,
        id: crate::v2026_04::models::common::ids::ProductId,
    ) -> crate::v2026_04::models::products::ProductsDelete<'a, ClientService> {
        crate::v2026_04::models::products::ProductsDelete {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a product by ID.
    ///
    /// **Scopes**: `products:read` `products:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::ProductId,
    ) -> crate::v2026_04::models::products::ProductsGet<'a, ClientService> {
        crate::v2026_04::models::products::ProductsGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List products.
    ///
    /// **Scopes**: `products:read` `products:write`
    pub fn list(&self) -> crate::v2026_04::models::products::ProductsList<'a, ClientService> {
        crate::v2026_04::models::products::ProductsList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a product.
    ///
    /// **Scopes**: `products:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::ProductId,
        payload: crate::v2026_04::models::products::types::ProductUpdate,
    ) -> crate::v2026_04::models::products::ProductsUpdate<'a, ClientService> {
        crate::v2026_04::models::products::ProductsUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }

    /// Update benefits granted by a product.
    ///
    /// **Scopes**: `products:write`
    pub fn update_benefits(
        &self,
        id: crate::v2026_04::models::common::ids::ProductId,
        payload: crate::v2026_04::models::products::types::ProductBenefitsUpdate,
    ) -> crate::v2026_04::models::products::ProductsUpdateBenefits<'a, ClientService> {
        crate::v2026_04::models::products::ProductsUpdateBenefits {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
