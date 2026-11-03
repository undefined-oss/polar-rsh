pub mod create;
pub mod create_external;
pub mod delete;
pub mod delete_external;
pub mod get;
pub mod get_external;
pub mod list;
pub mod list_external;
pub mod update;
pub mod update_external;
pub use self::{
    create::*, create_external::*, delete::*, delete_external::*, get::*, get_external::*, list::*,
    list_external::*, update::*, update_external::*,
};
pub struct CustomersMembers<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomersMembers<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Create a new member for a customer.
    ///
    /// Only B2B customers with the member management feature enabled can add
    /// members. The authenticated user or organization must have access to
    /// the customer's organization.
    ///
    /// **Scopes**: `members:write`
    pub fn create(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
        payload: crate::v2026_04::models::customers::types::MemberCreateFromCustomer,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersCreate<'a, ClientService>
    {
        crate::v2026_04::models::customers::members::CustomersMembersCreate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }

    /// Create a new member for a customer identified by its external ID.
    ///
    /// **Scopes**: `members:write`
    pub fn create_external(
        &self,
        external_id: String,
        payload: crate::v2026_04::models::customers::types::MemberCreateFromCustomer,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersCreateExternal<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customers::members::CustomersMembersCreateExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
            payload,
        }
    }

    /// Delete a member of a customer.
    ///
    /// **Scopes**: `members:write`
    pub fn delete(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
        member_id: crate::v2026_04::models::common::ids::MemberId,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersDelete<'a, ClientService>
    {
        crate::v2026_04::models::customers::members::CustomersMembersDelete {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            member_id,
        }
    }

    /// Delete a member by external ID for a customer identified by its external
    /// ID.
    ///
    /// **Scopes**: `members:write`
    pub fn delete_external(
        &self,
        external_id: String,
        member_external_id: String,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersDeleteExternal<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customers::members::CustomersMembersDeleteExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
            member_external_id,
        }
    }

    /// Get a member of a customer by its ID.
    ///
    /// **Scopes**: `members:read` `members:write`
    pub fn get(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
        member_id: crate::v2026_04::models::common::ids::MemberId,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersGet<'a, ClientService> {
        crate::v2026_04::models::customers::members::CustomersMembersGet {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            member_id,
        }
    }

    /// Get a member by external ID for a customer identified by its external
    /// ID.
    ///
    /// **Scopes**: `members:read` `members:write`
    pub fn get_external(
        &self,
        external_id: String,
        member_external_id: String,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersGetExternal<'a, ClientService>
    {
        crate::v2026_04::models::customers::members::CustomersMembersGetExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
            member_external_id,
        }
    }

    /// List the members of a customer.
    ///
    /// **Scopes**: `members:read` `members:write`
    pub fn list(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersList<'a, ClientService> {
        crate::v2026_04::models::customers::members::CustomersMembersList {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            id,
        }
    }

    /// List the members of a customer identified by its external ID.
    ///
    /// **Scopes**: `members:read` `members:write`
    pub fn list_external(
        &self,
        external_id: String,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersListExternal<'a, ClientService>
    {
        crate::v2026_04::models::customers::members::CustomersMembersListExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
            external_id,
        }
    }

    /// Update a member of a customer.
    ///
    /// Only name, email and role can be updated.
    ///
    /// **Scopes**: `members:write`
    pub fn update(
        &self,
        id: crate::v2026_04::models::common::ids::CustomerId,
        member_id: crate::v2026_04::models::common::ids::MemberId,
        payload: crate::v2026_04::models::customers::types::MemberUpdate,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersUpdate<'a, ClientService>
    {
        crate::v2026_04::models::customers::members::CustomersMembersUpdate {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            member_id,
            payload,
        }
    }

    /// Update a member by external ID for a customer identified by its external
    /// ID.
    ///
    /// **Scopes**: `members:write`
    pub fn update_external(
        &self,
        external_id: String,
        member_external_id: String,
        payload: crate::v2026_04::models::customers::types::MemberUpdate,
    ) -> crate::v2026_04::models::customers::members::CustomersMembersUpdateExternal<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customers::members::CustomersMembersUpdateExternal {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            external_id,
            member_external_id,
            payload,
        }
    }
}
