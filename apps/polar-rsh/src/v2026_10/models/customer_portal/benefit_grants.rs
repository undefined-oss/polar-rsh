pub mod get;
pub mod list;
pub mod update;
pub use self::{get::*, list::*, update::*};
pub struct CustomerPortalBenefitGrants<'a, ClientService> {
    client: &'a crate::v2026_10::client::PolarClient<ClientService>,
}
impl<'a, ClientService> CustomerPortalBenefitGrants<'a, ClientService> {
    pub(crate) fn new(client: &'a crate::v2026_10::client::PolarClient<ClientService>) -> Self {
        Self { client }
    }

    /// Get a benefit grant by ID for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn get(
        &self,
        id: crate::v2026_10::models::common::ids::BenefitGrantId,
    ) -> crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrantsGet<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrantsGet {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
        }
    }

    /// List benefits grants of the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:read` `customer_portal:write`
    pub fn list(
        &self,
    ) -> crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrantsList<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrantsList {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            params: None,
        }
    }

    /// Update a benefit grant for the authenticated customer.
    ///
    /// **Scopes**: `customer_portal:write`
    pub fn update(
        &self,
        id: crate::v2026_10::models::common::ids::BenefitGrantId,
        payload: crate::v2026_10::models::benefit_grants::unions::CustomerBenefitGrantUpdate,
    ) -> crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrantsUpdate<
        'a,
        ClientService,
    >{
        crate::v2026_10::models::customer_portal::benefit_grants::CustomerPortalBenefitGrantsUpdate {
            client: self.client,
            options: crate::v2026_10::client::RequestOptions::default(),
            id,
            payload,
        }
    }
}
