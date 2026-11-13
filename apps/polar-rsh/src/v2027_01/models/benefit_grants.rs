pub mod create;
pub mod enums;
pub mod list;
pub mod newtypes;
pub mod revoke;
pub mod types;
pub mod unions;
pub use self::{create::*, enums::*, list::*, newtypes::*, revoke::*, types::*, unions::*};
pub struct BenefitGrants<'a, ClientService> {
    client: &'a crate::v2027_01::client::PolarClient<ClientService>,
}
impl<'a, ClientService> BenefitGrants<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2027_01::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Manually grant a benefit to a customer or one of their members.
    ///
    /// Use either Polar IDs for every reference or external IDs for every
    /// reference. Fulfillment is asynchronous: the returned grant may not
    /// yet be granted. Meter-credit benefits cannot be granted manually.
    ///
    /// **Scopes**: `benefits:write`
    pub fn create(
        &self,
        payload: crate::v2027_01::models::benefit_grants::unions::BenefitGrantsCreateRequestUnion,
    ) -> crate::v2027_01::models::benefit_grants::BenefitGrantsCreate<'a, ClientService> {
        crate::v2027_01::models::benefit_grants::BenefitGrantsCreate {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            payload,
        }
    }

    /// List benefit grants across all benefits accessible to the authenticated
    /// subject.
    ///
    /// **Scopes**: `benefits:read` `benefits:write`
    pub fn list(
        &self,
    ) -> crate::v2027_01::models::benefit_grants::BenefitGrantsList<'a, ClientService> {
        crate::v2027_01::models::benefit_grants::BenefitGrantsList {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Permanently revoke a benefit grant, including grants from purchases.
    ///
    /// Revocation is asynchronous. Once manually revoked, this grant cannot be
    /// reactivated by renewal or fulfillment retries. Other grants may still
    /// provide access to the benefit. Repeating this request is safe.
    ///
    /// **Scopes**: `benefits:write`
    pub fn revoke(
        &self,
        id: crate::v2027_01::models::common::ids::BenefitGrantId,
    ) -> crate::v2027_01::models::benefit_grants::BenefitGrantsRevoke<'a, ClientService> {
        crate::v2027_01::models::benefit_grants::BenefitGrantsRevoke {
            client: self.client,
            options: crate::v2027_01::client::RequestOptions::default(),
            id,
        }
    }
}
