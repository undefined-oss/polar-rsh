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
pub struct CustomFields<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomFields<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a custom field.
    ///
    /// **Scopes**: `custom_fields:write`
    pub fn create(
        &self,
        payload: crate::v2026_10::models::custom_fields::unions::CustomFieldCreate,
    ) -> crate::v2026_10::models::custom_fields::CustomFieldsCreate<'a, ClientService> {
        crate::v2026_10::models::custom_fields::CustomFieldsCreate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a custom field.
    ///
    /// **Scopes**: `custom_fields:write`
    pub fn delete(
        &self,
        id: crate::v2026_10::models::common::ids::CustomFieldId,
    ) -> crate::v2026_10::models::custom_fields::CustomFieldsDelete<'a, ClientService> {
        crate::v2026_10::models::custom_fields::CustomFieldsDelete {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// Get a custom field by ID.
    ///
    /// **Scopes**: `custom_fields:read` `custom_fields:write`
    pub fn get(
        &self,
        id: crate::v2026_10::models::common::ids::CustomFieldId,
    ) -> crate::v2026_10::models::custom_fields::CustomFieldsGet<'a, ClientService> {
        crate::v2026_10::models::custom_fields::CustomFieldsGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List custom fields.
    ///
    /// **Scopes**: `custom_fields:read` `custom_fields:write`
    pub fn list(
        &self,
    ) -> crate::v2026_10::models::custom_fields::CustomFieldsList<'a, ClientService> {
        crate::v2026_10::models::custom_fields::CustomFieldsList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a custom field.
    ///
    /// **Scopes**: `custom_fields:write`
    pub fn update(
        &self,
        id: crate::v2026_10::models::common::ids::CustomFieldId,
        payload: crate::v2026_10::models::custom_fields::unions::CustomFieldUpdate,
    ) -> crate::v2026_10::models::custom_fields::CustomFieldsUpdate<'a, ClientService> {
        crate::v2026_10::models::custom_fields::CustomFieldsUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
