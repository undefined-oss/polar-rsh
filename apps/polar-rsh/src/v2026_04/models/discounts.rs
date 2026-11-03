pub mod create;
pub mod delete;
pub mod enums;
pub mod get;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, delete::*, enums::*, get::*, list::*, newtypes::*, types::*, unions::*, update::*,
};
pub struct Discounts<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Discounts<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a discount.
    ///
    /// **Scopes**: `discounts:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::discounts::unions::DiscountCreate,
    ) -> crate::v2026_04::models::discounts::DiscountsCreate<'a, ClientService> {
        crate::v2026_04::models::discounts::DiscountsCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a discount.
    ///
    /// **Scopes**: `discounts:write`
    pub fn delete(
        &self,
        id: crate::v2026_04::models::common::ids::DiscountId,
    ) -> crate::v2026_04::models::discounts::DiscountsDelete<'a, ClientService> {
        crate::v2026_04::models::discounts::DiscountsDelete {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a discount by ID.
    ///
    /// **Scopes**: `discounts:read` `discounts:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::DiscountId,
    ) -> crate::v2026_04::models::discounts::DiscountsGet<'a, ClientService> {
        crate::v2026_04::models::discounts::DiscountsGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List discounts.
    ///
    /// **Scopes**: `discounts:read` `discounts:write`
    pub fn list(&self) -> crate::v2026_04::models::discounts::DiscountsList<'a, ClientService> {
        crate::v2026_04::models::discounts::DiscountsList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a discount.
    ///
    /// **Scopes**: `discounts:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::DiscountId,
        payload: crate::v2026_04::models::discounts::types::DiscountUpdate,
    ) -> crate::v2026_04::models::discounts::DiscountsUpdate<'a, ClientService> {
        crate::v2026_04::models::discounts::DiscountsUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
