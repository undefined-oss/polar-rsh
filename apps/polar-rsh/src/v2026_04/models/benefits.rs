pub mod create;
pub mod delete;
pub mod enums;
pub mod files;
pub mod get;
pub mod grants;
pub mod list;
pub mod newtypes;
pub mod types;
pub mod unions;
pub mod update;
pub use self::{
    create::*, delete::*, enums::*, files::*, get::*, grants::*, list::*, newtypes::*, types::*,
    unions::*, update::*,
};
pub struct Benefits<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> Benefits<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a benefit, for example license keys, file downloads or Discord
    /// access.
    ///
    /// **Scopes**: `benefits:write`
    pub fn create(
        &self,
        payload: crate::v2026_04::models::benefits::unions::BenefitCreate,
    ) -> crate::v2026_04::models::benefits::BenefitsCreate<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// Delete a benefit.
    ///
    /// > [!WARNING]
    /// > Every grants associated with the benefit will be revoked.
    /// > Users will lose access to the benefit.
    ///
    /// **Scopes**: `benefits:write`
    pub fn delete(
        &self,
        id: crate::v2026_04::models::common::ids::BenefitId,
    ) -> crate::v2026_04::models::benefits::BenefitsDelete<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsDelete {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List the downloadable files for a benefit with their download
    /// statistics.
    ///
    /// **Scopes**: `benefits:read` `benefits:write`
    pub fn files(
        &self,
        id: crate::v2026_04::models::common::ids::BenefitId,
    ) -> crate::v2026_04::models::benefits::BenefitsFiles<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsFiles {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            id,
        }
    }

    /// Get a benefit by ID.
    ///
    /// **Scopes**: `benefits:read` `benefits:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::BenefitId,
    ) -> crate::v2026_04::models::benefits::BenefitsGet<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// List the individual grants for a benefit.
    ///
    /// It's especially useful to check if a user has been granted a benefit.
    ///
    /// **Scopes**: `benefits:read` `benefits:write`
    pub fn grants(
        &self,
        id: crate::v2026_04::models::common::ids::BenefitId,
    ) -> crate::v2026_04::models::benefits::BenefitsGrants<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsGrants {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            id,
        }
    }

    /// List benefits.
    ///
    /// **Scopes**: `benefits:read` `benefits:write`
    pub fn list(&self) -> crate::v2026_04::models::benefits::BenefitsList<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a benefit.
    ///
    /// **Scopes**: `benefits:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::BenefitId,
        payload: crate::v2026_04::models::benefits::unions::BenefitsUpdateRequestUnion,
    ) -> crate::v2026_04::models::benefits::BenefitsUpdate<'a, ClientService> {
        crate::v2026_04::models::benefits::BenefitsUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
