pub mod create;
pub mod enums;
pub mod get;
pub mod list_organizations;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, enums::*, get::*, list_organizations::*, newtypes::*, types::*, unions::*, update::*,
};
pub struct Organizations<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Organizations<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create an organization.
    ///
    /// **Scopes**: `organizations:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::organizations::types::OrganizationCreate,
    ) -> crate::v2026_04::models::organizations::OrganizationsCreate<'a, ClientService> {
        crate::v2026_04::models::organizations::OrganizationsCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Get an organization by ID.
    ///
    /// **Scopes**: `organizations:read` `organizations:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::OrganizationId,
    ) -> crate::v2026_04::models::organizations::OrganizationsGet<'a, ClientService> {
        crate::v2026_04::models::organizations::OrganizationsGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List organizations.
    ///
    /// **Scopes**: `organizations:read` `organizations:write`
    pub fn list_organizations(
        &self,
    ) -> crate::v2026_04::models::organizations::OrganizationsListOrganizations<'a, ClientService>
    {
        crate::v2026_04::models::organizations::OrganizationsListOrganizations {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update an organization.
    ///
    /// **Scopes**: `organizations:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::OrganizationId,
        payload: crate::v2026_04::models::organizations::types::OrganizationUpdate,
    ) -> crate::v2026_04::models::organizations::OrganizationsUpdate<'a, ClientService> {
        crate::v2026_04::models::organizations::OrganizationsUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
