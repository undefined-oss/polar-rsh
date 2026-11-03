pub mod add_member;
pub mod list_members;
pub mod remove_member;
pub mod update_member;
pub use self::{add_member::*, list_members::*, remove_member::*, update_member::*};
pub struct CustomerPortalMembers<'a, ClientService> {
    client: &'a crate::v2026_04::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalMembers<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_04::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Add a new member to the customer's team.
    ///
    /// Only available to owners and billing managers of team customers.
    ///
    /// Rules:
    /// - Cannot add a member with the owner role (there must be exactly one
    ///   owner)
    /// - If a member with this email already exists, the existing member is
    ///   returned
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn add_member(
        &self,
        payload: crate::v2026_04::models::members::types::CustomerPortalMemberCreate,
    ) -> crate::v2026_04::models::customer_portal::members::CustomerPortalMembersAddMember<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customer_portal::members::CustomerPortalMembersAddMember {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            payload,
        }
    }

    /// List all members of the customer's team.
    ///
    /// Only available to owners and billing managers of team customers.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn list_members(
        &self,
    ) -> crate::v2026_04::models::customer_portal::members::CustomerPortalMembersListMembers<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customer_portal::members::CustomerPortalMembersListMembers {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Remove a member from the team.
    ///
    /// Only available to owners and billing managers of team customers.
    ///
    /// Rules:
    /// - Cannot remove yourself
    /// - Cannot remove the only owner
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn remove_member(
        &self,
        id: crate::v2026_04::models::common::ids::MemberId,
    ) -> crate::v2026_04::models::customer_portal::members::CustomerPortalMembersRemoveMember<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customer_portal::members::CustomerPortalMembersRemoveMember {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
        }
    }

    /// Update a member's name or role.
    ///
    /// Only available to owners and billing managers of team customers.
    ///
    /// Rules:
    /// - Cannot modify your own role (to prevent self-demotion)
    /// - Customer must have exactly one owner at all times
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn update_member(
        &self,
        id: crate::v2026_04::models::common::ids::MemberId,
        payload: crate::v2026_04::models::members::types::CustomerPortalMemberUpdate,
    ) -> crate::v2026_04::models::customer_portal::members::CustomerPortalMembersUpdateMember<
        'a,
        ClientService,
    > {
        crate::v2026_04::models::customer_portal::members::CustomerPortalMembersUpdateMember {
            client: self.client,
            options: crate::v2026_04::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
